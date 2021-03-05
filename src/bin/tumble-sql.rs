#[macro_use]
extern crate log;

use std::fs::File;
use std::io::{self, prelude::*, BufReader};
// use std::io::SeekFrom;

// use regex::bytes::Regex;

// use sqlparser::dialect::MySqlDialect;
// use sqlparser::parser::Parser;

use std::collections::HashMap;

use nom_sql::{parser, Literal};

use fake::{Fake};
use fake::faker::name::raw::*;
use fake::locales::*;

use rayon::prelude::*;

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
use clap::{Arg,App};

fn main() -> io::Result<()> {
    env_logger::init();

        let args = App::new("sql-tumbler")
            .version("0.1.0")
            .about("your-app-description")
            .author("Silas Baronda")
            .args(&[
                  Arg::new("config")
                  .about("Config file")
                  .short('c')
                  .long("config")
                  .takes_value(true),
                  Arg::new("source")
                  .about("Source file")
                  .short('s')
                  .long("source")
                  .takes_value(true),
                  Arg::new("dest")
                  .about("Destination file")
                  .short('d')
                  .long("dest")
                  .takes_value(true),
            ]).get_matches();

    let mut config_file = File::open(args.value_of("config").unwrap())?;
    let mut toml_content = String::new();
    config_file.read_to_string(&mut toml_content)?;
    let config: Value = toml::from_str(&toml_content)?;

    let file = File::open(args.value_of("source").unwrap())?;
    let mut reader = BufReader::new(file);

    let mut out_file = File::create(args.value_of("dest").unwrap())?;

    println!("{:#?}", config);

    // return Ok(());

    // let mut uuid_index: SimpleIndex;
    // let mut n: usize;
    // let mut last_byte: usize = 0;

    // uuid_index = SimpleIndex::load_index();

    // let dialect = MySqlDialect {};

    // random content => random
    // mapper content => 0
    // stmt.data[0][mapper["content"]] = random["content"]

    // let previous = HashMap::new();

    // let random_stat_buff = |table_name, filed| {
    //   // config["docs"]["column1"]
    //   let data = Name(EN).fake::<String>();
    //   // println!("{}", config["docs"]["column1"]);
    //   println!("{}", data);

    //   data
    // };

    let mut manufacturer_map = HashMap::new();
    let mut table_columns = HashMap::new();

    let mut rline = String::new();

    let mut s = String::new();
    let mut table_config = None;
    let mut interesting_fields = None;

    let mut i = 0;
    let x = reader.split(';' as u8);
    // while reader.read_line(&mut rline).unwrap_or(0) > 0 {
    for line in x {
     //for (i, line) in reader.lines().enumerate() {
         let rline = line.unwrap();
        // let rline = line;
        //let s = rline;


         //println!("{}", rline);
        // if rline.ends_with("") {
        //     rline.pop(); // remove newline
         let line2 = String::from_utf8(rline).unwrap();
          s.push_str(&line2);
          // println!("{:#?}", s);
          match parser::parse_query(&s) {
            Ok(parser::SqlQuery::CreateTable(stmt)) => {
              let table_name = stmt.to_owned().table.name;
              // can set table_config here since we know that create statements come first
              table_config = Some(config.get(&table_name).expect("missing config"));

              let blah = table_config.expect("missing config mapping");
              match blah {
                  Value::Table(blah2) => {
                      let keys: Vec<_> = blah2.keys().cloned().collect();
                      println!("{:?}", keys);
                      interesting_fields = Some(keys);
                  }
                  (_) => {}
              }

              let fields: Vec<String> = stmt.to_owned().fields.into_iter().map(|m| m.column.name).collect();
              table_columns.entry(table_name).or_insert(fields);

              out_file.write_all(stmt.to_string().as_bytes())?;
              writeln!(out_file, ";")?;
              // out_file.write_all(format!("{};\n", stmt.to_string()).as_bytes())?;
            },
            Ok(parser::SqlQuery::Insert(mut stmt)) => {
              let table_name = stmt.to_owned().table.name;
               //println!("{:#?}", table_config);

              let fields: Vec<(usize, String)> = match stmt.to_owned().fields {
                Some(fields) => {

                    println!("{:?}", fields);

                  fields.into_iter()
                    .enumerate()
                    .filter(|(_, m)| {
                        let x = interesting_fields.as_ref().unwrap().iter().any(|i2| i2 == "driver");
                        println!("{:?}", x);
                        m.name == "driver"
                    })
                    .map(|(i, m)| (i, m.name))
                    .collect()
                },
                None => {
                  let fields = table_columns.get(&table_name).expect("trying to get fields from table but we have not scanned a table yet");

                  println!("{:?} {:?}", fields, interesting_fields);


                  fields.into_iter()
                    .enumerate()
                    .filter(|(_, m)| {
                        interesting_fields.as_ref().unwrap().iter().any(|i2| i2 == &m[..].to_string())
                    })
                    .map(|(i, m)| (i, m.to_owned()))
                    .collect()
                }
              };

              println!("{:?}", fields);

              for (i, field) in fields {
                if let Literal::String(data) = &mut stmt.data[0][i] {
                  // let field = hash.get(&field).unwrap();
                    let fake_data = manufacturer_map.entry(data.to_string())
                        .or_insert_with(|| {
                            // config["docs"]["column1"]
                            // if config["docs"][field]
                            let data = Name(EN).fake::<String>();
                            debug!("{}", config["docs"][field]);
                            debug!("{}", data);

                            data
                        });
                    debug!("from: {:#?} to: {:#?}", data, fake_data);

                    *data = fake_data.to_string();
                  //if let Value::String(conversion) = table_config.unwrap().get(field.to_owned()).expect("missing config mapping") {
                     //println!("from: {:#?} to: {:#?}", field, conversion);
                    // *data = conversion.to_string();
                  //}
                }
              }
              // stmt.fields[0][fields[0]]
              // println!("{:#?}", stmt.to_string());
              // writeln!(out_file, "{};", stmt)?;
              out_file.write_all(stmt.to_string().as_bytes())?;
              writeln!(out_file, ";")?;

              if i % 10_000 == 0 {
                println!("{} statements tumbled", i);
                if i == 200_000 {
                  return Ok(())
                }
              }
              if i == 5 {
                  return Ok(())
              }
              // out_file.write_all(format!("{};\n", stmt.to_string()).as_bytes())?;
            },
            _ => { println!("skipping: {}", i) }
          }
          // println!("{:#?}", ast);
          s.clear();
      // } else {
      //     if !rline.starts_with("--") {
      //         s.push_str(&rline);
      //     }
      // }

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
        i += 1;
        // rline.clear();
    }

    Ok(())
}
