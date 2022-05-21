mod filter;
mod hangeul;

// hangeul 모듈
pub fn join_phonemes(word: [char; 3]) -> char {
    hangeul::join_phonemes(word)
}

pub fn split_phonemes(word: char) -> [char; 3] {
    hangeul::split_phonemes(word)
}

// filter 모듈
pub fn find_last_letter(word: &str) -> char {
    filter::find_last_letter(word)
}

// 테스트
pub fn postfix(word: &str, tossi: &str) -> String {
    format!("{}{}", word, tossi)
}
