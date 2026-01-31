// Assignment 2.1: Abstract Syntax Tree (AST) Builder

enum Expr {
    Value(f64),
    Add(Box<Expr>, String, Box<Expr>),
    Mul(Box<Expr>, String, Box<Expr>),
}

fn print_expr(e: &Expr) {
    match e {
        Expr::Value(v) => print!("{}", v),
        Expr::Add(left, op, right) => {
            print!("(");
            print_expr(&left);
            print!(" {} ", op);
            print_expr(&right);
            print!(")");
        }
        Expr::Mul(left, op, right) => {
            print!("(");
            print_expr(&left);
            print!(" {} ", op);
            print_expr(&right);
            print!(")");
        }
    }
}

fn main() {
    let expression = Expr::Add(
        Box::new(Expr::Value(5.0)),
        String::from("+"),
        Box::new(Expr::Mul(
            Box::new(Expr::Value(10.0)),
            String::from("*"),
            Box::new(Expr::Value(2.0)),
        )),
    );
    print_expr(&expression);
    println!();
}
