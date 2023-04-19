use crate::lexer::token::{Token, TokenData};

pub mod token;

pub struct Lexer {
    keywords: Vec<String>,
    operators: Vec<String>,
    specials: Vec<String>,
    pub(crate) line: u32,
    pub(crate) char_pos: u32,
    src: String,
    input: Vec<char>,
    pub position: usize,
    pub read_position: usize,
    pub ch: char,
}

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z'
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

impl Lexer {
    pub fn new(
        input: Vec<char>,
        keywords: Vec<String>,
        operators: Vec<String>,
        specials: Vec<String>
    ) -> Self {
        Self {
            keywords,
            operators,
            specials,
            input,
            line: 0,
            char_pos: 0,
            src: "".to_string(),
            position: 0,
            read_position: 0,
            ch: char::from(0)
        }
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = char::from(0);
        } else {
            self.ch = self.input[self.read_position];
            self.char_pos += 1;
            if self.ch == '\n' {
                self.char_pos = 0;
                self.line += 1;
            }
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn peek_char(&mut self) -> char {
        return if self.read_position >= self.input.len() {
            char::from(0)
        } else {
            self.input[self.read_position]
        }
    }

    pub fn skip_whitespaces(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    pub fn reset(&mut self) {
        self.position = 0;
        self.read_position = 0;
        self.line = 0;
        self.char_pos = 0;
        self.ch = char::from(0);
    }
    pub fn peek_token(&mut self) -> Token {
        let position = self.position;
        let read_position = self.read_position;
        let line = self.line;
        let char_pos = self.char_pos;
        let ch = self.ch;

        let token = self.next_token();

        self.position = position;
        self.read_position = read_position;
        self.line = line;
        self.char_pos = char_pos;
        self.ch = ch;
        return token;
    }

    pub fn next_token(&mut self) -> Token {
        let read_identifier = |l: &mut Lexer| -> String {
            let position = l.position;
            while l.position < l.input.len() && (is_letter(l.ch) || l.ch == '_' || is_digit(l.ch)) {
                l.read_char();
            }
            return l.input[position..l.position].iter().collect();
        };
        let read_string = |l: &mut Lexer| -> String {
            let position = l.position;
            while l.position < l.input.len() && l.ch != '"' {
                l.read_char();
            }
            return l.input[position..l.position].iter().collect();
        };

        let read_number = |l: &mut Lexer| -> String {
            let position = l.position;
            while l.position < l.input.len() && is_digit(l.ch) {
                l.read_char();
            }
            return l.input[position..l.position].iter().collect();
        };

        let mut token: Token = Token::EOF;

        self.skip_whitespaces();

        let line = self.line;
        let char = self.char_pos;

        if is_letter(self.ch) || self.ch == '_' {
            let id = read_identifier(self);

            if self.keywords.contains(&id) {
                token = Token::Keyword(id.clone(), TokenData {
                    line,
                    char,
                    file: self.src.clone(),
                    raw: id.clone(),
                })
            }
            else {
                token = Token::Id(id.clone(), TokenData {
                    line,
                    char,
                    file: self.src.clone(),
                    raw: id.clone(),
                })
            }
        }
        else if is_digit(self.ch) {
            let mut num = read_number(self);

            if self.ch == '.' {
                self.read_char();
                num += &*(".".to_owned() + &*read_number(self));

                token = Token::Float(num.parse::<f64>().unwrap(), TokenData {
                    line,
                    char,
                    file: self.src.clone(),
                    raw: num.clone(),
                });
            }
            else {
                token = Token::Integer(num.parse::<i64>().unwrap(), TokenData {
                    line,
                    char,
                    file: self.src.clone(),
                    raw: num.clone(),
                });
            }
        }
        else if self.ch == char::from(0) {
            token = Token::EOF;
        }
        else if self.ch == '"' {
            self.read_char();

            let str = read_string(self);

            token = Token::String(str.clone().trim_matches('"').to_string(), TokenData {
                line,
                char,
                file: self.src.clone(),
                raw: str.clone(),
            });

            self.read_char();
        }
        else {
            let mut str = String::from(self.ch);

            loop {
                self.read_char();

                let mut place_tok = || {
                    if self.operators.contains(&str) {
                        token = Token::Operator(str.clone(), TokenData {
                            line,
                            char,
                            file: self.src.clone(),
                            raw: str.clone(),
                        });
                    }
                    else if self.specials.contains(&str) {
                        token = Token::Special(str.clone(), TokenData {
                            line,
                            char,
                            file: self.src.clone(),
                            raw: str.clone(),
                        });
                    }
                    else {
                        token = Token::Unknown(str.clone(), TokenData {
                            line,
                            char,
                            file: self.src.clone(),
                            raw: str.clone(),
                        });
                    }
                };

                if !is_digit(self.ch) && !is_letter(self.ch) && self.ch != '_' && self.ch != char::from(0) {
                    let chas = (str.clone() + &*String::from(self.ch));

                    if self.operators.iter().any(|op| op.starts_with(&chas))
                        || self.specials.iter().any(|op| op.starts_with(&chas)){
                        str = chas;
                    }
                    else {
                        place_tok();
                        break;
                    }
                }
                else {
                    place_tok();
                    break;
                }
            }
        }

        return token;
    }
}