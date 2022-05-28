//! # '~ 다' 또는 '~ 이다'인지 판단하는 모듈
//!
//! - '로'는 받침 없는 체언이나 ‘ㄹ’ 받침으로 끝나는 체언 뒤에 붙습니다.
//! - '으로'는 ‘ㄹ’을 제외한 받침 있는 체언 뒤에 붙습니다.
//!

use crate::find_last_letter;
use crate::split_phonemes;

pub fn change(word: &str) -> String {
    let last = find_last_letter(word);
    let last = split_phonemes(last);
    if last[1] == ' ' && last[2] == ' ' {
        "(이)다".to_string()
    } else if last[2] == 'ㄹ' || last[2] == ' ' {
        "다".to_string()
    } else {
        "이다".to_string()
    }
}
