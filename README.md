# Tossi for Rust

이 프로젝트는 [tossi](https://github.com/what-studio/tossi)에서 영감을 받았습니다. 파이썬으로 구현된 앞의 **토씨 라이브러리**처럼 이 프로젝트도 임의의 단어와 그 단어에 붙일 조사를 입력하면, 입력한 조사를 같이 입력한 단어에 자연스러운 형태로 바꿔 반환해 줍니다. 이 프로젝트는 Rust로 작성하고 있습니다. 자세한 내용은 아래를 내용을 참고하세요.

## 구현 함수

여기에서는 다음과 같은 2가지 기능을 지원합니다.

- `pick(word: &str, tossi: &str)`: 입력된 것들을 참고해 `word`에 적절한 `tossi`를 반환합니다.
- `postfix(word: &str, tossi: &str)`: 입력된 것들을 참고해 `word`에 적절한 `tossi`를 덧붙여 반환합니다.

## 터미널에서 사용하기

이 프로젝트는 라이브러리 형태로 목표하는 기능들을 구현한 다음 이를 다양한 방법으로 적용하고자 합니다. 기본적으로 CLI(Command-Line Interface, 이하 커맨드 라인 인터페이스)에서 작동하는 앱을 만들고자 합니다. 아래와 같이 컴파일을 하면 커맨드 라인 인터페이스에서도 사용할 수 있습니다.

### `cargo run`로 컴파일한 다음 사용하는 법

```rust
cargo run -- --word 테스트  --tossi 은
cargo run -- --word 테스트  --tossi 은 -o
```

구체적인 사용법 보기

```rust
➜ cargo run -- --help
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/tossi --help`
tossi 0.1.0
Tossi(토시)는 사용자가 입력한 단어와 토시를 입력하렸을 때, 입력한 단어에 적합한

USAGE:
    tossi [OPTIONS] --word <단어> --tossi <토시>

OPTIONS:
    -h, --help                       Print help information
    -o, --only-tossi <ONLY_TOSSI>    반환 값에 사용자가 입력한 단어 적용 유무 [possible values: true, false]
    -t, --tossi <토시>               입력한 단어에 적용할 토시
    -V, --version                    Print version information
    -w, --word <단어>                토시를 적용하고 싶은 단어
```

### `cargo build`로 컴파일한 다음 사용하는 법

```rust
target/debug/tossi -h                
target/debug/tossi -t 을 -w 나뭇가지 
target/debug/tossi -t 을 -w 나뭇가지 -o
```

구체적인 사용법 보기

```rust
➜ target/debug/tossi -h                
tossi 0.1.0
Tossi(토시)는 사용자가 입력한 단어와 토시를 입력하렸을 때, 입력한 단어에 적합한

USAGE:
    tossi [OPTIONS] --word <단어> --tossi <토시>

OPTIONS:
    -h, --help                       Print help information
    -o, --only-tossi <ONLY_TOSSI>    반환 값에 사용자가 입력한 단어 적용 유무 [possible values: true, false]
    -t, --tossi <토시>               입력한 단어에 적용할 토시
    -V, --version                    Print version information
    -w, --word <단어>                토시를 적용하고 싶은 단어
```

## 코드 작성에서 유의할 점

코딩 스타일을 맞추기 위해서 코드를 올리기 전에 다음 명령어를 이용하여 코드를 정리하여 올립니다.

```console
cargo fmt
```

## 한글 관련 용어

- '한글 음절(-音節, Hangul syllable)' 또는 '한글 글자마디': 한글 자모 첫소리와 가운뎃소리 글자, 또는 첫소리, 가운뎃소리, 끝소리 글자로 이루어진 한글의 단위, 참고:[한글 음절 - 위키백과, 우리 모두의 백과사전](https://ko.wikipedia.org/wiki/한글_음절)
- '한글 낱자' 또는 '자모(子母, 字母)': 한글의 닿소리 글자와 홀소리 글자를 같이 이르는 말, 참고:[한글 낱자 - 위키백과, 우리 모두의 백과사전](https://ko.wikipedia.org/wiki/한글_낱자)
