mod filter;
mod hangeul;
mod identifier;
mod number;
mod particle;
mod verifier;


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

// number 모듈
pub fn change_num_to_hangeul(word: &str) -> String {
    number::change_num_to_hangeul(word)
}

pub fn change_int_char(num: char) -> char {
    number::change_int_char(num)
}

// 테스트
pub fn postfix(word: &str, tossi: &str) -> String {
    identifier::postfix(word, tossi)
}

// verifier 모듈
pub fn verifiers(word: &str, tossi: &str) {
    verifier::verifiers(word, tossi)
}