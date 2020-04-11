use memmap::MmapOptions;
use std::fs::File;
use std::io::Write;
use std::io::SeekFrom;
use std::io::{self, prelude::*, BufReader};

// use hashbrown::hash_map::HashMap;

use std::str;
use std::env;

fn main() -> io::Result<()> {
    // let args: Vec<String> = env::args().collect();

    // let mut file = File::open("foo.txt")?;
    // let mut reader = BufReader::new(file);
    let file = File::open("foo.txt")?;
    let buffer = unsafe { MmapOptions::new().map(&file)? };

    // if let Ok(file) = File::open("foo.idx.json") {
    //     let reader2 = BufReader::new(file);
    //     blah = serde_json::from_reader(reader2).unwrap();
    //     let mut last_byte: usize = 0;

    //     for (_key, val) in blah.table.iter() {
    //         for byte in val {
    //             if byte > &last_byte {
    //                 last_byte = *byte;
    //             }
    //         }
    //     }
    //     println!("starting off on byte: {:#?}", last_byte);

    //     reader.seek(SeekFrom::Start(last_byte as u64))?;
    // } else {

    // }

    // let mut table: HashMap<String, Vec<(u32, usize)>> = HashMap::new();
    // let mut blah = Blah {
    //     table: HashMap::new(),
    // };

    // [0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}
    let mut n = 0;

    for pos in env::args() {
      if let Ok(my_pos) = pos.parse::<usize>() {
        let slice = buffer.get(my_pos..my_pos+36).unwrap();
        let stack_str: &str = str::from_utf8(slice).unwrap();
        println!("{}", stack_str);
      }

    }


    // for b in buffer.iter() {
    //     println!("{:#?}", b);
    //     if b == &nul {
    //         println!("bytes={}", bytes_count);
    //         break
    //     }

    //     n += 1;
    // }

    // for line in lines.into_iter() {
    //     let rline = line.unwrap();
    //     // let len = rline.len();
    //     let len = rline.len();
    //     println!("{} {}", n, len);

    //     // let v5: Vec<&str> = v.iter().map(|s| &s[..]).collect();

    //     for caps in uuid_regex.captures_iter(&rline) {
    //         for cap in caps.iter() {

    //             let capture = cap.unwrap();

    //             // println!("{:#?}", capture.as_bytes().to_owned());
    //             let entry = blah.table.entry(capture.as_bytes().to_owned()).or_insert(Vec::new());
    //             (*entry).push(n + capture.start());
    //         }
    //         // let capture = cap.get(0).unwrap();
    //     }
    //     n += len + 1; // taking into account the 1 byte newline

    //     // if rline.starts_with("insert") || rline.starts_with("INSERT") {
    //     //     index.push(n);
    //     // }
    // }


    Ok(())
}
