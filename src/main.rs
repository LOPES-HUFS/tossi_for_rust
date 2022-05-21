use std::io;
mod hangeul;
mod filter;

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
}
