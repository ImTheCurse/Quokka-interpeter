#[cfg(test)]
mod test {
    use crate::lexer::lexer::*;
    use crate::parser::parser::Parser;
    use crate::AST::ast::{Expression, Statment};
    use std::panic;

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
        let lex = Lexer::new(&mut l, input.to_string());
        let mut prsr = Parser::new(lex);

        let program = prsr.parse_program();
        if program.is_none() {
            panic!("Paniced @ parse_program() - no program exists.")
        }
        if program.clone().unwrap().statments.len() != 3 {
            check_parser_errors(prsr.errors);
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

    fn check_parser_errors(err: Vec<String>) {
        if err.len() == 0 {
            return;
        }

        println!("Parser has {} errors.", err.len());

        for msg in err {
            println!("parser error: {}", msg);
        }
        panic!();
    }
    /*
    #[test]
    fn test_return_statment() {
        let input = "
        return 5;
        return 10;
        return 9954;";

        let mut l = Lexer {
            ch: 'l',
            input: input.to_string(),
        };
        let lex = Lexer::new(&mut l, input.to_string());
        let mut prsr = Parser::new(lex);
        let program = prsr.parse_program();
        if program.is_none() {
            panic!("Paniced @ parse_program() - no program exists.")
        }
        if program.clone().unwrap().statments.len() != 3 {
            check_parser_errors(prsr.errors);
            panic!(
                "program.statments does not contain 3 statments, got: {}",
                program.unwrap().statments.len()
            );
        }
    }
    */
}
