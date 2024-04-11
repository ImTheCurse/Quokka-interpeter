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
    #[test]
    fn test_ident_expr() {
        let input = "
        foobar;
        ";
        let mut l = Lexer {
            ch: 'f',
            input: input.to_string(),
        };
        let lex = Lexer::new(&mut l, input.to_string());
        let mut prsr = Parser::new(lex);

        let program = prsr.parse_program();
        if program.is_none() {
            panic!("Paniced @ parse_program() - no program exists.")
        }
        if program.clone().unwrap().statments.len() != 1 {
            check_parser_errors(prsr.errors);
            panic!(
                "program.statments does not contain 1 statments, got: {}",
                program.unwrap().statments.len()
            );
        }

        if let Statment::Expr(expr_stmt) = &program.unwrap().statments[0] {
            if let Expression::Identifier(idtf) = &expr_stmt {
                if idtf.value != "foobar" {
                    panic!("ident value not foobar, got:{} ", idtf.value);
                }
                return;
            }
            panic!("statment isn't expression. @test_ident_expr");
        }

        panic!("pancied outside of expression check, @test_ident_expr");
    }
    #[test]
    fn test_int_lit_expr() {
        let input = "
        5;
        ";
        let mut l = Lexer {
            ch: '5',
            input: input.to_string(),
        };
        let lex = Lexer::new(&mut l, input.to_string());
        let mut prsr = Parser::new(lex);

        let program = prsr.parse_program();
        if program.is_none() {
            panic!("Paniced @ parse_program() - no program exists.")
        }
        if program.clone().unwrap().statments.len() != 1 {
            check_parser_errors(prsr.errors);
            panic!(
                "program.statments does not contain 1 statments, got: {}",
                program.unwrap().statments.len()
            );
        }

        if let Statment::Expr(expr_stmt) = &program.unwrap().statments[0] {
            if let Expression::Int(num) = &expr_stmt {
                if num.value != 5 {
                    panic!("value != 5 , got:{} ", num.value);
                }

                return;
            }
            panic!("statment isn't expression. @test_int_lit_expr");
        }

        panic!("pancied outside of expression check, @test_int_lit_expr");
    }
    #[test]
    fn test_parse_prefix_expr() {
        pub struct Pre<'a> {
            pub input: &'a str,
            pub op: &'a str,
            pub int_value: i32,
        }

        let prefix_tests: Vec<Pre> = vec![
            Pre {
                input: "!5;",
                op: "!",
                int_value: 5,
            },
            Pre {
                input: "-15",
                op: "-",
                int_value: 15,
            },
        ];

        for t_case in prefix_tests.iter() {
            let mut l = Lexer {
                ch: '5',
                input: t_case.input.to_string(),
            };
            let lex = Lexer::new(&mut l, t_case.input.to_string());
            let mut prsr = Parser::new(lex);

            let program = prsr.parse_program();
            if program.is_none() {
                panic!("Paniced @ parse_program() - no program exists.")
            }
            if program.clone().unwrap().statments.len() != 1 {
                check_parser_errors(prsr.errors);
                panic!(
                    "program.statments does not contain 1 statments, got: {}",
                    program.unwrap().statments.len()
                );
            }

            if let Statment::PrefixExpr(prefix_expr) = &program.unwrap().statments[0] {
                if prefix_expr.operator != t_case.op {
                    panic!(
                        "unexpected operator. expected: {}, got: {}",
                        t_case.op, prefix_expr.operator
                    );
                }
                test_int_lit(&prefix_expr.rhs, t_case.int_value);
                return;
            }

            panic!("Expression isn't an Prefix Expression ");
        }
    }

    fn test_int_lit(rhs: &Expression, value: i32) -> bool {
        if let Expression::Int(num) = rhs {
            if num.value != value {
                panic!(
                    "Unexpected integer value. expected: {}, got: {}",
                    num.value, value
                );
            }
            return true;
        }
        panic!("Expression isn't an int literal");
    }
}
