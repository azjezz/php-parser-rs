#[macro_export]
macro_rules! expect_token {
    ([ $($expected:pat => $out:expr),+ $(,)? ], $parser:expr, [ $($message:literal),+ $(,)? ]) => {{
        $parser.skip_comments();
        match $parser.current.kind.clone() {
            $(
                $expected => {
                    $parser.next();
                    $out
                }
            )+
            _ => {
                return $crate::expected_token_err!([ $($message,)+ ], $parser);
            }
        }
    }};
    ([ $($expected:pat),+ $(,)? ], $parser:expr, [ $($message:literal),+ $(,)? ]) => {{
        $parser.skip_comments();
        match $parser.current.kind.clone() {
            $(
                $expected => {
                    $parser.next();
                }
            )+
            _ => {
                return $crate::expected_token_err!([ $($message,)+ ], $parser);
            }
        }
    }};
    ([ $($expected:pat => $out:expr),+ $(,)? ], $parser:expr, $message:literal) => {
        $crate::expect_token!([ $($expected => $out,)+ ], $parser, [$message])
    };
    ([ $($expected:pat),+ $(,)? ], $parser:expr, $message:literal) => {
        $crate::expect_token!([ $($expected,)+ ], $parser, [$message])
    };
}

#[macro_export]
macro_rules! expect_literal {
    ($parser:expr) => {{
        $parser.skip_comments();
        match $parser.current.kind.clone() {
            TokenKind::LiteralInteger(i) => {
                let e = Expression::Int { i };
                $parser.next();
                e
            }
            TokenKind::ConstantFloat(f) => {
                let e = Expression::Float { f };
                $parser.next();
                e
            }
            TokenKind::ConstantString(s) => {
                let e = Expression::ConstantString { value: s.clone() };
                $parser.next();
                e
            }
            _ => {
                return $crate::expected_token_err!(["a literal"], $parser);
            }
        }
    }};
}

#[macro_export]
macro_rules! expected_token_err {
    ([ $($expected:literal),+ $(,)? ], $parser:expr $(,)?) => {{
        match &$parser.current.kind {
            TokenKind::Eof => {
                Err($crate::parser::error::ParseError::ExpectedToken(
                    vec![$($expected.into()),+],
                    None,
                    $parser.current.span,
                ))
            },
            _ => {
                Err($crate::parser::error::ParseError::ExpectedToken(
                    vec![$($expected.into()),+],
                    Some($parser.current.kind.to_string()),
                    $parser.current.span,
                ))
            }
        }
    }};

    ($expected:literal, $parser:expr $(,)?) => {
        $crate::expected_token_err!([$expected], $parser)
    };
}
