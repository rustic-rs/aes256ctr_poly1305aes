# AES256CTR-Poly1305AES

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Apache2/MIT licensed][license-image]
![Rust Version][rustc-image]
[![Crates.io Downloads][downloads-image]][crate-link]

Pure Rust implementation of **AES256-CTR/Poly1305-AES**: an
[Authenticated Encryption with Associated Data (AEAD)][2] cipher amenable to
fast, constant-time implementations in software, based on the [AES256-CTR][3]
stream cipher and the Poly1305-AES MAC which uses the [Poly1305][4] universal
hash function in combination with the [AES-128][5] block cipher.

A lot code is copied from the [chacha20poly1305 crate][6]

[Documentation][docs-link]

## About

AES256-CTR is widely known as stream cipher using the AES256 block cipher.
Poly1305-AES as MAC has been proposed by Daniel J. Bernstein in his famous
paper [The Poly1305-AES message-authentication code][7].

While both the stream cipher and the MAC are not commonly used, there exists
software which uses the combination to encrypt and authenticate the data, e.g.
the backup software [restic][8].


## License

Licensed under either of:

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
 * [MIT license](http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/aes256ctr_poly1305aes.svg
[crate-link]: https://crates.io/crates/aes256ctr_poly1305aes
[docs-image]: https://docs.rs/aes256ctr_poly1305aes/badge.svg
[docs-link]: https://docs.rs/aes256ctr_poly1305aes/
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[rustc-image]: https://img.shields.io/badge/rustc-1.51+-blue.svg
[downloads-image]: https://img.shields.io/crates/d/aes256ctr_poly1305aes.svg

[//]: # (general links)

[1]: https://tools.ietf.org/html/rfc8439
[2]: https://en.wikipedia.org/wiki/Authenticated_encryption
[3]: https://docs.rs/aes/latest/aes/struct.Aes256Ctr.html
[4]: https://github.com/RustCrypto/universal-hashes/tree/master/poly1305
[5]: https://docs.rs/aes/latest/aes/struct.Aes128.html
[6]: https://crates.io/crates/chacha20poly1305
[7]: https://cr.yp.to/mac/poly1305-20050329.pdf
[8]: https://github.com/restic/restic
