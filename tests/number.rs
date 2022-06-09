use library::{change_int_char, change_nun_to_hangeul};

#[test]

fn _change_int_char() {
    let temp = '1';
    assert_eq!('일', change_int_char(temp));

    let temp = '2';
    assert_eq!('이', change_int_char(temp));

    let temp = '3';
    assert_eq!('삼', change_int_char(temp));

    let temp = '4';
    assert_eq!('사', change_int_char(temp));

    let temp = '5';
    assert_eq!('오', change_int_char(temp));

    let temp = '6';
    assert_eq!('육', change_int_char(temp));

    let temp = '7';
    assert_eq!('칠', change_int_char(temp));

    let temp = '8';
    assert_eq!('팔', change_int_char(temp));

    let temp = '9';
    assert_eq!('구', change_int_char(temp));

    let temp = '0';
    assert_eq!('영', change_int_char(temp));
}

#[test]
fn _change_nun_to_hangeul() {
    let test = "10000";
    let result = "만";
    assert_eq!(result, change_nun_to_hangeul(test));

    let test = "10001";
    let result = "만일";
    assert_eq!(result, change_nun_to_hangeul(test));
}
