use library::{is_hangeul_or_number, postfix, verifiers};

#[test]
fn _postfix() {
    // '으로', '로' 테스트
    let word = "집";
    let tossi = "으로";
    let result = "집으로";
    assert_eq!(result, postfix(word, tossi));

    let word = "집";
    let tossi = "로";
    let result = "집으로";
    assert_eq!(result, postfix(word, tossi));

    let word = "집";
    let tossi = "(으)로";
    let result = "집으로";
    assert_eq!(result, postfix(word, tossi));

    let word = "나무";
    let tossi = "으로";
    let result = "나무로";
    assert_eq!(result, postfix(word, tossi));

    let word = "나무";
    let tossi = "로";
    let result = "나무로";
    assert_eq!(result, postfix(word, tossi));

    let word = "나무";
    let tossi = "(으)로";
    let result = "나무로";
    assert_eq!(result, postfix(word, tossi));

    let word = "칼";
    let tossi = "으로";
    let result = "칼로";
    assert_eq!(result, postfix(word, tossi));

    let word = "칼";
    let tossi = "로";
    let result = "칼로";
    assert_eq!(result, postfix(word, tossi));

    let word = "칼";
    let tossi = "(으)로";
    let result = "칼로";
    assert_eq!(result, postfix(word, tossi));

    let word = "test";
    let tossi = "으로";
    let result = "test(으)로";
    assert_eq!(result, postfix(word, tossi));

    let word = "test";
    let tossi = "로";
    let result = "test(으)로";
    assert_eq!(result, postfix(word, tossi));

    let word = "test";
    let tossi = "(으)로";
    let result = "test(으)로";
    assert_eq!(result, postfix(word, tossi));
}

#[test]
fn _verifiers() {
    // 둘 다 적절하다
    let temp = verifiers("하하하", "은");
    assert_eq!(Ok(()), temp);
    // 둘 다 적절하다
    let temp = verifiers("하하하", "는");
    assert_eq!(Ok(()), temp);
    // 단어는 적절하지만, 토시가 적절하지 않다.
    let temp = verifiers("하하하", "은은");
    assert_eq!(Err("This value is not correct tossi."), temp);
    // 단어도 마지막이 한글이고 토시도 적절하지만, 단어 길이가 50 글자 이상이다. 그래서 에러 처리
    let temp_word = "1테트리스2테트리스3테트리스4테트리스5테트리스6테트리스7테트리스8테트리스9테트리스10테트리스";
    let temp = verifiers(temp_word, "은");
    assert_eq!(
        Err("The length has been exceeded. Set the word length to less than 50."),
        temp
    );
    // 단어도 마지막이 숫자이고 토시도 적절하지만, 단어 길이가 50 글자 이상이다. 그래서 에러 처리
    let temp_word = "테트리스1테트리스2테트리스3테트리스4테트리스5테트리스6테트리스7테트리스8테트리스9테트리스10";
    let temp = verifiers(temp_word, "은");
    assert_eq!(
        Err("The length has been exceeded. Set the word length to less than 50."),
        temp
    );
}

#[test]
// is_hangeul_or_number() 함수는 다음과 같이 반환한다.
// 한글이고 숫자이다: 현재 이것은 불가능하다.
// 한글이지만 숫자는 아니다: (true, false)
// 한글은 아니고 숫자이다: (false, true)
// 한글은 아니고 숫자도 아니다: (false, false)
fn _is_hangeul_or_number_마지막_글자가_숫자인지_테스트() {
    let temp = "테트리스1테트리스2테트리스3테트리스4테트리스5테트리스6테트리스7테트리스8테트리스9테트리스10";
    assert_eq!((false, true), is_hangeul_or_number(temp.to_string()));
    let temp = "테트리스9";
    assert_eq!((false, true), is_hangeul_or_number(temp.to_string()));
    let temp = "테트리스a";
    assert_eq!((false, false), is_hangeul_or_number(temp.to_string()));
    let temp = "테트리스!";
    assert_eq!((false, false), is_hangeul_or_number(temp.to_string()));
    let temp = "tetris";
    assert_eq!((false, false), is_hangeul_or_number(temp.to_string()));
}

#[test]
fn _is_hangeul_or_number_마지막_글자가_한글인가() {
    let temp = "테트리스1테트리스2테트리스3테트리스4테트리스5테트리스6테트리스7테트리스8테트리스9테트리스10테트리스";
    assert_eq!((true, false), is_hangeul_or_number(temp.to_string()));
}
