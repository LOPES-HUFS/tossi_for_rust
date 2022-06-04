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

    // 테스트
    println!("{:?}", postfix("   ", "은"));
    println!("{:?}", postfix("나뭇가지(만렙)", "이다"));
    println!("{:?}", postfix("집", "는"));
}
