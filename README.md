
[![Crates.io](https://img.shields.io/crates/v/http-serve-folder)](https://crates.io/crates/http-serve-folder)
[![Crates.io](https://img.shields.io/crates/d/http-serve-folder)](https://crates.io/crates/http-serve-folder)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](#License)

# http-serve-folder

A command line configurable, single executable server for hosting a folder over HTTP. Only intended for use in local development/testing. 

I created this because no minimal servers I found supported setting custom response headers, and I needed to be able to test locally while using [SharedArrayBuffer](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer) which requires [certain headers](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer#security_requirements) to be set.

## Usage

### Install

You can download the latest version from the [Releases](https://github.com/paul-hansen/http-serve-folder/releases/latest) page under "Assets".

Or if you have Rust installed, you can compile and install it using:
```
cargo install http-serve-folder
```

### Run
To see the instructions, use the `-h` flag:
```
http-serve-folder -h
```

Which outputs:

```
A configurable HTTP server that hosts a folder. Suitable for local
testing.

Usage: http-serve-folder [OPTIONS] [DIR]

Arguments:
  [DIR]  The folder you want to host

Options:
  -H, --header <HEADERS>         Headers to add to all file responses
  -i, --ip_address <IP_ADDRESS>  The IP Address to bind to [default: 127.0.0.1]
  -p, --port <PORT>              The port number to bind to [default: 4040]
  -l, --log <LOG>                The level of logging to display [default: info] [possible values: debug, info, warn, error, off]
  -h, --help                     Print help (see more with '--help')
  -V, --version                  Print version
```

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
