use std::{cell::RefCell, collections::HashMap, fs::File, io::Write};

use crate::{
    lexer::enums::{TokenKind, ValueKind},
    parser::enums::{Argument, AstNode, Primitive},
};

type GeneratorResult<T> = Result<T, String>;

use super::enums::{
    Comparison, Data, DataItem, Function, Instruction, Linkage, Module, Statement, Type, TypeDef,
    Value,
};

pub struct Compiler {
    tmp_counter: u32,
    scopes: Vec<HashMap<String, (Type, Value)>>,
    data_sections: Vec<Data>,
    type_sections: Vec<TypeDef>,
    loop_labels: Vec<String>,
    ret_types: HashMap<String, Type>,
    tree: Vec<Primitive>,
}

impl Compiler {
    fn new_temporary(&mut self, name: Option<&str>) -> Value {
        self.tmp_counter += 1;
        Value::Temporary(format!("{}.{}", name.unwrap_or("tmp"), self.tmp_counter))
    }

    fn new_var(
        &mut self,
        ty: &Type,
        name: &str,
        func: Option<&RefCell<Function>>,
        new: bool,
    ) -> GeneratorResult<Value> {
        let tmp = if new {
            self.new_temporary(Some(name))
        } else {
            let existing_var = self.get_variable(name, func);

            match existing_var {
                Ok((_, val)) => match val.clone() {
                    Value::Temporary(_) => val.clone(),
                    _ => self.new_temporary(Some(name)),
                },
                Err(_) => self.new_temporary(Some(name)),
            }
        };

        let scope = self
            .scopes
            .last_mut()
            .expect("Expected last scope to exist");

        scope.insert(name.to_owned(), (ty.to_owned(), tmp.to_owned()));

        Ok(tmp)
    }

    fn get_variable(
        &mut self,
        name: &str,
        func: Option<&RefCell<Function>>,
    ) -> GeneratorResult<(Type, Value)> {
        let var = self
            .scopes
            .iter()
            .rev()
            .filter_map(|s| s.get(name))
            .next()
            .ok_or_else(|| format!("\nUndefined variable '{}'\n", name));

        if var.is_err() {
            for item in self.tree.clone() {
                match item {
                    Primitive::Constant {
                        name: const_name,
                        r#type: ty,
                        ..
                    } => {
                        if name == const_name && func.is_some() {
                            let temp = self
                                .new_var(&ty.clone().unwrap(), &name, func, true)
                                .unwrap();

                            func.unwrap().borrow_mut().assign_instruction(
                                temp.clone(),
                                ty.clone().unwrap(),
                                Instruction::Call(Value::Global(name.to_string()), vec![]),
                            );

                            return Ok((ty.unwrap(), temp));
                        }
                    }
                    Primitive::Function { name: op_name, .. } => {
                        if name == op_name {
                            return Ok((
                                Type::Pointer(Box::new(Type::Byte)),
                                Value::Global(name.to_string()),
                            ));
                        }
                    }
                    _ => {}
                }
            }
        }

        var.map(|item| item.to_owned())
    }

    fn compile_literal_to_ascii(&self, val: Value) -> Value {
        match val.clone() {
            Value::Literal(str) => {
                if str.len() > 1 {
                    return val.clone();
                }

                Value::Const(Type::Word, str.escape_debug().nth(0).unwrap() as i64)
            }
            Value::Global(name) => {
                let section = self
                    .data_sections
                    .iter()
                    .find(|section| section.name == name)
                    .unwrap();
                let item = section.items.get(0).unwrap();

                match item.1.clone() {
                    DataItem::Const(val) => Value::Const(Type::Word, val),
                    DataItem::String(str) => {
                        if str.len() > 1 {
                            return val.clone();
                        }

                        Value::Const(Type::Word, str.escape_debug().nth(0).unwrap() as i64)
                    }
                }
            }
            other => other,
        }
    }

    fn generate_function(
        &mut self,
        name: String,
        public: bool,
        variadic: bool,
        manual: bool,
        external: bool,
        arguments: &Vec<Argument>,
        return_type: Option<Type>,
        body: Vec<AstNode>,
        module: &RefCell<Module>,
    ) -> GeneratorResult<Function> {
        self.scopes.push(HashMap::new());

        let mut args = Vec::new();

        for argument in arguments.clone() {
            let ty = argument.r#type.clone();
            let tmp = self.new_var(&ty, &argument.name, None, false)?;

            args.push((ty.into_abi(), tmp));
        }

        let mut func = Function {
            linkage: if public {
                Linkage::public()
            } else {
                Linkage::private()
            },
            name: name.clone(),
            variadic,
            // Make a good guess if the function isn't defined as variadic
            variadic_index: if variadic { args.len() } else { 1 },
            manual,
            external,
            arguments: args,
            return_type,
            blocks: Vec::new(),
        };

        if external {
            return Ok(func);
        }

        if func.return_type.is_some() {
            self.ret_types
                .insert(name.clone(), func.clone().return_type.unwrap());
        }

        func.add_block("start".to_string());

        let func_ref = RefCell::new(func);

        for statement in body.clone() {
            // Ignore plain literals that aren't assigned to anything
            match statement.clone() {
                AstNode::LiteralStatement { kind, value: _ } => match kind.clone() {
                    TokenKind::ExactLiteral => {
                        match self.generate_statement(
                            &func_ref,
                            module,
                            statement.clone(),
                            None,
                            false,
                        ) {
                            Some((_, value)) => func_ref
                                .borrow_mut()
                                .add_instruction(Instruction::Literal(value)),
                            _ => {}
                        }
                    }
                    _ => {}
                },
                _ => {
                    match self.generate_statement(&func_ref, module, statement.clone(), None, false)
                    {
                        _ => {}
                    }
                }
            }
        }

        if !func_ref.borrow_mut().returns() && !func_ref.borrow_mut().manual {
            func_ref
                .borrow_mut()
                .add_instruction(Instruction::Return(Some(Value::Const(Type::Word, 0))));
        }

        if func_ref.borrow_mut().return_type.is_none() {
            let mut func_r = func_ref.borrow_mut().clone();
            let last = func_r.last_block().statements.last();

            let return_type = last
                .iter()
                .find(|statement| matches!(statement, Statement::Volatile(Instruction::Return(_))));

            match return_type {
                Some(res) => match res {
                    Statement::Volatile(instruction) => {
                        match instruction {
                            Instruction::Return(value) => {
                                if value.is_none() {
                                    func_ref.borrow_mut().return_type = Some(Type::Word);
                                } else {
                                    match value.clone().unwrap() {
                                        Value::Const(ty, _) => func_ref.borrow_mut().return_type = Some(ty),
                                        Value::Global(_) => func_ref.borrow_mut().return_type = Some(Type::Long),
                                        Value::Temporary(val) => {
                                            let val_temp = val.clone();
                                            let last = val_temp.split(".").last().unwrap().to_string().parse::<i32>().unwrap() - 1;
                                            let var = self.get_variable(format!("r.v{}", last).as_str(), Some(&func_ref));

                                            match var {
                                                Ok((ty, _)) => {
                                                    func_ref.borrow_mut().return_type = Some(ty.clone());
                                                }
                                                Err(msg) => {
                                                    eprintln!("{}", msg);
                                                    func_ref.borrow_mut().return_type = Some(Type::Word);
                                                }
                                            }
                                        }
                                        Value::Literal(_) => func_ref.borrow_mut().return_type = Some(Type::Long)
                                    }
                                }
                            },
                            _ => panic!("\nSomehow the underlying instruction under the volatile pointer changed.\nThere may be a race condition.\n")
                        };
                    }
                    _ => panic!("Pointer is not volatile."),
                },
                None => func_ref.borrow_mut().return_type = Some(Type::Word),
            }
        }

        self.scopes.pop();

        let func_new = func_ref.borrow_mut().to_owned();
        Ok(func_new)
    }

    fn generate_statement(
        &mut self,
        func: &RefCell<Function>,
        module: &RefCell<Module>,
        stmt: AstNode,
        ty: Option<Type>,
        is_return: bool,
    ) -> Option<(Type, Value)> {
        let res = match stmt {
            AstNode::DeclareStatement {
                name,
                r#type,
                value,
            } => {
                let existing = match self.get_variable(name.as_str(), Some(func)) {
                    Ok((ty, _)) => ty.clone(),
                    Err(_) => Type::Word,
                };

                if r#type.is_none() && self.get_variable(name.as_str(), Some(func)).is_err() {
                    panic!("\nVariable named '{}' hasn't been declared yet.\nPlease declare it before trying to re-declare it.\n", name);
                }

                let ty = r#type.unwrap_or(existing.clone());
                let temp = self.new_var(&ty, &name, Some(func), false).unwrap();
                let parsed =
                    self.generate_statement(func, module, *value.clone(), Some(ty.clone()), false);

                if parsed.is_some() {
                    let (ret_ty, value) = parsed.unwrap();

                    let (final_ty, final_val) = if ret_ty != ty {
                        self.convert_to_type(func, ret_ty.clone(), ty.clone(), value.clone())
                    } else {
                        (ty.clone(), value.clone())
                    };

                    func.borrow_mut().assign_instruction(
                        temp,
                        final_ty.clone(),
                        Instruction::Copy(final_val.clone()),
                    );

                    return Some((final_ty, final_val));
                }

                None
            }
            AstNode::ReturnStatement { value } => {
                match self.generate_statement(func, module, *value.clone(), ty.clone(), true) {
                    Some((ret_ty, value)) => {
                        // Generate a unique scoped temporary variable with the tmp_counter to ensure
                        // it can be yielded later for inferring the return type
                        let tmp = self.new_var(
                            &ret_ty.clone(),
                            format!("r.v{}", self.tmp_counter).as_str(),
                            Some(func),
                            false,
                        );

                        match tmp {
                            Ok(val) => {
                                if !func.borrow_mut().manual {
                                    func.borrow_mut().assign_instruction(
                                        val.clone(),
                                        ret_ty.clone(),
                                        Instruction::Copy(value),
                                    );

                                    func.borrow_mut()
                                        .add_instruction(Instruction::Return(Some(val)))
                                }
                            }
                            // In this case we should panic because the variable returned should be in scope
                            Err(msg) => panic!("{}", msg),
                        }
                    }
                    None => {
                        if !func.borrow_mut().manual {
                            func.borrow_mut().add_instruction(Instruction::Return(None))
                        }
                    }
                }

                None
            }
            AstNode::ArithmeticOperation {
                left,
                right,
                operator,
            } => {
                // Recursively parse the left and right nodes of the binary tree
                // This is why we cant do exponentiation if there isn't a built in instruction.
                // We essentially would have to pseudo-run the code to get the number of
                // multiplications that should occur (a ** b) can have a call on lhs or rhs
                let (left_ty_unparsed, left_val_unparsed) = self
                    .generate_statement(func, module, *left.clone(), ty.clone(), false)
                    .unwrap();

                let (right_ty_unparsed, right_val_unparsed) = self
                    .generate_statement(func, module, *right.clone(), ty.clone(), false)
                    .unwrap();

                let mut left_ty = left_ty_unparsed.clone();
                let mut left_val = left_val_unparsed.clone();
                let mut right_val = right_val_unparsed.clone();

                if left_ty_unparsed.weight() > right_ty_unparsed.weight() {
                    let (_, val) = self.convert_to_type(
                        func,
                        right_ty_unparsed,
                        left_ty_unparsed,
                        right_val_unparsed,
                    );

                    right_val = val;
                } else if left_ty_unparsed.weight() < right_ty_unparsed.weight() {
                    let (ty, val) = self.convert_to_type(
                        func,
                        left_ty_unparsed,
                        right_ty_unparsed,
                        left_val_unparsed,
                    );

                    left_ty = ty;
                    left_val = val;
                }

                let instruction_ty = left_ty;
                let left_temp = self.new_temporary(None);
                let right_temp = self.new_temporary(None);

                func.borrow_mut().assign_instruction(
                    left_temp.clone(),
                    instruction_ty.clone(),
                    Instruction::Copy(left_val),
                );

                // Use the left type as the type of them should match
                func.borrow_mut().assign_instruction(
                    right_temp.clone(),
                    instruction_ty.clone(),
                    Instruction::Copy(right_val),
                );

                let compare = match operator.clone() {
                    TokenKind::GreaterThan
                    | TokenKind::GreaterThanEqual
                    | TokenKind::LessThan
                    | TokenKind::LessThanEqual
                    | TokenKind::EqualTo
                    | TokenKind::NotEqualTo => true,
                    _ => false,
                };

                let res = match operator {
                    TokenKind::Add => Instruction::Add(left_temp, right_temp),
                    TokenKind::Subtract => Instruction::Subtract(left_temp, right_temp),
                    TokenKind::Multiply => Instruction::Multiply(left_temp, right_temp),
                    TokenKind::Divide => Instruction::Divide(left_temp, right_temp),
                    TokenKind::Modulus => Instruction::Modulus(left_temp, right_temp),
                    TokenKind::GreaterThan => Instruction::Compare(
                        instruction_ty.clone(),
                        Comparison::GreaterThan,
                        left_temp,
                        right_temp,
                    ),
                    TokenKind::GreaterThanEqual => Instruction::Compare(
                        instruction_ty.clone(),
                        Comparison::GreaterThanEqual,
                        left_temp,
                        right_temp,
                    ),
                    TokenKind::LessThan => Instruction::Compare(
                        instruction_ty.clone(),
                        Comparison::LessThan,
                        left_temp,
                        right_temp,
                    ),
                    TokenKind::LessThanEqual => Instruction::Compare(
                        instruction_ty.clone(),
                        Comparison::LessThanEqual,
                        left_temp,
                        right_temp,
                    ),
                    TokenKind::EqualTo => Instruction::Compare(
                        instruction_ty.clone(),
                        Comparison::Equal,
                        left_temp,
                        right_temp,
                    ),
                    TokenKind::NotEqualTo => Instruction::Compare(
                        instruction_ty.clone(),
                        Comparison::NotEqual,
                        left_temp,
                        right_temp,
                    ),
                    TokenKind::And => Instruction::BitwiseAnd(left_temp, right_temp),
                    TokenKind::Or => Instruction::BitwiseOr(left_temp, right_temp),
                    TokenKind::Xor => Instruction::BitwiseXor(left_temp, right_temp),
                    _ => panic!("Invalid operator token"),
                };

                let op_temp = self.new_temporary(None);

                let final_ty = if compare {
                    Type::Word
                } else {
                    instruction_ty.clone()
                };

                func.borrow_mut()
                    .assign_instruction(op_temp.clone(), final_ty.clone(), res);

                Some((final_ty, op_temp))
            }
            AstNode::LiteralStatement { kind, value } => match kind.clone() {
                TokenKind::Identifier => match value {
                    ValueKind::String(val) => {
                        let var = self.get_variable(val.as_str(), Some(func));

                        match var {
                            Ok((ty, val)) => Some((ty.into_abi(), val)),
                            Err(msg) => {
                                // If it fails to get the variable from the current scope
                                // then attempt to get it from a global instead
                                let tmp_module = module.borrow_mut();
                                let global = tmp_module
                                    .data
                                    .iter()
                                    .find(|item| item.name == val.as_str());

                                if global.is_some() {
                                    let item = global.unwrap();

                                    Some((Type::Long, Value::Global(item.name.clone())))
                                } else {
                                    panic!("{}", msg);
                                }
                            }
                        }
                    }
                    _ => None,
                },
                TokenKind::ExactLiteral => match value {
                    ValueKind::String(val) => Some((Type::Null, Value::Literal(val))),
                    _ => None,
                },
                TokenKind::Break => {
                    if let Some(label) = &self.loop_labels.last() {
                        func.borrow_mut()
                            .add_instruction(Instruction::Jump(format!("{}.end", label)));
                    } else {
                        panic!("Break can only be used while in a loop.");
                    }

                    None
                }
                TokenKind::Continue => {
                    if let Some(label) = &self.loop_labels.last() {
                        func.borrow_mut()
                            .add_instruction(Instruction::Jump(format!("{}.cond", label)));
                    } else {
                        panic!("Continue can only be used while in a loop.");
                    }

                    None
                }
                _ => match value {
                    ValueKind::Number(val) => {
                        let num_ty = match kind {
                            TokenKind::IntegerLiteral => Type::Word,
                            TokenKind::DoubleLiteral => Type::Double,
                            TokenKind::FloatLiteral => Type::Single,
                            TokenKind::LongLiteral => Type::Long,
                            _ => Type::Word,
                        };

                        let mut final_ty = ty.unwrap_or(num_ty);

                        if is_return {
                            final_ty = func.borrow_mut().return_type.clone().unwrap_or(final_ty);
                        }

                        Some((final_ty.clone(), Value::Const(final_ty, val)))
                    }
                    ValueKind::String(val) => {
                        self.tmp_counter += 1;

                        let name = format!(
                            "{}.{}",
                            func.borrow_mut().name.to_string(),
                            self.tmp_counter
                        );

                        self.data_sections.push(Data::new(
                            Linkage::private(),
                            name.clone(),
                            None,
                            vec![
                                (Type::Byte, DataItem::String(val)),
                                (Type::Byte, DataItem::Const(0)),
                            ],
                        ));

                        Some((Type::Pointer(Box::new(Type::Byte)), Value::Global(name)))
                    }
                    // Characters are just strings enforced to be a single character
                    // This is done at tokenization time
                    ValueKind::Character(val) => {
                        self.tmp_counter += 1;

                        let name = format!(
                            "{}.{}",
                            func.borrow_mut().name.to_string(),
                            self.tmp_counter
                        );

                        self.data_sections.push(Data::new(
                            Linkage::private(),
                            name.clone(),
                            None,
                            vec![(Type::Byte, DataItem::Const(val as i64))],
                        ));

                        Some((Type::Long, Value::Global(name)))
                    }
                    ValueKind::Nil => {
                        self.tmp_counter += 1;

                        let name = format!(
                            "{}.{}",
                            func.borrow_mut().name.to_string(),
                            self.tmp_counter
                        );

                        self.data_sections.push(Data::new(
                            Linkage::private(),
                            name.clone(),
                            None,
                            vec![(Type::Byte, DataItem::Const(0))],
                        ));

                        Some((Type::Long, Value::Global(name)))
                    }
                },
            },
            AstNode::FunctionCall { name, parameters } => {
                let mut params = vec![];

                for parameter in parameters.clone() {
                    params.push(
                        self.generate_statement(func, module, parameter, ty.clone(), false)
                            .unwrap(),
                    );
                }

                // Get the type of the functions based on the ones currently imported into this module
                let cached_ty = self.ret_types.get(&name).unwrap_or(&Type::Word).to_owned();
                let declarative_ty = ty.unwrap_or(cached_ty);

                let module_ref = module.borrow_mut();
                let tmp_function = module_ref
                    .functions
                    .iter()
                    .find(|function| function.name == name.clone())
                    .map(|function| function.clone())
                    .unwrap_or(Function::new(
                        Linkage::public(),
                        name.clone(),
                        false,
                        0,
                        false,
                        false,
                        vec![],
                        Some(declarative_ty.clone()),
                    ));

                let ty = tmp_function.return_type.clone().unwrap_or(declarative_ty);

                if tmp_function.variadic {
                    let res = self
                        .generate_statement(
                            func,
                            module,
                            AstNode::LiteralStatement {
                                kind: TokenKind::ExactLiteral,
                                value: ValueKind::String("...".to_string()),
                            },
                            Some(ty.clone()),
                            false,
                        )
                        .unwrap();

                    params.insert(tmp_function.variadic_index, res);
                }

                self.tmp_counter += 1;
                let temp = self
                    .new_var(
                        &ty,
                        format!("tmp.{}", self.tmp_counter).as_str(),
                        Some(func),
                        true,
                    )
                    .unwrap();

                let (_, val) = self
                    .get_variable(&name, Some(func))
                    .unwrap_or((Type::Long, Value::Global(name)));

                func.borrow_mut().assign_instruction(
                    temp.clone(),
                    ty.clone(),
                    Instruction::Call(val, params),
                );

                Some((ty, temp))
            }
            AstNode::BufferStatement { name, r#type, size } => {
                let buf_ty = Type::Pointer(Box::new(r#type.clone().unwrap()));
                let buf_size = match size {
                    ValueKind::Number(val) => val,
                    _ => panic!("Invalid value kind"),
                };

                let tmp = self
                    .new_var(&buf_ty, &name.clone(), Some(func), true)
                    .unwrap();

                func.borrow_mut().assign_instruction(
                    tmp.clone(),
                    buf_ty.clone(),
                    Instruction::Alloc8(
                        Type::Long.size()
                            + if let Some(ty) = r#type.clone() {
                                ty.size() * buf_size as u64
                            } else {
                                0
                            },
                    ),
                );

                func.borrow_mut().add_instruction(Instruction::Store(
                    Type::Long,
                    tmp.clone(),
                    Value::Const(Type::Word, buf_size as i64),
                ));

                self.tmp_counter += 1;
                let name = format!("buffer.{}.{}", name, self.tmp_counter);
                let typedef = TypeDef {
                    name: name.clone(),
                    align: None,
                    items: if let Some(ty) = r#type {
                        vec![(Type::Long, 1), (ty, buf_size as usize)]
                    } else {
                        vec![(Type::Long, 1)]
                    },
                };

                self.type_sections.push(typedef.clone());
                Some((Type::Aggregate(typedef.clone()), tmp))
            }
            AstNode::StoreStatement {
                name,
                offset,
                value,
            } => {
                let (existing_ty, _) = self.get_variable(&name, Some(func)).unwrap();

                assert!(
                    matches!(existing_ty, Type::Pointer(_)),
                    "Cannot store data to non-pointer types"
                );

                let inner = existing_ty.clone().unwrap().unwrap();

                // Hacky fix to ensure the `argv` is not offset by 8 because it
                // does not have the size at the 0th arg like other arrays
                let is_argv = &func.borrow_mut().name == "main"
                    && &name == "argv"
                    && existing_ty.clone()
                        == Type::Pointer(Box::new(Type::Pointer(Box::new(Type::Char))));

                // Ensure strings do not have their size set as to maintain the
                // C compatibility for passing strings made in QBE to printf
                let is_string = existing_ty.clone() == Type::Pointer(Box::new(Type::Char));

                let node = AstNode::ArithmeticOperation {
                    left: Box::new(AstNode::LiteralStatement {
                        kind: TokenKind::Identifier,
                        value: ValueKind::String(name),
                    }),
                    right: Box::new(AstNode::ArithmeticOperation {
                        left: Box::new(AstNode::LiteralStatement {
                            kind: TokenKind::LongLiteral,
                            value: ValueKind::Number(if is_argv || is_string {
                                0
                            } else {
                                Type::Long.size() as i64
                            }),
                        }),
                        right: Box::new(AstNode::ArithmeticOperation {
                            left: Box::new(AstNode::LiteralStatement {
                                kind: TokenKind::LongLiteral,
                                value: ValueKind::Number(inner.size() as i64),
                            }),
                            right: offset,
                            operator: TokenKind::Multiply,
                        }),
                        operator: TokenKind::Add,
                    }),
                    operator: TokenKind::Add,
                };

                let (ty, compiled_location) = self
                    .generate_statement(func, module, node.clone(), None, false)
                    .unwrap();

                let (_, compiled) = self
                    .generate_statement(func, module, *value.clone(), Some(ty.clone()), false)
                    .unwrap();

                let constant = self.compile_literal_to_ascii(compiled);

                func.borrow_mut().add_instruction(Instruction::Store(
                    inner.to_owned(),
                    compiled_location,
                    constant,
                ));

                None
            }
            AstNode::LoadStatement { name, offset } => {
                let (existing_ty, _) = self.get_variable(&name, Some(func)).unwrap();

                assert!(
                    matches!(existing_ty, Type::Pointer(_)),
                    "Cannot load data from non-pointer types"
                );

                let inner = existing_ty.clone().unwrap().unwrap();

                // Hacky fix to ensure the `argv` is not offset by 8 because it
                // does not have the size at the 0th arg like other arrays
                let is_argv = &func.borrow_mut().name == "main"
                    && &name == "argv"
                    && existing_ty.clone()
                        == Type::Pointer(Box::new(Type::Pointer(Box::new(Type::Char))));

                // Ensure strings do not have their size set as to maintain the
                // C compatibility for passing strings made in QBE to printf
                let is_string = existing_ty.clone() == Type::Pointer(Box::new(Type::Char));

                let node = AstNode::ArithmeticOperation {
                    left: Box::new(AstNode::LiteralStatement {
                        kind: TokenKind::Identifier,
                        value: ValueKind::String(name),
                    }),
                    right: Box::new(AstNode::ArithmeticOperation {
                        left: Box::new(AstNode::LiteralStatement {
                            kind: TokenKind::LongLiteral,
                            value: ValueKind::Number(if is_argv || is_string {
                                0
                            } else {
                                Type::Long.size() as i64
                            }),
                        }),
                        right: Box::new(AstNode::ArithmeticOperation {
                            left: Box::new(AstNode::LiteralStatement {
                                kind: TokenKind::LongLiteral,
                                value: ValueKind::Number(inner.size() as i64),
                            }),
                            right: offset,
                            operator: TokenKind::Multiply,
                        }),
                        operator: TokenKind::Add,
                    }),
                    operator: TokenKind::Add,
                };

                let (_, compiled_location) = self
                    .generate_statement(func, module, node.clone(), ty, false)
                    .unwrap();

                self.tmp_counter += 1;
                let temp = self
                    .new_var(
                        &inner.clone().into_abi().clone(),
                        format!("{}.{}", "tmp", self.tmp_counter).as_str(),
                        Some(func),
                        true,
                    )
                    .expect("Variable to be created");

                func.borrow_mut().assign_instruction(
                    temp.clone(),
                    inner.clone().into_abi(),
                    Instruction::Load(existing_ty.clone(), compiled_location.clone()),
                );

                Some((inner.clone().into_abi(), temp))
            }
            AstNode::IfStatement {
                condition,
                body,
                else_body,
            } => {
                let (_, value) = self
                    .generate_statement(func, module, *condition.clone(), ty, false)
                    .unwrap();

                self.tmp_counter += 1;
                let true_label = format!("ift.{}", self.tmp_counter);
                let false_label = format!("iff.{}", self.tmp_counter);
                let end_label = format!("end.{}", self.tmp_counter);

                func.borrow_mut().add_instruction(Instruction::JumpNonZero(
                    value,
                    true_label.clone(),
                    if else_body.len() > 0 {
                        false_label.clone()
                    } else {
                        end_label.clone()
                    },
                ));

                func.borrow_mut().add_block(true_label);

                for statement in body.clone() {
                    match statement.clone() {
                        AstNode::LiteralStatement {
                            kind,
                            value: literal_value,
                        } => match kind.clone() {
                            TokenKind::ExactLiteral => {
                                match self.generate_statement(
                                    func,
                                    module,
                                    statement.clone(),
                                    None,
                                    false,
                                ) {
                                    Some((_, value)) => match literal_value.clone() {
                                        ValueKind::String(val) => match val.as_str() {
                                            "__MANUAL_RETURN__" => {
                                                func.borrow_mut().manual = true;
                                            }
                                            _ => func
                                                .borrow_mut()
                                                .add_instruction(Instruction::Literal(value)),
                                        },
                                        _ => todo!(),
                                    },
                                    _ => {}
                                }
                            }
                            TokenKind::Break | TokenKind::Continue => {
                                self.generate_statement(
                                    func,
                                    module,
                                    statement.clone(),
                                    None,
                                    false,
                                );
                            }
                            _ => {}
                        },
                        _ => match self.generate_statement(
                            func,
                            module,
                            statement.clone(),
                            None,
                            false,
                        ) {
                            _ => {}
                        },
                    }
                }

                if else_body.len() > 0 {
                    if !func.borrow_mut().blocks.last().map_or(false, |b| b.jumps()) {
                        func.borrow_mut()
                            .add_instruction(Instruction::Jump(end_label.clone()));
                    }

                    func.borrow_mut().add_block(false_label.clone());

                    for statement in else_body.clone() {
                        match statement.clone() {
                            AstNode::LiteralStatement { kind, value: _ } => match kind.clone() {
                                TokenKind::ExactLiteral => match self.generate_statement(
                                    func,
                                    module,
                                    statement.clone(),
                                    None,
                                    false,
                                ) {
                                    Some((_, value)) => func
                                        .borrow_mut()
                                        .add_instruction(Instruction::Literal(value)),
                                    _ => {}
                                },
                                TokenKind::Break | TokenKind::Continue => {
                                    self.generate_statement(
                                        func,
                                        module,
                                        statement.clone(),
                                        None,
                                        false,
                                    );
                                }
                                _ => {}
                            },
                            _ => {
                                match self.generate_statement(
                                    func,
                                    module,
                                    statement.clone(),
                                    None,
                                    false,
                                ) {
                                    _ => {}
                                }
                            }
                        }
                    }
                }

                func.borrow_mut().add_block(end_label);
                None
            }
            AstNode::WhileLoop { condition, body } => {
                self.tmp_counter += 1;

                let cond_label = format!("loop.{}.cond", self.tmp_counter);
                let body_label = format!("loop.{}.body", self.tmp_counter);
                let end_label = format!("loop.{}.end", self.tmp_counter);

                self.loop_labels.push(format!("loop.{}", self.tmp_counter));

                func.borrow_mut().add_block(cond_label.clone());

                let (_, value) = self
                    .generate_statement(func, module, *condition.clone(), ty.clone(), false)
                    .unwrap();

                func.borrow_mut().add_instruction(Instruction::JumpNonZero(
                    value,
                    body_label.clone(),
                    end_label.clone(),
                ));

                func.borrow_mut().add_block(body_label.clone());

                for statement in body.clone() {
                    match statement.clone() {
                        AstNode::LiteralStatement { kind, value: _ } => match kind.clone() {
                            TokenKind::ExactLiteral => {
                                match self.generate_statement(
                                    func,
                                    module,
                                    statement.clone(),
                                    None,
                                    false,
                                ) {
                                    Some((_, value)) => func
                                        .borrow_mut()
                                        .add_instruction(Instruction::Literal(value)),
                                    _ => {}
                                }
                            }
                            _ => {}
                        },
                        _ => match self.generate_statement(
                            func,
                            module,
                            statement.clone(),
                            None,
                            false,
                        ) {
                            _ => {}
                        },
                    }
                }

                if !func.borrow_mut().blocks.last().map_or(false, |b| b.jumps()) {
                    func.borrow_mut()
                        .add_instruction(Instruction::Jump(cond_label));
                }

                func.borrow_mut().add_block(end_label);
                self.loop_labels.pop();

                None
            }
            AstNode::VariadicStatement { name, size } => {
                let compiled_size = self
                    .generate_statement(func, module, *size.clone(), ty, false)
                    .unwrap();

                let var = self.new_var(&Type::Long, &name, Some(func), false).unwrap();

                func.borrow_mut().assign_instruction(
                    var.clone(),
                    Type::Long,
                    Instruction::Call(Value::Global("malloc".to_string()), vec![compiled_size]),
                );

                func.borrow_mut().add_instruction(Instruction::VAStart(var));
                None
            }
            AstNode::NextStatement { name, r#type } => {
                let ptr = self.get_variable(&name, Some(func)).unwrap().1.clone();
                let ty = r#type.unwrap_or(Type::Long);
                let tmp = self.new_temporary(None);

                func.borrow_mut().assign_instruction(
                    tmp.clone(),
                    ty.clone(),
                    Instruction::VAArg(ptr),
                );

                Some((ty, tmp))
            }
            AstNode::BlockStatement { body } => {
                self.tmp_counter += 1;
                let body_label = format!("block.start.{}", self.tmp_counter);
                let end_label = format!("block.end.{}", self.tmp_counter);
                func.borrow_mut().add_block(body_label.clone());

                for statement in body.clone() {
                    match statement.clone() {
                        AstNode::LiteralStatement { kind, value: _ } => match kind.clone() {
                            TokenKind::ExactLiteral => {
                                match self.generate_statement(
                                    func,
                                    module,
                                    statement.clone(),
                                    None,
                                    false,
                                ) {
                                    Some((_, value)) => func
                                        .borrow_mut()
                                        .add_instruction(Instruction::Literal(value)),
                                    _ => {}
                                }
                            }
                            _ => {}
                        },
                        _ => match self.generate_statement(
                            func,
                            module,
                            statement.clone(),
                            None,
                            false,
                        ) {
                            _ => {}
                        },
                    }
                }

                func.borrow_mut().add_block(end_label);
                None
            }
            AstNode::ConversionStatement {
                r#type: second,
                value,
            } => {
                let (first, val) = self
                    .generate_statement(func, module, *value.clone(), ty, false)
                    .unwrap();

                Some(self.convert_to_type(func, first, second.unwrap(), val))
            }
            AstNode::NotStatement { value } => {
                let (ty, val) = self
                    .generate_statement(func, module, *value.clone(), ty, false)
                    .unwrap();

                let temp = self
                    .new_var(&ty.clone(), &format!("tmp"), Some(func), true)
                    .unwrap();

                func.borrow_mut().assign_instruction(
                    temp.clone(),
                    ty.clone(),
                    Instruction::Compare(
                        ty.clone(),
                        Comparison::Equal,
                        val,
                        Value::Const(ty.clone(), 0),
                    ),
                );

                Some((ty, temp))
            }
            AstNode::ArrayStatement {
                name,
                r#type,
                values,
            } => {
                let mut first_type: Option<Type> = None;
                let mut results: Vec<Value> = Vec::new();

                for value in values.iter() {
                    let (ty, val) = self
                        .generate_statement(func, module, value.clone(), r#type.clone(), false)
                        .unwrap();

                    results.push(val);

                    if let Some(first_type) = first_type.clone() {
                        if ty != first_type {
                            panic!(
                                "Inconsistent array types {:?} and {:?} (possibly more)",
                                first_type, ty
                            );
                        }
                    } else {
                        first_type = Some(ty);
                    }
                }

                let buf_ty = Type::Pointer(Box::new(first_type.clone().unwrap()));
                let tmp = self
                    .new_var(&buf_ty, &name.clone(), Some(func), true)
                    .unwrap();

                func.borrow_mut().assign_instruction(
                    tmp.clone(),
                    buf_ty.clone(),
                    Instruction::Alloc8(
                        Type::Long.size()
                            + if let Some(ty) = first_type.clone() {
                                ty.size() * (values.len() as u64)
                            } else {
                                0
                            },
                    ),
                );

                func.borrow_mut().add_instruction(Instruction::Store(
                    Type::Long,
                    tmp.clone(),
                    Value::Const(Type::Word, values.len() as i64),
                ));

                for (i, value) in results.iter().enumerate() {
                    let value_ptr = self.new_var(&Type::Long, "tmp", Some(func), true).unwrap();

                    func.borrow_mut().assign_instruction(
                        value_ptr.clone(),
                        Type::Long,
                        Instruction::Add(
                            tmp.clone(),
                            Value::Const(
                                Type::Word,
                                Type::Long.size() as i64
                                    + i as i64 * first_type.as_ref().unwrap().size() as i64,
                            ),
                        ),
                    );

                    func.borrow_mut().add_instruction(Instruction::Store(
                        first_type.as_ref().unwrap().clone(),
                        value_ptr,
                        value.to_owned(),
                    ));
                }

                self.tmp_counter += 1;
                let name = format!("array.{}.{}", name, self.tmp_counter);
                let typedef = TypeDef {
                    name: name.clone(),
                    align: None,
                    items: if let Some(ty) = first_type {
                        vec![(Type::Long, 1), (ty, values.len())]
                    } else {
                        vec![(Type::Long, 1)]
                    },
                };

                self.type_sections.push(typedef.clone());
                Some((Type::Aggregate(typedef.clone()), tmp))
            }
            AstNode::SizeStatement { value, standalone } => match value {
                Ok(ty) => {
                    let tmp_ty = Type::Word;
                    let temp = self.new_var(&tmp_ty, "tmp", Some(func), true).unwrap();

                    func.borrow_mut().assign_instruction(
                        temp.clone(),
                        tmp_ty.clone(),
                        Instruction::Copy(Value::Const(tmp_ty.clone(), ty.size() as i64)),
                    );

                    Some((tmp_ty, temp))
                }

                Err(value) => {
                    let (ty, val) = self
                        .generate_statement(func, module, *value.clone(), ty.clone(), false)
                        .unwrap();

                    match ty.clone() {
                        Type::Pointer(inner) => {
                            let ty = Type::Long;
                            let len = self.new_var(&ty, "tmp", Some(func), true).unwrap();

                            func.borrow_mut().assign_instruction(
                                len.clone(),
                                ty.clone(),
                                Instruction::Load(ty.clone(), val),
                            );

                            let size = self.new_var(&ty, "tmp", Some(func), true).unwrap();

                            func.borrow_mut().assign_instruction(
                                size.clone(),
                                ty.clone(),
                                Instruction::Multiply(
                                    len.clone(),
                                    Value::Const(Type::Word, (*inner).size() as i64),
                                ),
                            );

                            Some((ty, if standalone { size } else { len }))
                        }
                        other => {
                            let temp = self.new_var(&other, "tmp", Some(func), true).unwrap();

                            func.borrow_mut().assign_instruction(
                                temp.clone(),
                                other.clone(),
                                Instruction::Copy(Value::Const(
                                    other.clone(),
                                    ty.clone().size() as i64,
                                )),
                            );

                            Some((other, temp))
                        }
                    }
                }
                _ => panic!("Invalid value to calculate size for"),
            },
            _ => todo!("statement: {:?}", stmt),
        };

        res
    }

    fn convert_to_type(
        &mut self,
        func: &RefCell<Function>,
        first: Type,
        second: Type,
        val: Value,
    ) -> (Type, Value) {
        if first.weight() == second.weight() {
            return (first, val);
        } else if first.is_int() && second.is_int() {
            let conv = self
                .new_var(
                    &second,
                    &format!("tmp.{}", self.tmp_counter),
                    Some(func),
                    true,
                )
                .unwrap();
            self.tmp_counter += 1;

            let is_first_higher = first.weight() > second.weight();

            func.borrow_mut().assign_instruction(
                conv.clone(),
                if is_first_higher {
                    first.clone()
                } else {
                    second.clone()
                },
                Instruction::Extension(first, val),
            );

            return (second, conv);
        } else if first.is_float() && second.is_float() {
            let conv = self
                .new_var(
                    &second,
                    &format!("tmp.{}", self.tmp_counter),
                    Some(func),
                    true,
                )
                .unwrap();
            self.tmp_counter += 1;

            let is_first_higher = first.weight() > second.weight();

            func.borrow_mut().assign_instruction(
                conv.clone(),
                if is_first_higher {
                    first.clone()
                } else {
                    second.clone()
                },
                Instruction::Extension(first, val),
            );

            return (second, conv);
        } else {
            let conv = self
                .new_var(
                    &second,
                    &format!("tmp.{}", self.tmp_counter),
                    Some(func),
                    true,
                )
                .unwrap();
            self.tmp_counter += 1;

            func.borrow_mut().assign_instruction(
                conv.clone(),
                second.clone(),
                Instruction::Conversion(first, second.clone(), val),
            );

            return (second, conv);
        }
    }

    pub fn compile(tree: Vec<Primitive>, output_path: String) {
        let mut generator = Compiler {
            tmp_counter: 0,
            scopes: Vec::new(),
            data_sections: Vec::new(),
            type_sections: Vec::new(),
            loop_labels: Vec::new(),
            ret_types: HashMap::new(),
            tree,
        };

        let module = Module::new();

        // We need internal mutability here
        // Each string data section needs to be added to the module
        let module_ref = RefCell::new(module);

        for primitive in generator.tree.clone() {
            match primitive {
                Primitive::Constant {
                    name,
                    public,
                    r#type: ty,
                    value,
                } => match generator.generate_function(
                    name.clone(),
                    public,
                    false,
                    false,
                    false,
                    &vec![],
                    ty,
                    vec![AstNode::ReturnStatement { value }],
                    &module_ref,
                ) {
                    Ok(function) => {
                        module_ref.borrow_mut().add_function(function);
                    }
                    Err(msg) => eprintln!("{}", msg),
                },
                Primitive::Function {
                    name,
                    public,
                    variadic,
                    manual,
                    external,
                    arguments,
                    r#return,
                    body,
                } => match generator.generate_function(
                    name,
                    public,
                    variadic,
                    manual,
                    external,
                    &arguments,
                    r#return,
                    body,
                    &module_ref,
                ) {
                    Ok(function) => {
                        module_ref.borrow_mut().add_function(function);
                    }
                    Err(msg) => eprintln!("{}", msg),
                },
                _ => {}
            }
        }

        for data in generator.data_sections {
            module_ref.borrow_mut().add_data(data);
        }

        for def in generator.type_sections {
            module_ref.borrow_mut().add_type(def);
        }

        let mut file = File::create(output_path).expect("Failed to create the file.");
        file.write_all(module_ref.borrow().to_string().as_bytes())
            .expect("Failed to write to file.");

        file.flush().expect("Failed to flush file");
    }
}
