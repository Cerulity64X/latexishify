/// Transposes a char from one base to another.
/// 
/// An example would be:\
/// ch matches `'a'..='z'` checked beforehand,\
/// base = 'a',\
/// and to = 'A',\
/// This would capitalize the character (`'a'..='z'` to `'A'..='Z'`)
pub fn trans_char(ch: char, base: char, to: char) -> Option<char> {
    char::from_u32(ch as u32 - base as u32 + to as u32)
}

/// See `latexishify`.
pub fn char_latexishify(ch: char) -> char {
    match ch {
        // h is not included in the math symbols italicized block (??????????)
        'h' => 'ℎ',
        'a'..='z' => trans_char(ch, 'a', '𝑎').unwrap(),
        'A'..='Z' => trans_char(ch, 'A', '𝐴').unwrap(),

        'α'..='ω' => trans_char(ch, 'α', '𝛺').unwrap(),
        'Α'..='Ω' => trans_char(ch, 'Α', '𝛢').unwrap(),

        '0'..='9' => trans_char(ch, '0', '𝟶').unwrap(),

        other => other,
    }
}

/// Characters that can be represented as characters in the Unicode "Mathematical Alphanumeric Symbols" block (`1D400-1D7FF`) will be converted to their corresponding versions (italicized, serif, unbolded). Otherwise, they are left untouched.
///
/// Converts Latin, Greek and numeric characters.
pub fn latexishify(string: &str) -> String {
    string
        .chars()
        .map(|c| char_latexishify(c))
        .collect::<String>()
}
