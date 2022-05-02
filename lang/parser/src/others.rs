use super::*;

pub struct ZeroOrMore<T>(pub T);
impl<'a, T> Parser<'a> for ZeroOrMore<T>
where
    T: Parser<'a>,
{
    type Output = Vec<T::Output>;

    fn parse(&self, state: &ParseState<'a>) -> ParseResult<'a, Self::Output> {
        let mut collection = Vec::new();
        let mut state = state.next(0);
        loop {
            match self.0.parse(&state) {
                Ok((ns, member)) => {
                    state = ns;
                    collection.push(member);
                }
                Err(ParseError {
                    ty: ParseErrorType::Internal(InternalParseError::NoMatch),
                    ..
                }) => break,
                Err(err) => return Err(err),
            }
        }
        Ok((state, collection))
    }
}

pub struct ZeroOrMorePunctuated<T, P>(pub T, pub P);
impl<'a, T, P> Parser<'a> for ZeroOrMorePunctuated<T, P>
where
    T: Parser<'a>,
    P: Parser<'a>,
{
    type Output = Vec<T::Output>;

    fn parse(&self, state: &ParseState<'a>) -> ParseResult<'a, Self::Output> {
        let mut collection = Vec::new();
        let mut state = state.next(0);
        loop {
            match self.0.parse(&state) {
                Ok((ns, member)) => {
                    let (ns, punct) = optional!(&ns, self.1);
                    let has_punct = punct.is_some();
                    state = ns;
                    collection.push(member);
                    if !has_punct {
                        break;
                    }
                }
                Err(ParseError {
                    ty: ParseErrorType::Internal(InternalParseError::NoMatch),
                    ..
                }) => break,
                Err(err) => return Err(err),
            }
        }
        Ok((state, collection))
    }
}

pub struct OneOrMorePunctuated<T, P>(pub T, pub P, pub String);
impl<'a, T, P> Parser<'a> for OneOrMorePunctuated<T, P>
where
    T: Parser<'a>,
    P: Parser<'a>,
{
    type Output = Vec<T::Output>;

    fn parse(&self, state: &ParseState<'a>) -> ParseResult<'a, Self::Output> {
        let mut collection = Vec::new();
        let state = state.next(0);

        // expect something the first one there
        let (mut state, member_0) = expect!(&state, self.0, self.2.to_string());
        collection.push(member_0);

        if let Ok((new_state, _)) = self.1.parse(&state) {
            state = new_state;
            loop {
                match self.0.parse(&state) {
                    Ok((ns, member)) => {
                        let (ns, punct) = optional!(&ns, self.1);
                        let has_punct = punct.is_some();
                        state = ns;
                        collection.push(member);
                        if !has_punct {
                            break;
                        }
                    }
                    Err(ParseError {
                        ty: ParseErrorType::Internal(InternalParseError::NoMatch),
                        ..
                    }) => break,
                    Err(err) => return Err(err),
                }
            }
        }

        Ok((state, collection))
    }
}

pub struct ZeroOrMorePunctuatedTrailed<T, P>(pub T, pub P);
impl<'a, T, P> Parser<'a> for ZeroOrMorePunctuatedTrailed<T, P>
where
    T: Parser<'a>,
    P: Parser<'a>,
{
    type Output = Vec<T::Output>;

    fn parse(&self, state: &ParseState<'a>) -> ParseResult<'a, Self::Output> {
        let mut collection = Vec::new();
        let mut state = state.next(0);
        loop {
            match self.0.parse(&state) {
                Ok((ns, member)) => {
                    let (ns, _) = optional!(&ns, self.1);
                    state = ns;
                    collection.push(member);
                }
                Err(ParseError {
                    ty: ParseErrorType::Internal(InternalParseError::NoMatch),
                    ..
                }) => break,
                Err(err) => return Err(err),
            }
        }
        Ok((state, collection))
    }
}
