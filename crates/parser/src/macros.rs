#[macro_export]
macro_rules! no_match {
    ($state:expr) => {{
        let span = $state.current().unwrap().span();
        return Err(ParseError {
            span,
            ty: ParseErrorType::Internal(InternalParseError::NoMatch),
        });
    }};
}

#[macro_export]
macro_rules! parser_struct {
    ($name:tt, $ret:ty, $body:expr) => {
        impl<'a> Parser<'a> for $name {
            type Output = $ret;

            fn parse(&self, state: &ParseState<'a>) -> ParseResult<'a, Self::Output> {
                $body(self, state)
            }
        }
    };
}

#[macro_export]
macro_rules! parse_either {
    (
        $state:expr,
        { $( $parser:expr => $ret:expr, )* }
    ) => {
        {
            $(
                #[allow(unreachable_patterns)]
                match $parser.parse($state) {
                    Ok(res) => return Ok((res.0, $ret(res.1))),
                    Err(ParseError {
                        ty: ParseErrorType::Internal(InternalParseError::NoMatch),
                        ..
                    }) => {},
                    Err(err) => return Err(err),
                }
            )*
            no_match!($state);
        }
    };
}

#[macro_export]
macro_rules! optional {
    ($state:expr, $parser:expr) => {
        match $parser.parse($state) {
            Ok((s, res)) => (s, Some(res)),
            Err(ParseError {
                ty: ParseErrorType::Internal(InternalParseError::NoMatch),
                ..
            }) => ($state.next(0), None),
            Err(err) => return Err(err),
        }
    };
}

#[macro_export]
macro_rules! expect {
    ($state:expr, $parser:expr, $expected:expr) => {
        match $parser.parse($state) {
            Ok(res) => res,
            Err(ParseError {
                ty: ParseErrorType::Internal(InternalParseError::NoMatch),
                ..
            }) => {
                let token = $state.current().unwrap();
                let span = token.span();
                return Err(ParseError {
                    span,
                    ty: ParseErrorType::Expected {
                        expected: $expected.into(),
                        token: token.clone(),
                    },
                });
            }
            Err(err) => return Err(err),
        }
    };
}

#[macro_export]
macro_rules! no_match_ignore {
    ($state:expr, $parser:expr) => {
        match $parser.parse($state) {
            Ok(res) => Ok(res),
            err @ Err(ParseError {
                ty: ParseErrorType::Internal(InternalParseError::NoMatch),
                ..
            }) => Err(err),
            Err(err) => return Err(err),
        }
    };
}
