use std::io;

use library::postfix;
mod filter;
mod hangeul;

fn main() {
    println!("단어를 입력해주세요.");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("줄을 읽는데 실패했습니다.");

    let word = input.trim();

    let output = filter::find_last_letter(word);
    println!("{:?}", output);
    let splited = hangeul::split_phonemes(output);
    println!("{:?}", splited);
    println!("{:?}", hangeul::join_phonemes(splited));
    // 테스트
    println!("{:?}", postfix("집", "으로"));
    println!("{:?}", postfix("집", "로"));
    println!("{:?}", postfix("집", "(으)로"));
    println!("{:?}", postfix("집", "은"));
    println!("{:?}", postfix("집", "는"));
}
