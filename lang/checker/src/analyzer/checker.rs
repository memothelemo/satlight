use super::*;
use crate::{
    types::{
        variants::{self, LiteralType},
        Type,
    },
    utils,
};
use salite_ast::Span;

impl<'a, 'b> Analyzer<'a, 'b> {
    pub fn check_table(
        &mut self,
        left: &variants::Table,
        right: &variants::Table,
        span: Span,
    ) -> AnalyzeResult {
        let mut counted_indexes = Vec::new();

        // check for type similarites?
        for (key, value) in right.entries.iter() {
            let intrisnic_type = if let variants::TableFieldKey::Computed(ty, ..) = key {
                match &ty {
                    Type::Literal(..) => Some(ty),
                    _ => None,
                }
            } else {
                None
            };
            match left.entries.get_retrieve_id(key) {
                Some((id, other_value)) => {
                    counted_indexes.push(id);
                    self.compare_types(other_value, value, span)
                        .map_err(|err| AnalyzeError::InvalidField {
                            span,
                            key: utils::table_key_description(&self.ctx, key),
                            reason: Box::new(err),
                        })?;
                }
                None if intrisnic_type.is_some() => {
                    let mut batched_values = Vec::new();
                    let intrisnic_type = intrisnic_type.as_ref().unwrap();
                    for (id, (ak, av)) in left.entries.iter().enumerate() {
                        if let variants::TableFieldKey::Computed(a, ..) = ak {
                            if self.compare_types(a, intrisnic_type, span).is_ok() {
                                batched_values.push((id, (ak, av)));
                            }
                        } else if let variants::TableFieldKey::Name(..) = ak {
                            // make sure the intrisnic type must be string >:(
                            if matches!(
                                intrisnic_type,
                                Type::Literal(variants::Literal {
                                    typ: variants::LiteralType::String,
                                    ..
                                })
                            ) {
                                batched_values.push((id, (ak, av)));
                            }
                        }
                    }
                    for (id, (ak, av)) in batched_values.iter() {
                        counted_indexes.push(*id);
                        self.compare_types(av, value, span).map_err(|err| {
                            AnalyzeError::InvalidField {
                                span,
                                key: utils::table_key_description(&self.ctx, ak),
                                reason: Box::new(err),
                            }
                        })?;
                    }
                }
                None => {
                    return Err(AnalyzeError::MissingField {
                        span,
                        key: utils::table_key_description(&self.ctx, key),
                        expected: utils::type_description(&self.ctx, value),
                    })
                }
            }
        }

        // table leftovers?
        for (id, (key, ..)) in left.entries.iter().enumerate() {
            if counted_indexes.contains(&id) {
                continue;
            }
            return Err(AnalyzeError::ExcessiveField {
                span,
                key: utils::table_key_description(&self.ctx, key),
            });
        }

        Ok(())
    }

    pub fn compare_types(&mut self, value: &Type, assertion: &Type, span: Span) -> AnalyzeResult {
        let leftd = utils::type_description(&self.ctx, value);
        let rightd = utils::type_description(&self.ctx, assertion);
        // let left = self.skip_downwards(value.clone());
        // let right = self.skip_downwards(assertion.clone());
        let left = value;
        let right = assertion;
        match (left, right) {
            (value, Type::Intersection(..))
                if {
                    // we need to solve the intersection, maybe there's table
                    // merge or something like that?
                    let inter = match &right {
                        Type::Intersection(inter) => inter,
                        result => return self.compare_types(value, result, span),
                    };

                    let mut did_match = true;
                    for member in inter.members.iter() {
                        if self.compare_types(value, member, span).is_err() {
                            did_match = false;
                            break;
                        }
                    }
                    did_match
                } =>
            {
                Ok(())
            }

            (value, Type::Union(union))
                if {
                    let mut did_match = false;
                    for member in union.members.iter() {
                        if self.compare_types(value, member, span).is_ok() {
                            did_match = true;
                            break;
                        }
                    }
                    did_match
                } =>
            {
                Ok(())
            }

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
                    self.compare_types(&param.typ, expected, span)?;
                }
                match (&a.varidiac_param, &b.varidiac_param) {
                    (None, None | Some(_)) => {}
                    (Some(_), None) => return Err(AnalyzeError::ExcessiveVarargParam { span }),
                    (Some(a), Some(b)) => self.compare_types(&a.typ, &b.typ, span)?,
                };
                self.compare_types(&a.return_type, &b.return_type, span)
            }

            (Type::Table(left), Type::Table(right)) => self.check_table(left, right, span),

            (_, Type::Any(..) | Type::Unknown(..)) => Ok(()),
            (Type::Any(..) | Type::Unknown(..), _) => Ok(()),

            (Type::Literal(a), Type::Literal(b))
                if matches!(
                    (&a.typ, &b.typ),
                    (
                        LiteralType::Nil | LiteralType::Void,
                        LiteralType::Void | LiteralType::Nil
                    )
                ) =>
            {
                Ok(())
            }

            // (_, Type::Ref(_)) => {
            //     let real_type = self.solve_type_ref(&right)?;
            //     self.compare_types(&left, &real_type, span)
            // }
            // (Type::Ref(_), _) => {
            //     let real_type = self.solve_type_ref(&left)?;
            //     self.compare_types(&real_type, &right, span)
            // }
            (_, Type::Tuple(tupl)) if tupl.members.len() == 1 => {
                let member_type = tupl.members.get(0).unwrap();
                self.compare_types(left, member_type, span)
            }
            (Type::Tuple(tupl), _) if tupl.members.len() == 1 => {
                let member_type = tupl.members.get(0).unwrap();
                self.compare_types(member_type, right, span)
            }
            (Type::Tuple(a), Type::Tuple(b)) if a.members.len() <= b.members.len() => {
                for (idx, member) in b.members.iter().enumerate() {
                    let value_member = a.members.get(idx);
                    match value_member {
                        Some(val) => {
                            self.compare_types(val, member, span)?;
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
