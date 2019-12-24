use elliptic_curve::{
    p256::AffinePoint,
    weierstrass::{
        curve::NistP256, CompressedCurvePoint, PublicKey as EcPublicKey, UncompressedCurvePoint,
    },
};
use std::fmt;

/// Wrapper around a compressed secp256r1 curve point.
#[derive(Clone)]
pub struct PublicKey(CompressedCurvePoint<NistP256>);

impl fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PublicKey({:?})", self.as_bytes())
    }
}

impl PublicKey {
    /// Attempts to parse a valid secp256r1 public key from a byte slice.
    ///
    /// The slice must contain an SEC-1-encoded public key.
    pub(crate) fn from_bytes(bytes: &[u8]) -> Option<Self> {
        Self::from_pubkey(&EcPublicKey::from_bytes(bytes)?)
    }

    /// Attempts to parse a valid secp256r1 public key from its SEC-1 encoding.
    pub(crate) fn from_pubkey(pubkey: &EcPublicKey<NistP256>) -> Option<Self> {
        let point = AffinePoint::from_pubkey(pubkey);
        if point.is_some().into() {
            Some(PublicKey(point.unwrap().to_compressed_pubkey()))
        } else {
            None
        }
    }

    /// Returns the compressed SEC-1 encoding of this public key.
    pub(crate) fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    /// Returns the uncompressed SEC-1 encoding of this public key.
    pub(crate) fn decompress(&self) -> UncompressedCurvePoint<NistP256> {
        AffinePoint::from_pubkey(&EcPublicKey::Compressed(self.0))
            .unwrap()
            .to_uncompressed_pubkey()
    }
}
