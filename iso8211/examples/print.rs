use iso8211::DataDescriptiveFile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1).ok_or("usage: print <path-to-iso8211-file>")?;

    let ddf = DataDescriptiveFile::read(path)?;

    println!("\nData Descriptive Record:");
    println!("  File Control Field:");
    println!("    Tag Pairs:");
    for tp in ddf.data_descriptive_record().file_control_field().tag_pairs() {
        println!("      {} -> {}", tp.0, tp.1);
    }

    println!("    Data Descriptive Fields:");
    for f in ddf.data_descriptive_record().data_descriptive_fields() {
        println!("      Field Controls:");
        println!(
            "        Data Structure: {}",
            f.field_controls().data_structure()
        );
        println!("        Data Type: {}", f.field_controls().data_type());
        println!(
            "        Escape Sequence: {}",
            f.field_controls().escape_sequence()
        );

        println!("      Field Name: {}", f.field_name());

        println!("      Array Descriptor: {}", f.array_descriptor());

        println!("      Format Controls:");
        for fc in f.format_controls().formats() {
            println!("        Format: {}", fc);
        }
    }

    for dr in ddf.data_records() {
        println!("\nData Record:");
        for df in dr.data_fields() {
            println!("  Data Field: {} bytes", df.user_data().len());
        }
    }

    Ok(())
}
