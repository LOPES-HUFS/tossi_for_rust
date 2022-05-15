use std::io;
mod hangeul;

fn main() {
    println!("단어를 입력해주세요.");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("줄을 읽는데 실패했습니다.");

    let word = input.trim();

    for c in word.chars() {
        println!("{}", c);
        let splited = hangeul::split_phonemes(c);
        println!("{:?}", splited);
        println!("{:?}", hangeul::join_phonemes(splited));
    }
}
