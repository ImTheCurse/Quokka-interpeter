#[cfg(test)]
mod test {
    use crate::ast::Statment;
    use crate::lexer::lexer::*;
    use crate::parser::parser::Parser;
    use crate::token;
    use crate::token::token::Token;
    use crate::AST::ast::Expression;
    use std::panic;
    use token::token::TokenType;

    #[test]
    fn test_let_statments() {
        let input = "
        let x = 5;
        let y = 8;
        let foobar = 83838;
        ";
        let mut l = Lexer {
            ch: 'l',
            input: input.to_string(),
        };
        let lex = Lexer::new(&mut l,input.to_string());
        let mut prsr = Parser::new(lex);
        

        let program = prsr.parse_program();
        if program.is_none() {
            panic!("Paniced @ parse_program() - no program exists.")
        }
        if program.clone().unwrap().statments.len() != 3 {
            panic!(
                "program.statments does not contain 3 statments, got: {}",
                program.unwrap().statments.len()
            );
        }
        let tests = vec!["x", "y", "foobar"];

        for (i, val) in tests.iter().enumerate() {
            let stmt = &program.clone().unwrap().statments[i];
            test_let_helper(&stmt, val);
        }
    }
    fn test_let_helper(stmt: &Statment, ident: &str) {
        if let Statment::Let(st) = stmt {
            assert_eq!(st.ident.value, ident);
            if let Expression::Literal(lit) = &st.value {
                assert!(lit.value.trim().parse::<i32>().is_ok());
            }
            return;
        }
        panic!("Statment isn't a let statment. @ test_let_helper");
    }
}
