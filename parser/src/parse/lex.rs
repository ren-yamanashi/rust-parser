use super::token::Token;

pub struct Lexer {
    input: String,
    position: isize,
    read_position: isize,
    ch: char,
}
impl Lexer {
    pub fn new(input: String) -> Self {
        // 初期値を定義
        Lexer {
            input,
            position: -1,
            read_position: 0,
            ch: '\0', // null
        }
    }
    pub fn next(&mut self) -> Token {
        self.skip_white_space();
        self.read();

        match self.ch {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => self.digit(),
            '\0' => Token::EOF, // `\0`はnullを表すので、ファイルの終端扱いにする
            // ワイルドカード
            _ => Token::Illegal,
        }
    }

    fn digit(&mut self) -> Token {
        let start_pos = self.position as usize;
        // 数値が続く限り繰り返す
        //				  |-----
        //	              ↓		|
        //   |----[0-9]--[]--[0-9]--[0-9]以外-->(Number)
        while Lexer::is_digit(self.ch) {
            self.read();
        }

        let literal = &self.input[start_pos..(self.position as usize)];
        let num = literal.parse::<i32>().unwrap(); // ここで文字列を数値にする

        self.back();
        
        Token::Number(num)
    }

    // *** と位置情報の更新
    fn read(&mut self) {
        // 現在位置が入力文字数以上だったらnullを返す
        if self.read_position >= self.input.len() as isize {
            self.ch = '\0';
        } else {
            // 入力文字のn番目を`ch`に代入 n=read_position ※この段階でread_positionは現在位置を表すのでOK
            self.ch = self
                .input
                .chars()
                .nth(self.read_position.try_into().unwrap())
                .unwrap();
        }
        // 位置情報の更新
        self.position = self.read_position;
        self.read_position += 1;
    }

    // 次の文字を覗き見する
    fn peek(&mut self) -> char {
        // 現在位置が入力文字数以上だったらnullを返す
        if self.read_position >= self.input.len() as isize {
            '\0'
        } else {
            // 入力値のn番目を返す n=read_position
            self.input
                .chars()
                .nth(self.read_position.try_into().unwrap())
                .unwrap()
        }
    }

    // 1文字戻る
    fn back(&mut self) {
        self.position -= 1;
        self.read_position -= 1;
        self.ch = self
            .input
            .chars()
            .nth(self.position.try_into().unwrap())
            .unwrap();
    }

    // 空白処理
    fn skip_white_space(&mut self) {
        while self.peek() == ' '
            || self.peek() == '\t'
            || self.peek() == '\n'
            || self.peek() == '\r'
        {
            self.read();
        }
    }

    // 数値かどうかの確認 小数点も許可
    fn is_digit(ch: char) -> bool {
        '0' <= ch && ch <= '9' || ch == '.'
    }
}


#[cfg(test)]
mod tests {
    use super::Lexer;
    use crate::parse::token::Token;

    #[test]
    fn test_lexer() {
        let input = String::from(
            r#" 1   + 
        23 + 1                  * 3 / 0 - 1 "#,
        );

        let mut l = Lexer::new(input);

        let got = l.next();
        assert_eq!(got, Token::Number(1));

        let got = l.next();
        assert_eq!(got, Token::Plus);

        let got = l.next();
        assert_eq!(got, Token::Number(23));

        let got = l.next();
        assert_eq!(got, Token::Plus);

        let got = l.next();
        assert_eq!(got, Token::Number(1));

        let got = l.next();
        assert_eq!(got, Token::Asterisk);

        let got = l.next();
        assert_eq!(got, Token::Number(3));

        let got = l.next();
        assert_eq!(got, Token::Slash);

        let got = l.next();
        assert_eq!(got, Token::Number(0));

        let got = l.next();
        assert_eq!(got, Token::Minus);

        let got = l.next();
        assert_eq!(got, Token::Number(1));

        let got = l.next();
        assert_eq!(got, Token::EOF);
    }
}