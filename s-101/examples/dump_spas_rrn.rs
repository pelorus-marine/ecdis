//! List distinct **SPAS** `RRNM` values on feature records.

use s_101::{S101Dataset, record::Record};
use std::collections::HashSet;

fn main() {
    let p = std::env::args().nth(1).expect("path .000");
    let d = S101Dataset::load(p).unwrap();
    let mut rrns = HashSet::new();
    for r in d.typed_records() {
        let Record::Feature(f) = r else {
            continue;
        };
        for s in &f.spatial {
            rrns.insert(s.rrn);
        }
    }
    let mut v: Vec<_> = rrns.into_iter().collect();
    v.sort_unstable();
    println!("{v:?}");
}
