//! # 기타 문자들을 처리해주는 모듈 -> 숫자는 number 모듈로 이동
//!
//! ## guess_final
//! 종성만 찾아서 도출해주는 함수
//! ```text
//! ex) 류 -> ' '
//! ex) 영 -> 'ㅇ'
//! ex) K(한글이 아닌 경우) -> 'N'
//! ```
//!
//! ## find_last_letter
//! 단어에서 마지막 글자를 찾아주는 함수.
//! 불필요한 요소를 삭제한 후 그 결과에서 마지막 글자를 반환한다.  
//! ```text
//! ex) 넥슨(코리아) -> 넥슨 -> 슨  
//! ex) 비타500 -> 비타오영영 -> 영  
//! ```
//!
//! ## filter_only_significant
//! 단어에서 불필요한 요소(기호 등)들을 제거하는 함수.
//! 기호인 경우 삭제하며, 괄호에 들어간 글자들도 삭제한다.  
//! 숫자인 경우 숫자의 한글발음으로 변경해준다.  
//! ```text
//! ex) 넥슨(코리아) -> [넥,슨]  
//! ex) 비타500 -> [비,타,오,영,영]  
//! ```
//!
//! ## is_digits
//! 숫자인지 아닌지 확인하는 함수.
//! ```text
//! ex) 500 -> True  
//! ```
//!
//! ## change_int_char
//! 숫자를 한글발음으로 변환해주는 함수.
//! ```text
//! ex) 5 -> 오  
//! ```

use crate::hangeul::{is_hangeul, split_phonemes};
use crate::number::{is_digits, change_int_char};

// ## 종성만 찾아서 도출해주는 함수
// 이 함수는 특정 글자의 종성만 도출합니다.
#[allow(dead_code)]
pub fn guess_final(word: &str) -> char {
    let filtered = find_last_letter(word);
    // find_last_letter()은 한글이나 숫자가 없을 경우 ' '을 출력한다.
    if filtered == 'N' {
        return 'N';
    } else {
        return split_phonemes(filtered)[2];
    }
}

/// ## 단어에서 마지막 글자를 찾아주는 함수
/// 'N'을 도출한 경우 영어 포함 외국어이다. -> 병기로 연결
pub fn find_last_letter(word: &str) -> char {
    let filtered = filter_only_significant(word);
    if filtered.len() > 0 {
        return filtered[filtered.len() - 1];
    } else {
        return 'N';
    }
}

/// ##단어에서 불필요한 요소 제거하는 함수
pub fn filter_only_significant(word: &str) -> Vec<char> {
    let mut output: Vec<char> = Vec::new();
    let mut bracket: bool = false;
    for c in word.chars() {
        //괄호 있는지 확인
        if c == '(' {
            bracket = true;
        } else if c == ')' {
            bracket = false;
        }
        //한글이 아니라면 제거
        if bracket {
            continue;
        } else if is_hangeul(c) {
            output.push(c);
        } else if is_digits(c) {
            let num = change_int_char(c);
            output.push(num);
        }
    }
    return output;
}

/// 비 공개 함수 테스트
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _filter_only_significant() {
        let temp = "넥슨(코리아)";
        let result = vec!['넥', '슨'];
        assert_eq!(result, filter_only_significant(temp));
    }
}
