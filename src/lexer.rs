
#[derive(Debug)]

pub enum Token {
    Keyword(String),
    Identifier(String),
    Literal(String),
    EndOfInput,
}

pub fn lexer(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let words: Vec<&str> = input.split_whitespace().collect(); 

    for word in words {
        match word {
           
            "connect" | "fetch" | "filter" | "print" => {
                tokens.push(Token::Keyword(word.to_string())),
                "+" | "-" | "*" | "/" => tokens.push(Token::Operator(word.to_string())),
                "(" | ")" => tokens.push(Token::Delimiter(word.to_string())),
            }
            
            _ if word.starts_with('"') && word.ends_with('"') && word.len() > 1 => {
                tokens.push(Token::Literal(word.trim_matches('"').to_string()))
            }

            _ if word.chars().all(char::is_alphanumeric) => {
                tokens.push(Token::Identifier(word.to_string()))
            }
            _ => {
                eprintln!("Unexpected token: {}", word);
                return vec![];
            }
           
            _ => tokens.push(Token::Identifier(word.to_string())),
        }
    }

    tokens.push(Token::EndOfInput); 
    tokens
}

mod tests {
    use super::*;

    ##[test]
    fn test_lexer() {
        let input = r# "connect backend "https//api.example.com""#;
        let tokens = lexer(input);
        assert_eq!(tokens.len(), 4);
    }
}