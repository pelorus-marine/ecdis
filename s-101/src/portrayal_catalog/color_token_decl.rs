//! `<color token="…" name="…">` declaration from the top of `colorProfile.xml`.

/// `<color token="CHRED" name="red"><description>…</description></color>` declaration.
#[derive(Debug, Clone, Default)]
pub struct ColorTokenDecl {
    pub token: String,
    pub name: String,
    pub description: Option<String>,
}
