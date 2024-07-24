use csv::Position;
use rayon::prelude::*;
use std::error::Error;
use std::fmt::format;
use std::fs::OpenOptions;
use std::io::prelude::*;
use unicode_segmentation::UnicodeSegmentation;
fn get_strings_chunked_and_offsetted(
    positions: Vec<Position>,
    chunk_size: usize,
    OFFSET: u64,
    stringcsv: String,
) {
    let positions = positions.chunks(100).next().unwrap();
    let total_positions = positions.len();
    let words = positions
        .into_par_iter()
        .enumerate()
        .map(|(n, pos)| {
            (pos.byte() - OFFSET..pos.byte() + OFFSET)
                .into_iter()
                .enumerate()
                .map(|(ngp, strpos)| {
                    if ngp.eq(&0) {
                        println!("pos {}/{}", n, total_positions);
                    }
                    let char = UnicodeSegmentation::graphemes(stringcsv.as_str(), true)
                        .nth(strpos as usize)
                        .unwrap_or("X");
                    if ngp.eq(&21) {
                        format!("{}{}", "|", char)
                    } else {
                        char.to_owned()
                    }
                })
                .collect::<String>()
                .replace("\n", "_")
        })
        .collect::<Vec<String>>();
    let mut text_file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("broken_lines_dump_c1.txt")
        .unwrap();
    println!("started writing to file");
    words.into_iter().enumerate().for_each(|(line_n, block)| {
        let line_n_string_error = format!("Error escribiendo linea {} al archivo", line_n);
        writeln!(text_file, "{}", block).expect(&line_n_string_error)
    })
}
