use bellman::groth16::Proof;
use bls12_381::{Bls12, G1Affine, G2Affine};
use serde::{
    de::{self, Deserializer, SeqAccess, Visitor},
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};
use serde_bytes::{ByteBuf, Bytes};
use std::fmt;

#[derive(Debug)]
pub struct SerializableProof(pub Proof<Bls12>);

impl Serialize for SerializableProof {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SerializableProof", 3)?;
        state.serialize_field("a", &Bytes::new(&self.0.a.to_compressed()))?;
        state.serialize_field("b", &Bytes::new(&self.0.b.to_compressed()))?;
        state.serialize_field("c", &Bytes::new(&self.0.c.to_compressed()))?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for SerializableProof {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProofVisitor;

        impl<'de> Visitor<'de> for ProofVisitor {
            type Value = SerializableProof;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct SerializableProof")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let a_compressed: ByteBuf = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let b_compressed: ByteBuf = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let c_compressed: ByteBuf = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;

                let a_compressed: &[u8; 48] = a_compressed.as_ref().try_into().unwrap();
                let b_compressed: &[u8; 96] = b_compressed.as_ref().try_into().unwrap();
                let c_compressed: &[u8; 48] = c_compressed.as_ref().try_into().unwrap();

                let a = G1Affine::from_compressed(a_compressed).unwrap();
                let b = G2Affine::from_compressed(b_compressed).unwrap();
                let c = G1Affine::from_compressed(c_compressed).unwrap();

                Ok(SerializableProof(Proof { a, b, c }))
            }
        }

        const FIELDS: &[&str; 3] = &["a", "b", "c"];
        deserializer.deserialize_struct("SerializableProof", FIELDS, ProofVisitor)
    }
}
