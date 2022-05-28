//! # '~ 로' 또는 '~ 으로'인지 판단하는 모듈
//!
//! - '로'는 받침 없는 체언이나 ‘ㄹ’ 받침으로 끝나는 체언 뒤에 붙습니다.
//! - '으로'는 ‘ㄹ’을 제외한 받침 있는 체언 뒤에 붙습니다.
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
