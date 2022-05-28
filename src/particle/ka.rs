//! # '~ 가' 또는 '~이'인지 판단하는 모듈
//!
//! - '가'는 받침 없는 체언 뒤에 붙습니다.
//! - '이'는 받침 있는 체언 뒤에 붙습니다.
//! - 외국어가 앞 단어로 오는 경우 병기 '(이)가'이 출력됩니다.
//!

use crate::{find_last_letter, is_hangeul, include_final};

pub fn change(word: &str) -> String {
    let last = find_last_letter(word);
    if !is_hangeul(last) {
        return "(이)가".to_string()
    }
    if  include_final(last) {
        "이".to_string()
    } else {
        "가".to_string()
    }
}
