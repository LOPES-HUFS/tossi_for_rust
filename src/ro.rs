//! # '~ 로' 또는 '~ 으로'인지 판단하는 모듈
//!
//! 아래와 같은 형식으로 입력된 것 중 두 번째 입력된 토시가 어떤 종류인지 파악합니다.
//!
//! https://github.com/LOPES-HUFS/tossi_for_rust/wiki/'~-로---~-으로'에-대하여
//!

use crate::find_last_letter;
use crate::split_phonemes;

pub fn change(word: &str) -> String {
    let last_letter = find_last_letter(word);
    let last_phonemes = split_phonemes(last_letter);

    if last_phonemes[2] == 'ㄹ' {
        "로".to_string()
    } else if last_phonemes[2] == ' ' {
        "로".to_string()
    } else {
        "으로".to_string()
    }
}
