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

// postfix() 함수
pub fn postfix(word: &str, tossi: &str) -> String {
    let temp = identifier::postfix(word, tossi);
    temp.0 + &temp.1
}

// pick() 함수
pub fn pick(word: &str, tossi: &str) -> String {
    let temp = identifier::postfix(word, tossi);
    return temp.1
}

// verifier 모듈
pub fn verifiers(word: &str, tossi: &str) {
    verifier::verifiers(word, tossi)
}
