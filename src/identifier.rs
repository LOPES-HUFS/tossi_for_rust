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
use crate::particle::*;
use crate::verifier::verifiers;

#[derive(Debug)]
pub enum TossiKind {
    Neun,
    Ka,
    Ro,
    Ida,
    None,
}

/// ## 토시 구조체
/// 사용자가 입력한 토시를 변환해서 저장하고,
/// 변환한 값을 토대로 어떤 종류인지 분류한 다음 분류한 결과를 저장한다.
/// 사용법은 아래와 같다.
///
/// ```rust
/// let test_tossi = "으로";
/// let temp = Tossi::new(test_tossi);
/// println!("입력된 토시: {:?}", temp.modified);
/// println!("토시 종류: {:?}", temp.kind);
/// ```

pub struct Tossi {
    pub modified: Vec<char>,
    pub kind: TossiKind,
}

impl Tossi {
    pub fn new(raw: &str) -> Self {
        let temp_modified = filter_only_significant(raw);
        // 앞에서 변환 것을 토대로 글자 수에 따라 조사 종류를 찾는다.
        let temp_kind = match temp_modified.len() {
            1 => one_letter(temp_modified[0]),
            2 => two_letters(&temp_modified),
            _ => TossiKind::None,
        };
        Self {
            modified: temp_modified,
            kind: temp_kind,
        }
    }
}

/// ## 한 글자로 된 토시를 분류하는 함수
/// 한 글자로 된 토시가 들어오면 이를 종류 별로 분류하는 함수
fn one_letter(element: char) -> TossiKind {
    let result = match element {
        '은' | '는' => TossiKind::Neun,
        '이' | '가' => TossiKind::Ka,
        '로' => TossiKind::Ro,
        '다' => TossiKind::Ida,
        _ => TossiKind::None,
    };
    result
}

/// ## 두 글자로 된 토시를 분류하는 함수
/// 두 글자로 된 토시가 들어오면 이를 종류 별로 분류하는 함수
fn two_letters(elements: &Vec<char>) -> TossiKind {
    let result = match (elements[0], elements[1]) {
        ('으', '로') => TossiKind::Ro,
        ('이', '다') => TossiKind::Ida,
        (_, _) => TossiKind::None,
    };
    result
}

//테스트
pub fn postfix(word: &str, tossi: &str) -> String {
    //파라미터에 올바른 규격의 값이 들어왔는지 확인하기
    verifiers(word,tossi);
    let temp = Tossi::new(tossi);
    println!(
        "입력된 토시: {:?}, 토시 종류: {:?}",
        temp.modified, temp.kind
    );

    let result = match temp.kind {
        TossiKind::Neun => neun::change(&word),
        TossiKind::Ka => ka::change(&word),
        TossiKind::Ro => ro::change(&word),
        TossiKind::Ida => ida::change(&word),
        TossiKind::None => tossi.to_string(),
    };

    let front = word.to_string();
    front + &result
}
