pub mod graphemes;
pub mod model;
use csv::{ErrorKind, Position};
use indexmap::IndexMap;
use serde::de::value;
use split_iter::Splittable;
use std::{
    collections::{BTreeMap, HashMap},
    fs,
};

use crate::model::*;

const OFFSET: u64 = 20;
#[derive(Debug, Clone, Copy)]
struct RecordData {
    error: bool,
    record_length: usize,
    error_position: Option<u64>,
    iterator_position: u64,
    previous_position: u64,
    diff: u64,
    record: u64,
    line: u64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'~')
        .flexible(true)
        .from_path("bpc_ingredientes_proc.csv")
        .unwrap();
    for record in reader.deserialize() {
        let des: SerdeNewIngrediente = record?;
        //        println!("{des:#?}");
    }
    return Ok(());
}
