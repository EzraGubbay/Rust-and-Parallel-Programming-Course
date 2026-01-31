// Assignment 2.3: Parser with Error Recovery

#[derive(Debug)]
enum Expr {
    Value(f64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

// Note: Removed 'return' keywords as per Rust idiom, but kept the logic same
fn eval(e: &Expr) -> f64 {
    match e {
        Expr::Value(v) => *v,
        Expr::Add(left, right) => eval(left) + eval(right),
        Expr::Mul(left, right) => eval(left) * eval(right),
        Expr::Sub(left, right) => eval(left) - eval(right),
        Expr::Div(left, right) => eval(left) / eval(right),
    }
}

fn parse(tokens: &mut Vec<String>) -> Result<Expr, String> {
    let token = tokens.pop(); // FIXED: Added missing semicolon

    match token {
        None => Err("Unexpected end of input".to_string()),
        Some(e) => match &*e {
            // FIXED: Convert String to &str for matching
            "+" => match parse(tokens) {
                Ok(e_left) => match parse(tokens) {
                    Ok(e_right) => Ok(Expr::Add(Box::new(e_left), Box::new(e_right))),
                    Err(msg) => Err(msg),
                },
                Err(msg) => Err(msg),
            },
            "*" => match parse(tokens) {
                Ok(e_left) => match parse(tokens) {
                    Ok(e_right) => Ok(Expr::Mul(Box::new(e_left), Box::new(e_right))),
                    Err(msg) => Err(msg),
                },
                Err(msg) => Err(msg),
            },
            "-" => match parse(tokens) {
                Ok(e_left) => match parse(tokens) {
                    Ok(e_right) => Ok(Expr::Sub(Box::new(e_left), Box::new(e_right))),
                    Err(msg) => Err(msg),
                },
                Err(msg) => Err(msg),
            },
            "/" => match parse(tokens) {
                Ok(e_left) => match parse(tokens) {
                    Ok(e_right) => Ok(Expr::Div(Box::new(e_left), Box::new(e_right))),
                    Err(msg) => Err(msg),
                },
                Err(msg) => Err(msg),
            },
            // FIXED: We must parse the string 'e', not the Option 'token'
            _ => match e.parse::<f64>() {
                Ok(n) => Ok(Expr::Value(n)),
                Err(_) => Err(format!("Invalid number: {}", e)), // FIXED: Dynamic error message
            },
        },
    }
}

/**
 * Tests:
 * ["+", "2"]
 * ["+", "garbage", "2"]
 */
fn main() {
    // Standard test
    let input = vec!["4", "20", "/", "2", "10", "+", "*"];
    let mut processed: Vec<String> = input.iter().map(|s| s.to_string()).collect();

    println!("Testing valid input...");
    match parse(&mut processed) {
        Ok(e) => {
            println!("Parsed: {:?}", e);
            println!("Result: {}", eval(&e));
        }
        Err(msg) => println!("Error: {}", msg),
    }

    // Failure Mode Test: Missing Operand
    println!("\nTesting missing operand...");
    let mut bad_input = vec!["2", "+"].iter().map(|s| s.to_string()).collect();
    match parse(&mut bad_input) {
        Ok(e) => println!("Parsed: {:?}", e),
        Err(msg) => println!("Caught expected error: {}", msg),
    }

    // Failure Mode Test: Bad Number
    println!("\nTesting bad number...");
    let mut bad_input = vec!["2", "garbage", "+"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    match parse(&mut bad_input) {
        Ok(e) => println!("Parsed: {:?}", e),
        Err(msg) => println!("Caught expected error: {}", msg),
    }
}
