//! Print `(record_idx, field_tag)` pairs for inspecting ENC interchange layout.

use iso8211::DataDescriptiveFile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1).ok_or("usage: dump_record_tags <.000>")?;
    let ddf = DataDescriptiveFile::read(path)?;

    for (i, dr) in ddf.data_records().iter().enumerate() {
        for tag in &dr.field_tags {
            println!("{i}\t{tag}");
        }
    }
    Ok(())
}
