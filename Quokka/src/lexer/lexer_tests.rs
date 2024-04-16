#[cfg(test)]
mod tests {
    use crate::lexer::lexer::*;
    use crate::token;
    use token::token::TokenType;
    #[test]
    fn test_next_token_chars_only() {
        let input: &str = "= +(){},;";
        let vec: Vec<(TokenType, &str)> = vec![
            (TokenType::Assign, "="),
            (TokenType::Plus, "+"),
            (TokenType::Lparen, "("),
            (TokenType::Rparen, ")"),
            (TokenType::Lbrack, "{"),
            (TokenType::Rbrack, "}"),
            (TokenType::Comma, ","),
            (TokenType::Semicolon, ";"),
            (TokenType::EOF, ""),
        ];

        let mut x = Lexer {
            input: input.to_string(),
            ch: '=',
        };
        let mut lex = Lexer::new(&mut x, input.to_string());

        for (_, testTup) in vec.iter().enumerate() {
            let tok = lex.next_token();
            assert_eq!(tok.tok_type, testTup.0);
            assert_eq!(tok.literal, testTup.1);
        }
    }
    #[test]
    fn test_next_token_full() {
        let input: &str = "let five = 5;
            let ten = 10;
            let add = fn(x,y){
            x + y;
            };
            let result = add(five,ten);";
        let vec: Vec<(TokenType, &str)> = vec![
            (TokenType::Let, "let"),
            (TokenType::Ident, "five"),
            (TokenType::Assign, "="),
            (TokenType::Int(5), "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "ten"),
            (TokenType::Assign, "="),
            (TokenType::Int(10), "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "add"),
            (TokenType::Assign, "="),
            (TokenType::Function, "fn"),
            (TokenType::Lparen, "("),
            (TokenType::Ident, "x"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "y"),
            (TokenType::Rparen, ")"),
            (TokenType::Lbrack, "{"),
            (TokenType::Ident, "x"),
            (TokenType::Plus, "+"),
            (TokenType::Ident, "y"),
            (TokenType::Semicolon, ";"),
            (TokenType::Rbrack, "}"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "result"),
            (TokenType::Assign, "="),
            (TokenType::Ident, "add"),
            (TokenType::Lparen, "("),
            (TokenType::Ident, "five"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "ten"),
            (TokenType::Rparen, ")"),
            (TokenType::Semicolon, ";"),
            (TokenType::EOF, ""),
        ];

        let mut x = Lexer {
            input: input.to_string(),
            ch: 'l',
        };
        let mut lex = Lexer::new(&mut x, input.to_string());

        for (_, test_tup) in vec.iter().enumerate() {
            let tok = lex.next_token();
            assert_eq!(tok.tok_type, test_tup.0);
            assert_eq!(tok.literal, test_tup.1);
        }
    }
    #[test]
    fn test_next_token_extended() {
        let input: &str = "let five = 5;
                    let ten = 10;
                    let add = fn(x,y){
                    x + y;
                    };
                    let result = add(five,ten);
                    !-/*5;
                    5 < 10 > 5;";
        let vec: Vec<(TokenType, &str)> = vec![
            (TokenType::Let, "let"),
            (TokenType::Ident, "five"),
            (TokenType::Assign, "="),
            (TokenType::Int(5), "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "ten"),
            (TokenType::Assign, "="),
            (TokenType::Int(10), "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "add"),
            (TokenType::Assign, "="),
            (TokenType::Function, "fn"),
            (TokenType::Lparen, "("),
            (TokenType::Ident, "x"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "y"),
            (TokenType::Rparen, ")"),
            (TokenType::Lbrack, "{"),
            (TokenType::Ident, "x"),
            (TokenType::Plus, "+"),
            (TokenType::Ident, "y"),
            (TokenType::Semicolon, ";"),
            (TokenType::Rbrack, "}"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "result"),
            (TokenType::Assign, "="),
            (TokenType::Ident, "add"),
            (TokenType::Lparen, "("),
            (TokenType::Ident, "five"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "ten"),
            (TokenType::Rparen, ")"),
            (TokenType::Semicolon, ";"),
            (TokenType::Not, "!"),
            (TokenType::Minus, "-"),
            (TokenType::Fslash, "/"),
            (TokenType::Asterisk, "*"),
            (TokenType::Int(5), "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Int(5), "5"),
            (TokenType::Larrow, "<"),
            (TokenType::Int(10), "10"),
            (TokenType::Rarrow, ">"),
            (TokenType::Int(5), "5"),
            (TokenType::Semicolon, ";"),
        ];

        let mut x = Lexer {
            input: input.to_string(),
            ch: 'l',
        };
        let mut lex = Lexer::new(&mut x, input.to_string());

        for (_, test_tup) in vec.iter().enumerate() {
            let tok = lex.next_token();
            assert_eq!(tok.tok_type, test_tup.0);
            assert_eq!(tok.literal, test_tup.1);
        }
    }
    #[test]
    fn next_token_extended_keywords() {
        let input: &str = "let five = 5;
                        let ten = 10;
                        let add = fn(x,y){
                        x + y;
                        };
                        let result = add(five,ten);
                        !-/*5;
                        5 < 10 > 5;

                        if 5 < 10 {
                            return true;
                        }else{
                            return false;
                        }

                        10 == 10;
                        10 != 9;";
        let vec: Vec<(TokenType, &str)> = vec![
            (TokenType::Let, "let"),
            (TokenType::Ident, "five"),
            (TokenType::Assign, "="),
            (TokenType::Int(5), "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "ten"),
            (TokenType::Assign, "="),
            (TokenType::Int(10), "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "add"),
            (TokenType::Assign, "="),
            (TokenType::Function, "fn"),
            (TokenType::Lparen, "("),
            (TokenType::Ident, "x"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "y"),
            (TokenType::Rparen, ")"),
            (TokenType::Lbrack, "{"),
            (TokenType::Ident, "x"),
            (TokenType::Plus, "+"),
            (TokenType::Ident, "y"),
            (TokenType::Semicolon, ";"),
            (TokenType::Rbrack, "}"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "result"),
            (TokenType::Assign, "="),
            (TokenType::Ident, "add"),
            (TokenType::Lparen, "("),
            (TokenType::Ident, "five"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "ten"),
            (TokenType::Rparen, ")"),
            (TokenType::Semicolon, ";"),
            (TokenType::Not, "!"),
            (TokenType::Minus, "-"),
            (TokenType::Fslash, "/"),
            (TokenType::Asterisk, "*"),
            (TokenType::Int(5), "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Int(5), "5"),
            (TokenType::Larrow, "<"),
            (TokenType::Int(10), "10"),
            (TokenType::Rarrow, ">"),
            (TokenType::Int(5), "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::If, "if"),
            (TokenType::Int(5), "5"),
            (TokenType::Larrow, "<"),
            (TokenType::Int(10), "10"),
            (TokenType::Lbrack, "{"),
            (TokenType::Return, "return"),
            (TokenType::True, "true"),
            (TokenType::Semicolon, ";"),
            (TokenType::Rbrack, "}"),
            (TokenType::Else, "else"),
            (TokenType::Lbrack, "{"),
            (TokenType::Return, "return"),
            (TokenType::False, "false"),
            (TokenType::Semicolon, ";"),
            (TokenType::Rbrack, "}"),
            (TokenType::Int(10), "10"),
            (TokenType::EQ, "=="),
            (TokenType::Int(10), "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Int(10), "10"),
            (TokenType::NotEQ, "!="),
            (TokenType::Int(9), "9"),
            (TokenType::Semicolon, ";"),
        ];

        let mut x = Lexer {
            input: input.to_string(),
            ch: 'l',
        };
        let mut lex = Lexer::new(&mut x, input.to_string());

        for (_, test_tup) in vec.iter().enumerate() {
            let tok = lex.next_token();
            assert_eq!(tok.tok_type, test_tup.0);
            assert_eq!(tok.literal, test_tup.1);
        }
    }
}
