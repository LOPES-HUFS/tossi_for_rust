//! # 숫자에 관련된 모듈
//!
//! 이 프로젝트에서 숫자에 관련된 것들을 처리하는 모듈입니다.
//! 이 모듈은 아래와 같이 2개의 함수를 가지고 있습니다.
//!
//! - `change_int_char()': 숫자 한 글자를 한글 한 글자로 바꿔 줍니다.
//! - `change_nun_to_hangeul()`: 문자열로 된 숫자를 한글 발음으로 바꿔 줍니다.


// 숫자 한 글자를 한글 한 글자로 바꾸기 위한 목록
const DIGITS: [char; 10] = ['영', '일', '이', '삼', '사', '오', '육', '칠', '팔', '구'];

// 10부터 1000까지 한글로 숫자 자리수 읽기 위한 목록
const EXPS_UNTIL_1000: [char; 3] = ['십', '백', '천'];

// 4번째 자리수부터 4의 배수로 48번째 자리수까지 일기 위한 목록
const EXPS: [char; 12] = [
    '만', '억', '조', '경', '해', '자', '양', '구', '간', '정', '재', '극',
];

/// ## 해당 문자가 숫자인지 아닌지 확인하는 함수
/// 
/// 입력된 문자가 숫자이면 `true`, 아니면 `false`를 반환합니다.
pub fn is_digits(num: char) -> bool {
    return '0' <= num && num <= '9';
}

/// ## 숫자 한 글자를 한글 발음으로 변환해주는 함수
pub fn change_int_char(num: char) -> char {
    let idx: usize = num as usize;
    return DIGITS[idx - 48];
}

/// ## 숫자를 한글 발음으로 바꿔주는 함수
/// 입력된 숫자를 한글 발음으로 바꿔줍니다.
pub fn change_nun_to_hangeul(num: &str) -> String {
    // 입력된 숫자 문자열을 뒤에서부터 읽기 위해서 입력된 숫자 문자열을 뒤집는다.
    let char_vec: Vec<char> = num.chars().rev().collect();
    let mut temp_result: Vec<char> = Vec::new();

    let mut temp_exps = 0;

    for (i, x) in char_vec.iter().enumerate() {
        temp_result.push(change_int_char(*x));
        temp_result.push(' ');
        if ((i + 1) % 4) == 0 {
            temp_result.push(EXPS[temp_exps]);
            temp_exps += 1;
        } else if ((i + 1) % 4) == 1 {
            temp_result.push(EXPS_UNTIL_1000[0]);
        } else if ((i + 1) % 4) == 2 {
            temp_result.push(EXPS_UNTIL_1000[1]);
        } else {
            temp_result.push(EXPS_UNTIL_1000[2]);
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


/// 비 공개 함수 테스트
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _is_digits() {
        let temp = '5';
        assert_eq!(true, is_digits(temp));

        let temp = '일';
        assert_eq!(false, is_digits(temp));

        let temp = '영';
        assert_eq!(false, is_digits(temp));

        let temp = ' ';
        assert_eq!(false, is_digits(temp));

        let temp = '😀';
        assert_eq!(false, is_digits(temp));
    }
}