# Vcdump [![latest]][crate]

[latest]: https://img.shields.io/crates/v/vcdump.svg
[crate]: https://crates.io/crates/vcdump

A simple tool to convert vcd [Value Change Dump](https://en.wikipedia.org/wiki/Value_change_dump) files into [json](https://en.wikipedia.org/wiki/JSON) files. The motivation for this is to
allow vcd files to be better integrated into normal Unix command line workflows.

## Install
You can install this tool with a simple `cargo install vcdump`.

## Usage
`vcdump` accepts a file as an argument or via `stdin`.
 - `cat test.vcd | vcdump`
 - `vcdump test.vcd`
 
This produces a `json` file on `stdout`. You can then write this to a file, or use something
like `jq` to explore the json file.
 - `vcdump test.vcd | jq '.TOP.main.a0.out'`
