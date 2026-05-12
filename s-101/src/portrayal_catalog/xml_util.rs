//! Shared XML helpers used by the manifest and color-profile parsers.

/// Look up `local`-named child element text, trimmed, dropping empty strings.
pub(crate) fn child_text(node: roxmltree::Node<'_, '_>, local: &str) -> Option<String> {
    node.children()
        .find(|n| n.tag_name().name() == local)
        .and_then(|n| n.text())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// Decode XML bytes, handling the `ISO-8859-1` declaration used by IHO's
/// `portrayal_catalogue.xml`. roxmltree needs UTF-8 input, so for non-UTF-8 inputs we
/// transcode byte-by-byte (Latin-1 codepoints map 1:1 to Unicode) and rewrite the
/// XML declaration to `encoding="UTF-8"`.
pub(crate) fn decode_xml_string(xml: &[u8]) -> Result<String, std::str::Utf8Error> {
    let declared_iso_8859 = declares_iso_8859_1(xml);

    if !declared_iso_8859
        && let Ok(s) = std::str::from_utf8(xml)
    {
        return Ok(s.to_string());
    }

    let mut decoded: String = xml.iter().map(|&b| b as char).collect();
    if let Some(decl_end) = decoded.find("?>") {
        let head = &decoded[..decl_end];
        if head.contains("encoding=") {
            decoded.replace_range(..decl_end + 2, r#"<?xml version="1.0" encoding="UTF-8"?>"#);
        }
    }
    Ok(decoded)
}

fn declares_iso_8859_1(xml: &[u8]) -> bool {
    let head: &[u8] = if xml.len() > 256 { &xml[..256] } else { xml };
    let Ok(s) = std::str::from_utf8(head) else {
        return false;
    };
    let Some(idx) = s.find("encoding=") else {
        return false;
    };
    let tail = &s[idx + "encoding=".len()..];
    let tail = tail.trim_start_matches(['"', '\'']);
    tail.to_ascii_lowercase().starts_with("iso-8859")
}
