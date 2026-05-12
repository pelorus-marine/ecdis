//! Debug first feature graph geometry for a dataset + FC path.

use s_101::{FeatureCatalogue, S101Dataset};

fn main() {
    let enc = std::env::args().nth(1).expect("enc .000");
    let fc_bytes = std::process::Command::new("unzip")
        .args([
            "-p",
            "/tmp/S-64_1.2.0.zip",
            "S-100/InitialCatalogues/S100_ROOT/S-101/CATALOGUES/S-101_1.0.2_20220524.xml",
        ])
        .output()
        .expect("unzip")
        .stdout;
    let fc = FeatureCatalogue::parse_xml(&fc_bytes).unwrap();
    let d = S101Dataset::load(&enc).unwrap();
    let g = d.build_feature_graph(&fc).unwrap();
    let empty = g.features.iter().filter(|f| f.geometry.is_empty()).count();
    println!("features={} empty_geom={}", g.features.len(), empty);
    if let Some(f) = g.features.first() {
        println!(
            "first foid {:?} class={:?}",
            f.foid,
            f.class.map(|c| c.code.as_str())
        );
        println!("first geom empty {}", f.geometry.is_empty());
    }
}
