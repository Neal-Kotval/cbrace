
   
// extern crate phf;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Token {
    ILLEGAL,
    EOF,
    IDENT(Vec<char>),
    INT(Vec<char>),
    ASSIGN(char),
    PLUS(char),
    COMMA(char),
    SEMICOLON(char),
    COLON(char),
    LPAREN(char),
    RPAREN(char),
    LBRACE(char),
    RBRACE(char),
    FUNCTION,
    DEC,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
    MINUS(char),
    BANG(char),
    ASTERISK(char),
    SLASH(char),
    LT(char),
    GT(char)
}

pub fn get_keyword_token(ident: &Vec<char>) -> Result<Token, String> {
    let identifier: String = ident.into_iter().collect();
    match &identifier[..] {
        "fn" => Ok(Token::FUNCTION),
        "dec" => Ok(Token::DEC),
        "true" => Ok(Token::TRUE),
        "false" => Ok(Token::FALSE),
        "if" => Ok(Token::IF),
        "else" => Ok(Token::ELSE),
        "return" => Ok(Token::RETURN),
        _ => Err(String::from("Not a keyword"))
    }
}