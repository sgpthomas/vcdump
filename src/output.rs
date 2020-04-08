use crate::path::Path;
use serde::Serialize;
use serde::Serializer;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub enum Output {
    Scope(BTreeMap<String, Output>),
    Timeseries(Vec<u64>),
}

impl Default for Output {
    fn default() -> Self {
        Output::Scope(BTreeMap::new())
    }
}

impl Serialize for Output {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Output::Scope(m) => m.serialize(serializer),
            Output::Timeseries(ts) => ts.serialize(serializer),
        }
    }
}

impl Output {
    pub fn insert(&mut self, path: Path<String>, series: &[u64]) {
        let mut out = self;
        for name in path.to_vec() {
            match out {
                Output::Scope(m) => {
                    out = m
                        .entry(name.to_string())
                        .or_insert_with(Output::default)
                }
                Output::Timeseries(..) => panic!("bad boy"),
            }
        }
        *out = Output::Timeseries(series.to_vec())
    }
}
