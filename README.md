A command line configurable, single executable server for hosting a folder over HTTP. Only intended for use in local development/testing. 

I created this because no minimal servers I found supported setting custom response headers, and I needed to be able to test locally while using [SharedArrayBuffer](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer#security_requirements).

```
http-folder-server 0.1.0
Paul Hansen <mail@paul.rs>
A configurable HTTP server that hosts a folder. Suitable for local testing.

USAGE:
    http-folder-server[.exe] [OPTIONS] [DIR]

ARGS:
    <DIR>    The folder you want to host

OPTIONS:
    -h, --help                       Print help information
    -H, --header <HEADERS>           Headers to add to all file responses
    -i, --ip_address <IP_ADDRESS>    The IP Address to bind to [default: 127.0.0.1]
    -l, --log <LOG>                  The level of logging to display [default: info] [possible
                                     values: debug, info, warn, error, off]
    -p, --port <PORT>                The port number to bind to [default: 4040]
    -V, --version                    Print version information
```