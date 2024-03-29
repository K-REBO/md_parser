#[cfg(test)]
mod tests {

    use crate::parser::parser::Parsed;

    use crate::SpanField;
    use crate::SpanType;
    use nom::{Err, IResult};

    #[test]
    fn is_work_nested_italic_strong() {
        let inputs = ["***S+I***", "*I**Strong**I*", "**S*Italic*S**"];
        let expected_results: [Parsed; 3] = [
            Parsed {
                left: "".to_string(),
                span: SpanField {
                    span_type: SpanType::Decoration {
                        text: "S+I".to_string(),
                        is_strong: true,
                        is_italic: true,
                    },
                    child: None,
                },
                right: "".to_string(),
            },
            Parsed {
                left: "".to_string(),
                span: SpanField {
                    span_type: SpanType::Decoration {
                        text: "I".to_string(),
                        is_strong: false,
                        is_italic: true,
                    },
                    child: Some(Box::new(SpanField {
                        span_type: SpanType::Decoration {
                            text: "Strong".to_string(),
                            is_strong: true,
                            is_italic: false,
                        },
                        child: Some(Box::new(SpanField {
                            span_type: SpanType::Decoration {
                                text: "I".to_string(),
                                is_strong: false,
                                is_italic: true,
                            },
                            child: None,
                        })),
                    })),
                },
                right: "".to_string(),
            },
            Parsed {
                left: "".to_string(),
                span: SpanField {
                    span_type: SpanType::Decoration {
                        text: "S".to_string(),
                        is_strong: true,
                        is_italic: false,
                    },
                    child: Some(Box::new(SpanField {
                        span_type: SpanType::Decoration {
                            text: "Italic".to_string(),
                            is_strong: false,
                            is_italic: true,
                        },
                        child: Some(Box::new(SpanField {
                            span_type: SpanType::Decoration {
                                text: "S".to_string(),
                                is_strong: true,
                                is_italic: false,
                            },
                            child: None,
                        })),
                    })),
                },
                right: "".to_string(),
            },
        ];
    }

    #[test]
    fn is_work_italic() {
        let inputs = ["left*italic*right*"];
        let expected_results: [IResult<&str, Parsed>; 1] = [Ok((
            "right*",
            Parsed {
                left: "left".to_string(),
                span: SpanField {
                    span_type: SpanType::Decoration {
                        text: "italic".to_string(),
                        is_italic: true,
                        is_strong: false,
                    },
                    child: None,
                },
                right: "right*".to_string(),
            },
        ))];

        for (i, input) in inputs.iter().enumerate() {
            let result = super::parser::parse_italic(input);
            println!("{}", input);
            println!("{:#?}", result);
            assert_eq!(result, expected_results[i]);
        }
    }

    #[test]
    fn is_work_strong() {
        let inputs = ["left**strong**right**"];
        let expected_results: [IResult<&str, Parsed>; 1] = [Ok((
            "right**",
            Parsed {
                left: "left".to_string(),
                span: SpanField {
                    span_type: SpanType::Decoration {
                        text: "strong".to_string(),
                        is_strong: true,
                        is_italic: false,
                    },
                    child: None,
                },
                right: "right**".to_string(),
            },
        ))];

        for (i, input) in inputs.iter().enumerate() {
            let result = super::parser::parse_strong(input);
            println!("{}", input);
            println!("{:#?}", result);
            assert_eq!(result, expected_results[i]);
        }
    }

    #[test]
    fn is_work_strong_italic() {
        let inputs = ["left***strong italic***right***"];
        let expected_results: [IResult<&str, Parsed>; 1] = [Ok((
            "right***",
            Parsed {
                left: "left".to_string(),
                span: SpanField {
                    span_type: SpanType::Decoration {
                        text: "strong italic".to_string(),
                        is_strong: true,
                        is_italic: true,
                    },
                    child: None,
                },
                right: "right***".to_string(),
            },
        ))];

        for (i, input) in inputs.iter().enumerate() {
            let result = super::parser::parse_strong_italic(input);
            println!("{}", input);
            println!("{:#?}", result);
            assert_eq!(result, expected_results[i]);
        }
    }

    // !TODO add ERR case
    #[test]
    fn is_work_image() {
        let input = "aa![alt](https://img.example.com/image.jpg)bb![alt](link)";
        let result = super::parser::parse_image(input);

        println!("{:#?}", result);

        let expected_result: IResult<&str, Parsed> = Ok((
            "bb![alt](link)",
            Parsed {
                left: "aa".to_string(),
                span: SpanField {
                    span_type: SpanType::Image {
                        alt: "alt".to_string(),
                        src: "https://img.example.com/image.jpg".to_string(),
                    },
                    child: None,
                },
                right: "bb![alt](link)".to_string(),
            },
        ));

        assert_eq!(result, expected_result);
    }

    #[test]
    fn is_work_link() {
        let input = "aa[link](https://example.com)bb![link](https://example.com)";
        let result = super::parser::parse_link(input);

        println!("{:#?}", result);

        let expected_result: IResult<&str, Parsed> = Ok((
            "bb![link](https://example.com)",
            Parsed {
                left: "aa".to_string(),
                span: SpanField {
                    span_type: SpanType::Link {
                        text: "link".to_string(),
                        href: "https://example.com".to_string(),
                    },
                    child: None,
                },
                right: "bb![link](https://example.com)".to_string(),
            },
        ));

        assert_eq!(result, expected_result);
    }

    #[test]
    fn is_work_strikethrough() {
        let input = "aa~~text~~bb~~";
        let result = super::parser::parse_strikethrough(input);

        println!("{:#?}", result);

        let expected_result: IResult<&str, Parsed> = Ok((
            "bb~~",
            Parsed {
                left: "aa".to_string(),
                span: SpanField {
                    span_type: SpanType::StrikeThrough("text".to_string()),
                    child: None,
                },
                right: "bb~~".to_string(),
            },
        ));

        assert_eq!(result, expected_result);
    }

    #[test]
    fn is_work_highlight() {
        let input = "aa==text==bb==";
        let result = super::parser::parse_highlight(input);

        println!("{:#?}", result);

        let expected_result: IResult<&str, Parsed> = Ok((
            "bb==",
            Parsed {
                left: "aa".to_string(),
                span: SpanField {
                    span_type: SpanType::Highlight("text".to_string()),
                    child: None,
                },
                right: "bb==".to_string(),
            },
        ));

        assert_eq!(result, expected_result);
    }
}

pub mod parser {
    use std::fmt::format;

    use crate::SpanField;
    use crate::SpanType;
    use nom::bytes::complete::{is_not, tag, take_until, take_while};
    use nom::character::complete::{
        char, digit1, none_of, space0, space1,
    };
    use nom::{Err, IResult};

    #[derive(Debug, PartialEq)]
    pub struct Parsed {
        pub left: String,
        pub span: SpanField,
        pub right: String,
    }

    pub fn parse_image(input: &str) -> IResult<&str, Parsed> {
        let (input, left) = take_until("![")(input)?;
        let (input, _) = tag("![")(input)?;
        let (input, alt) = take_until("]")(input)?;
        let (input, _) = tag("](")(input)?;
        let (input, href) = take_until(")")(input)?;
        let (input, _) = tag(")")(input)?;

        println!("alt: {}", alt);
        println!("href: {}", href);

        let parsed = Parsed {
            left: left.to_string(),
            span: SpanField {
                span_type: SpanType::Image {
                    alt: alt.to_string(),
                    src: href.to_string(),
                },
                child: None,
            },
            right: input.to_string(),
        };

        return Ok((input, parsed));
    }
    pub fn parse_link(input: &str) -> IResult<&str, Parsed> {
        let (input, left) = take_until("[")(input)?;
        let (input, _) = tag("[")(input)?;
        let (input, text) = take_until("]")(input)?;
        let (input, _) = tag("](")(input)?;
        let (input, link) = take_until(")")(input)?;
        let (input, _) = tag(")")(input)?;

        println!("alt: {}", text);
        println!("link: {}", link);

        let parsed = Parsed {
            left: left.to_string(),
            span: SpanField {
                span_type: SpanType::Link {
                    text: text.to_string(),
                    href: link.to_string(),
                },
                child: None,
            },
            right: input.to_string(),
        };

        return Ok((input, parsed));
    }
    pub fn parse_strikethrough(input: &str) -> IResult<&str, Parsed> {
        let (input, left) = take_until("~~")(input)?;
        let (input, _) = tag("~~")(input)?;
        let (input, text) = take_until("~~")(input)?;
        let (input, _) = tag("~~")(input)?;

        let parsed = Parsed {
            left: left.to_string(),
            span: SpanField {
                span_type: SpanType::StrikeThrough(text.to_string()),
                child: None,
            },
            right: input.to_string(),
        };

        return Ok((input, parsed));
    }
    pub fn parse_highlight(input: &str) -> IResult<&str, Parsed> {
        let (input, left) = take_until("==")(input)?;
        let (input, _) = tag("==")(input)?;
        let (input, text) = take_until("==")(input)?;
        let (input, _) = tag("==")(input)?;

        let parsed = Parsed {
            left: left.to_string(),
            span: SpanField {
                span_type: SpanType::Highlight(text.to_string()),
                child: None,
            },
            right: input.to_string(),
        };

        return Ok((input, parsed));
    }
    pub fn parse_strong(input: &str) -> IResult<&str, Parsed> {
        let (input, left) = take_until("**")(input)?;
        let (input, _) = tag("**")(input)?;
        let (input, text) = take_until("**")(input)?;
        let (input, _) = tag("**")(input)?;

        let parsed = Parsed {
            left: left.to_string(),
            span: SpanField {
                span_type: SpanType::Decoration {
                    text: text.to_string(),
                    is_strong: true,
                    is_italic: false,
                },
                child: None,
            },
            right: input.to_string(),
        };

        return Ok((input, parsed));
    }
    pub fn parse_italic(input: &str) -> IResult<&str, Parsed> {
        let (input, left) = take_until("*")(input)?;
        let (input, _) = tag("*")(input)?;
        let (input, text) = take_until("*")(input)?;
        let (input, _) = tag("*")(input)?;

        let parsed = Parsed {
            left: left.to_string(),
            span: SpanField {
                span_type: SpanType::Decoration {
                    text: text.to_string(),
                    is_italic: true,
                    is_strong: false,
                },
                child: None,
            },
            right: input.to_string(),
        };

        return Ok((input, parsed));
    }
    pub fn parse_strong_italic(input: &str) -> IResult<&str, Parsed> {
        let (input, left) = take_until("***")(input)?;
        let (input, _) = tag("***")(input)?;
        let (input, text) = take_until("***")(input)?;
        let (input, _) = tag("***")(input)?;

        let parsed = Parsed {
            left: left.to_string(),
            span: SpanField {
                span_type: SpanType::Decoration {
                    text: text.to_string(),
                    is_italic: true,
                    is_strong: true,
                },
                child: None,
            },
            right: input.to_string(),
        };

        return Ok((input, parsed));
    }
    pub fn parse_inline_code(input: &str) -> IResult<&str, Parsed> {
        let (input, left) = take_until("`")(input)?;
        let (input, _) = tag("`")(input)?;
        let (input, text) = take_until("`")(input)?;
        let (input, _) = tag("`")(input)?;

        let parsed = Parsed {
            left: left.to_string(),
            span: SpanField {
                span_type: SpanType::InlineCode(text.to_string()),
                child: None,
            },
            right: input.to_string(),
        };

        return Ok((input, parsed));
    }
    pub fn parse_footnote_ref(input: &str) -> IResult<&str, Parsed> {
        let (input, left) = take_until("[^")(input)?;
        let (input, _) = tag("[^")(input)?;
        let (input, text) = take_until("]")(input)?;
        let (input, _) = tag("]")(input)?;

        let parsed = Parsed {
            left: left.to_string(),
            span: SpanField {
                span_type: SpanType::FootnoteReference {
                    id: text.to_string(),
                },
                child: None,
            },
            right: input.to_string(),
        };

        return Ok((input, parsed));
    }
    pub fn parse_inline_html(input: &str) -> IResult<&str, Parsed> {
        let (input, left) = take_until("<")(input)?;
        let (input, _) = tag("<")(input)?;
        let (input, first_tag) = take_until(">")(input)?;
        let (input, _) = tag(">")(input)?;
        let (input, text) = take_until(format!("</{}", first_tag).as_str())(input)?;
        let mut span = text.to_string();

        let first_tag_count = text.split(format!("<{}>", first_tag).as_str()).count();

        return Ok((
            input,
            Parsed {
                left: left.to_string(),
                span: SpanField {
                    span_type: SpanType::InlineHTML(span.to_string()),
                    child: None,
                },
                right: input.to_string(),
            },
        ));
    }
}
