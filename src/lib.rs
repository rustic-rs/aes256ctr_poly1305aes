//! [`Aes256CtrPoly1305Aes`] is an [Authenticated Encryption with Associated Data
//! (AEAD)][2] cipher amenable to fast, constant-time implementations in software,
//! based on the [AES256-CTR][3] stream cipher and the [Poly1305-AES MAC] [4] 
//! which uses the [Poly1305][5] universal hash function in combination with the
//! [AES-128][6] block cipher.
//! 
//! A lot code is copied from the [chacha20poly1305 crate][7]
//!
//! This crate contains pure Rust implementations of [`Aes256CtrPoly1305Aes`]
//! (with optional AVX2 acceleration) as well as the following variants thereof:
//!
//! All implementations contained in the crate are designed to execute in
//! constant time, either by relying on hardware intrinsics (i.e. AVX2 on
//! x86/x86_64), or using a portable implementation which is only constant time
//! on processors which implement constant-time multiplication.
//!
//! It is not suitable for use on processors with a variable-time multiplication
//! operation (e.g. short circuit on multiply-by-zero / multiply-by-one, such as
//! certain 32-bit PowerPC CPUs and some non-ARM microcontrollers).
//!
//! # Usage
//!
//! ```
//! # #[cfg(feature = "alloc")]
//! # {
//! use aes256ctr_poly1305aes::{Aes256CtrPoly1305Aes, Key, Nonce};
//! use aes256ctr_poly1305aes::aead::{Aead, NewAead};
//!
//! // 64 bytes key
//! let key = Key::from_slice(b"This is an example of a very secret key. Keep it always secret!!"); 
//! let cipher = Aes256CtrPoly1305Aes::new(key);
//!
//! let nonce = Nonce::from_slice(b"my unique nonce!"); // 16-bytes; unique per message
//!
//! let ciphertext = cipher.encrypt(nonce, b"plaintext message".as_ref())
//!     .expect("encryption failure!");  // NOTE: handle this error to avoid panics!
//! let plaintext = cipher.decrypt(nonce, ciphertext.as_ref())
//!     .expect("decryption failure!");  // NOTE: handle this error to avoid panics!
//!
//! assert_eq!(&plaintext, b"plaintext message");
//! # }
//! ```
//!
//! ## In-place Usage (eliminates `alloc` requirement)
//!
//! This crate has an optional `alloc` feature which can be disabled in e.g.
//! microcontroller environments that don't have a heap.
//!
//! The [`AeadInPlace::encrypt_in_place`] and [`AeadInPlace::decrypt_in_place`]
//! methods accept any type that impls the [`aead::Buffer`] trait which
//! contains the plaintext for encryption or ciphertext for decryption.
//!
//! Note that if you enable the `heapless` feature of this crate,
//! you will receive an impl of [`aead::Buffer`] for `heapless::Vec`
//! (re-exported from the [`aead`] crate as [`aead::heapless::Vec`]),
//! which can then be passed as the `buffer` parameter to the in-place encrypt
//! and decrypt methods:
//!
//! ```
//! # #[cfg(feature = "heapless")]
//! # {
//! use aes256ctr_poly1305aes::{Aes256CtrPoly1305Aes, Key, Nonce};
//! use aes256ctr_poly1305aes::aead::{AeadInPlace, NewAead};
//! use aes256ctr_poly1305aes::aead::heapless::Vec;
//!
//! // 64 bytes key
//! let key = Key::from_slice(b"This is an example of a very secret key. Keep it always secret!!");
//! let cipher = Aes256CtrPoly1305Aes::new(key);
//!
//! let nonce = Nonce::from_slice(b"my unique nonce!"); // 16-bytes; unique per message
//!
//! let mut buffer: Vec<u8, 128> = Vec::new();
//! buffer.extend_from_slice(b"plaintext message");
//!
//! // Encrypt `buffer` in-place, replacing the plaintext contents with ciphertext
//! cipher.encrypt_in_place(nonce, b"", &mut buffer).expect("encryption failure!");
//!
//! // `buffer` now contains the message ciphertext
//! assert_ne!(&buffer, b"plaintext message");
//!
//! // Decrypt `buffer` in-place, replacing its ciphertext context with the original plaintext
//! cipher.decrypt_in_place(nonce, b"", &mut buffer).expect("decryption failure!");
//! assert_eq!(&buffer, b"plaintext message");
//! # }
//! ```
//! 
//! [1]: https://tools.ietf.org/html/rfc8439
//! [2]: https://en.wikipedia.org/wiki/Authenticated_encryption
//! [3]: https://docs.rs/aes/latest/aes/struct.Aes256Ctr.html
//! [4]: https://cr.yp.to/mac/poly1305-20050329.pdf
//! [5]: https://github.com/RustCrypto/universal-hashes/tree/master/poly1305
//! [6]: https://docs.rs/aes/latest/aes/struct.Aes128.html
//! [7]: https://crates.io/crates/chacha20poly1305 

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustCrypto/meta/master/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/RustCrypto/meta/master/logo.svg"
)]
#![warn(missing_docs, rust_2018_idioms)]

mod cipher;

pub use aead;

use self::cipher::Cipher;
use aead::{
    consts::{U0, U16, U32, U64},
    generic_array::GenericArray,
    AeadCore, AeadInPlace, Error, NewAead,
};
use zeroize::Zeroize;

use ::cipher::{BlockEncrypt, NewBlockCipher, FromBlockCipher};
use aes::{Aes128, Aes256, Aes256Ctr};
use poly1305::{
    universal_hash::NewUniversalHash,
    Poly1305,
};

/// Key type (512-bits/64-bytes).
///
/// Implemented as an alias for [`GenericArray`].
pub type Key = GenericArray<u8, U64>;

/// Nonce type (128-bits/16-bytes).
///
/// Implemented as an alias for [`GenericArray`].
pub type Nonce = GenericArray<u8, U16>;

/// Poly1305 tag.
///
/// Implemented as an alias for [`GenericArray`].
pub type Tag = GenericArray<u8, U16>;

/// Authenticated Encryption with Additional Data (AEAD) using AES256-CTR and Poly1305-AES.
///
/// See the [toplevel documentation](index.html) for a usage example.
#[derive(Clone, Debug)]
pub struct Aes256CtrPoly1305Aes {
    /// Secret key for Aes256Ctr
    aes256ctr_key: GenericArray<u8, U32>,

    /// Secret key for Aes128r
    aes128_key: GenericArray<u8, U16>,

    /// Secret value r for Poly1305
    poly1305_r: GenericArray<u8, U16>,
}

impl Aes256CtrPoly1305Aes {
    fn cipher_from_nonce(&self, nonce: &aead::Nonce<Self>) -> Cipher<Aes256Ctr> {
        // Derive Poly1305 key from r and AES_k(nonce)
        let mut mac_key = poly1305::Key::default();
        mac_key[0..16].copy_from_slice(&self.poly1305_r);

        let mut block = *nonce;
        Aes128::new(&self.aes128_key).encrypt_block(&mut block);
        mac_key[16..32].copy_from_slice(&block);
        block.zeroize();
    
        let cipher = Cipher::new(
            Aes256Ctr::from_block_cipher(Aes256::new(&self.aes256ctr_key), nonce),
            Poly1305::new(&mac_key),
        );
        mac_key.zeroize();
        cipher
    }

}

impl NewAead for Aes256CtrPoly1305Aes {
    type KeySize = U64;

    /// New AEAD using AES256-CTR and Poly1305-AES from the given 64-byte key.
    /// The first 32 bytes are used as key for AES256-CTR.
    /// The following 16 bytes are used as key for the AES128 used in Poly1305-AES.
    /// The last 16 bytes are used as r in Poly1305-AES.
    fn new(key: &Key) -> Self {
        Self {
            aes256ctr_key: GenericArray::clone_from_slice(&key[0..32]),
            aes128_key: GenericArray::clone_from_slice(&key[32..48]),
            poly1305_r: GenericArray::clone_from_slice(&key[48..64]),
        }
    }
}

impl AeadCore for Aes256CtrPoly1305Aes {
    type NonceSize = U16;
    type TagSize = U16;
    type CiphertextOverhead = U0;
}

impl AeadInPlace for Aes256CtrPoly1305Aes {
    fn encrypt_in_place_detached(
        &self,
        nonce: &aead::Nonce<Self>,
        associated_data: &[u8],
        buffer: &mut [u8],
    ) -> Result<Tag, Error> {
        self.cipher_from_nonce(nonce).encrypt_in_place_detached(associated_data, buffer)
    }

    fn decrypt_in_place_detached(
        &self,
        nonce: &aead::Nonce<Self>,
        associated_data: &[u8],
        buffer: &mut [u8],
        tag: &Tag,
    ) -> Result<(), Error> {
        self.cipher_from_nonce(nonce).decrypt_in_place_detached(associated_data, buffer, tag)
    }
}

impl Drop for Aes256CtrPoly1305Aes {
    fn drop(&mut self) {
        self.aes256ctr_key.as_mut_slice().zeroize();
        self.aes128_key.as_mut_slice().zeroize();
        self.poly1305_r.as_mut_slice().zeroize();
    }
}
