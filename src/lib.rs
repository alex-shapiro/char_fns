//! This crate provides the `CharFns` trait which is implemented
//! for `&str`s and provides methods for unicode character-indexed
//! string manipulation.

/// Provides methods for unicode character-indexed string
/// manipulation.
pub trait CharFns {

    /// Returns the number of unicode characters in the string.
    fn char_len(&self) -> usize;

    /// Splits the string at the unicode character index.
    fn char_split(&self, index: usize) -> (&str, &str);

    /// Replaces a range of unicode characters with a new substring.
    fn char_replace(&self, index: usize, len: usize, text: &str) -> String;
}

impl CharFns for str {
    fn char_len(&self) -> usize {
        self.chars().count()
    }

    fn char_split(&self, index: usize) -> (&str, &str) {
        self.split_at(byte_index(self, index))
    }

    fn char_replace(&self, index: usize, len: usize, text: &str) -> String {
        let (pre, remaining) = self.char_split(index);
        let (_, post) = remaining.char_split(len);
        format!("{}{}{}", pre, text, post)
    }
}

#[inline]
fn byte_index(string: &str, char_index: usize) -> usize {
    let mut bidx = 0;
    let mut cidx = 0;

    for b in string.bytes() {
        // This is bit magic equivalent to: b < 128 || b >= 192.
        // It checks whether the current byte index is a character
        // boundary.
        if (b as i8) >= -0x40 {
            if cidx == char_index { break }
            cidx += 1;
        }
        bidx += 1
    }
    bidx
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_len() {
        assert!("".char_len() == 0);
        assert!("hello".char_len() == 5);
        assert!("😝😀".char_len() == 2);
    }

    #[test]
    fn test_split() {
        assert!("".char_split(0) == ("", ""));
        assert!("hello".char_split(2) == ("he", "llo"));
        assert!("🤗🍋🍠ñXwow₮".char_split(5) == ("🤗🍋🍠ñX","wow₮"));
    }

    #[test]
    fn test_replace_empty() {
        assert!("".char_replace(0, 0, "") == "");
        assert!("".char_replace(0, 0, "₢₸") == "₢₸");
    }

    #[test]
    fn test_replace_delete_only() {
        assert!("he∅⊆⊇o".char_replace(0, 3, "") == "⊆⊇o");
        assert!("he∅⊆⊇o".char_replace(2, 2, "") == "he⊇o");
        assert!("he∅⊆⊇o".char_replace(4, 2, "") == "he∅⊆");
    }

    #[test]
    fn test_replace_insert_only() {
        assert!("🤗🍋🍠ñXwow₮".char_replace(0, 0, "⊆⊇o") == "⊆⊇o🤗🍋🍠ñXwow₮");
        assert!("🤗🍋🍠ñXwow₮".char_replace(1, 0, "⊆⊇o") == "🤗⊆⊇o🍋🍠ñXwow₮");
        assert!("🤗🍋🍠ñXwow₮".char_replace(9, 0, "⊆⊇o") == "🤗🍋🍠ñXwow₮⊆⊇o");
    }

    #[test]
    fn test_replace_delete_and_insert() {
        assert!("🤗🍋🍠ñXwow₮".char_replace(0, 2, "⊆⊇o") == "⊆⊇o🍠ñXwow₮");
        assert!("🤗🍋🍠ñXwow₮".char_replace(4, 4, "⊆⊇o") == "🤗🍋🍠ñ⊆⊇o₮");
        assert!("🤗🍋🍠ñXwow₮".char_replace(1, 8, "⊆⊇o") == "🤗⊆⊇o");
    }
}
