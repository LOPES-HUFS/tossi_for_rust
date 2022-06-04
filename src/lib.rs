mod filter;
mod hangeul;
mod identifier;
mod particle;

use identifier::Tossi;

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
    let temp = Tossi::new(tossi);
    println!(
        "입력된 토시: {:?}, 토시 종류: {:?}",
        temp.modified, temp.kind
    );

    let result = match temp.kind {
        identifier::TossiKind::Neun => particle::neun::change(&word),
        identifier::TossiKind::Ka => particle::ka::change(&word),
        identifier::TossiKind::Ro => particle::ro::change(&word),
        identifier::TossiKind::Ida => particle::ida::change(&word),
        identifier::TossiKind::None => " ".to_string(),
    };

    let front = word.to_string();
    front + &result
}
