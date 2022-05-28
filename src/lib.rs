mod filter;
mod hangeul;
mod identify;
mod particle;

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
        identify::TossiKind::Neun => particle::neun::change(&word),
        identify::TossiKind::Ka => particle::ka::change(&word),
        identify::TossiKind::Ro => particle::ro::change(&word),
        identify::TossiKind::Ida => " ".to_string(),
        identify::TossiKind::None => " ".to_string(),
    };

    let front = word.to_string();
    front + &result
}
