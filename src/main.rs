mod lexer;

use crate::lexer::lexer;


fn main() {
    let code = r#"
        connect backend "https://api.example.com"
        fetch data from "users"
    "#;

    let tokens = lexer(code);
    println!("{:?}", tokens);
}


