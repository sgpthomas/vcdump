mod errors;
mod map;
mod output;
mod path;

use crate::errors::Error;
use crate::map::Map;
use crate::output::Output;
use atty::Stream;
use std::fs::File;
use std::io::stdin;
use std::path::PathBuf;
use structopt::StructOpt;
use vcd::{Command, Value};

fn to_bitstring(vals: &[Value]) -> String {
    let mut res = String::new();
    for v in vals {
        res += match v {
            Value::V0 => "0",
            Value::V1 => "1",
            Value::X => "1",
            Value::Z => "1",
        }
    }
    res
}

// Definition of the command line interface. Uses the `structopt` derive macro
#[derive(StructOpt, Debug)]
#[structopt(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS")
)]
#[allow(clippy::option_option)]
pub struct Opts {
    /// Input vcd file. This is prioritized over stdin.
    pub file: Option<PathBuf>,
}

fn main() -> Result<(), Error> {
    let opts = Opts::from_args();

    let (header, commands) = match opts.file {
        Some(f) => {
            let content = File::open(f)?;
            let mut parser = vcd::Parser::new(content);
            let header = parser.parse_header()?;
            let commands = parser.collect::<Vec<_>>();
            (header, commands)
        }
        None => {
            if atty::isnt(Stream::Stdin) {
                let mut parser = vcd::Parser::new(stdin());
                let header = parser.parse_header()?;
                let commands = parser.collect::<Vec<_>>();
                (header, commands)
            } else {
                return Err(Error::NoFile);
            }
        }
    };

    let mut map: Map = Map::new(header);

    for cmd in commands {
        match cmd? {
            Command::Timestamp(_t) => {
                map.push_timestep();
            }
            Command::ChangeScalar(id, x) => map.change_value(
                id,
                u64::from_str_radix(&to_bitstring(&[x]), 2)?,
            )?,
            Command::ChangeVector(id, x) => map
                .change_value(id, u64::from_str_radix(&to_bitstring(&x), 2)?)?,
            _ => (),
        }
    }

    let mut output = Output::default();

    for (path, var) in map.paths {
        output.insert(path, &map.items[&var.code]);
    }
    if atty::is(Stream::Stdout) {
        let s = serde_json::to_string_pretty(&output)?;
        println!("{}", s);
    } else {
        let s = serde_json::to_string(&output)?;
        println!("{}", s);
    }
    Ok(())
}
