use super::*;
use types::LiteralKind;

impl<'a> Analyzer<'a> {
    fn check_table(
        &mut self,
        left: &types::Table,
        right: &types::Table,
        span: Span,
    ) -> Result<(), AnalyzeError> {
        let mut counted_indexes = Vec::new();

        // check for type similarites?
        for (key, value) in right.entries.iter() {
            let intrisnic_type = if let types::TableFieldKey::Computed(ty) = key {
                let ty = self.skip_downwards(ty.clone());
                match &ty {
                    types::Type::Literal(..) => Some(ty),
                    _ => None,
                }
            } else {
                None
            };
            match left.entries.get_retrieve_id(key) {
                Some((id, other_value)) => {
                    counted_indexes.push(id);
                    self.check_lr_types(value, other_value, span)
                        .map_err(|err| AnalyzeError::InvalidField {
                            span,
                            key: self.table_key_description(key),
                            reason: Box::new(err),
                        })?;
                }
                None if intrisnic_type.is_some() => {
                    let mut batched_values = Vec::new();
                    let intrisnic_type = intrisnic_type.as_ref().unwrap();
                    for (id, (ak, av)) in left.entries.iter().enumerate() {
                        if let types::TableFieldKey::Computed(a) = ak {
                            if self.check_lr_types(a, intrisnic_type, span).is_ok() {
                                batched_values.push((id, (ak, av)));
                            }
                        } else if let types::TableFieldKey::Name(..) = ak {
                            // make sure the intrisnic type must be string >:(
                            if matches!(
                                intrisnic_type,
                                Type::Literal(types::LiteralType {
                                    kind: types::LiteralKind::String,
                                    ..
                                })
                            ) {
                                batched_values.push((id, (ak, av)));
                            }
                        }
                    }
                    for (id, (ak, av)) in batched_values.iter() {
                        counted_indexes.push(*id);
                        self.check_lr_types(value, av, span).map_err(|err| {
                            AnalyzeError::InvalidField {
                                span,
                                key: self.table_key_description(ak),
                                reason: Box::new(err),
                            }
                        })?;
                    }
                }
                None => {
                    return Err(AnalyzeError::MissingField {
                        span,
                        key: self.table_key_description(key),
                        expected: self.type_description(value),
                    })
                }
            }
        }

        // table leftovers?
        for (id, (key, value)) in left.entries.iter().enumerate() {
            if counted_indexes.contains(&id) {
                continue;
            }
            return Err(AnalyzeError::ExcessiveField {
                span,
                key: self.table_key_description(key),
            });
        }

        Ok(())
    }

    pub fn check_lr_types(
        &mut self,
        left: &Type,
        right: &Type,
        span: Span,
    ) -> Result<(), AnalyzeError> {
        let leftd = self.type_description(left);
        let rightd = self.type_description(right);
        let left = self.skip_downwards(left.clone());
        let right = self.skip_downwards(right.clone());
        match (&left, &right) {
            (Type::Function(a), Type::Function(b)) => {
                for (idx, param) in a.parameters.iter().enumerate() {
                    let expected = match b.parameters.get(idx) {
                        Some(ty) => &ty.typ,
                        None => {
                            return Err(AnalyzeError::ExcessiveParameter {
                                span: param.span,
                                key: idx + 1,
                            })
                        }
                    };
                    self.check_lr_types(&param.typ, expected, span)?;
                }
                match (&a.varidiac_param, &b.varidiac_param) {
                    (None, None | Some(_)) => {}
                    (Some(_), None) => return Err(AnalyzeError::ExcessiveVarargParam { span }),
                    (Some(a), Some(b)) => self.check_lr_types(&a.typ, &b.typ, span)?,
                };
                self.check_lr_types(&a.return_type, &b.return_type, span)
            }

            (Type::Table(left), Type::Table(right)) => self.check_table(left, right, span),

            (_, Type::Literal(l)) if matches!(l.kind, LiteralKind::Any | LiteralKind::Unknown) => {
                Ok(())
            }
            (Type::Literal(l), _) if matches!(l.kind, LiteralKind::Any) => Ok(()),
            (Type::Literal(a), Type::Literal(b))
                if matches!(
                    (&a.kind, &b.kind),
                    (
                        LiteralKind::Nil | LiteralKind::Void,
                        LiteralKind::Void | LiteralKind::Nil
                    )
                ) =>
            {
                Ok(())
            }

            (_, Type::Ref(_)) => {
                let real_type = self.solve_type_ref(&right)?;
                self.check_lr_types(&left, &real_type, span)
            }
            (Type::Ref(_), _) => {
                let real_type = self.solve_type_ref(&left)?;
                self.check_lr_types(&real_type, &right, span)
            }

            (_, Type::Tuple(tupl)) if tupl.members.len() == 1 => {
                let member_type = tupl.members.get(0).unwrap();
                self.check_lr_types(&left, member_type, span)
            }
            (Type::Tuple(tupl), _) if tupl.members.len() == 1 => {
                let member_type = tupl.members.get(0).unwrap();
                self.check_lr_types(member_type, &right, span)
            }
            (Type::Tuple(a), Type::Tuple(b)) if a.members.len() <= b.members.len() => {
                for (idx, member) in b.members.iter().enumerate() {
                    let value_member = a.members.get(idx);
                    match value_member {
                        Some(val) => {
                            self.check_lr_types(val, member, span)?;
                        }
                        None => {
                            return Err(AnalyzeError::NotExtendable {
                                value: leftd,
                                assertion: rightd,
                                span,
                            })
                        }
                    }
                }
                Ok(())
            }

            _ if left == right => Ok(()),
            _ => Err(AnalyzeError::NotExtendable {
                value: leftd,
                assertion: rightd,
                span,
            }),
        }
    }
}
