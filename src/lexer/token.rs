#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub struct TokenData {
    pub line: u32,
    pub char: u32,
    pub file: String,
    pub raw: String

}
#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum Token {
    Keyword(String, TokenData),
    Id(String, TokenData),
    Integer(i64, TokenData),
    Float(f64, TokenData),
    Operator(String, TokenData),
    String(String, TokenData),
    Special(String, TokenData),
    Unknown(String, TokenData),
    EOF,
}