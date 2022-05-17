use crate::{
    types::{variants, Type},
    ModuleContext,
};

pub fn table_key_description<'a, 'b>(
    ctx: &ModuleContext<'a, 'b>,
    key: &variants::TableFieldKey,
) -> String {
    match key {
        variants::TableFieldKey::Name(str, ..) => str.to_string(),
        variants::TableFieldKey::Computed(typ, ..) => type_description(ctx, typ),
        variants::TableFieldKey::None(id, ..) => format!("[array {}]", id),
    }
}

pub fn table_description<'a, 'b>(ctx: &ModuleContext<'a, 'b>, tbl: &variants::Table) -> String {
    // that's very long, but the maximum of table entries is about 5?
    let mut entry_result = Vec::new();
    let limited_entries = tbl.entries.pick_limit(5);

    if limited_entries.is_empty() {
        return String::from("{{}}");
    }

    for (key, value) in limited_entries {
        entry_result.push(format!(
            "{}{}",
            {
                let result = table_key_description(ctx, key);
                if result.is_empty() {
                    String::new()
                } else {
                    format!("{}: ", result)
                }
            },
            type_description(ctx, value)
        ));
    }

    if let Some(ref meta) = tbl.metatable {
        entry_result.push(format!("LUA_METATABLE = {}", table_description(ctx, meta)));
    }

    if tbl.entries.len() > 5 {
        entry_result.push("..".to_string());
    }

    format!("{{ {} }}", entry_result.join(", "))
}

pub fn type_description<'a, 'b>(ctx: &ModuleContext<'a, 'b>, typ: &Type) -> String {
    macro_rules! member_description {
        ($members:expr, $prefix:expr) => {
            $members
                .iter()
                .map(|v| type_description(ctx, v))
                .collect::<Vec<String>>()
                .join(&$prefix.to_string())
        };
    }
    match typ {
        Type::Reference(info) => info.name.to_string(),
        Type::Tuple(info) => {
            let mut result = Vec::new();
            for typ in info.members.iter() {
                result.push(type_description(ctx, typ));
            }
            format!("({})", result.join(","))
        }
        Type::Literal(info) => match info.typ {
            variants::LiteralType::Bool => "bool",
            variants::LiteralType::Number => "number",
            variants::LiteralType::Nil => "nil",
            variants::LiteralType::String => "string",
            variants::LiteralType::Void => "void",
        }
        .to_string(),
        Type::Table(tbl) => table_description(ctx, tbl),
        Type::Function(info) => {
            let mut params = Vec::new();
            for param in info.parameters.iter() {
                let name = format!("{}: ", param.name.clone());
                let typ = type_description(ctx, &param.typ);
                params.push(format!("{}{}", name, typ));
            }
            format!(
                "({}) -> {}",
                params.join(","),
                type_description(ctx, &info.return_type)
            )
        }
        Type::Unresolved(info) => panic!("Unresolved type: {:#?}", info),
        Type::Intersection(node) => member_description!(node.members, " & "),
        Type::Union(node) => member_description!(node.members, " | "),
        Type::Any(..) => "any".to_string(),
        Type::Recursive(info) => {
            let sym = ctx.symbols.get(info.symbol).unwrap();
            match &sym.kind {
                crate::SymbolKind::BlockVariable(var) => var.name.to_string(),
                crate::SymbolKind::FunctionParameter(name, ..) => name.to_string(),
                crate::SymbolKind::TypeParameter(name, ..) => name.to_string(),
                crate::SymbolKind::TypeAlias(info) => info.name.to_string(),
                crate::SymbolKind::UnknownVariable => "!UNKNOWN_VAR!".to_string(),
                crate::SymbolKind::Value(_) => "value".to_string(),
            }
        }
        Type::Unknown(..) => "unknown".to_string(),
    }
}
