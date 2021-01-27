#[macro_export]
macro_rules! nodes {
    (
        $nodes: ident,
        $($group: ident {
            $($node: ident),*
        }),*
    ) => {
        #[allow(non_upper_case_globals)]
        #[allow(dead_code)]
        impl $nodes {
            $( $( pub const $node: Name = Name::new(stringify!($node)); )* )*
        }
    }
}
