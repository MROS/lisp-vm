#[derive(Debug, PartialEq)]
enum Token {
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
        }
        else {
            buf.push(c);
        }
    }
    return ret;
}

fn lexer(program: &String) -> Vec<Token> {
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

enum Expression {
    // 基礎構造
    Let { variable: String, binding_value: Box<Expression>, value: Box<Expression> },
    Lambda {variable: String, value: Box<Expression>},
    If { condition: Box<Expression>, true_value: Box<Expression>, false_value: Box<Expression> },
    // 變數
    Variable(String),
    // 基礎資料型別
    Int(i64),
    Float(f64), 
    True,
    False,
    // 運算子
    Equal(Box<Expression>, Box<Expression>),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
}

fn parse_one_expression(tokens: &Vec<Token>, position: usize) -> (Expression, usize) {
    let mut cur = position;

    // TODO: 再加一個字串型別的變數，讓 panic 時輸出
    let assert_token = |target: Token| {
        if tokens[cur] != target {
            panic!("語法分析錯誤，預期是 {:?} ，實際是 {:?}", target, tokens[cur]);
        }
    };

    match tokens[cur] {
        // 基礎表達式
        Token::True => {
            return (Expression::True, cur + 1)
        },
        Token::False => {
            return (Expression::False, cur + 1)
        },
        Token::Int(n) => {
            return (Expression::Int(n), cur + 1)
        },
        Token::Float(n) => {
            return (Expression::Float(n), cur + 1)
        },
        Token::Variable(s) => {
            return (Expression::Variable(s), cur + 1)
        },
        // 複合表達式（需要）
        Token::LeftBrace => {
            cur += 1;

            let expression: Expression = match tokens[cur] {
                Token::Let => {
                    assert_token(Token::LeftBrace); cur += 1;

                    let (variable, cur) = parse_one_expression(tokens, cur);
                    let variable_s = match variable {
                        Expression::Variable(s) => s,
                        _ => panic!("語法分析錯誤，let 表達式之後不接變數")
                    };

                    let (binding_value, cur) = parse_one_expression(tokens, cur);

                    assert_token(Token::RightBrace); cur += 1;

                    let (value, cur) = parse_one_expression(tokens, cur);

                    Expression::Let{
                        variable: variable_s,
                        binding_value: Box::new(binding_value),
                        value: Box::new(value) 
                    }

                },
                Token::Lambda => {
                    assert_token(Token::LeftBrace); cur += 1;

                    let (variable, cur) = parse_one_expression(tokens, cur);
                    let variable_s = match variable {
                        Expression::Variable(s) => s,
                        _ => panic!("語法分析錯誤，lambda 表達式之後不接變數")
                    };
                    
                    assert_token(Token::RightBrace); cur += 1;

                    let (value, cur) = parse_one_expression(tokens, cur);

                    Expression::Lambda{
                        variable: variable_s,
                        value: Box::new(value)
                    }
                },
                Token::If => {
                    let (condition, cur) = parse_one_expression(tokens, cur);
                    let (true_vale, cur) = parse_one_expression(tokens, cur);
                    let (false_value, cur) = parse_one_expression(tokens, cur);
                    Expression::If{
                        condition: Box::new(condition),
                        true_value: Box::new(true_vale),
                        false_value: Box::new(false_value)
                    }
                },
                Token::Equal => {
                    let (arg1, cur) = parse_one_expression(tokens, cur);
                    let (arg2, cur) = parse_one_expression(tokens, cur);
                    Expression::Equal(Box::new(arg1), Box::new(arg2))
                },
                Token::Add => {
                    let (arg1, cur) = parse_one_expression(tokens, cur);
                    let (arg2, cur) = parse_one_expression(tokens, cur);
                    Expression::Add(Box::new(arg1), Box::new(arg2))
                },
                Token::Sub => {
                    let (arg1, cur) = parse_one_expression(tokens, cur);
                    let (arg2, cur) = parse_one_expression(tokens, cur);
                    Expression::Sub(Box::new(arg1), Box::new(arg2))
                },
                Token::Mul => {
                    let (arg1, cur) = parse_one_expression(tokens, cur);
                    let (arg2, cur) = parse_one_expression(tokens, cur);
                    Expression::Mul(Box::new(arg1), Box::new(arg2))
                },
                Token::Div => {
                    let (arg1, cur) = parse_one_expression(tokens, cur);
                    let (arg2, cur) = parse_one_expression(tokens, cur);
                    Expression::Div(Box::new(arg1), Box::new(arg2))
                }
            };

            if tokens[cur] != Token::RightBrace {
                panic!("語法分析錯誤，表達式不以右括號結尾，而是 {:?}", tokens[cur]);
            } else {
                return (expression, cur + 1);
            }
        },
        _ => panic!("語法分析錯誤，表達式開頭不以左括號開頭，而是 {:?}", tokens[cur]);
    }
}

fn parser(tokens: &Vec<Token>) -> Vec<Expression> {
    let mut buf = Vec::new();
    let mut cur = 0;
    while cur < tokens.len() {
        let (expression, position) = parse_one_expression(&tokens, cur);
        cur = position;
        buf.push(expression);
    }

    return buf;
}

fn show_process(program: &String) {
    let tokens = lexer(program);
    
    for token in tokens {
        println!("{:?}", token);
    }
}

fn main() {

    // println!("{:?}", Token::Add);
    let program1 = "(let (a 1) (+ a 2))".to_string();

    show_process(&program1);

}