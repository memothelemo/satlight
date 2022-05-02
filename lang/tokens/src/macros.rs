#[macro_export]
macro_rules! symbols {
	{
		$(
			$( #[$meta:meta] )*
			pub enum $enum_name:ident {
				$( $name:ident => $text:expr, )*
			}
		)*
	} => {$(
		$( #[$meta] )*
		pub enum $enum_name {
			$( $name, )*
		}

		#[allow(dead_code)]
		impl $enum_name {
			pub fn parse(str: &'_ str) -> Option<Self> {
				match str {
					$(
						_ if str == $text => Some($enum_name::$name),
					)*
					_ => None,
				}
			}

			pub fn to_str(&self) -> String {
				match self {
					$(
						Self::$name => $text.into(),
					)*
				}
			}
		}

		impl std::fmt::Debug for $enum_name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				match self {
					$(
						Self::$name => $text,
					)*
				}
				.fmt(f)
			}
		}

		impl std::fmt::Display for $enum_name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				std::fmt::Debug::fmt(&match self {
					$(
						Self::$name => $text,
					)*
				}, f)
			}
		}
	)*};
}
