// 초성, 중성, 종성 배열 정의
static BEGINS: [char; 19] = ['ㄱ','ㄲ','ㄴ','ㄷ','ㄸ','ㄹ','ㅁ','ㅂ','ㅃ','ㅅ','ㅆ','ㅇ','ㅈ','ㅉ','ㅊ','ㅋ','ㅌ','ㅍ','ㅎ'];    
static MIDDLES: [char; 21] = ['ㅏ','ㅐ','ㅑ','ㅒ','ㅓ','ㅔ','ㅕ','ㅖ','ㅗ','ㅘ','ㅙ','ㅚ','ㅛ','ㅜ','ㅝ','ㅞ','ㅟ','ㅠ','ㅡ','ㅢ','ㅣ'];
static ENDS: [char; 28] = [' ','ㄱ','ㄲ','ㄳ','ㄴ','ㄵ','ㄶ','ㄷ','ㄹ','ㄺ','ㄻ','ㄼ','ㄽ','ㄾ','ㄿ','ㅀ','ㅁ','ㅂ','ㅄ','ㅅ','ㅆ','ㅇ','ㅈ','ㅊ','ㅋ','ㅌ','ㅍ','ㅎ'];

// 한글인지 체크하는 함수
fn is_hangeul(word: char) -> bool {
    return '가' <= word && word <= '힣';
}

// 자음인지 체크하는 함수
fn is_consonant(word: char) -> bool {
    return 'ㄱ' <= word && word <= 'ㅎ';
}

// 초,중,종성을 하나의 글자로 합쳐주는 함수
pub fn join_phonemes(word: [char; 3]) -> char {
    // 파라미터로 받은 초,중,종성 인덱스 추출
    let idx_begin = BEGINS.iter().position(|&x| x == word[0]).unwrap();
    let idx_middle = MIDDLES.iter().position(|&x| x == word[1]).unwrap();
    let idx_end = ENDS.iter().position(|&x| x == word[2]).unwrap();
    // 추가될 값 계산
    let initial = '가' as u32;
    let offset = ((idx_begin * MIDDLES.len() + idx_middle) * ENDS.len() + idx_end) as u32;
    let output = char::from_u32(initial + offset).unwrap();
    return output
}

// 한글자를 초,중,종성으로 구분하는 함수
pub fn split_phonemes(word: char) -> [char; 3] { 
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
    //초성
    let idx_begin: usize = (offset/(21*28)) as usize;
    phonemes[0] = BEGINS[idx_begin];
    //중성
    let idx_middle: usize = ((offset/28)%21) as usize;
    phonemes[1] = MIDDLES[idx_middle];
    //종성은 있는 경우에만 계산
    if (((unicode-0xAC00)%(21*28))%28) != 0 {
        let idx_end: usize = (offset%21) as usize;
        phonemes[2] = ENDS[idx_end];
    }
    //초,중,종성이 배열로 묶여서 전달
    return phonemes
}
