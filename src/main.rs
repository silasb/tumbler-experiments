use bytelines::*;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::io::SeekFrom;

// use hashbrown::hash_map::HashMap;
use std::collections::HashMap;
use regex::bytes::Regex;

use serde::{Serializer, Deserializer};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Blah {
    #[serde(flatten)]
    #[serde(serialize_with = "serialize_hashmap_with_u8vec_key", deserialize_with = "deserialize_hashmap_with_u8vec_key")]
    table: HashMap<Vec<u8>, Vec<usize>>
}

fn serialize_hashmap_with_u8vec_key<S>(table: &HashMap<Vec<u8>, Vec<usize>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    fn serialize_u8vec_to_string<S>(key: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = String::from_utf8(key.to_owned()).unwrap(); //.map_err(serde::se::Error::custom)?;
        s.serialize(serializer)
    }

    #[derive(Serialize, Hash, Eq, PartialEq)]
    struct Wrapper<'a>(#[serde(serialize_with = "serialize_u8vec_to_string")] &'a Vec<u8>);

    let map = table.iter().map(|(k, v)| (Wrapper(k), v));
    serializer.collect_map(map)
}

fn deserialize_hashmap_with_u8vec_key<'de, D>(deserializer: D) -> Result<HashMap<Vec<u8>, Vec<usize>>, D::Error>
where
    D: Deserializer<'de>,
{
    fn deserialize_string_to_u8vec<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer).map_err(serde::de::Error::custom)?;
        Ok(s.as_bytes().to_owned()) //.map_err(serde::de::Error::custom)
        // &s[..]
        // let stack_str: &str = str::from_utf8(k).map_err(serde::de::Error::custom)?;
        // s.parse::<u32>().map_err(serde::de::Error::custom)
    }

    #[derive(Deserialize, Hash, Eq, PartialEq)]
    struct Wrapper(#[serde(deserialize_with = "deserialize_string_to_u8vec")] Vec<u8>);

    let v = HashMap::<Wrapper, Vec<usize>>::deserialize(deserializer)?;
    Ok(v.into_iter().map(|(Wrapper(k), v)| (k, v)).collect())
}



fn main() -> io::Result<()> {
    let mut file = File::open("foo.txt")?;
    let mut reader = BufReader::new(file);

    let mut blah: Blah;
    let mut n: usize;

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
        n = last_byte + 1;

        reader.seek(SeekFrom::Start(n as u64))?;
    } else {
        blah = Blah {
            table: HashMap::new(),
        };

        n = 0;
    }

    // let mut table: HashMap<String, Vec<(u32, usize)>> = HashMap::new();
    // let mut blah = Blah {
    //     table: HashMap::new(),
    // };

    // [0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}
    let uuid_regex_literal = r#"[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}"#;
    let uuid_regex: Regex = Regex::new(uuid_regex_literal).unwrap();

    let mut index: Vec<u32> = Vec::new();

    let mut lines = reader.byte_lines();
    for line in lines.into_iter() {
        let rline = line.unwrap();
        // let len = rline.len();
        let len = rline.len();
        // println!("{} {}", n, len);

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
