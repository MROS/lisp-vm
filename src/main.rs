mod lexer;
mod parser;

use lexer::{ lexer };
use parser::{ parser };


fn show_process(program: &String) {
    let tokens = lexer(program);
    
    for token in &tokens {
        println!("{:?}", token);
    }

    let expressions = parser(&tokens);

    for expression in &expressions {
        println!("{:?}", expression);
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