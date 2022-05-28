//! # 입력된 토시(tossi)가 어떤 것인지 알아내는 모듈 
//! 
//! 아래와 같은 형식으로 입력된 것 중 두 번째 입력된 토시가 어떤 종류인지 파악합니다.
//! 
//! ```text
//! postfix('집', '(으)로')
//! postfix("집", "로"))
//! postfix("집", "(으)로")
//! ```
//! 
//! ## 조사 종류를 영어로 표기하기
//! 
//! 아래 로마자 표기법에 따라 변환했다.
//! 조사를 로마자로 변환했을 때 글자 수가 적은 것을 선택하여 이를 대표 이름으로 선택했다.
//!  
//! https://ko.wiktionary.org/wiki/부록:로마자_표기법/국어
//! 
//! - 은, 는: neun
//! - 이, 가: ka
//! - 으로, 로: ro

use crate::filter::filter_only_significant;

// enum Tossis {
//     Neun,
//     Ka,
// }

pub struct Tossi {
    pub modified: Vec<char>,
    pub kind: String,
}

impl Tossi {
    pub fn new(raw: &str) -> Self {
        let temp_modified = modify(raw);
        let temp_kind = test(temp_modified[0]);

        Self {
            modified: temp_modified,
            kind: temp_kind,
        }
    }
}

fn modify(raw: &str) -> Vec<char> {
    filter_only_significant(&raw)
}

fn test(element: char) -> String{

    let result = match element {
        '은' | '는' => "Neun".to_string(),
        '이' | '가' => "ka".to_string(),
        '로'=> "ro".to_string(),
        _ => "Not Available".to_string(),
    };
    result
}
