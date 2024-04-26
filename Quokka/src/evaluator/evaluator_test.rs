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

        let tests: Vec<_> = vec![Test::new("5", 5), Test::new("10", 10)];

        for t_case in tests.iter() {
            let evaluated = test_eval_helper(t_case.input.to_string());
            test_int_obj_helper(evaluated.unwrap(), t_case.expected);
        }
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
