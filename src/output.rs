use crate::path::Path;
use serde::Serialize;
use serde::Serializer;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub enum Output<V> {
    Scope(BTreeMap<String, Output<V>>),
    Timeseries(Vec<V>),
}

impl<V> Default for Output<V> {
    fn default() -> Self {
        Output::Scope(BTreeMap::new())
    }
}

impl<V: Serialize> Serialize for Output<V> {
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

impl<V: Clone> Output<V> {
    pub fn insert(&mut self, path: Path<String>, series: Vec<V>) {
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
        *out = Output::Timeseries(series)
    }
}
