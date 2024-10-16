# xml-include

[![Crates.io](https://img.shields.io/crates/v/xml-include.svg)](https://crates.io/crates/xml-include)
[![Documentation](https://docs.rs/xml-include/badge.svg)](https://docs.rs/xml-include/)
[![Build status](https://github.com/python-systems/xml-include/workflows/CI/badge.svg)](https://github.com/python-systems/xml-include/actions?query=workflow%3ACI)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENCE)

**xml-include** is library to pre-process XML include statements and merge them into a single XML file.

## Usage
```rust
use std::path::PathBuf;
use xml_include::resolve_xml_includes;

fn main() {
    let input_file = PathBuf::from("tests/examples/TradingApi.xml");
    let reference_file = PathBuf::from("tests/examples/TradingApi.ref.xml");

    let resolved_content = resolve_xml_includes(&input_file).unwrap();
}
```
