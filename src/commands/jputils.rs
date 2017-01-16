extern crate nipponium;

use self::nipponium::romaji;

pub fn to_romaji(text: &str) -> String {
    let mut content = text.replace("!romaji", "");
    return romaji::to_romaji(content.trim());
}

pub fn from_romaji(text: &str) -> String {
    let mut content = text.replace("!hiragana", "");
    return romaji::from_romaji(content.trim());
}
