use clap::Parser;

use library::pick;
use library::postfix;

///
/// Tossi(토시)는 사용자가 입력한 단어와 토시를 입력하였을 때,
/// 입력한 토시를 입력한 단어에 적절하게 변환해주는 프로그램입니다.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// 토시를 적용하고 싶은 단어
    #[clap(short, long, value_parser, value_name = "단어")]
    word: String,

    /// 입력한 단어에 적용할 토시
    #[clap(short, long, value_parser, value_name = "토시")]
    tossi: String,

    /// 반환 값에 사용자가 입력한 단어 적용 유무
    #[clap(short, long, action = clap::ArgAction::SetTrue)]
    only_tossi: Option<bool>,
}

fn main() {
    let args = Args::parse();
    // 아래 값을 `Some(true)`으로 설정해야 이 `only_tossi`을 넣지 않더라고
    // `false`라고 인식하여 단어와 프로그램이 찾는 적절한 토시를 합쳐서 출력하게 된다.
    if args.only_tossi == Some(true) {
        println!("{}", pick(&args.word, &args.tossi));
    } else {
        println!("{}", postfix(&args.word, &args.tossi));
    }

    // 테스트
    // println!("{} 성공했습니다.", postfix("테스트", "은"));
    // println!("{} 시민이었습니다.", postfix("임포스터", "은"));
    // println!("이 아이템은 {}", postfix("나뭇가지", "이다"));
    // println!("{} 획득했습니다.", postfix("비타500", "를"));
}
