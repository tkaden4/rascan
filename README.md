# rascan

A simple port scanner written in Rust.

### Usage
```
    rascan [FLAGS] [OPTIONS] <HOST> --start <start> --end <end> 

FLAGS:
    -h, --help       Prints help information
    -o, --open       Only show open ports
    -V, --version    Prints version information

OPTIONS:
    -s, --start <start>        Start port
    -e, --end <end>            End port
    -t, --timeout <timeout>    Set port timeout (in ms)

ARGS:
    <HOST>    Host to scan
```
