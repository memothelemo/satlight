use super::Type;

pub fn destruct_tuples(base_typ: Type, vector: &mut Vec<Type>) {
    match base_typ {
        Type::Tuple(tuple) => {
            for typ in tuple.members {
                vector.push(typ);
            }
        }
        _ => vector.push(base_typ),
    }
}
