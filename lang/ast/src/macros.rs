#[macro_export]
macro_rules! operator {
	{
		$(
            $( #[$meta:meta] )*
            pub enum $name:ident {
				fn is_right_associate(&self) = $associative_body:expr,
                $( $member:ident => $precedence:expr, )*
            }
        )*
	} => {
		$(
            $( #[$meta] )*
            pub enum $name {
                $( $member, )*
            }

            impl $name {
                /// Gets the operator precedence
                pub fn precedence(&self) -> usize {
                    match self {
                        $(
                            $name::$member => $precedence,
                        )*
                    }
                }

                /// A helper method checks if the operator is right associative.
                pub fn is_right_associative(&self) -> bool {
                    $associative_body(self)
                }
            }
        )*
	};
}
