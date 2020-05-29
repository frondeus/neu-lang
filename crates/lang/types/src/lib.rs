use std::collections::BTreeMap;

pub enum Type {
    Number,
    Boolean, // Replace with ADT
    String,
    Array(Box<Type>),
    Struct(BTreeMap<String, Type>),
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
