use super::lexer::{Token, TokenType};
#[cfg(test)]
mod tests {

    use super::*;
    use crate::lexer::lexer::{
        Lexer, ASSIGN, COMMA, EOF, LBRACK, LPAREN, PLUS, RBRACK, RPAREN, SEMICOLON,
    };

    #[test]
    fn test_next_token() {
        let input: &str = "=+(){},;";
        let vec: Vec<(TokenType, &str)> = vec![
            (TokenType(ASSIGN.to_string()), "="),
            (TokenType(PLUS.to_string()), "+"),
            (TokenType(LPAREN.to_string()), "("),
            (TokenType(RPAREN.to_string()), ")"),
            (TokenType(LBRACK.to_string()), "{"),
            (TokenType(RBRACK.to_string()), "}"),
            (TokenType(COMMA.to_string()), ","),
            (TokenType(SEMICOLON.to_string()), ";"),
            (TokenType(EOF.to_string()), ""),
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
}
