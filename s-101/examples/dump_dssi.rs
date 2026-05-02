//! Hex-dump first DSSI payload for empirical layout inspection.

use iso8211::DataDescriptiveFile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1).ok_or("usage: dump_dssi <.000>")?;
    let ddf = DataDescriptiveFile::read(path)?;
    for (i, dr) in ddf.data_records().iter().enumerate() {
        for (tag, df) in dr.field_tags.iter().zip(dr.data_fields.iter()) {
            if tag == "DSSI" {
                println!("record {i} DSSI {} bytes:", df.user_data().len());
                for chunk in df.user_data().chunks(16) {
                    println!(
                        "  {}",
                        chunk.iter().map(|b| format!("{b:02x}")).collect::<Vec<_>>().join(" ")
                    );
                }
                return Ok(());
            }
        }
    }
    println!("no DSSI found");
    Ok(())
}
