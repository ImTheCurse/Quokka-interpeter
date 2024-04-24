#[cfg(test)]
mod test {
    use crate::lexer::lexer::*;
    use crate::parser::parser::Parser;
    use crate::token::token::TokenType;
    use crate::AST::ast::{Expression, Identifier, Statment};
    use castaway::cast;
    use std::panic;

    trait Matchable {
        fn callback<T>(expr: &Expression, expected: T)
        where
            T: Matchable + 'static;
    }

    impl Matchable for i32 {
        fn callback<T>(expr: &Expression, expected: T)
        where
            T: Matchable + 'static,
        {
            let x = cast!(expected, i32);
            test_int_lit(expr, x.unwrap_or(0));
        }
    }

    impl Matchable for &str {
        fn callback<T>(expr: &Expression, expected: T)
        where
            T: Matchable + 'static,
        {
            let x = cast!(expected, &str);
            test_ident(expr, x.unwrap_or(""));
        }
    }

    impl Matchable for bool {
        fn callback<T>(expr: &Expression, expected: T)
        where
            T: Matchable,
        {
            let x = cast!(expected, bool);
            if x.is_err() {
                panic!("Expected isn't a boolen type. @Matchable - callback()");
            }
            test_bool_helper(expr, x.unwrap_or(true));
        }
    }

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
    fn test_bool_expr() {
        let input = "true;";
        let mut l = Lexer {
            ch: 't',
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
            if let Expression::BoolenExpr(bool_expr) = &expr_stmt {
                if bool_expr.value != true {
                    panic!(
                        "Boolen expresion isn't correct, expected: {}, got: {}",
                        true, bool_expr.value
                    );
                }
                if bool_expr.tok_type != TokenType::True {
                    panic!(
                        "TokenType isn't correct, expected: {}, got: {}",
                        TokenType::True,
                        bool_expr.tok_type
                    );
                }
                return;
            }
            panic!("Expression isn't Boolen expresion");
        }
        panic!("Statment isn't an expression.");
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
    fn test_if_expr() {
        let input = "if (x < y) { x }";
        let mut l = Lexer {
            ch: 'i',
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
            if let Expression::If(stmt) = expr_stmt {
                test_infix_helper(&stmt.condition, "x", "<", "y");

                if stmt.consequence.stmts.len() != 1 {
                    panic!(
                        "Consequence is not 1 statment. got: {}",
                        stmt.consequence.stmts.len()
                    );
                }

                if let Statment::Expr(expr) = &stmt.consequence.stmts[0] {
                    if !test_ident(&expr, "x") {
                        panic!("Expected ident : {}, but got something else", "x");
                    }
                }
                if stmt.alternative.is_some() {
                    panic!("stmt.alternative has a value.");
                }
            } else {
                panic!("Expression isn't an if expression.");
            }
        }
    }

    #[test]
    fn test_if_expr_alternative() {
        let input = "if (x < y) { x } else { y }";
        let mut l = Lexer {
            ch: 'i',
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
            if let Expression::If(stmt) = expr_stmt {
                test_infix_helper(&stmt.condition, "x", "<", "y");

                if stmt.consequence.stmts.len() != 1 {
                    panic!(
                        "Consequence is not 1 statment. got: {}",
                        stmt.consequence.stmts.len()
                    );
                }

                if let Statment::Expr(expr) = &stmt.consequence.stmts[0] {
                    if !test_ident(&expr, "x") {
                        return;
                    }
                }
                if let Statment::Expr(expr) = &stmt.alternative.clone().unwrap().stmts[0] {
                    if !test_ident(&expr, "y") {
                        panic!("Expected different identifier in else statment.");
                    }
                }
            } else {
                panic!("Expression isn't an if expression.");
            }
        }
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

            if let Statment::Expr(expr) = &program.unwrap().statments[0] {
                match expr {
                    Expression::Prefix(p_ex) => {
                        if p_ex.operator != t_case.op {
                            panic!(
                                "unexpected operator. expected: {}, got: {}",
                                t_case.op, p_ex.operator
                            );
                        }
                        test_int_lit(&p_ex.rhs, t_case.int_value);
                        return;
                    }
                    _ => panic!("Expression isn't an Prefix Expression "),
                }
            }
        }
    }
    #[test]
    fn test_infix_expr() {
        enum Dtype {
            Int(i32),
            Bool(bool),
        }
        struct Infix<'a> {
            input: &'a str,
            lhs: Dtype,
            op: &'a str,
            rhs: Dtype,
        }

        let infix_tests = vec![
            Infix {
                input: "5 + 5;",
                lhs: Dtype::Int(5),
                op: "+",
                rhs: Dtype::Int(5),
            },
            Infix {
                input: "5 - 5;",
                lhs: Dtype::Int(5),
                op: "-",
                rhs: Dtype::Int(5),
            },
            Infix {
                input: "5 * 5;",
                lhs: Dtype::Int(5),
                op: "*",
                rhs: Dtype::Int(5),
            },
            Infix {
                input: "5 / 5;",
                lhs: Dtype::Int(5),
                op: "/",
                rhs: Dtype::Int(5),
            },
            Infix {
                input: "5 > 5;",
                lhs: Dtype::Int(5),
                op: ">",
                rhs: Dtype::Int(5),
            },
            Infix {
                input: "5 < 5;",
                lhs: Dtype::Int(5),
                op: "<",
                rhs: Dtype::Int(5),
            },
            Infix {
                input: "5 == 5;",
                lhs: Dtype::Int(5),
                op: "==",
                rhs: Dtype::Int(5),
            },
            Infix {
                input: "5 != 5;",
                lhs: Dtype::Int(5),
                op: "!=",
                rhs: Dtype::Int(5),
            },
            Infix {
                input: "true == true",
                lhs: Dtype::Bool(true),
                op: "==",
                rhs: Dtype::Bool(true),
            },
            Infix {
                input: "true != false",
                lhs: Dtype::Bool(true),
                op: "!=",
                rhs: Dtype::Bool(false),
            },
            Infix {
                input: "false == false",
                lhs: Dtype::Bool(true),
                op: "==",
                rhs: Dtype::Bool(false),
            },
        ];
        for t_case in &infix_tests {
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

            if let Statment::Expr(expr) = &program.unwrap().statments[0] {
                match expr {
                    Expression::Infix(infix) => {
                        if infix.operator != t_case.op {
                            panic!(
                                "unexpected operator. expected: {}, got: {}",
                                t_case.op, infix.operator
                            );
                        }

                        if let Dtype::Bool(lhs) = t_case.lhs {
                            if let Dtype::Bool(rhs) = t_case.rhs {
                                test_infix_helper(expr, lhs, t_case.op, rhs)
                            }
                        }

                        if let Dtype::Int(lhs) = t_case.lhs {
                            if let Dtype::Int(rhs) = t_case.rhs {
                                test_infix_helper(expr, lhs, t_case.op, rhs)
                            }
                        }

                        // test_int_lit(&infix.lhs, t_case.lhs);
                        // test_int_lit(&infix.rhs, t_case.rhs);
                        return;
                    }
                    _ => panic!("Expression isn't an infix Expression "),
                }
            }

            panic!("pancied outside of expression check, @test_infix_expr");
        }
    }

    fn test_int_lit(expr: &Expression, value: i32) -> bool {
        if let Expression::Int(num) = expr {
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

    fn test_ident(expr: &Expression, val: &str) -> bool {
        let ident = match expr {
            Expression::Identifier(i) => i,
            _ => panic!("expression isn't identifier."),
        };

        if ident.value != val {
            panic!("ident.value not {}. got {}", val, ident.value);
        }
        return true;
    }

    fn test_bool_helper(expr: &Expression, value: bool) {
        if let Expression::BoolenExpr(bool_expr) = expr {
            if bool_expr.value != value {
                panic!(
                    "Boolen expression is incorrect, Expected: {},Got: {}",
                    value, bool_expr.value
                );
            }
            return;
        }
        panic!("Expression isn't Boolen.");
    }

    fn test_lit_expr<T>(expr: &Expression, expected: T)
    where
        T: Matchable + 'static,
    {
        T::callback(expr, expected)
    }
    fn test_infix_helper<T>(expr: &Expression, lhs: T, op: &str, rhs: T)
    where
        T: Matchable + 'static,
    {
        if let Expression::Infix(inf) = expr {
            if inf.operator != op {
                panic!(
                    "infix operator is not correct. Expected: {}, Got: {}",
                    op, inf.operator
                );
            }
            test_lit_expr(&inf.lhs, lhs);
            test_lit_expr(&inf.rhs, rhs);
            return;
        }
        panic!("Expression is not Infix.");
    }

    #[test]
    fn test_op_precedence_parse() {
        struct Tst<'a> {
            inp: &'a str,
            expected: &'a str,
        }

        let tests = vec![
            Tst {
                inp: "-a * b",
                expected: "((-a) * b)",
            },
            Tst {
                inp: "!-a",
                expected: "(!(-a))",
            },
            Tst {
                inp: "a + b + c",
                expected: "((a + b) + c)",
            },
            Tst {
                inp: "a * b * c",
                expected: "((a * b) * c)",
            },
            Tst {
                inp: "a * b / c",
                expected: "((a * b) / c)",
            },
            Tst {
                inp: "a + b / c",
                expected: "(a + (b / c))",
            },
            Tst {
                inp: "a + b - c",
                expected: "((a + b) - c)",
            },
            Tst {
                inp: "a + b * c + d / e - f",
                expected: "(((a + (b * c)) + (d / e)) - f)",
            },
            Tst {
                inp: "3 + 4; -5 * 5",
                expected: "(3 + 4)((-5) * 5)",
            },
            Tst {
                inp: "5 > 4 == 3 < 4",
                expected: "((5 > 4) == (3 < 4))",
            },
            Tst {
                inp: "5 < 4 != 3 > 4",
                expected: "((5 < 4) != (3 > 4))",
            },
            Tst {
                inp: "3 + 4 * 5 == 3 * 1 + 4 * 5",
                expected: "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            },
            Tst {
                inp: "3 < 5 == true",
                expected: "((3 < 5) == true)",
            },
            Tst {
                inp: "3 > 5 == false",
                expected: "((3 > 5) == false)",
            },
            Tst {
                inp: "true",
                expected: "true",
            },
            Tst {
                inp: "false",
                expected: "false",
            },
            Tst {
                inp: "(5 + 5) * 2",
                expected: "((5 + 5) * 2)",
            },
            Tst {
                inp: "2 / (5 + 5)",
                expected: "(2 / (5 + 5))",
            },
            Tst {
                inp: "-(5 + 5)",
                expected: "(-(5 + 5))",
            },
            Tst {
                inp: "!(true == true)",
                expected: "(!(true == true))",
            },
            Tst {
                inp: "1 + (2 + 3) + 4",
                expected: "((1 + (2 + 3)) + 4)",
            },
        ];

        for t_case in &tests {
            let mut l = Lexer {
                ch: t_case.inp.chars().next().unwrap(),
                input: t_case.inp.to_string(),
            };
            let lex = Lexer::new(&mut l, t_case.inp.to_string());
            let mut prsr = Parser::new(lex);
            let program = prsr.parse_program();
            if program.is_none() {
                panic!("Paniced @ parse_program() - no program exists.")
            }
            check_parser_errors(prsr.errors);

            let actual = program.unwrap().to_string();
            if actual != t_case.expected {
                panic!("Expected: {}, got: {}", t_case.expected, actual);
            }
        }
    }
}
