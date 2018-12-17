mod lexer;
mod parser;

use lexer::{ lexer };
use parser::{ parser, Expression };

use std::collections::HashMap;

// 求值的結果，亦即在本 lisp 方言中的的型別
#[derive(Debug, Clone)]
enum LispValue {
    Int(i64),
    Float(f64),
    Bool(bool),
    Closure(String, Box<Expression>, HashMap<String, LispValue>)
}

fn number_operation(
    arg1: &Expression,
    arg2: &Expression,
    environment: &HashMap<String, LispValue>,
    int_op: fn(i64, i64) -> LispValue,
    float_op: fn(f64, f64) -> LispValue
) -> LispValue {
    let arg1_value = arg1.evaluation(environment);
    let arg2_value = arg2.evaluation(environment);

    match arg1_value {
        LispValue::Int(n1) => {
            match arg2_value {
                LispValue::Int(n2) => {
                    int_op(n1, n2)
                },
                _ => {
                    panic!("型別錯誤，{:?} 與 {:?} 運算", arg1_value, arg2_value)
                }
            }
        },
        LispValue::Float(n1) => {
            match arg2_value {
                LispValue::Float(n2) => {
                    float_op(n1, n2)
                },
                _ => {
                    panic!("型別錯誤，{:?} 與 {:?} 運算", arg1_value, arg2_value)
                }
            }
        },
        _ => {
            panic!("型別錯誤，{:?} 與 {:?} 運算", arg1_value, arg2_value);
        }
    }
}

// TODO: 太多 clone, 性能損耗過多
impl Expression {
    fn evaluation(&self, environment: &HashMap<String, LispValue>) -> LispValue {
        match self {
            Expression::Int(n) => {
                LispValue::Int(*n)
            },
            Expression::Float(n) => {
                LispValue::Float(*n)
            },
            Expression::Bool(b) => {
                LispValue::Bool(*b)
            },
            Expression::Variable(v) => {
                match environment.get(v) {
                    Some(lisp_value) => lisp_value.clone(),
                    None => panic!("變數 {} 未綁定", v)
                }
            }
            Expression::Equal(arg1, arg2) => {
                number_operation(
                    &arg1, &arg2, environment,
                    |a: i64, b: i64| -> LispValue { if a == b { LispValue::Bool(true) } else { LispValue::Bool(false) } },
                    |a: f64, b: f64| -> LispValue { if a == b { LispValue::Bool(true) } else { LispValue::Bool(false) } }
                )
            },
            Expression::Add(arg1, arg2) => {
                number_operation(
                    &arg1, &arg2, environment,
                    |a: i64, b: i64| -> LispValue { LispValue::Int(a + b) },
                    |a: f64, b: f64| -> LispValue { LispValue::Float(a + b) }
                )
            },
            Expression::Sub(arg1, arg2) => {
                number_operation(
                    &arg1, &arg2, environment,
                    |a: i64, b: i64| -> LispValue { LispValue::Int(a - b) },
                    |a: f64, b: f64| -> LispValue { LispValue::Float(a - b) }
                )
            },
            Expression::Mul(arg1, arg2) => {
                number_operation(
                    &arg1, &arg2, environment,
                    |a: i64, b: i64| -> LispValue { LispValue::Int(a * b) },
                    |a: f64, b: f64| -> LispValue { LispValue::Float(a * b) }
                )
            },
            Expression::Div(arg1, arg2) => {
                number_operation(
                    &arg1, &arg2, environment,
                    |a: i64, b: i64| -> LispValue { LispValue::Int(a / b) },
                    |a: f64, b: f64| -> LispValue { LispValue::Float(a / b) }
                )
            },
            Expression::If{ condition, true_value, false_value } => {
                let cond_value = condition.evaluation(environment);
                match cond_value {
                    LispValue::Bool(b) => {
                        if b { true_value.evaluation(environment) } else { false_value.evaluation(environment) }
                    },
                    _ => {
                        panic!("if 表達式的條件並非真假值")
                    }
                }
            },
            Expression::Lambda{ variable, value } => {
                LispValue::Closure(variable.clone(), value.clone(), environment.clone())
            },
            Expression::Call(func, arg) => {
                match func.evaluation(environment) {
                    LispValue::Closure(variable, value, closure_env) => {
                        let mut new_env = closure_env.clone();
                        new_env.insert(variable, arg.evaluation(environment));

                        value.evaluation(&new_env)
                    },
                    _ => {
                        panic!("只有 lambda 可以接受呼叫")
                    }
                }
            },
            Expression::Let{ variable, binding_value, value } => {
                let mut new_env = environment.clone();
                new_env.insert(variable.clone(), binding_value.evaluation(environment));
                value.evaluation(&new_env)
            }
        }
    }
}

fn show_process(program: &String) {
    let tokens = lexer(program);
    
    for token in &tokens {
        println!("{:?}", token);
    }

    let expressions = parser(&tokens);

    for expression in &expressions {
        println!("{:?}", expression);
        println!("{:?}", expression.evaluation(&HashMap::new()));
    }
}

fn main() {

    let program1 = "(let (a 1) (+ a 2))".to_string();
    show_process(&program1);

    let program2 = "
(let (f (lambda (a) (+ a 1)))
    (let (g (lambda (a) (* a 2)))
        (g (f 2))))
".to_string();
    show_process(&program2);
}