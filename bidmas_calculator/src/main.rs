fn main() {
    println!("This program is a simple calculator to play with \n");
    println!("enums with associated data and recursion");

    let expression = Expression::Add(
        Box::new(Expression::Number(1.0)),
        Box::new(Expression::Number(2.0)),
    );

    println!("{}", Expression::evaluate_statement("1 + 2"));
    println!("{}", Expression::evaluate_statement("1 - 2"));
    println!("{}", Expression::evaluate_statement("1 * 2"));
    println!("{}", Expression::evaluate_statement("1 / 2"));
    println!("{}", Expression::evaluate_statement("(1 + 2) * 3"));
    println!("{}", Expression::evaluate_statement("(1 + 2) * 3 / 4"));
    println!("{}", Expression::evaluate_statement("(1 + 2) * (3 / 4) + 5"));
    println!("{}", Expression::evaluate_statement("(1 + 2) * 3 / (4 + 5) - 6"));
    println!("{}", Expression::evaluate_statement("(1 + 2) * 3 / 4 + (5 - 6) * 7"));
}

#[derive(Debug)]
pub enum Expression {
    Number(f64),
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
}

impl Expression {
    fn evaluate(&self) -> f64 {
        match self {
            Expression::Number(number) => *number,
            Expression::Add(left, right) => left.evaluate() + right.evaluate(),
            Expression::Subtract(left, right) => left.evaluate() - right.evaluate(),
            Expression::Multiply(left, right) => left.evaluate() * right.evaluate(),
            Expression::Divide(left, right) => left.evaluate() / right.evaluate(),
        }
    }


    fn evaluate_statement(statement: &str) -> f64 {
        let tokens = tokenize(statement.to_string());
        let expression = parse_expression(&tokens);
        expression.evaluate()
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LeftParen,
    RightParen,
}

fn tokenize(statement: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars = statement.chars().collect::<Vec<char>>();
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            '0'..='9' => {
                let mut num = 0.0;
                while i < chars.len() && chars[i].is_digit(10) {
                    num = num * 10.0 + chars[i].to_digit(10).unwrap() as f64;
                    i += 1;
                }
                tokens.push(Token::Number(num));
            }
            '+' => {
                tokens.push(Token::Plus);
                i += 1;
            }
            '-' => {
                tokens.push(Token::Minus);
                i += 1;
            }
            '*' => {
                tokens.push(Token::Multiply);
                i += 1;
            }
            '/' => {
                tokens.push(Token::Divide);
                i += 1;
            }
            '(' => {
                tokens.push(Token::LeftParen);
                i += 1;
            }
            ')' => {
                tokens.push(Token::RightParen);
                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }
    tokens
}

fn parse_expression(tokens: &Vec<Token>) -> Expression {
    let mut position = 0;
    parse_add_subtract(tokens, &mut position)
}

fn parse_add_subtract(tokens: &Vec<Token>, position: &mut usize) -> Expression {
    let mut left = parse_multiply_divide(tokens, position);
   
    while *position < tokens.len() {
        match tokens[*position] {
            Token::Plus => {
                *position += 1;
                let right = parse_multiply_divide(tokens, position);
                left = Expression::Add(Box::new(left), Box::new(right));
            }
            Token::Minus => {
                *position += 1;
                let right = parse_multiply_divide(tokens, position);
                left = Expression::Subtract(Box::new(left), Box::new(right));
            }
            Token::RightParen => {
                // End of parenthesized expression, let caller handle it
                break;
            }
            _ => break,
        }
    }
    
    left
}

fn parse_multiply_divide(tokens: &Vec<Token>, position: &mut usize) -> Expression {
    let mut left = parse_factor(tokens, position);
 
    while *position < tokens.len() {
        match tokens[*position] {
            Token::Multiply => {
                *position += 1;
                let right = parse_factor(tokens, position);
                left = Expression::Multiply(Box::new(left), Box::new(right));
            }
            Token::Divide => {
                *position += 1;
                let right = parse_factor(tokens, position       );
                left = Expression::Divide(Box::new(left), Box::new(right));
            }
            _ => break,
        }
    }
    
    left
}

fn parse_factor(tokens: &Vec<Token>, position: &mut usize) -> Expression {
    if *position >= tokens.len() {
        panic!("Unexpected end of input");
    }
    
    match &tokens[*position] {
        Token::Number(n) => {
            *position += 1;
            Expression::Number(*n)
        }
        Token::LeftParen => {
            *position += 1; // consume '('
            let expr = parse_add_subtract(tokens, position); // parse inner expression
            if *position < tokens.len() && matches!(tokens[*position], Token::RightParen) {
                *position += 1; // consume ')'
            } else {
                panic!("Expected ')'");
            }
            expr
        }
        _ => panic!("Unexpected token at position {}", position),
    }
}
