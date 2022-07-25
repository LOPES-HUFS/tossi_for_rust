use library::postfix;

fn main() {
    // 테스트
    println!("{} 성공했습니다.", postfix("테스트", "은"));
    println!("{} 시민이었습니다.", postfix("임포스터", "은"));
    println!("이 아이템은 {}", postfix("나뭇가지", "이다"));
    println!("{} 획득했습니다.", postfix("비타500", "를"));
}
