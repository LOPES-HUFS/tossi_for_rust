mod filter;
mod hangeul;
mod identify;

mod ro;

use identify::Tossi;

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
    let temp = Tossi::new(tossi);
    println!(
        "입력된 토시: {:?}, 토시 종류: {:?}",
        temp.modified, temp.kind
    );

    let result = match temp.kind {
        identify::TossiKind::Neun => " ".to_string(),
        identify::TossiKind::Ka => " ".to_string(),
        identify::TossiKind::Ro => ro::change(&word),
        identify::TossiKind::None => " ".to_string(),
    };

    let front = word.to_string();
    front + &result
}
