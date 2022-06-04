use library::postfix;

fn main() {
    // 테스트
    println!("{:?}", postfix("   ", "은"));
    println!("{:?}", postfix("나뭇가지(만렙)", "이다"));
    println!("{:?}", postfix("집", "는"));
}
