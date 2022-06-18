use library::postfix;
use library::find_last_letter;

fn main() {
    // 테스트
    println!("{:?}", postfix("테스트", "은"));
    println!("{:?}", postfix("나뭇가지(만렙)", "이다"));
    println!("{:?}", postfix("집", "는"));
    println!("{:?}", find_last_letter("비타500"));
    println!("{:?}", find_last_letter("비타5"));
}
