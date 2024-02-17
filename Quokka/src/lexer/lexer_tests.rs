use super::lexer::{Token, TokenType};
#[cfg(test)]
mod tests {

    use super::*;
    use crate::lexer::lexer::{
        Lexer, ASSIGN, COMMA, EOF, IDENT, LBRACK, LPAREN, PLUS, RBRACK, RPAREN, SEMICOLON,
    };

    #[test]
    fn test_next_token_chars_only() {
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
    fn test_next_token_full() {
        let input: &str = "let five = 5;
      let ten = 10;
      let add = fn(x,y){
          x + y;
          };
      let result = add(five,ten);";
        let vec: Vec<(TokenType, &str)> = vec![
            (TokenType(LET.to_string()), "let"),
            (TokenType(IDENT.to_string()), "five"),
            (TokenType(ASSIGN.to_string()), "="),
            (TokenType(INT.to_string()), "5"),
            (TokenType(SEMICOLON.to_string()), ";"),
            (TokenType(LET.to_string()), "let"),
            (TokenType(IDENT.to_string()), "ten"),
            (TokenType(ASSIGN.to_string()), "="),
            (TokenType(INT.to_string()), "10"),
            (TokenType(LET.to_string()), "let"),
            (TokenType(IDENT.to_string()), "add"),
            (TokenType(ASSIGN.to_string()), "="),
            (TokenType(FUNCTION.to_string()), "fn"),
            (TokenType(LPAREN.to_string()), "("),
            (TokenType(IDENT.to_string()), "x"),
            (TokenType(COMMA.to_string()), ","),
            (TokenType(IDENT.to_string()), "y"),
            (TokenType(RPAREN.to_string()), ")"),
            (TokenType(LBRACK.to_string()), "{"),
            (TokenType(IDENT.to_string()), "x"),
            (TokenType(PLUS.to_string()), "+"),
            (TokenType(IDENT.to_string()), "y"),
            (TokenType(SEMICOLON.to_string()), ";"),
            (TokenType(RBRACK.to_string()), "}"),
            (TokenType(SEMICOLON.to_string()), ";"),
            (TokenType(LET.to_string()), "let"),
            (TokenType(IDENT.to_string()), "result"),
            (TokenType(ASSIGN.to_string()), "="),
            (TokenType(IDENT.to_string()), "add"),
            (TokenType(LPAREN.to_string()), "("),
            (TokenType(IDENT.to_string()), "five"),
            (TokenType(COMMA.to_string()), ","),
            (TokenType(IDENT.to_string()), "ten"),
            (TokenType(RPAREN.to_string()), ")"),
            (TokenType(SEMICOLON.to_string()), ";"),
            (TokenType(EOF.to_string()), ""),
        ];

        let mut x = Lexer {
            input: input.to_string(),
            ch: 'l',
        };
        let mut lex = Lexer::new(&mut x, input.to_string());

        for (_, testTup) in vec.iter().enumerate() {
            let tok = lex.next_token();
            assert_eq!(tok.tok_type, testTup.0);
            assert_eq!(tok.literal, testTup.1);
        }
    }
}
