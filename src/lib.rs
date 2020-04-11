use std::fs::File;
use std::io::{BufReader};

use serde::{Serializer, Deserializer};
use serde::{Serialize, Deserialize};

// use hashbrown::hash_map::HashMap;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct SimpleIndex {
    #[serde(flatten)]
    #[serde(serialize_with = "serialize_hashmap_with_u8vec_key", deserialize_with = "deserialize_hashmap_with_u8vec_key")]
    pub table: HashMap<Vec<u8>, Vec<usize>>
}

impl SimpleIndex {
  pub fn new() -> SimpleIndex {
    SimpleIndex {
      table: HashMap::new(),
    }
  }

  pub fn load_index() -> SimpleIndex {
    if let Ok(file) = File::open("foo.idx.json") {
      let reader = BufReader::new(file);
      serde_json::from_reader(reader).unwrap()
    } else {
      SimpleIndex::new()
    }
  }
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
