use memmap::MmapOptions;
use std::fs::OpenOptions;
use std::io::{self};


// use std::sync::atomic::{AtomicU128, Ordering};

use std::collections::HashMap;

use blah::SimpleIndex;

// https://stackoverflow.com/questions/28516996/how-to-create-and-write-to-memory-mapped-files
// https://docs.rs/uuid/0.8.1/src/uuid/adapter/mod.rs.html
mod sequencial_uid {
    use lazy_static::lazy_static; // 1.4.0
    use std::sync::Mutex;
    use std::mem::transmute;

    const UPPER: [u8; 16] = [
      b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B',
      b'C', b'D', b'E', b'F',
    ];
    const LOWER: [u8; 16] = [
        b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b',
        b'c', b'd', b'e', b'f',
    ];

    /// The segments of a UUID's [u8; 16] corresponding to each group.
    const BYTE_POSITIONS: [usize; 6] = [0, 4, 6, 8, 10, 16];
    /// The locations that hyphens are written into the buffer, after each
    /// group.
    const HYPHEN_POSITIONS: [usize; 4] = [8, 13, 18, 23];

    lazy_static! {
      static ref GLOBAL_NUM: Mutex<u128> = Mutex::new(0);
    }

    pub fn gen_random_uuid_as_string() -> String {
      let upper = true;
      let hyphens = true;

      let len = if hyphens { 36 } else { 32 };

      // let buffer = &mut full_buffer[start..start + len];
      let mut buffer = vec![0; len];
      let bytes: [u8; 16] = unsafe { transmute(GLOBAL_NUM.lock().unwrap().to_be()) };
      *GLOBAL_NUM.lock().unwrap() += 1;

      let hex = if upper { &UPPER } else { &LOWER };

      for group in 0..5 {
        // If we're writing hyphens, we need to shift the output
        // location along by how many of them have been written
        // before this point. That's exactly the (0-indexed) group
        // number.
        let hyphens_before = if hyphens { group } else { 0 };
        for idx in BYTE_POSITIONS[group]..BYTE_POSITIONS[group + 1] {
            let b = bytes[idx];
            let out_idx = hyphens_before + 2 * idx;

            buffer[out_idx] = hex[(b >> 4) as usize];
            buffer[out_idx + 1] = hex[(b & 0b1111) as usize];
        }

        if group != 4 && hyphens {
            buffer[HYPHEN_POSITIONS[group]] = b'-';
        }
      }

      String::from_utf8(buffer).unwrap()
    }
}


fn main() -> io::Result<()> {
    // let args: Vec<String> = env::args().collect();

    let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open("foo.txt")?;

    let length = file.metadata().unwrap().len() as u64;
    file.set_len(length)?;

    let buffer = unsafe { MmapOptions::new().map(&file)? };
    let mut mut_buffer = buffer.make_mut()?;
    let uuid_index = SimpleIndex::load_index();
    let mut uuid_map = HashMap::new();

    for (_uuid, positions) in &uuid_index.table {
      let new_uid = uuid_map.entry(_uuid).or_insert(sequencial_uid::gen_random_uuid_as_string());

      for pos in positions {
        let range = *pos..(*pos+36);
        mut_buffer[range].copy_from_slice(new_uid.as_bytes());
      }
    }

    Ok(())
}
