# rascan

A simple port scanner written in Rust.

### Usage
```
    rascan [FLAGS] [OPTIONS] <HOST>

FLAGS:
    -h, --help       Prints help information
    -o, --open       Only show open ports
    -V, --version    Prints version information

OPTIONS:
    -e, --end <end>            End port
    -s, --start <start>        Start port
    -t, --timeout <timeout>    Set port timeout (in ms)

ARGS:
    <HOST>    Host to scan
```
