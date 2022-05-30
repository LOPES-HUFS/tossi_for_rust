use library::postfix;

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
    let result = "집로";
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
