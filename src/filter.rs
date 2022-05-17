use crate::hangeul::is_hangeul;

static DIGITS: [char; 10] = ['영','일','이','삼','사','오','육','칠','팔','구'];

//단어에서 마지막 글자를 찾아주는 함수
pub fn find_last_letter(word: &str) -> char {
    let filtered = filter_only_significant(word);
    if filtered.len() > 0 {
        return filtered[filtered.len()-1];
    }
    else {
        return ' ';
    }
}

//단어에서 불필요한 요소 제거하는 함수
fn filter_only_significant(word: &str) -> Vec<char> {
    let mut output: Vec<char> = Vec::new();
    let mut bracket: bool = false;
    for c in word.chars() {
        //괄호 있는지 확인
        if c == '(' {
            bracket = true;
        }
        else if c == ')' {
            bracket = false;
        }
        //한글이 아니라면 제거
        if bracket {
            continue;
        }
        else if is_hangeul(c) {
            output.push(c);
        }
    }
    return output;
}

//숫자를 한글발음으로 변환해주는 함수
//fn change_int_char(int: char) -> char {
//}