use crate::lexer::enums::{Token, TokenKind, ValueKind};

use super::enums::AstNode;

pub struct Statement {
    tokens: Vec<Token>,
    position: usize,
}

impl Statement {
    pub fn new(tokens: Vec<Token>, position: usize) -> Self {
        Statement { tokens, position }
    }

    pub fn advance(&mut self) {
        if self.is_eof() {
            println!("The position of {:?} is the last index of the token stack. Staying at the same position.", self.position);
        } else {
            self.position += 1;
        }
    }

    fn current_token(&self) -> Token {
        self.tokens[self.position].clone()
    }

    fn next_token(&mut self) -> Option<Token> {
        match self.is_eof() {
            true => None,
            false => Some(self.tokens[self.position + 1].clone()),
        }
    }

    fn is_eof(&mut self) -> bool {
        self.position + 1 >= self.tokens.len()
    }

    fn expect_token_with_message(&self, expected: TokenKind, message: Option<&str>) {
        if self.current_token().kind != expected {
            panic!(
                "[{}] Expected {:?}, found {:?}. {}",
                self.current_token().location.display(),
                expected,
                self.current_token().kind,
                message.unwrap_or("")
            );
        }
    }

    fn expect_token(&self, expected: TokenKind) {
        self.expect_token_with_message(expected, None);
    }

    fn get(&mut self, expected: TokenKind) -> String {
        self.expect_token(expected.clone());

        let identifier = if let Token {
            value: ValueKind::String(identifier),
            ..
        } = self.current_token()
        {
            identifier.clone()
        } else {
            panic!(
                "Expected {:?}, got {:?}",
                expected.clone(),
                self.current_token()
            );
        };

        identifier
    }

    pub fn get_identifier(&mut self) -> String {
        self.get(TokenKind::Identifier)
    }

    pub fn get_type(&mut self) -> String {
        self.get(TokenKind::Type)
    }

    fn parse_declare(&mut self) -> AstNode {
        let r#type = self.get_type();
        self.advance();

        let name = self.get_identifier();

        self.advance();
        self.expect_token(TokenKind::Equal);
        self.advance();

        let mut value = vec![];

        while self.current_token().kind != TokenKind::Semicolon && !self.is_eof() {
            value.push(self.current_token());
            self.advance();
        }

        self.expect_token(TokenKind::Semicolon);

        AstNode::DeclareStatement {
            name,
            r#type,
            value: Box::new(Statement::new(value, 0).parse().0),
        }
    }

    fn parse_declarative_like(&mut self) -> AstNode {
        let name = self.get_identifier();

        self.advance();
        let operation = self.current_token();
        self.advance();

        let res = match operation.kind {
            TokenKind::AddOne => Some(AstNode::DeclareStatement {
                name: name.clone(),
                r#type: "Nil".to_owned(),
                value: Box::new(AstNode::ArithmeticOperation {
                    left: Box::new(AstNode::LiteralStatement {
                        kind: TokenKind::Identifier,
                        value: ValueKind::String(name.clone()),
                    }),
                    right: Box::new(AstNode::LiteralStatement {
                        kind: TokenKind::IntegerLiteral,
                        value: ValueKind::Number(1),
                    }),
                    operator: TokenKind::Add,
                }),
            }),
            TokenKind::SubtractOne => Some(AstNode::DeclareStatement {
                name: name.clone(),
                r#type: "Nil".to_owned(),
                value: Box::new(AstNode::ArithmeticOperation {
                    left: Box::new(AstNode::LiteralStatement {
                        kind: TokenKind::Identifier,
                        value: ValueKind::String(name.clone()),
                    }),
                    right: Box::new(AstNode::LiteralStatement {
                        kind: TokenKind::IntegerLiteral,
                        value: ValueKind::Number(1),
                    }),
                    operator: TokenKind::Subtract,
                }),
            }),
            _ => None,
        };

        if res.is_some() {
            self.expect_token(TokenKind::Semicolon);
            return res.unwrap();
        }

        let mut value = vec![];

        while self.current_token().kind != TokenKind::Semicolon && !self.is_eof() {
            value.push(self.current_token());
            self.advance();
        }

        self.expect_token(TokenKind::Semicolon);

        let mapping = match operation.kind {
            TokenKind::AddEqual => TokenKind::Add,
            TokenKind::SubtractEqual => TokenKind::Subtract,
            TokenKind::MultiplyEqual => TokenKind::Multiply,
            TokenKind::DivideEqual => TokenKind::Divide,
            TokenKind::ModulusEqual => TokenKind::Modulus,
            other => panic!("Invalid identifier operation {:?}", other),
        };

        AstNode::DeclareStatement {
            name: name.clone(),
            r#type: "Nil".to_owned(),
            value: Box::new(AstNode::ArithmeticOperation {
                left: Box::new(AstNode::LiteralStatement {
                    kind: TokenKind::Identifier,
                    value: ValueKind::String(name),
                }),
                right: Box::new(Statement::new(value, 0).parse().0),
                operator: mapping,
            }),
        }
    }

    fn parse_literal(&mut self) -> AstNode {
        if self.tokens.len() - self.position == 1 {
            let current = self.current_token();

            AstNode::LiteralStatement {
                kind: current.kind,
                value: current.value,
            }
        } else {
            match self.next_token() {
                Some(token) => match token.kind {
                    TokenKind::Semicolon => {
                        let current = self.current_token();

                        self.advance();

                        AstNode::LiteralStatement {
                            kind: current.kind,
                            value: current.value,
                        }
                    }
                    _ => self.parse_arithmetic(),
                },
                None => self.parse_arithmetic(),
            }
        }
    }

    fn parse_return(&mut self) -> AstNode {
        self.advance();

        let mut value = vec![];

        while self.current_token().kind != TokenKind::Semicolon && !self.is_eof() {
            value.push(self.current_token());
            self.advance();
        }

        self.expect_token(TokenKind::Semicolon);

        AstNode::ReturnStatement {
            value: Box::new(Statement::new(value, 0).parse().0),
        }
    }

    fn parse_function(&mut self, variadic: bool) -> AstNode {
        let name = self.get_identifier();

        self.advance();
        match self.current_token().kind {
            TokenKind::LeftParenthesis => {
                self.advance();
            }
            TokenKind::Not => {
                self.advance();
                self.expect_token(TokenKind::LeftParenthesis);
                self.advance();
            }
            other => panic!("Expected left parethesis or exclamation mark (for variadic functions) but got {:?}", other)
        }

        let mut parameters = vec![];

        while self.current_token().kind != TokenKind::RightParenthesis && !self.is_eof() {
            let mut tokens = vec![];
            let mut nesting = 0;

            loop {
                if self.current_token().kind == TokenKind::LeftParenthesis {
                    nesting += 1;
                }

                tokens.push(self.current_token());
                self.advance();

                if self.current_token().kind == TokenKind::Comma {
                    if nesting > 0 {
                        // Comma in an inner function should just be added to the
                        // token list to be parsed
                        tokens.push(self.current_token());
                        self.advance();
                        continue;
                    } else {
                        // Continue to the next parameter in the outer function
                        self.advance();
                        break;
                    }
                }

                if self.current_token().kind == TokenKind::RightParenthesis {
                    if nesting > 0 {
                        nesting -= 1;
                    } else {
                        break; // The function call has ended
                    }
                }

                if self.is_eof() {
                    break;
                }
            }

            parameters.push(Statement::new(tokens, 0).parse().0);
        }

        self.expect_token_with_message(
            TokenKind::RightParenthesis,
            Some("Perhaps you forgot to close a nested expression?"),
        );

        self.advance();

        if variadic {
            parameters.insert(
                1,
                AstNode::LiteralStatement {
                    kind: TokenKind::ExactLiteral,
                    value: ValueKind::String("...".to_owned()),
                },
            )
        }

        AstNode::FunctionCall { name, parameters }
    }

    fn find_lowest_precedence(&mut self) -> usize {
        let tokens = self.tokens.clone();
        let mut precedence = TokenKind::highest_precedence();
        let mut precedence_index = 0;
        let mut index = self.position.clone();

        loop {
            index += 1;

            if index >= tokens.len() - 1 {
                break;
            }

            let token = tokens[index].clone();

            // Set the precedence to the last lowest precedence found.
            // If the expression is 1 + 2 * 3 + 4 * 5 for example,
            // it'll return the position of the second '+' token
            if token.kind.is_arithmetic() && token.kind.precedence() <= precedence {
                precedence_index = index;
                precedence = token.kind.precedence();
            }
        }

        precedence_index
    }

    fn parse_arithmetic(&mut self) -> AstNode {
        let position = self.find_lowest_precedence();
        let operator = self.tokens[position].clone().kind;

        let tokens = self.tokens.clone();
        let left =
            tokens[self.position..=if position > 0 { position - 1 } else { position }].to_vec();
        let mut raw_right = tokens[position..=tokens.len() - 1].to_vec();

        raw_right.remove(0); // Get rid of the operator

        // Shift the position across the size of the expression
        self.position += left.len();
        self.position += raw_right.len();

        let right = if let Some(index) = raw_right
            .iter()
            .position(|token| token.kind == TokenKind::Semicolon)
        {
            raw_right[..index].to_vec()
        } else {
            raw_right
        };

        AstNode::ArithmeticOperation {
            left: Box::new(Statement::new(left, 0).parse().0),
            right: Box::new(Statement::new(right, 0).parse().0),
            operator,
        }
    }

    fn parse_expression(&mut self) -> AstNode {
        let mut node = self.parse_primary();

        while self.current_token().kind.is_arithmetic() {
            let operator = self.current_token().kind;

            self.advance();

            let right = self.parse_primary();

            node = AstNode::ArithmeticOperation {
                left: Box::new(node),
                right: Box::new(right),
                operator,
            };
        }

        node
    }

    fn parse_primary(&mut self) -> AstNode {
        match self.current_token().kind {
            TokenKind::IntegerLiteral
            | TokenKind::StringLiteral
            | TokenKind::CharLiteral
            | TokenKind::ExactLiteral => self.parse_literal(),
            TokenKind::Identifier => {
                if self.is_eof() {
                    self.parse_literal()
                } else {
                    let next = self.tokens[self.position + 1].clone();

                    if next.kind == TokenKind::LeftParenthesis {
                        self.parse_function(false)
                    } else if next.kind == TokenKind::Not {
                        if self.position + 2 > self.tokens.len() - 1 {
                            panic!("EOF but specified a variadic identifier")
                        }

                        self.parse_function(true)
                    } else if next.kind.is_declarative() {
                        self.parse_declarative_like()
                    } else if next.kind.is_arithmetic() {
                        self.parse_arithmetic()
                    } else {
                        panic!(
                            "[{:?}] Expected left parenthesis or arithmetic, got {:?}",
                            self.current_token().location.display(),
                            self.current_token().kind
                        );
                    }
                }
            }
            _ => panic!(
                "[{:?}] Expected expression, got {:?}",
                self.current_token().location.display(),
                self.current_token().kind
            ),
        }
    }

    pub fn parse(&mut self) -> (AstNode, usize) {
        let node = match self.current_token().kind {
            TokenKind::Type => self.parse_declare(),
            TokenKind::Return => self.parse_return(),
            _ => self.parse_expression(),
        };

        (node, self.position)
    }
}
