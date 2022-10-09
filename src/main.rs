// TODO
// - brackets

use std::io;

#[derive(Clone, Debug)]
enum TokenKind {
    Number(i32),
    Operator(char),
    OpenningBracket,
    ClosingBracket,
}

#[derive(Clone, Debug)]
struct Node {
    operation: TokenKind,
    children: Vec<Node>,
}

fn lex(expr: &String) -> Vec<TokenKind> {
    let mut result: Vec<TokenKind> = Vec::new();

    for c in expr.chars() {
        match c {
            '(' => {
                result.push(TokenKind::OpenningBracket);
            },
            ')' => {
                result.push(TokenKind::ClosingBracket);
            },
            '+' | '-' | '*' | '/'=> {
                result.push(TokenKind::Operator(c));
            },
            ' '|'\n' => {},
            '0'..='9' => {
                match result.last().unwrap_or(&TokenKind::ClosingBracket) {
                    TokenKind::Number(number) => {
                        let max = result.len() - 1;
                        result[max] = TokenKind::Number(number * 10 + (c.to_digit(10).unwrap() as i32))
                    },
                    _ => {
                        result.push(TokenKind::Number(c.to_digit(10).unwrap() as i32))
                    }
                }
            },
            _ => {
                panic!();
            }
        }
    }
    return result;
}

fn parse(tokens: Vec<TokenKind>) -> Node {
    let mut operators: Vec<char> = Vec::new();
    let mut result: Vec<Node> = Vec::new();

    for token in tokens {
        match token {
            TokenKind::Number(number) => {
                result.push(Node { operation: TokenKind::Number(number), children: Vec::new() });
            },
            TokenKind::Operator(operator) => {
                if ['*', '/'].contains(&operators.last().unwrap_or(&' ')) {
                    let first = result.pop().unwrap();
                    let second = result.pop().unwrap();
                    result.push(Node { 
                        operation: TokenKind::Operator(operators.pop().unwrap()),
                        children: vec![second, first]
                    });
                }
                operators.push(operator);
            }
            _ => {}
        }
    }

    while !operators.is_empty() {
        let first = result.pop().unwrap();
        let second = result.pop().unwrap();
        result.push(Node { 
           operation: TokenKind::Operator(operators.pop().unwrap()),
           children: vec![second, first]
        });
    }

    return result[0].clone();
}

fn eval(node: &Node) -> i32 {
    return match node.operation {
        TokenKind::Operator(operator) => {
            let first = eval(&node.children[0]);
            let second = eval(&node.children[1]);
            return match operator {
                '+' => first + second,
                '-' => first - second,
                '*' => first * second,
                '/' => first / second,
                _ => 0
            }
        },
        TokenKind::Number(number) => number,
        _ => 0,
    };
}

fn main() {
    println!("Enter expression: ");

    let mut expr = String::new();

    io::stdin()
        .read_line(&mut expr)
        .expect("Failed to read line");

    println!("{}", eval(&parse(lex(&expr))));
}
