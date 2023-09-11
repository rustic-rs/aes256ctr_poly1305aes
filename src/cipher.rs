//! Core AEAD cipher implementation a StreamCipher and Poly1305.

use ::cipher::StreamCipher;
use aead::Error;
use poly1305::Poly1305;

use super::Tag;

/// Cipher is a generic AEAD from an arbitrary StreamCipher and an Poly1305.
/// Note that this cipher implements an AEAD but does not authenticate the message length
/// in contrast to other Poly1305-based AEADs, e.g. ChaCha20Poly1305
pub(crate) struct Cipher<C>
where
    C: StreamCipher,
{
    cipher: C,
    mac: Poly1305,
}

impl<C> Cipher<C>
where
    C: StreamCipher,
{
    /// Create a new AEAD from Cipher and Poly1305
    pub(crate) fn new(cipher: C, mac: Poly1305) -> Self {
        Self { cipher, mac }
    }

    /// Encrypt the given message in-place, returning the authentication tag
    pub(crate) fn encrypt_in_place_detached(
        mut self,
        associated_data: &[u8],
        buffer: &mut [u8],
    ) -> Result<Tag, Error> {
        if !associated_data.is_empty() {
            return Err(Error);
        }

        // TODO(tarcieri): interleave encryption with Poly1305
        // See: <https://github.com/RustCrypto/AEADs/issues/74>
        self.cipher.apply_keystream(buffer);
        Ok(self.mac.compute_unpadded(buffer).into_bytes())
    }

    /// Decrypt the given message, first authenticating ciphertext integrity
    /// and returning an error if it's been tampered with.
    pub(crate) fn decrypt_in_place_detached(
        mut self,
        associated_data: &[u8],
        buffer: &mut [u8],
        tag: &Tag,
    ) -> Result<(), Error> {
        if !associated_data.is_empty() {
            return Err(Error);
        }

        use subtle::ConstantTimeEq;
        let expected_tag = self.mac.compute_unpadded(buffer).into_bytes();

        // This performs a constant-time comparison using the `subtle` crate
        if expected_tag.ct_eq(tag).unwrap_u8() == 1 {
            // TODO(tarcieri): interleave decryption with Poly1305
            // See: <https://github.com/RustCrypto/AEADs/issues/74>
            self.cipher.apply_keystream(buffer);
            Ok(())
        } else {
            Err(Error)
        }
    }
}
