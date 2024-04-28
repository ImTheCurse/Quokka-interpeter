#[cfg(test)]
mod tests {
    use self::object::Object;
    use crate::evaluator::eval::eval;
    use crate::evaluator::*;
    use crate::lexer::lexer::Lexer;
    use crate::parser::parser::Parser;

    #[test]
    fn test_eval_int_expr() {
        struct Test<'a> {
            input: &'a str,
            expected: i32,
        }
        impl<'a> Test<'a> {
            fn new(inp: &'a str, exp: i32) -> Test {
                Test {
                    input: inp,
                    expected: exp,
                }
            }
        }

        let tests: Vec<_> = vec![
            Test::new("5", 5),
            Test::new("10", 10),
            Test::new("-5", -5),
            Test::new("-10", -10),
            Test::new("5 + 5 + 5 + 5 - 10", 10),
            Test::new("2 * 2 * 2 * 2 * 2", 32),
            Test::new("-50 + 100 -50", 0),
            Test::new("5 * 2 + 10", 20),
            Test::new("5 + 2 * 10", 25),
            Test::new("20 + 2 * -10", 0),
            Test::new("50 / 2 * 2 + 10", 60),
            Test::new("2 * (5 + 10)", 30),
            Test::new("3 * 3 * 3 + 10", 37),
            Test::new("3 * (3 * 3) + 10", 37),
            Test::new("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
        ];

        for t_case in tests.iter() {
            let evaluated = test_eval_helper(t_case.input.to_string());
            test_int_obj_helper(evaluated.unwrap(), t_case.expected);
        }
    }
    #[test]
    fn test_eval_bool_expr() {
        struct Test<'a> {
            input: &'a str,
            expected: bool,
        }
        impl<'a> Test<'a> {
            fn new(inp: &'a str, exp: bool) -> Test {
                Test {
                    input: inp,
                    expected: exp,
                }
            }
        }

        let tests: Vec<_> = vec![
            Test::new("true", true),
            Test::new("false", false),
            Test::new("true == true", true),
            Test::new("false == false", true),
            Test::new("true == false", false),
            Test::new("true != false", true),
            Test::new("false != true", true),
            Test::new("(1 < 2) == true", true),
            Test::new("(1 < 2) == false", false),
            Test::new("(1 > 2) == true", false),
            Test::new("(1 > 2) == false", true),
            Test::new("1 < 2", true),
            Test::new("1 > 2", false),
            Test::new("1 < 1", false),
            Test::new("1 > 1", false),
            Test::new("1 == 1", true),
            Test::new("1 != 1", false),
            Test::new("1 == 2", false),
            Test::new("1 != 2", true),
        ];

        for t_case in tests.iter() {
            let evaluated = test_eval_helper(t_case.input.to_string());
            test_bool_obj(evaluated.unwrap(), t_case.expected);
        }
    }

    #[test]
    fn test_bang_op() {
        struct Test<'a> {
            input: &'a str,
            expected: bool,
        }
        impl<'a> Test<'a> {
            fn new(inp: &'a str, exp: bool) -> Test {
                Test {
                    input: inp,
                    expected: exp,
                }
            }
        }

        let tests = vec![
            Test::new("!true", false),
            Test::new("!false", true),
            Test::new("!5", false),
            Test::new("!!false", false),
            Test::new("!!5", true),
        ];

        for t_case in tests.iter() {
            let evaluated = test_eval_helper(t_case.input.to_string());
            test_bool_obj(evaluated.unwrap(), t_case.expected);
        }
    }

    fn test_bool_obj(obj: Object, expected: bool) {
        if let Object::Boolean(b) = obj {
            if b != expected {
                panic!(
                    "Object has the wrong value. Expected:{}, Got:{}",
                    expected, b
                );
            }
            return;
        }
        panic!("Object is not an Boolean.");
    }

    fn test_eval_helper(input: String) -> Option<Object> {
        let mut l = Lexer {
            ch: 'l',
            input: input.clone(),
        };
        let lex = Lexer::new(&mut l, input);
        let mut prsr = Parser::new(lex);

        let program = prsr.parse_program();
        if program.is_none() {
            panic!("Paniced @ parse_program() - no program exists.")
        }
        return eval(&program.unwrap().statments[0]);
    }

    fn test_int_obj_helper(obj: Object, expected: i32) {
        if let Object::Integer(i) = obj {
            if i != expected {
                panic!(
                    "Object has the wrong value. Expected:{}, Got:{}",
                    expected, i
                );
            }
            return;
        }
        panic!("Object is not an integer.");
    }
}
