use super::*;

impl<'a, 'b> Transform<'a, 'b> for ast::TypeCallback {
    type Output = Type;

    fn transform(&'b self, tfmr: &mut Transformer<'a, 'b>) -> Self::Output {
        let parameters = self
            .parameters()
            .iter()
            .map(|param| match param.name().ty() {
                ast::TokenType::Identifier(str) => {
                    let typ = param.type_info().transform(tfmr);
                    variants::FunctionParameter {
                        optional: *param.optional(),
                        span: param.span(),
                        name: str.to_string(),
                        typ,
                    }
                }
                _ => unreachable!(),
            })
            .collect();

        Type::Function(variants::Function {
            span: self.span(),
            parameters,
            return_type: Box::new(self.return_type().transform(tfmr)),

            // TODO(memothelemo): Add varidiac type parameter?
            varidiac_param: None,
        })
    }
}
