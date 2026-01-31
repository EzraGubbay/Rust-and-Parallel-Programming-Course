// Assignment 2.2: Expression Evaluator

#[derive(Debug)]
enum Expr {
    Value(f64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

fn eval(e: &Expr) -> f64 {
    match e {
        Expr::Value(v) => return *v,
        Expr::Add(left, right) => return eval(left) + eval(right),
        Expr::Mul(left, right) => return eval(left) * eval(right),
        Expr::Sub(left, right) => return eval(left) - eval(right),
        Expr::Div(left, right) => return eval(left) / eval(right),
    }
}

fn main() {
    let expression = Expr::Mul(
        Box::new(Expr::Add(
            Box::new(Expr::Value(10.0)),
            Box::new(Expr::Value(2.0)),
        )),
        Box::new(Expr::Div(
            Box::new(Expr::Value(20.0)),
            Box::new(Expr::Value(4.0)),
        )),
    );

    println!("{:?}", expression);

    println!("{}", eval(&expression));
}
