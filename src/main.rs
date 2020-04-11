use bytelines::*;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::io::SeekFrom;

// use hashbrown::hash_map::HashMap;
use std::collections::HashMap;
use regex::bytes::Regex;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Blah {
    table: HashMap<Vec<u8>, Vec<usize>>
}

fn main() -> io::Result<()> {
    let mut file = File::open("foo.txt")?;
    let mut reader = BufReader::new(file);

    let mut blah: Blah;

    if let Ok(file) = File::open("foo.idx.json") {
        let reader2 = BufReader::new(file);
        blah = serde_json::from_reader(reader2).unwrap();
        let mut last_byte: usize = 0;

        for (_key, val) in blah.table.iter() {
            for byte in val {
                if byte > &last_byte {
                    last_byte = *byte;
                }
            }
        }
        println!("starting off on byte: {:#?}", last_byte);

        reader.seek(SeekFrom::Start(last_byte as u64))?;
    } else {
        blah = Blah {
            table: HashMap::new(),
        };
    }

    // let mut table: HashMap<String, Vec<(u32, usize)>> = HashMap::new();
    // let mut blah = Blah {
    //     table: HashMap::new(),
    // };

    // [0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}
    let uuid_regex_literal = r#"[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}"#;
    let uuid_regex: Regex = Regex::new(uuid_regex_literal).unwrap();

    let mut index: Vec<u32> = Vec::new();

    let mut n = 0;
    let mut lines = reader.byte_lines();
    for line in lines.into_iter() {
        let rline = line.unwrap();
        // let len = rline.len();
        let len = rline.len();
        println!("{} {}", n, len);

        // let v5: Vec<&str> = v.iter().map(|s| &s[..]).collect();

        for caps in uuid_regex.captures_iter(&rline) {
            for cap in caps.iter() {

                let capture = cap.unwrap();

                // println!("{:#?}", capture.as_bytes().to_owned());
                let entry = blah.table.entry(capture.as_bytes().to_owned()).or_insert(Vec::new());
                (*entry).push(n + capture.start());
            }
            // let capture = cap.get(0).unwrap();
        }
        n += len + 1; // taking into account the 1 byte newline

        // if rline.starts_with("insert") || rline.starts_with("INSERT") {
        //     index.push(n);
        // }
    }

    println!("{:#?}", index);
    // println!("{:#?}", table);

    for (key, val) in blah.table.iter() {
        println!("{:#?}", val);
    }

    // let serialized = serde_json::to_vec(&blah).unwrap();
    // let mut file = File::create("foo.idx.json")?;
    // file.write_all(&serialized)?;
    // println!("{}", serialized);

    Ok(())
}
