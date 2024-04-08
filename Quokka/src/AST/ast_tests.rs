#[cfg(test)]
mod test {
    use core::panic;

    use crate::AST::ast::{Expression, Identifier, LetStatment, Program, Statment};

    #[test]
    fn test_to_string() {
        let prog = &mut Program {
            statments: Vec::new(),
        };

        let ident = Identifier {
            value: "myvar".to_string(),
        };

        let expr = Expression::Identifier(Identifier {
            value: "anotherVar".to_string(),
        });

        let letStmt = &LetStatment {
            ident: ident,
            value: expr,
        };

        prog.statments.push(Statment::Let(letStmt.clone()));

        assert_eq!(prog.to_string(), "let myvar = anotherVar;")
    }
}
