# Tossi for Rust

[tossi](https://github.com/what-studio/tossi)에 영감을 받아 이 프로젝트를 Rust로 작성하고 더 많은 기능을 제공하고자 한다.

## 구현 함수

여기에서는 다음과 같은 2가지 기능을 지원합니다.

- `pick(word: &str, tossi: &str)`: 입력된 것들을 참고해 `word`에 적절한 `tossi`를 반환합니다.
- `postfix(word: &str, tossi: &str)`: 입력된 것들을 참고해 `word`에 적절한 `tossi`를 덧붙여 반환합니다.

## 코드 작성에서 유의할 점

코딩 스타일을 맞추기 위해서 코드를 올리기 전에 다음 명령어를 이용하여 코드를 정리하여 올립니다.

```console
cargo fmt
```

## 한글 관련 용어

- '한글 음절(-音節, Hangul syllable)' 또는 '한글 글자마디': 한글 자모 첫소리와 가운뎃소리 글자, 또는 첫소리, 가운뎃소리, 끝소리 글자로 이루어진 한글의 단위, 참고:[한글 음절 - 위키백과, 우리 모두의 백과사전](https://ko.wikipedia.org/wiki/한글_음절)
- '한글 낱자' 또는 '자모(子母, 字母)': 한글의 닿소리 글자와 홀소리 글자를 같이 이르는 말, 참고:[한글 낱자 - 위키백과, 우리 모두의 백과사전](https://ko.wikipedia.org/wiki/한글_낱자)
