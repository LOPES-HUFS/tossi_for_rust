use std::io;

fn main() {
    tossi_for_rust::hello();

    let mut sentence = String::new();

    io::stdin()
        .read_line(&mut sentence)
        .expect("줄을 읽는데 실패했습니다.");

    let words: Vec<&str> = sentence.split_whitespace().collect();

    println!("콘솔에서 입력한 값: {:?}", words);
}