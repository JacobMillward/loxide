use crate::frontend::lex::token::TokenLiteral;

use super::expression::*;

pub struct AstPrinter {}

impl AstPrinter {
    pub fn new() -> AstPrinter {
        AstPrinter {}
    }

    pub fn print(&mut self, expr: &Expression) -> String {
        expr.accept(self)
    }

    fn parenthesize(&mut self, name: &str, exprs: Vec<&Expression>) -> String {
        let mut result = String::new();
        result.push('(');
        result.push_str(name);
        for expr in exprs {
            result.push(' ');
            result.push_str(&expr.accept(self));
        }
        result.push(')');
        result
    }
}

impl ExpressionVisitor<String> for AstPrinter {
    fn visit_binary_expr(&mut self, expr: &Binary) -> String {
        self.parenthesize(&expr.operator.lexeme, vec![&expr.left, &expr.right])
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> String {
        self.parenthesize("group", vec![&expr.expression])
    }

    fn visit_literal_expr(&mut self, expr: &Literal) -> String {
        if expr.value.is_none() {
            return "nil".to_string();
        }

        match expr.value.as_ref().unwrap() {
            TokenLiteral::Identifier(identifier) => identifier.to_string(),
            TokenLiteral::Number(number) => number.to_string(),
            TokenLiteral::String(string) => string.to_string(),
        }
    }

    fn visit_unary_expr(&mut self, expr: &Unary) -> String {
        self.parenthesize(&expr.operator.lexeme, vec![&expr.right])
    }
}

#[cfg(test)]
mod test {
    use crate::frontend::lex::token::{Token, TokenLiteral, TokenType};

    use super::*;

    #[test]
    fn test_astprinter_print() {
        let mut ast_printer = AstPrinter::new();

        // Expression for -123 * (45.67)
        let expr = Expression::Binary(Binary {
            left: Box::new(Expression::Unary(Unary {
                operator: Token {
                    token_type: TokenType::Minus,
                    lexeme: "-".to_string(),
                    literal: None,
                    line_number: 1,
                },
                right: Box::new(Expression::Literal(Literal {
                    value: Some(TokenLiteral::Number(123.0)),
                })),
            })),
            operator: Token {
                token_type: TokenType::Star,
                lexeme: "*".to_string(),
                literal: None,
                line_number: 1,
            },
            right: Box::new(Expression::Grouping(Grouping {
                expression: Box::new(Expression::Literal(Literal {
                    value: Some(TokenLiteral::Number(45.67)),
                })),
            })),
        });
        let result = ast_printer.print(&expr);

        assert_eq!(result, "(* (- 123) (group 45.67))");
    }
}
