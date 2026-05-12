//! Print hex for selected tags on a given record index (debug helper).

use s_101::S101Dataset;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let path = args.next().ok_or("usage: dump_field_hex <.000> [record_idx]")?;
    let idx: usize = args.next().map(|s| s.parse()).transpose()?.unwrap_or(446);

    let d = S101Dataset::load(&path)?;
    let rec = d.iso8211().data_records().get(idx).ok_or("record index out of range")?;
    for (t, f) in rec.field_tags.iter().zip(rec.data_fields()) {
        if matches!(
            t.as_str(),
            "DSID"
                | "FTCS"
                | "FRID"
                | "FOID"
                | "ATTR"
                | "SPAS"
                | "PRID"
                | "C2IT"
                | "CRID"
                | "SRID"
                | "RIAS"
                | "SEGH"
                | "PTAS"
                | "C2IL"
                | "CCID"
                | "CUCO"
                | "MRID"
                | "C3IL"
        ) {
            let b = f.user_data();
            println!("{t} len={} first32={:02x?}", b.len(), &b[..b.len().min(32)]);
        }
    }
    Ok(())
}
