//! Optional load test when `../../testdata/s101_sample.000` exists.
use std::path::Path;

use s_101::S101Dataset;

#[test]
fn load_workspace_enc_fixture_if_present() {
    let p = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../testdata/s101_sample.000");
    if !p.exists() {
        return;
    }
    let ds = S101Dataset::load(&p).expect("S-101 load should succeed for workspace fixture");
    assert!(
        ds.record_count() > 0,
        "fixture should contain at least one data record"
    );
    assert!(
        ds.first_record_dsid_payload().is_some(),
        "first record should expose DSID payload"
    );
}
