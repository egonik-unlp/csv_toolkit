pub mod graphemes;
pub mod model;
use csv::Position;
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

fn main() {
    let stringcsv = std::fs::read_to_string("bpc_ingredientes_proc.csv").unwrap();
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'~')
        .flexible(true)
        .from_path("bpc_ingredientes_proc.csv")
        .unwrap();
    let mut reader2 = csv::ReaderBuilder::new()
        .delimiter(b'~')
        .flexible(true)
        .from_path("bpc_ingredientes_proc.csv")
        .unwrap();
    let mut positions = vec![];
    // lt mut current_position = 0u64;
    let mut previous_position = 0u64;
    while let Some(record_result) = reader.records().next() {
        let current_position = reader.position().byte();
        let record = record_result.unwrap();
        let br = record.as_byte_record().as_slice().len();
        let des = record
            .deserialize::<SerdeNewIngrediente>(Some(reader2.headers().unwrap()))
            .inspect_err(|e| {
                positions.push(RecordData {
                    error: true,
                    record_length: br,
                    error_position: Some(e.clone().position().unwrap().byte()),
                    iterator_position: current_position,
                    previous_position: previous_position,
                    diff: current_position - previous_position,
                    record: e.position().unwrap().record(),
                    line: e.position().unwrap().line(),
                })
            })
            .inspect(|cv| {
                positions.push(RecordData {
                    error: false,
                    record_length: br,
                    error_position: None,
                    iterator_position: current_position,
                    previous_position: previous_position,
                    diff: current_position - previous_position,
                    record: record.position().unwrap().record(),
                    line: record.position().unwrap().line(),
                })
            });
        previous_position = current_position;
    }

    let erroring = positions
        .clone()
        .into_iter()
        .filter(|pos| pos.error.eq(&true))
        .collect::<Vec<RecordData>>();

    let fine = positions
        .clone()
        .into_iter()
        .filter(|pos| pos.error.eq(&false))
        .collect::<Vec<RecordData>>();

    let first_good_one = positions
        .clone()
        .into_iter()
        .filter(|x| x.error.eq(&false))
        .collect::<Vec<_>>();
    println!("First good one:{:#?}", first_good_one.first().unwrap());
    let mut sorted_by_record: IndexMap<u64, Vec<RecordData>> = IndexMap::new();
    positions
        .into_iter()
        .for_each(|node| match sorted_by_record.get_mut(&node.record) {
            Some(value) => value.push(node),
            None => {
                sorted_by_record.insert(node.record, vec![node]);
            }
        });
    sorted_by_record.sort_keys();
    for (pos, node) in sorted_by_record {
        assert!(node.len().eq(&1));

        if pos < 5 {
            println!("{}: {:#?}", pos, node)
        }
    }
}
