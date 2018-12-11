use lexer::{ Token };

#[derive(Debug)]
pub enum Expression {
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
    // 函式呼叫
    Call(Box<Expression>, Box<Expression>),
}

fn parse_one_expression(tokens: &Vec<Token>, position: usize) -> (Expression, usize) {
    let mut cur = position;

    // TODO: 再加一個字串型別的變數，讓 panic 時輸出
    fn assert_token (token: &Token, target: &Token) {
        if token != target {
            panic!("語法分析錯誤，預期是 {:?} ，實際是 {:?}", target, token);
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
        Token::Variable(ref s) => {
            return (Expression::Variable(s.clone()), cur + 1)
        },
        // 複合表達式（需要）
        Token::LeftBrace => {
            cur += 1;

            let expression: Expression = match tokens[cur] {
                Token::Let => {
                    cur += 1;

                    assert_token(&tokens[cur], &Token::LeftBrace); cur += 1;

                    let (variable, _cur) = parse_one_expression(tokens, cur);
                    cur = _cur;

                    let variable_s = match variable {
                        Expression::Variable(s) => s,
                        _ => panic!("語法分析錯誤，let 表達式之後不接變數")
                    };

                    let (binding_value, _cur) = parse_one_expression(tokens, cur);
                    cur = _cur;

                    assert_token(&tokens[cur], &Token::RightBrace); cur += 1;

                    let (value, _cur) = parse_one_expression(tokens, cur);
                    cur = _cur;

                    Expression::Let{
                        variable: variable_s,
                        binding_value: Box::new(binding_value),
                        value: Box::new(value) 
                    }

                },
                Token::Lambda => {
                    cur += 1;

                    assert_token(&tokens[cur], &Token::LeftBrace); cur += 1;

                    let (variable, _cur) = parse_one_expression(tokens, cur);
                    cur = _cur;

                    let variable_s = match variable {
                        Expression::Variable(s) => s,
                        _ => panic!("語法分析錯誤，lambda 表達式之後不接變數")
                    };
                    
                    assert_token(&tokens[cur], &Token::RightBrace); cur += 1;

                    let (value, _cur) = parse_one_expression(tokens, cur);
                    cur = _cur;

                    Expression::Lambda{
                        variable: variable_s,
                        value: Box::new(value)
                    }
                },
                Token::If => {
                    cur += 1;

                    let (condition, _cur) = parse_one_expression(tokens, cur);
                    cur = _cur;

                    let (true_vale, _cur) = parse_one_expression(tokens, cur);
                    cur = _cur;

                    let (false_value, _cur) = parse_one_expression(tokens, cur);
                    cur = _cur;

                    Expression::If{
                        condition: Box::new(condition),
                        true_value: Box::new(true_vale),
                        false_value: Box::new(false_value)
                    }
                },
                Token::Equal => {
                    cur += 1;
                    let (arg1, _cur) = parse_one_expression(tokens, cur);
                    let (arg2, _cur) = parse_one_expression(tokens, _cur);
                    cur = _cur;
                    Expression::Equal(Box::new(arg1), Box::new(arg2))
                },
                Token::Add => {
                    cur += 1;
                    let (arg1, _cur) = parse_one_expression(tokens, cur);
                    let (arg2, _cur) = parse_one_expression(tokens, _cur);
                    cur = _cur;
                    Expression::Add(Box::new(arg1), Box::new(arg2))
                },
                Token::Sub => {
                    cur += 1;
                    let (arg1, _cur) = parse_one_expression(tokens, cur);
                    let (arg2, _cur) = parse_one_expression(tokens, _cur);
                    cur = _cur;
                    Expression::Sub(Box::new(arg1), Box::new(arg2))
                },
                Token::Mul => {
                    cur += 1;
                    let (arg1, _cur) = parse_one_expression(tokens, cur);
                    let (arg2, _cur) = parse_one_expression(tokens, _cur);
                    cur = _cur;
                    Expression::Mul(Box::new(arg1), Box::new(arg2))
                },
                Token::Div => {
                    cur += 1;
                    let (arg1, _cur) = parse_one_expression(tokens, cur);
                    let (arg2, _cur) = parse_one_expression(tokens, _cur);
                    cur = _cur;
                    Expression::Div(Box::new(arg1), Box::new(arg2))
                },
                Token::LeftBrace => {
                    let (function, _cur) = parse_one_expression(tokens, cur);
                    let (arg, _cur) = parse_one_expression(tokens, _cur);
                    cur = _cur;
                    Expression::Call(Box::new(function), Box::new(arg))
                },
                Token::Variable(ref s) => {
                    cur += 1;
                    let (arg, _cur) = parse_one_expression(tokens, cur);
                    cur = _cur;
                    Expression::Call(Box::new(Expression::Variable(s.clone())), Box::new(arg))
                }
                _ => {
                    panic!("語法分析錯誤，左掛號之後接 {:?}", tokens[cur])
                }
            };

            if tokens[cur] != Token::RightBrace {
                panic!("語法分析錯誤，表達式不以右括號結尾，而是 {:?}", tokens[cur]);
            } else {
                return (expression, cur + 1);
            }
        },
        _ => panic!("語法分析錯誤，表達式開頭不以左括號開頭，而是 {:?}", tokens[cur])
    }
}

pub fn parser(tokens: &Vec<Token>) -> Vec<Expression> {
    let mut buf = Vec::new();
    let mut cur = 0;
    while cur < tokens.len() {
        let (expression, position) = parse_one_expression(&tokens, cur);
        cur = position;
        buf.push(expression);
    }

    return buf;
}

// fn print_expression_tree(expression: &Expression) {
//     fn print_with_level(expression: &Expression, level: u32) {
//         match expression {
//             Expression::If{ condition: condition, true_value: true_value, false_value: false_value } => {
//                 condition
//                 println!("{} {}", )
//             }
//         }
//     } 
 
//     print_with_level(expression, 0);
// }