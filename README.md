# Performance

Trice is a minimal replacement for `std::time::Instant` that works in WASM for the web and Node.js, enabling time measurement and performance calculations.

[![](https://img.shields.io/badge/status-stable-ff00bb.svg?style=flat-square)](https://github.com/surrealdb/trice) [![docs.rs](https://img.shields.io/docsrs/trice?style=flat-square)](https://docs.rs/trice/) [![Crates.io](https://img.shields.io/crates/v/trice?style=flat-square)](https://crates.io/crates/trice) [![](https://img.shields.io/badge/license-Apache_License_2.0-00bfff.svg?style=flat-square)](https://github.com/surrealdb/trice) 

#### Features

- Replacement for `std::time::Instant`
- Uses `performance.now()` in WASM environments
- Uses `std::time::Instant` in normal environments
