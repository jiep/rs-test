# rs-test

<div align="center">

  <h1><code>rs-test</code></h1>

  <strong>Implementation of `rs-test`</strong>

  [![ci](https://github.com/jiep/rs-test/actions/workflows/ci.yml/badge.svg)](https://github.com/jiep/rs-test/actions/workflows/ci.yml)
  [![dependency status](https://deps.rs/repo/github/jiep/rs-test/status.svg)](https://deps.rs/repo/github/jiep/rs-test)

  <sub>Built with 🦀</sub>
</div>


## Supported algorithms

<details>
  <summary>Click to expand supported hashes </summary>
  
    * SHA256
    * SHA512
    
</details>

## Binaries

Download the latest version from [Releases](https://github.com/jiep/rs-test/releases).

## Build from source

1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Check source code

```
cargo check
``` 

3. Compile binary

```
cargo build
``` 

4. Run tests

```
cargo test
```

> Note: for release target, add --release

5. Run binary

```
cargo run
# or
./target/release/rs-test # for release version
./target/debug/rs-test # for debug version
```

## 🚴 Usage

```
Usage: rs-test --message <MESSAGE> --hash-algorithm <HASH_ALGORITHM>

Options:
      --message <MESSAGE>                
      --hash-algorithm <HASH_ALGORITHM>  
  -h, --help                             Print help information
  -V, --version                          Print version information
```

### Example

```
rs-test --message "hello world" --hash-algorithm sha512
309ecc489c12d6eb4cc40f50c902f2b4d0ed77ee511a7c7a9bcd3ca86d4cd86f989dd35bc5ff499670da34255b45b0cfd830e81f605dcf7dc5542e93ae9cd76f
```

## License
This project is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE] and [LICENSE-MIT] for details.