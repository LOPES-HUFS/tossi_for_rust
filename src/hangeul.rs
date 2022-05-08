fn main() {
    println!("{:?}", split_phonemes('현',true,true,true));
    println!("{:?}", join_phonemes('ㅎ','ㅕ','ㄴ'));
}

// 초,중,종성을 하나의 글자로 합쳐주는 함수
fn join_phonemes(begin: char, middle: char, end: char) -> char {
    // 초성, 중성, 종성 배열 정의
    let begins: [char; 19] = ['ㄱ','ㄲ','ㄴ','ㄷ','ㄸ','ㄹ','ㅁ','ㅂ','ㅃ','ㅅ','ㅆ','ㅇ','ㅈ','ㅉ','ㅊ','ㅋ','ㅌ','ㅍ','ㅎ'];    
    let middles: [char; 21] = ['ㅏ','ㅐ','ㅑ','ㅒ','ㅓ','ㅔ','ㅕ','ㅖ','ㅗ','ㅘ','ㅙ','ㅚ','ㅛ','ㅜ','ㅝ','ㅞ','ㅟ','ㅠ','ㅡ','ㅢ','ㅣ'];
    let ends: [char; 28] = [' ','ㄱ','ㄲ','ㄳ','ㄴ','ㄵ','ㄶ','ㄷ','ㄹ','ㄺ','ㄻ','ㄼ','ㄽ','ㄾ','ㄿ','ㅀ','ㅁ','ㅂ','ㅄ','ㅅ','ㅆ','ㅇ','ㅈ','ㅊ','ㅋ','ㅌ','ㅍ','ㅎ'];
    // 파라미터로 받은 초,중,종성 인덱스 추출
    let idx_begin = begins.iter().position(|&x| x == begin).unwrap();
    let idx_middle = middles.iter().position(|&x| x == middle).unwrap();
    let idx_end = ends.iter().position(|&x| x == end).unwrap();
    // 추가될 값 계산
    let initial = '가' as u32;
    let offset = ((idx_begin * middles.len() + idx_middle) * ends.len() + idx_end) as u32;
    let output = char::from_u32(initial + offset).unwrap();
    return output
}

// 한글자를 초,중,종성으로 구분하는 코드
fn split_phonemes(word: char, begin: bool, middle: bool, end: bool) -> [char; 3] { 
    // 초성, 중성, 종성 배열 정의
    let begins: [char; 19] = ['ㄱ','ㄲ','ㄴ','ㄷ','ㄸ','ㄹ','ㅁ','ㅂ','ㅃ','ㅅ','ㅆ','ㅇ','ㅈ','ㅉ','ㅊ','ㅋ','ㅌ','ㅍ','ㅎ'];    
    let middles: [char; 21] = ['ㅏ','ㅐ','ㅑ','ㅒ','ㅓ','ㅔ','ㅕ','ㅖ','ㅗ','ㅘ','ㅙ','ㅚ','ㅛ','ㅜ','ㅝ','ㅞ','ㅟ','ㅠ','ㅡ','ㅢ','ㅣ'];
    let ends: [char; 28] = [' ','ㄱ','ㄲ','ㄳ','ㄴ','ㄵ','ㄶ','ㄷ','ㄹ','ㄺ','ㄻ','ㄼ','ㄽ','ㄾ','ㄿ','ㅀ','ㅁ','ㅂ','ㅄ','ㅅ','ㅆ','ㅇ','ㅈ','ㅊ','ㅋ','ㅌ','ㅍ','ㅎ'];
    // 조,중,종성을 담을 배열 정의
    let mut phonemes: [char; 3] = [' '; 3]; 
    // 받은 문자가 한글인지 확인, 한글이 아닐 경우 공백으로 출력
    if !is_hangeul(word) {
        println!("The word is not hangeul");
        return phonemes
    }
    //'가'와의 차이값 계산
    let unicode = word as u32;
    let initial = '가' as u32;
    let offset = unicode - initial;
    //초,중,종성 값 계산
    if begin {
        let idx_begin: usize = (offset/(21*28)) as usize;
        phonemes[0] = begins[idx_begin];
    }
    if middle {
        let idx_middle: usize = ((offset/28)%21) as usize;
        phonemes[1] = middles[idx_middle];
    }
    if end {
        let idx_end: usize = (offset%21) as usize;
        phonemes[2] = ends[idx_end];
    }
    //초,중,종성이 배열로 묶여서 전달
    return phonemes
}

// 한글단어인지를 체크하는 함수
fn is_hangeul(word: char) -> bool {
    return '가' <= word && word <= '힣';
}
