//! The pairing-friendly curve the tests run on: BLS12-381, or BN254 with the
//! `bn254` feature.

#[cfg(not(feature = "bn254"))]
pub use ark_bls12_381::{Bls12_381 as Curve, Fr, G1Affine, G1Projective, G2Affine, G2Projective};
#[cfg(feature = "bn254")]
pub use ark_bn254::{Bn254 as Curve, Fr, G1Affine, G1Projective, G2Affine, G2Projective};
