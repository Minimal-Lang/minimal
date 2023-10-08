#[macro_export]
macro_rules! assert_token {
    ($chars:expr, $token:expr, lexeme=$lexeme:expr, span=$span:expr, kind=whitespace) => {
        assert_eq!(
            $token,
            ::minimal_compiler::tokenizer::token::Token {
                span: $span,
                value: ::minimal_compiler::tokenizer::token::TokenValue::Whitespace,
            }
        );
        assert_eq!(&Vec::from_iter($lexeme.chars()), &$chars[$span], "the lexemes don't match")
    };
    ($chars:expr, $token:expr, lexeme=$lexeme:expr, span=$span:expr, kind=comment, block=$block:expr, doc=$doc:expr, content=$content:expr) => {
        assert_eq!(
            $token,
            ::minimal_compiler::tokenizer::token::Token {
                span: $span,
                value: ::minimal_compiler::tokenizer::token::TokenValue::Comment(
                    ::minimal_compiler::tokenizer::token::comment::Comment {
                        block: $block,
                        doc: $doc,
                        content: &Vec::from_iter($content.chars())
                    }
                ),
            }
        );
        assert_eq!(&Vec::from_iter($lexeme.chars()), &$chars[$span], "the lexemes don't match")
    };
    ($chars:expr, $token:expr, lexeme=$lexeme:expr, span=$span:expr, kind=ident) => {
        assert_eq!(
            $token,
            ::minimal_compiler::tokenizer::token::Token {
                span: $span,
                value: ::minimal_compiler::tokenizer::token::TokenValue::Ident(
                    ::minimal_compiler::tokenizer::token::ident::Ident { value: $lexeme }
                ),
            }
        );
        assert_eq!(&Vec::from_iter($lexeme.chars()), &$chars[$span], "the lexemes don't match")
    };
    ($chars:expr, $token:expr, lexeme=$lexeme:expr, span=$span:expr, kind=number, base=10, int=$int:expr, fract=$fract:expr) => {
        assert_eq!(
            $token,
            ::minimal_compiler::tokenizer::token::Token {
                span: $span,
                value: ::minimal_compiler::tokenizer::token::TokenValue::Ident(
                    ::minimal_compiler::tokenizer::token::number::Number {
                        base: ::minimal_compiler::tokenizer::token::literal::Base::Decimal
                        int_part: $int,
                        fract_part: $fract
                    }
                ),
            }
        );
        assert_eq!(&Vec::from_iter($lexeme.chars()), &$chars[$span], "the lexemes don't match")
    };
    ($chars:expr, $token:expr, lexeme=$lexeme:expr, span=$span:expr, kind=number, base=2, int=$int:expr, fract=$fract:expr) => {
        assert_eq!(
            $token,
            ::minimal_compiler::tokenizer::token::Token {
                span: $span,
                value: ::minimal_compiler::tokenizer::token::TokenValue::Ident(
                    ::minimal_compiler::tokenizer::token::number::Number {
                        base: ::minimal_compiler::tokenizer::token::literal::Base::Binary
                        int_part: $int,
                        fract_part: $fract
                    }
                ),
            }
        );
        assert_eq!(&Vec::from_iter($lexeme.chars()), &$chars[$span], "the lexemes don't match")
    };
    ($chars:expr, $token:expr, lexeme=$lexeme:expr, span=$span:expr, kind=number, base=8, int=$int:expr, fract=$fract:expr) => {
        assert_eq!(
            $token,
            ::minimal_compiler::tokenizer::token::Token {
                span: $span,
                value: ::minimal_compiler::tokenizer::token::TokenValue::Ident(
                    ::minimal_compiler::tokenizer::token::number::Number {
                        base: ::minimal_compiler::tokenizer::token::literal::Base::Octal
                        int_part: $int,
                        fract_part: $fract
                    }
                ),
            }
        );
        assert_eq!(&Vec::from_iter($lexeme.chars()), &$chars[$span], "the lexemes don't match")
    };
    ($chars:expr, $token:expr, lexeme=$lexeme:expr, span=$span:expr, kind=number, base=16, int=$int:expr, fract=$fract:expr) => {
        assert_eq!(
            $token,
            ::minimal_compiler::tokenizer::token::Token {
                span: $span,
                value: ::minimal_compiler::tokenizer::token::TokenValue::Ident(
                    ::minimal_compiler::tokenizer::token::number::Number {
                        base: ::minimal_compiler::tokenizer::token::literal::Base::Hexadecimal
                        int_part: $int,
                        fract_part: $fract
                    }
                ),
            }
        );
        assert_eq!(&Vec::from_iter($lexeme.chars()), &$chars[$span], "the lexemes don't match")
    };
    ($chars:expr, $token:expr, lexeme=$lexeme:expr, span=$span:expr, kind=string, value=$value:expr) => {
        assert_eq!(
            $token,
            ::minimal_compiler::tokenizer::token::Token {
                span: $span,
                value: ::minimal_compiler::tokenizer::token::TokenValue::String(
                    ::minimal_compiler::tokenizer::token::literal::String {
                        value: ::std::string::String::from($value)
                    }
                ),
            }
        );
        assert_eq!(&Vec::from_iter($lexeme.chars()), &$chars[$span], "the lexemes don't match")
    };
    ($chars:expr, $token:expr, lexeme=$lexeme:expr, span=$span:expr, kind=char, value=$value:expr) => {
        assert_eq!(
            $token,
            ::minimal_compiler::tokenizer::token::Token {
                span: $span,
                value: ::minimal_compiler::tokenizer::token::TokenValue::Character(
                    ::minimal_compiler::tokenizer::token::literal::String {
                        value: ::std::string::String::from($value)
                    }
                ),
            }
        );
        assert_eq!(&Vec::from_iter($lexeme.chars()), &$chars[$span], "the lexemes don't match")
    };
    ($chars:expr, $token:expr, lexeme=$lexeme:expr, span=$span:expr, kind=delim, value=$value:ident) => {
        assert_eq!(
            $token,
            ::minimal_compiler::tokenizer::token::Token {
                span: $span,
                value: ::minimal_compiler::tokenizer::token::TokenValue::Delim(
                    ::minimal_compiler::tokenizer::token::delim::Delim::$value
                ),
            }
        );
        assert_eq!(&Vec::from_iter($lexeme.chars()), &$chars[$span], "the lexemes don't match")
    };
    ($chars:expr, $token:expr, lexeme=$lexeme:expr, span=$span:expr, kind=operator, value=$value:ident) => {
        assert_eq!(
            $token,
            ::minimal_compiler::tokenizer::token::Token {
                span: $span,
                value: ::minimal_compiler::tokenizer::token::TokenValue::Operator(
                    ::minimal_compiler::tokenizer::token::operator::Operator::$value
                ),
            }
        );
        assert_eq!(&Vec::from_iter($lexeme.chars()), &$chars[$span], "the lexemes don't match")
    };
    ($chars:expr, $token:expr, lexeme=$lexeme:expr, span=$span:expr, kind=error, value=$value:ident) => {
        assert_eq!(
            $token,
            ::minimal_compiler::tokenizer::token::Token {
                span: $span,
                value: ::minimal_compiler::tokenizer::token::TokenValue::Error(
                    ::minimal_compiler::tokenizer::token::Error::$value
                ),
            }
        );
        assert_eq!(&Vec::from_iter($lexeme.chars()), &$chars[$span], "the lexemes don't match")
    };
    ($chars:expr, $token:expr, lexeme=$lexeme:expr, span=$span:expr, kind=unescape_error, value=$value:ident) => {
        assert_eq!(
            $token,
            Token {
                span: $span,
                value: ::minimal_compiler::tokenizer::token::TokenValue::UnescapeError(
                    ::minimal_compiler::util::unescape::UnescapeError::$value
                ),
            }
        );
        assert_eq!(&Vec::from_iter($lexeme.chars()), &$chars[$span], "the lexemes don't match")
    };
}
