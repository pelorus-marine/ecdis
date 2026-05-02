//! Hex-dump first C2IL payload.

use iso8211::DataDescriptiveFile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1).ok_or("usage: dump_c2il <.000>")?;
    let ddf = DataDescriptiveFile::read(path)?;
    for (i, dr) in ddf.data_records().iter().enumerate() {
        for (tag, df) in dr.field_tags.iter().zip(dr.data_fields.iter()) {
            if tag == "C2IL" {
                println!("record {i} C2IL {} bytes:", df.user_data().len());
                let p = df.user_data();
                for chunk in p.chunks(16) {
                    println!(
                        "  {}",
                        chunk.iter().map(|b| format!("{b:02x}")).collect::<Vec<_>>().join(" ")
                    );
                }
                // Decode first few points as i32 pairs
                let mut off = 0usize;
                let mut n = 0usize;
                while off + 8 <= p.len() && n < 4 {
                    let y = i32::from_le_bytes(p[off..off + 4].try_into().unwrap());
                    let x = i32::from_le_bytes(p[off + 4..off + 8].try_into().unwrap());
                    println!("  pair{n}: Y={y} X={x}");
                    off += 8;
                    n += 1;
                }
                return Ok(());
            }
        }
    }
    println!("no C2IL found");
    Ok(())
}
