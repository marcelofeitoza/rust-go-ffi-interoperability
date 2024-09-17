use bellman::{
    gadgets::{
        boolean::{AllocatedBit, Boolean},
        multipack,
        sha256::sha256,
    },
    Circuit, ConstraintSystem, SynthesisError,
};
use ff::PrimeField;

pub struct FFICircuit {
    pub preimage: Option<[u8; 80]>,
}

impl<Scalar: PrimeField> Circuit<Scalar> for FFICircuit {
    fn synthesize<CS: ConstraintSystem<Scalar>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let bit_values = if let Some(preimage) = self.preimage {
            preimage
                .into_iter()
                .flat_map(|byte| (0..8).map(move |i| (byte >> i) & 1u8 == 1u8))
                .map(Some)
                .collect()
        } else {
            vec![None; 80 * 8]
        };
        assert_eq!(bit_values.len(), 80 * 8);

        let preimage_bits = bit_values
            .into_iter()
            .enumerate()
            .map(|(i, b)| AllocatedBit::alloc(cs.namespace(|| format!("preimage bit {}", i)), b))
            .map(|b| b.map(Boolean::from))
            .collect::<Result<Vec<_>, _>>()?;

        let hash = sha256d(cs.namespace(|| "SHA-256d(preimage)"), &preimage_bits)?;

        multipack::pack_into_inputs(cs.namespace(|| "pack hash"), &hash)
    }
}

fn sha256d<Scalar: PrimeField, CS: ConstraintSystem<Scalar>>(
    mut cs: CS,
    data: &[Boolean],
) -> Result<Vec<Boolean>, SynthesisError> {
    let input: Vec<_> = data
        .chunks(8)
        .flat_map(|c| c.iter().rev())
        .cloned()
        .collect();
    let mid = sha256(cs.namespace(|| "SHA-256(input)"), &input)?;
    let res = sha256(cs.namespace(|| "SHA-256(mid)"), &mid)?;

    Ok(res
        .chunks(8)
        .flat_map(|c| c.iter().rev())
        .cloned()
        .collect())
}
