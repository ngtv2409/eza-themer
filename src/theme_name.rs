/// Validate amd canonicallize theme name
use std::path::PathBuf;

#[derive(Debug)]
pub struct ThemeName {
    s: String,
}

impl ThemeName {
    /// creates a canonical theme name string
    pub fn from_str(s: &str) -> Option<Self> {
        if !is_valid_theme_name(s) {
            return None;
        }
        let s: String = canon_sep(&s.to_ascii_lowercase());
        if s.is_empty() {
            return None;
        }

        Some(Self { s: s })
    }

    pub fn prettify(&self) -> String {
        self.s
            .split('-')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                    None => String::new(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn to_filename(&self) -> PathBuf {
        let p = PathBuf::from(&self.s);
        p.with_extension("yml")
    }
}

fn canon_sep(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut last_was_sep = false;

    for c in input.chars() {
        let is_sep = c == '-' || c == '_' || c == ' ';

        if is_sep {
            if !last_was_sep {
                out.push('-');
                last_was_sep = true;
            }
        } else {
            out.push(c);
            last_was_sep = false;
        }
    }

    out.trim_matches('-').to_string()
}

fn is_valid_theme_name(s: &str) -> bool {
    s.bytes().all(|b| {
        matches!(b,
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' | b'-' | b' '
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_theme_name_should_success() {
        assert_eq!(is_valid_theme_name("onedark"), true);
        assert_eq!(is_valid_theme_name("one -dark"), true);
        assert_eq!(is_valid_theme_name("One _dark123"), true);
    }
    #[test]
    fn is_valid_theme_name_should_fail() {
        assert_eq!(is_valid_theme_name("one@dark"), false);
        assert_eq!(is_valid_theme_name("one $dark"), false);
        assert_eq!(is_valid_theme_name("One] -dark"), false);
    }
    #[test]
    fn canon_sep_expected_result() {
        assert_eq!(&canon_sep("w1-w2"), "w1-w2");
        assert_eq!(&canon_sep("w1 w2"), "w1-w2");
        assert_eq!(&canon_sep("-w1_w2_"), "w1-w2");
    }
}
