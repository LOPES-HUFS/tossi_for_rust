mod filter;
mod hangeul;
mod identifier;
mod number;
mod particle;
mod verifier;

use crate::particle::*;
use identifier::{Tossi, TossiKind};

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

/// ## 입력된 토시(tossi)가 어떤 것인지 알아내 입력된 값과 반환하는 함수
///
/// 아래와 같은 형식으로 입력된 것 중 두 번째 입력된 토시가 어떤 종류인지 파악합니다.
/// 이 프로젝트에서 구현하고자 하는
/// `postfix()`와 `pick()`를 이 함수를 이용해서 구현하고 있습니다.
///
/// ```rust
/// use library::postfix;
/// postfix("집", "(으)로");
/// postfix("집", "로");
/// postfix("집", "(으)로");
/// ```

fn postfix_raw(word: &str, tossi: &str) -> (String, String) {
    //파라미터에 올바른 규격의 값이 들어왔는지 확인하기
    verifiers(word, tossi);
    let temp = Tossi::new(tossi);
    let result = match temp.kind {
        TossiKind::Neun => neun::change(&word),
        TossiKind::Ka => ka::change(&word),
        TossiKind::Ro => ro::change(&word),
        TossiKind::Ida => ida::change(&word),
        TossiKind::Eul => eul::change(&word),
        TossiKind::None => tossi.to_string(),
    };

    let front = word.to_string();
    (front, result)
}

/// postfix() 함수
pub fn postfix(word: &str, tossi: &str) -> String {
    let temp = postfix_raw(word, tossi);
    temp.0 + &temp.1
}

/// pick() 함수
pub fn pick(word: &str, tossi: &str) -> String {
    let temp = postfix_raw(word, tossi);
    return temp.1;
}

// verifier 모듈
pub fn verifiers(word: &str, tossi: &str) {
    verifier::verifiers(word, tossi)
}
