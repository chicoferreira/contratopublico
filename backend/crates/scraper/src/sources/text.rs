use std::{ops::Deref, sync::LazyLock};

use encoding_rs::WINDOWS_1252;
use regex::{Captures, Regex};
use serde::{Deserialize, Deserializer, Serialize};

/// A C1 control character, optionally after the Latin-1 lead character of a UTF-8 letter.
static MOJIBAKE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[\x{C2}-\x{DF}]?[\x{80}-\x{9F}]").unwrap());

/// A newtype that fixes Windows-1252 encoding errors when deserialized.
///
/// It exists because both sources send some text with these errors, like `–` as `\u{96}`.
/// Instead of using [String] in their fields, we use this struct instead.
///
/// Example: "Lote 1 \u{96} Gás" becomes "Lote 1 – Gás"
#[derive(Debug, Default, Serialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct Text(String);

impl<'de> Deserialize<'de> for Text {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer).map(|text| Self(repair(text)))
    }
}

impl Text {
    pub fn non_empty(&self) -> Option<String> {
        (!self.trim().is_empty()).then(|| self.0.clone())
    }
}

impl Deref for Text {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl PartialEq<str> for Text {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

fn repair(text: String) -> String {
    if !MOJIBAKE.is_match(&text) {
        return text;
    }
    MOJIBAKE
        .replace_all(&text, |caps: &Captures| {
            let bytes: Vec<u8> = caps[0].chars().map(|c| c as u8).collect();
            let text = String::from_utf8(bytes).unwrap_or_else(|e| {
                WINDOWS_1252
                    .decode_without_bom_handling(e.as_bytes())
                    .0
                    .into_owned()
            });
            text.replace(char::is_control, "")
        })
        .into_owned()
}

#[cfg(test)]
mod tests {
    use super::repair;

    #[test]
    fn repairs_text_decoded_as_latin1() {
        let repaired = |text: &str| repair(text.to_string());
        assert_eq!(repaired("Lote 1 \u{96} Gás"), "Lote 1 – Gás");
        assert_eq!(repaired("\u{93}Guimarães\u{94}"), "“Guimarães”");
        assert_eq!(repaired("FÃ\u{81}TIMA"), "FÁTIMA");
        assert_eq!(repaired("AUTOMAÃ\u{87}Ã\u{83}O"), "AUTOMAÇÃO");
        assert_eq!(repaired("a\u{8d}b"), "ab");
        assert_eq!(repaired("Ã sem erros\u{a0}Â n.º"), "Ã sem erros\u{a0}Â n.º");
    }
}
