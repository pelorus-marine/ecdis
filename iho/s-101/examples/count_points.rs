use s_101::{S101Dataset, record::Record};

fn main() {
    let p = std::env::args().nth(1).expect("path");
    let d = S101Dataset::load(&p).unwrap();
    let mut n = 0u64;
    let mut has_110_1 = false;
    for r in d.typed_records() {
        if let Record::Point(pt) = r {
            n += 1;
            if pt.id.rcnm == 110 && pt.id.rcid == 1 {
                has_110_1 = true;
            }
        }
    }
    println!("points={n} has_110_1={has_110_1}");
}
