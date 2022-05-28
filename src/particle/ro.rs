//! # '~ 로' 또는 '~ 으로'인지 판단하는 모듈
//!
//! - '로'는 받침 없는 체언이나 ‘ㄹ’ 받침으로 끝나는 체언 뒤에 붙습니다.
//! - '으로'는 ‘ㄹ’을 제외한 받침 있는 체언 뒤에 붙습니다.
//! - 외국어가 앞 단어로 오는 경우 병기 '(으)로'가 출력됩니다.
//!
//! https://github.com/LOPES-HUFS/tossi_for_rust/wiki/'~-로---~-으로'에-대하여
//!

use crate::find_last_letter;
use crate::split_phonemes;

pub fn change(word: &str) -> String {
    let last = find_last_letter(word);
    let last = split_phonemes(last);
    if last[1] == ' ' && last[2] == ' ' {
        "(으)로".to_string()
    } else if last[2] == 'ㄹ' || last[2] == ' ' {
        "로".to_string()
    } else {
        "으로".to_string()
    }
}
