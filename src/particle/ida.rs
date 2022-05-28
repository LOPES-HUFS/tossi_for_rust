//! # '~ 다' 또는 '~ 이다'인지 판단하는 모듈
//!
//! - '로'는 받침 없는 체언이나 ‘ㄹ’ 받침으로 끝나는 체언 뒤에 붙습니다.
//! - '으로'는 ‘ㄹ’을 제외한 받침 있는 체언 뒤에 붙습니다.
//!

use crate::{find_last_letter, is_hangeul, include_final};

pub fn change(word: &str) -> String {
    let last = find_last_letter(word);
    if !is_hangeul(last) {
        return "(이)다".to_string()
    }
    if  include_final(last) {
        "이다".to_string()
    } else {
        "다".to_string()
    }
}
