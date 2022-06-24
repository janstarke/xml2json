# xml2json
CLI tool to convert XML into JSON

# Installation

```shell
cargo install xml2json
```

# Usage
```shell
xml2json 0.1.0
Jan Starke <Jan.Starke@t-systems.com>
CLI tool to convert XML into JSON

USAGE:
    xml2json [OPTIONS]

OPTIONS:
    -h, --help                     Print help information
    -J, --json-file <JSON_FILE>    path of the JSON file to write to; '-' denotes STDOUT, which is
                                   the default
    -q, --quiet                    Less output per occurrence
    -v, --verbose                  More output per occurrence
    -V, --version                  Print version information
    -X, --xml-file <XML_FILE>      path of the XML file to read from; '-' denotes STDIN, which is
                                   the default
```