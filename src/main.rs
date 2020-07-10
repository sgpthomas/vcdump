mod errors;
mod map;
mod output;
mod path;

use crate::errors::Error;
use crate::map::Map;
use crate::output::Output;
use atty::Stream;
use serde::Serialize;
use std::fs::File;
use std::io::stdin;
use std::{borrow::Cow, path::PathBuf};
use structopt::StructOpt;
use vcd::{Command, IdCode, Var};

fn to_bitstring(vals: &[vcd::Value], x_value: char, z_value: char) -> String {
    let mut res = String::new();
    for v in vals {
        res.push(match v {
            vcd::Value::V0 => '0',
            vcd::Value::V1 => '1',
            vcd::Value::X => x_value,
            vcd::Value::Z => z_value,
        });
    }
    res
}

/// Simple tool to convert vcd files into json files.
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
    /// Use string values
    #[structopt(long)]
    pub strings: bool,
    /// Use pretty output
    #[structopt(long)]
    pub pretty: bool,
}

pub trait ValueType: Serialize + Clone {
    fn empty(item: &Var) -> Self;
    fn from_scalar(value: vcd::Value) -> Result<Option<Self>, Error>;
    fn from_vector(value: Cow<[vcd::Value]>) -> Result<Option<Self>, Error>;
    fn from_real(value: f64) -> Result<Option<Self>, Error>;
    fn from_string(value: String) -> Result<Option<Self>, Error>;
}

pub trait FromValue<T>: Sized {
    fn from_value(value: T) -> Result<Option<Self>, Error>;
}

impl<T: ValueType> FromValue<vcd::Value> for T {
    fn from_value(value: vcd::Value) -> Result<Option<Self>, Error> {
        Self::from_scalar(value)
    }
}

impl<T: ValueType> FromValue<Vec<vcd::Value>> for T {
    fn from_value(value: Vec<vcd::Value>) -> Result<Option<Self>, Error> {
        Self::from_vector(Cow::Owned(value))
    }
}

impl<T: ValueType> FromValue<f64> for T {
    fn from_value(value: f64) -> Result<Option<Self>, Error> {
        Self::from_real(value)
    }
}

impl<T: ValueType> FromValue<String> for T {
    fn from_value(value: String) -> Result<Option<Self>, Error> {
        Self::from_string(value)
    }
}

impl ValueType for u128 {
    fn empty(_item: &Var) -> Self {
        0
    }
    fn from_scalar(value: vcd::Value) -> Result<Option<Self>, Error> {
        Self::from_vector(Cow::Borrowed(&[value]))
    }
    fn from_vector(value: Cow<[vcd::Value]>) -> Result<Option<Self>, Error> {
        Ok(Some(u128::from_str_radix(
            &to_bitstring(&value, '1', '1'),
            2,
        )?))
    }
    fn from_real(_value: f64) -> Result<Option<Self>, Error> {
        Ok(None)
    }
    fn from_string(_value: String) -> Result<Option<Self>, Error> {
        Ok(None)
    }
}

impl ValueType for String {
    fn empty(_item: &Var) -> Self {
        String::new()
    }
    fn from_scalar(value: vcd::Value) -> Result<Option<Self>, Error> {
        Self::from_vector(Cow::Borrowed(&[value]))
    }
    fn from_vector(value: Cow<[vcd::Value]>) -> Result<Option<Self>, Error> {
        Ok(Some(to_bitstring(&value, 'X', 'Z')))
    }
    fn from_real(value: f64) -> Result<Option<Self>, Error> {
        Ok(Some(value.to_string()))
    }
    fn from_string(value: String) -> Result<Option<Self>, Error> {
        Ok(Some(value))
    }
}

fn process<V: ValueType>(opts: Opts) -> Result<(), Error> {
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

    let mut map: Map<V> = Map::new(header);

    fn process_change<T, V: FromValue<T> + ValueType>(
        map: &mut Map<V>,
        id: IdCode,
        value: T,
    ) -> Result<(), Error> {
        if let Some(value) = V::from_value(value)? {
            map.change_value(id, value)?;
        }
        Ok(())
    }

    for cmd in commands {
        match cmd? {
            Command::Timestamp(_t) => {
                map.push_timestep();
            }
            Command::ChangeScalar(id, x) => process_change(&mut map, id, x)?,
            Command::ChangeVector(id, x) => process_change(&mut map, id, x)?,
            Command::ChangeReal(id, x) => process_change(&mut map, id, x)?,
            Command::ChangeString(id, x) => process_change(&mut map, id, x)?,
            _ => (),
        }
    }

    let mut output = Output::default();

    let Map { mut items, paths } = map;

    for (path, var) in paths {
        output.insert(path, items.remove(&var.code).unwrap().values);
    }
    if opts.pretty || atty::is(Stream::Stdout) {
        let s = serde_json::to_string_pretty(&output)?;
        println!("{}", s);
    } else {
        let s = serde_json::to_string(&output)?;
        println!("{}", s);
    }
    Ok(())
}

fn main() -> Result<(), Error> {
    let opts = Opts::from_args();
    if opts.strings {
        process::<String>(opts)
    } else {
        process::<u128>(opts)
    }
}
