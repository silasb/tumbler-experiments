use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use json_tools::BufferType;
use json_tools::Lexer;
use json_tools::FilterTypedKeyValuePairs;
use json_tools::TokenType;
use json_tools::IteratorExt;
use json_tools::{Span, Buffer};


fn main() -> io::Result<()> {
    let file = File::open("examples/json.sql")?;
    let reader = BufReader::new(file);

    let mut open_brace: usize = 0;
    let mut close_brace: usize = 0;;

    for line in reader.lines() {
      let line = line.unwrap();
      println!("{:#?}", line);
      let mut json = vec![];
      for token in Lexer::new(line.bytes(), BufferType::Span) {
        match token.kind {
          TokenType::CurlyOpen => {
            println!("{:?}", token);
            if let Buffer::Span(x) = token.buf {
              open_brace = x.first as usize;
              // json.push(x.first as usize);
            }
          },
          TokenType::CurlyClose => {
            println!("{:?}", token);
            if let Buffer::Span(x) = token.buf {
              close_brace = x.end as usize;
              json.push(open_brace..close_brace);
            }
          },
          _ => {}
        }
      }

      println!("start/stop json markers for {:#?}", json);
      if json.len() > 0 {
        println!("{}", &line[json[0].to_owned()])
      }
    }

    // for token in Lexer::new(r#"{ "face": "😂" }"#.bytes(), BufferType::Span) {
    //   println!("{:?}", token);
    // }

    Ok(())
}
