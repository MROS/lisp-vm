#[derive(Debug, PartialEq)]
pub enum Token {
    // 括號
    LeftBrace,
    RightBrace,
    // 基礎構造
    Let,
    Lambda,
    If,
    // 變數
    Variable(String),
    // 基礎資料型別
    Int(i64),
    Float(f64), 
    True,
    False,
    // 運算子
    Equal,
    Add,
    Sub,
    Mul,
    Div,
}

fn split_program(program: &String) -> Vec<String> {
    let mut ret = Vec::new();

    let mut buf = String::new();
    for c in program.chars() {
        if c == '(' {
            ret.push("(".to_string());
        } else if c == ')' {

            if buf.len() > 0 {
                ret.push(buf);
                buf = String::new();
            }

            ret.push(")".to_string());
        } else if (c == ' ' || c == '\t' || c == '\n') && buf.len() > 0 {
            ret.push(buf);
            buf = String::new();
        } else if !(c == ' ' || c == '\t' || c == '\n') {
            buf.push(c);
        }
    }
    return ret;
}

pub fn lexer(program: &String) -> Vec<Token> {
    let mut buf = Vec::new();
    let splited_program = split_program(program);
    for word in splited_program {
        if word == "(" {
            buf.push(Token::LeftBrace);
        } else if word == ")" {
            buf.push(Token::RightBrace);
        } else if word == "let" {
            buf.push(Token::Let);
        } else if word == "lambda" {
            buf.push(Token::Lambda);
        } else if word == "if" {
            buf.push(Token::If);
        } else if word == "=" {
            buf.push(Token::Equal);
        } else if word == "+" {
            buf.push(Token::Add);
        } else if word == "-" {
            buf.push(Token::Sub);
        } else if word == "*" {
            buf.push(Token::Mul);
        } else if word == "/" {
            buf.push(Token::Div);
        } else if word == "#t" {
            buf.push(Token::True);
        } else if word == "#f" {
            buf.push(Token::False);
        } else if word.parse::<i64>().is_ok() {
            buf.push(Token::Int(word.parse::<i64>().unwrap()))
        } else if word.parse::<f64>().is_ok() {
            buf.push(Token::Float(word.parse::<f64>().unwrap()))
        } else {
            buf.push(Token::Variable(word))
        }
    }
    return buf;
}