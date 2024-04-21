use std::cell::RefCell;

use crate::lexer::enums::{TokenKind, ValueKind};

use super::{
    enums::{Argument, AstNode, Primitive},
    parser::Parser,
    statement::Statement,
};

pub struct Function<'a> {
    parser: &'a mut Parser,
}

impl<'a> Function<'a> {
    pub fn new(parser: &'a mut Parser) -> Self {
        Function { parser }
    }

    pub fn parse(&mut self, public: bool) -> Primitive {
        self.parser.advance();

        let name = self.parser.get_identifier();

        self.parser.advance();
        self.parser.expect_token(TokenKind::LeftParenthesis);
        self.parser.advance();

        let mut arguments = vec![];

        if self.parser.match_token(TokenKind::Type, false) {
            while self.parser.current_token().kind != TokenKind::RightParenthesis {
                let r#type = self.parser.get_type();

                self.parser.advance();

                let name = self.parser.get_identifier();

                self.parser.advance();
                self.parser.match_token(TokenKind::Comma, true);

                arguments.push(Argument { r#type, name })
            }
        }

        self.parser.expect_token(TokenKind::RightParenthesis);
        self.parser.advance();

        let mut r#return: String = "Nil".to_owned();

        if self.parser.match_token(TokenKind::Arrow, true) {
            r#return = self.parser.get_type();
            self.parser.advance();
        }

        self.parser.expect_token(TokenKind::LeftCurlyBrace);

        let body: RefCell<Vec<AstNode>> = RefCell::new(vec![]);

        loop {
            self.parser.advance();

            let current = self.parser.current_token();

            match current.kind {
                TokenKind::RightCurlyBrace => {
                    self.parser.advance();
                    break;
                }
                _ => {
                    let (node, position) = Statement::new(
                        self.parser.tokens.clone(),
                        self.parser.position.clone(),
                        &body,
                    )
                    .parse();

                    let len = body.borrow().len();

                    if len <= 2 {
                        body.borrow_mut().push(node);
                    } else {
                        let possible_len = len - 2;
                        let mut body_ref = body.borrow_mut();
                        let res = body_ref.get(possible_len);

                        match res {
                            Some(val) => match val.clone() {
                                AstNode::LiteralStatement { kind, value } => {
                                    if kind.clone() == TokenKind::ExactLiteral {
                                        match value.clone() {
                                            ValueKind::String(val) => {
                                                if val == "__<#insert#>__".to_owned() {
                                                    body_ref[possible_len] = node;
                                                } else {
                                                    body_ref.push(node)
                                                }
                                            }
                                            _ => body_ref.push(node),
                                        }
                                    } else {
                                        body_ref.push(node);
                                    }
                                }
                                _ => body_ref.push(node),
                            },
                            _ => body_ref.push(node),
                        }
                    }

                    self.parser.position = position;
                }
            };
        }

        let res = body.borrow_mut().to_owned().clone();

        Primitive::Operation {
            public,
            name,
            arguments,
            r#return,
            body: res,
        }
    }
}
