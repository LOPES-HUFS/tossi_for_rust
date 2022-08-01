//! # 올바른 파라미터로 주어졌는지 확인하는 모듈
//! ## 현재 취급하는 tossi: 은/는, 이/가, 로/으로, 다/이다
//!

const TOSSI_LIST: [&str; 35] = [
    "은", "는", "이", "가", "이다", "다", "으로", "로", "의", "도", "께", "에", "만", "뿐", "보다",
    "같이", "밖에", "부터", "까지", "마냥", "처럼", "마저", "조차", "마냥", "커녕", "을", "를",
    "(으)로", "은(는)", "(는)은", "(이)다", "(을)를", "(를)을", "(이)가", "(가)이",
];

pub fn verifiers(word: &str, tossi: &str) {
    match verifier_tossi(tossi) {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    }
    match limit_word_len(word) {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    }
}

// 올바른 토씨를 입력했는지 확인해주는 함수
fn verifier_tossi(tossi: &str) -> Result<(), &str> {
    let mut status = 0;
    for check in TOSSI_LIST.iter() {
        if check == &tossi {
            status = 1;
            break;
        }
    }
    if status == 1 {
        return Ok(());
    } else {
        return Err("This value is not correct tossi.");
    }
}

// 파라미터롤 받는 단어를 제한 기준 함수
fn limit_word_len(word: &str) -> Result<(), &str> {
    let limitation = 50;
    if word.chars().count() <= limitation {
        return Ok(());
    } else {
        return Err("The length has been exceeded. Set the word length to less than 50.");
    }
}

#[test]
fn _limit_word_len() {
    let temp = "12345";
    assert_eq!(Ok(()), limit_word_len(temp));

    let temp = "아이디는 50자까지 설정이 가능합니다.";
    assert_eq!(Ok(()), limit_word_len(temp));

    let temp = "10000000000000000000000000000000000000000000000000000";
    assert_eq!(
        Err("The length has been exceeded. Set the word length to less than 50."),
        limit_word_len(temp)
    );
}

#[test]
fn _verifier_tossi() {
    let temp = "까지";
    assert_eq!(Ok(()), verifier_tossi(temp));

    let temp = "류현지";
    assert_eq!(
        Err("This value is not correct tossi."),
        verifier_tossi(temp)
    );
}
