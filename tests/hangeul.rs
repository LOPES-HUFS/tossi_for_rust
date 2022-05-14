extern crate hangeul;

#[test]
fn _join_phonemes() {
    let temp = ['ㄸ', 'ㅗ', 'ㅁ'];
    let result = '똠';
    assert_eq!(result, hangeul::join_phonemes(temp));

    let temp = ['ㄱ', 'ㅏ', ' '];
    let result = '가';
    assert_eq!(result, hangeul::join_phonemes(temp));
}
#[test]
fn _split_phonemes() {
    let temp = '똠';
    let result = ['ㄸ', 'ㅗ', 'ㅁ'];
    assert_eq!(result, hangeul::split_phonemes(temp));

    let temp = '가';
    let result = ['ㄱ', 'ㅏ', ' '];
    assert_eq!(result, hangeul::split_phonemes(temp));
}
