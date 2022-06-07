//! # 숫자를 문자로 바꿔주는 모듈
//!
//!
//!
//! 

use crate::filter::change_int_char;

// pub struct NunToHangeul {
//     pub raw: String,
//     pub modified: String,
//     pub hangeul: TossiKind,
// }

// impl NunToHangeul {
//     // pub fn new(raw: &str) -> Self {
//     //     let temp_modified = filter_only_significant(raw);
//     //     // 앞에서 변환 것을 토대로 글자 수에 따라 조사 종류를 찾는다.
//     //     let temp_kind = match temp_modified.len() {
//     //         1 => one_letter(temp_modified[0]),
//     //         2 => two_letters(&temp_modified),
//     //         _ => TossiKind::None,
//     //     };
//     //     Self {
//     //         modified: temp_modified,
//     //         kind: temp_kind,
//     //     }
//     // }
//     pub fn new(raw: &str) -> Self {

//     }
// }

const DIGITS: [char; 10] = ['영', '일', '이', '삼', '사', '오', '육', '칠', '팔', '구'];

// 10부터 1000까지 한글로 숫자 자리수 읽기 위한 목록
const EXPS_until_1000: [char; 3] = ['십', '백', '천'];

// 4번째 자리수부터 4의 배수로 48번째 자리수까지 일기 위한 목록
const EXPS: [char; 12] = [
    '만', '억', '조', '경', '해', '자', '양', '구', '간', '정', '재', '극',
];

/// ## 숫자에 쉼표를 넣어주는 함수
/// 한글에서
fn add_comma(num: &str) -> String {
    // 입력된 숫자 문자열을 뒤에서부터 읽기 위해서 입력된 숫자 문자열을 뒤집는다.
    let char_vec: Vec<char> = num.chars().rev().collect();
    let mut temp_result: Vec<char> = Vec::new();

    let mut temp_EXPS_until_1000 = 0;
    let mut temp_EXPS = 0;

    for (i, x) in char_vec.iter().enumerate() {
        // if ((i + 1) % 4) == 0 {
        //     temp_result.push(*x);
        //     temp_result.push(EXPS[temp_EXPS]);
        //     temp_EXPS += 1;
        // } else {
        //     temp_result.push(*x);
        // }
        match (i + 1) % 4 {
            0 => {
                temp_result.push(*x);
                temp_result.push(EXPS[temp_EXPS]);
                temp_EXPS += 1;
            }
            1 => {
                temp_result.push(*x);
                temp_result.push(EXPS_until_1000[1]);
            }
            2 => {
                temp_result.push(*x);
                temp_result.push(EXPS_until_1000[2]);
            }
            3 => {
                temp_result.push(*x);
                temp_result.push(EXPS_until_1000[3]);
            }
        }
    }
    // 뒤집어 입력된 숫자 문자열을 뒤집어 정상으로 되돌려 놓는다.
    temp_result.reverse();
    let temp_result: String = temp_result.iter().collect();
    temp_result
}

/// ## 숫자에 쉼표를 넣어주는 함수
/// 한글에서
fn change_nun_to_hangeul(num: &str) -> String {
    // 입력된 숫자 문자열을 뒤에서부터 읽기 위해서 입력된 숫자 문자열을 뒤집는다.
    let char_vec: Vec<char> = num.chars().rev().collect();
    let mut temp_result: Vec<char> = Vec::new();

    let mut temp_EXPS = 0;

    for (i, x) in char_vec.iter().enumerate() {
        temp_result.push(change_int_char(*x));
        temp_result.push(' ');
        if ((i + 1) % 4) == 0 {
            temp_result.push(EXPS[temp_EXPS]);
            temp_EXPS += 1;
        } else if ((i + 1) % 4) == 1 {
            temp_result.push(EXPS1000[0]);
        } else if ((i + 1) % 4) == 2 {
            temp_result.push(EXPS1000[1]);
        } else {
            temp_result.push(EXPS1000[2]);
        }
    }
    // 맨 마지막에 '십'이라는 글자가 들어가는 버그 때문에 들어간 것을 제거한다.
    temp_result.pop();
    // 뒤집어 입력된 숫자 문자열을 뒤집어 정상으로 되돌려 놓는다.
    temp_result.reverse();

    let mut temp_result: String = temp_result.iter().collect();
    temp_result = temp_result.replace("영천", "");
    temp_result = temp_result.replace("영백", "");
    temp_result = temp_result.replace("영십", "");
    temp_result = temp_result.replace("영", "");
    temp_result = temp_result.replace("  ", "");
    temp_result = temp_result.replace(" ", "");
    temp_result.trim_start_matches('일').to_string()
}

/// 비 공개 함수틑 테스트
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _change_nun_to_hangeul() {
        let test = "10000";
        let result = "만";
        assert_eq!(result, add_comma(test));

        // let test = "1234567890";
        // let result = "12,3456,7890";
        // assert_eq!(result, add_comma(test));
    }
}
