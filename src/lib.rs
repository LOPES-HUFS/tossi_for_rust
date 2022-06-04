mod filter;
mod hangeul;
mod identifier;
mod particle;

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

pub fn guess_final(word: &str) -> char {
    filter::guess_final(word)
}
// 테스트
pub fn postfix(word: &str, tossi: &str) -> String {
    identifier::postfix(word, tossi)
}
