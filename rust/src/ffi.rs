use crate::circuit::FFICircuit;
use crate::proof::SerializableProof;
use bellman::groth16;
use bls12_381::Bls12;
use rand::rngs::OsRng;
use std::ptr;

#[no_mangle]
pub unsafe extern "C" fn create_proof(preimage: *const u8, preimage_len: usize) -> *mut u8 {
    let preimage = std::slice::from_raw_parts(preimage, preimage_len);
    let mut fixed_preimage = [0u8; 80];
    fixed_preimage[..preimage.len()].copy_from_slice(&preimage[..preimage.len().min(80)]);

    let params = {
        let c = FFICircuit { preimage: None };
        groth16::generate_random_parameters::<Bls12, _, _>(c, &mut OsRng).unwrap()
    };

    let c = FFICircuit {
        preimage: Some(fixed_preimage),
    };
    let proof = groth16::create_random_proof(c, &params, &mut OsRng).unwrap();

    let proof = SerializableProof(proof);
    let proof_bytes = bincode::serialize(&proof).unwrap();
    let len = proof_bytes.len();
    let ptr = libc::malloc(len) as *mut u8;
    ptr::copy_nonoverlapping(proof_bytes.as_ptr(), ptr, len);

    ptr
}

#[no_mangle]
pub unsafe extern "C" fn verify_proof(proof: *const u8, proof_len: usize) -> bool {
    let proof = std::slice::from_raw_parts(proof, proof_len);

    let proof: SerializableProof = match bincode::deserialize(proof) {
        Ok(proof) => proof,
        Err(_) => return false,
    };

    let params = {
        let c = FFICircuit { preimage: None };
        match groth16::generate_random_parameters::<Bls12, _, _>(c, &mut OsRng) {
            Ok(params) => params,
            Err(_) => return false,
        }
    };

    let pvk = groth16::prepare_verifying_key(&params.vk);

    groth16::verify_proof(&pvk, &proof.0, &[]).is_ok()
}
