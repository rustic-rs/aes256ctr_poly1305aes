# Changelog

All notable changes to this project will be documented in this file.

## [0.2.2](https://github.com/rustic-rs/aes256ctr_poly1305aes/compare/v0.2.1...v0.2.2) - 2024-12-02

### Other

- *(deps)* update embarkstudios/cargo-deny-action digest to 2d8c992 ([#40](https://github.com/rustic-rs/aes256ctr_poly1305aes/pull/40))
- *(deps)* add aws-lc-rs and aws-lc-sys to deny list due to build complexity and cross-compilation issues
- add .vscode to ignore list
- enhance cross-check job with Rust version and add support for aarch64-unknown-linux-musl
- *(deps)* update actions ([#39](https://github.com/rustic-rs/aes256ctr_poly1305aes/pull/39))
- *(deps)* update marcoieni/release-plz-action digest to f0fdfff ([#38](https://github.com/rustic-rs/aes256ctr_poly1305aes/pull/38))
- *(deps)* update marcoieni/release-plz-action digest to 301fd6d ([#37](https://github.com/rustic-rs/aes256ctr_poly1305aes/pull/37))
- add installation script for default feature on x86_64-unknown-linux-musl
- use runners according to available images and target triple ([#36](https://github.com/rustic-rs/aes256ctr_poly1305aes/pull/36))
- *(deps)* update marcoieni/release-plz-action digest to fff938e ([#35](https://github.com/rustic-rs/aes256ctr_poly1305aes/pull/35))
- *(deps)* update actions ([#34](https://github.com/rustic-rs/aes256ctr_poly1305aes/pull/34))
- *(deps)* update marcoieni/release-plz-action digest to 394e0e4 ([#33](https://github.com/rustic-rs/aes256ctr_poly1305aes/pull/33))
- *(deps)* update actions ([#32](https://github.com/rustic-rs/aes256ctr_poly1305aes/pull/32))
- update dprint config ([#31](https://github.com/rustic-rs/aes256ctr_poly1305aes/pull/31))
- *(deps)* update marcoieni/release-plz-action digest to 693f6d4 ([#29](https://github.com/rustic-rs/aes256ctr_poly1305aes/pull/29))

## [0.2.1](https://github.com/rustic-rs/aes256ctr_poly1305aes/compare/v0.2.0...v0.2.1) - 2024-10-03

### Fixed

- *(ci)* remove unmaintained `actions-rs` ci actions

### Other

- *(deps)* update marcoieni/release-plz-action digest to 5c48341 ([#27](https://github.com/rustic-rs/aes256ctr_poly1305aes/pull/27))
- generate lockfile in ci for rustsec/audit-check ([#28](https://github.com/rustic-rs/aes256ctr_poly1305aes/pull/28))
- add triage label to new issues only if no label has been set when creating it ([#25](https://github.com/rustic-rs/aes256ctr_poly1305aes/pull/25))
- *(deps)* fix cargo-deny config ([#26](https://github.com/rustic-rs/aes256ctr_poly1305aes/pull/26))
- *(deps)* update actions ([#24](https://github.com/rustic-rs/aes256ctr_poly1305aes/pull/24))
- ignore CHANGELOG.md in dprint formatting
- *(deps)* Update renovate.json
- use release-plz
- break old ci jobs when new commits are pushed so we don't fill up the queue
- don't let renovate rebase PRs to circumvent resource exhaustion issues
- add project-related shared cache key
- *(deps)* update swatinem/rust-cache digest to 23bce25 ([#20](https://github.com/rustic-rs/aes256ctr_poly1305aes/pull/20))
- *(deps)* update embarkstudios/cargo-deny-action digest to 2fad080 ([#19](https://github.com/rustic-rs/aes256ctr_poly1305aes/pull/19))
- *(deps)* update taiki-e/install-action digest to 3ed9916 ([#18](https://github.com/rustic-rs/aes256ctr_poly1305aes/pull/18))
- *(deps)* update taiki-e/install-action digest to f34807f ([#17](https://github.com/rustic-rs/aes256ctr_poly1305aes/pull/17))
- *(deps)* update taiki-e/install-action digest to 56ab793 ([#16](https://github.com/rustic-rs/aes256ctr_poly1305aes/pull/16))
- *(deps)* update taiki-e/install-action digest to fea51d0 ([#15](https://github.com/rustic-rs/aes256ctr_poly1305aes/pull/15))
- dprint fmt
- activate automerge for github action digest update
- activate automerge for github action digest update
- *(deps)* update taiki-e/install-action digest to b59252d ([#14](https://github.com/rustic-rs/aes256ctr_poly1305aes/pull/14))
- *(deps)* update taiki-e/install-action digest to 57fbff3 ([#13](https://github.com/rustic-rs/aes256ctr_poly1305aes/pull/13))
- *(deps)* update taiki-e/install-action digest to 2b8d4e0 ([#12](https://github.com/rustic-rs/aes256ctr_poly1305aes/pull/12))
- *(deps)* update taiki-e/install-action digest to a1f180f ([#11](https://github.com/rustic-rs/aes256ctr_poly1305aes/pull/11))
- *(deps)* update taiki-e/install-action digest to c264868 ([#10](https://github.com/rustic-rs/aes256ctr_poly1305aes/pull/10))
- dprint fmt
- update rustsec/audit-check
- update taiki-e/install-action
- update dtolnay/rust-toolchain
- Set MSRV to 1.60.0
- Run actions that need secrets.GITHUB_TOKEN only on rustic-rs org

## [0.2.0] - 2023-11-13

### Bug Fixes

- Clippy
- Update rust crate zeroize to >=1, <1.7
- Update rust crate subtle to >=2, <2.6

### Miscellaneous Tasks

- Add ci
- Update workflows
- Update dprint plugins
- Add changelog generation
- Update CI to match other crates in the ecosystem
- Add config.toml for cargo
- Remove lockfile from library
- Remove lockfile maintenance from library
- Add release workflow
- Update changelog

<!-- generated by git-cliff -->
