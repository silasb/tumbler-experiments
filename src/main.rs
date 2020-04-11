use bytelines::*;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::io::SeekFrom;

use regex::bytes::Regex;

use blah::Blah;

fn main() -> io::Result<()> {
    let file = File::open("foo.txt")?;
    let mut reader = BufReader::new(file);

    let mut blah: Blah;
    let mut n: usize;
    let mut last_byte: usize = 0;

    blah = Blah::load_index();

    for (_key, val) in blah.table.iter() {
        for byte in val {
            if byte > &last_byte {
                last_byte = *byte;
            }
        }
    }
    println!("starting off on byte: {:#?}", last_byte);
    n = last_byte + 1;
    reader.seek(SeekFrom::Start(n as u64))?;

    // [0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}
    let uuid_regex_literal = r#"[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}"#;
    let uuid_regex: Regex = Regex::new(uuid_regex_literal).unwrap();

    let lines = reader.byte_lines();
    for line in lines.into_iter() {
        let rline = line.unwrap();
        let len = rline.len();
        // println!("{} {}", n, len);

        for caps in uuid_regex.captures_iter(&rline) {
            for cap in caps.iter() {

                let capture = cap.unwrap();

                // println!("{:#?}", capture.as_bytes().to_owned());
                let entry = blah.table.entry(capture.as_bytes().to_owned()).or_insert(Vec::new());
                (*entry).push(n + capture.start());
            }
        }
        n += len + 1; // taking into account the 1 byte newline
    }

    // println!("{:#?}", table);

    // for (key, val) in blah.table.iter() {
    //     println!("{:#?} => {:#?}", key, val);
    // }

    println!("{}", serde_json::to_string_pretty(&blah)?);

    let serialized = serde_json::to_vec(&blah)?;
    let mut file = File::create("foo.idx.json")?;
    file.write_all(&serialized)?;
    // println!("{:#?}", serialized);

    Ok(())
}
