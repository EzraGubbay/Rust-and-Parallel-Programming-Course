// Assignment 2.1: Abstract Syntax Tree (AST) Builder

enum Expr {
    Value(f64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

fn print_expr(e: &Expr) {
    match e {
        Expr::Value(v) => print!("{}", v),
        Expr::Add(left, right) => {
            print!("(");
            print_expr(&left);
            print!(" + ");
            print_expr(&right);
            print!(")");
        }
        Expr::Mul(left, right) => {
            print!("(");
            print_expr(&left);
            print!(" * ");
            print_expr(&right);
            print!(")");
        }
    }
}

fn main() {
    let expression = Expr::Add(
        Box::new(Expr::Value(5.0)),
        Box::new(Expr::Mul(
            Box::new(Expr::Value(10.0)),
            Box::new(Expr::Value(2.0)),
        )),
    );
    print_expr(&expression);
    println!();
}
