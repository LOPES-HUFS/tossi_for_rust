//! # '~ 는' 또는 '~은'인지 판단하는 모듈
//!
//! - '는'는 받침 없는 체언 뒤에 붙습니다.
//! - '은'는 받침 있는 체언 뒤에 붙습니다.
//! - 외국어가 앞 단어로 오는 경우 병기 '(은)는'이 출력됩니다.
//!

use crate::find_last_letter;
use crate::split_phonemes;

pub fn change(word: &str) -> String {
    let last = find_last_letter(word);
    let last = split_phonemes(last);
    if last[1] == ' ' && last[2] == ' ' {
        "(은)는".to_string()
    } else if  last[2] == ' ' {
        "는".to_string()
    } else {
        "은".to_string()
    }
}
