use std::fs::File;
use std::io::{self, prelude::*, BufReader};
// use std::io::SeekFrom;

// use regex::bytes::Regex;

// use sqlparser::dialect::MySqlDialect;
// use sqlparser::parser::Parser;

// use std::collections::HashMap;

use nom_sql::{parser, Literal};

// use serde_derive::Deserialize;

// #[derive(Deserialize)]
// struct Config {
//     ip: String,
//     port: Option<u16>,
//     keys: Keys,
// }

// #[derive(Deserialize)]
// struct Tables {

// }

use toml::{Value};

fn main() -> io::Result<()> {
    let mut config_file = File::open("tumble.toml")?;
    let mut toml_content = String::new();
    config_file.read_to_string(&mut toml_content)?;
    let config: Value = toml::from_str(&toml_content)?;

    let file = File::open("foo.txt")?;
    let reader = BufReader::new(file);

    let mut out_file = File::create("foo.txt.out")?;

    // let mut uuid_index: SimpleIndex;
    // let mut n: usize;
    // let mut last_byte: usize = 0;

    // uuid_index = SimpleIndex::load_index();

    // let dialect = MySqlDialect {};

    // random content => random
    // mapper content => 0
    // stmt.data[0][mapper["content"]] = random["content"]


    let mut s = String::new();

    for line in reader.lines() {
        let rline = line.unwrap();

        if rline.ends_with(";") {
            s.push_str(&rline);
            // println!("{:#?}", s);
            let ast = parser::parse_query(&s).unwrap();
            match ast {
              parser::SqlQuery::CreateTable(stmt) => {
                // hash.entry(stmt.table.docs.to_owned()).or_insert()
                let _fields: Vec<String> = stmt.to_owned().fields.into_iter().map(|m| m.column.name).collect();
                // println!("{:#?}", stmt.fields);
                out_file.write_all(format!("{};\n", stmt.to_string()).as_bytes())?;
              },
              parser::SqlQuery::Insert(mut stmt) => {
                let table_name = stmt.to_owned().table.name;
                let fields: Vec<(usize, String)> = stmt.to_owned().fields.unwrap().into_iter()
                  .enumerate()
                  .filter(|(_, m)| m.name == "content")
                  .map(|(i, m)| (i, m.name))
                  .collect();

                for (i, field) in fields {
                  if let Literal::String(string) = &mut stmt.data[0][i] {
                    // let field = hash.get(&field).unwrap();
                    if let Value::String(conversion) = &config[&table_name[..]][field] {
                      // println!("from: {:#?} to: {:#?}", string, conversion);
                      // stmt.data[0][i] = Literal::String(conversion.to_string());
                      *string = conversion.to_string();
                    }
                  }
                }
                // stmt.fields[0][fields[0]]
                // println!("{:#?}", stmt.to_string());
                out_file.write_all(format!("{};\n", stmt.to_string()).as_bytes())?;
              },
              _ => { }
            }
            // println!("{:#?}", ast);
            s.clear();
        } else {
            if !rline.starts_with("--") {
                s.push_str(&rline);
            }
        }

        // let ast = Parser::parse_sql(&dialect, rline.to_string()).unwrap();

        // for caps in uuid_regex.captures_iter(&rline) {
        //     for cap in caps.iter() {

        //         let capture = cap.unwrap();

        //         // println!("{:#?}", capture.as_bytes().to_owned());
        //         let entry = uuid_index.table.entry(capture.as_bytes().to_owned()).or_insert(Vec::new());
        //         (*entry).push(n + capture.start());
        //     }
        // }
        // n += len + 1; // taking into account the 1 byte newline
    }

    Ok(())
}
