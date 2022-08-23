pub(crate) fn take_while(accept: impl Fn(char) -> bool, s: &str) -> (&str, &str) {
    let extracted_end = s
        .char_indices()
        .find_map(|(idx, c)| if accept(c) { None } else { Some(idx) })
        .unwrap_or(s.len());

    let extracted = &s[..extracted_end];
    let remainder = &s[extracted_end..];

    (remainder, extracted)
}

pub(crate) fn take_while1(
    accept: impl Fn(char) -> bool,
    s: &str,
    error_msg: String,
) -> Result<(&str, &str), String> {
    let (remainder, extracted) = take_while(accept, s);

    if extracted.is_empty() {
        Err(error_msg)
    } else {
        Ok((remainder, extracted))
    }
}

#[inline]
pub(crate) fn extract_digits(s: &str) -> Result<(&str, &str), String> {
    take_while1(|c| c.is_ascii_digit(), s, "expected digits".to_string())
}

pub(crate) fn extract_op(s: &str) -> (&str, &str) {
    match &s[0..1] {
        "+" | "-" | "*" | "/" => {}
        _ => panic!("invalid operator"),
    }

    (&s[1..], &s[0..1])
}

const WHITESPACE: &[char] = &[' ', '\n'];

#[inline]
pub(crate) fn extract_whitespace(s: &str) -> (&str, &str) {
    take_while(|c| WHITESPACE.contains(&c), s)
}

#[inline]
pub(crate) fn extract_whitespace1(s: &str) -> Result<(&str, &str), String> {
    take_while1(
        |c| WHITESPACE.contains(&c),
        s,
        "expected whitespace".to_string(),
    )
}

pub(crate) fn extract_ident(s: &str) -> Result<(&str, &str), String> {
    let input_start_with_alphabetic = s
        .chars()
        .next()
        .map(|c| c.is_ascii_alphabetic())
        .unwrap_or(false);

    if input_start_with_alphabetic {
        Ok(take_while(|c| c.is_ascii_alphanumeric(), s))
    } else {
        Err("expected identifier".to_string())
    }
}

pub(crate) fn tag<'a, 'b>(starting_text: &'a str, s: &'b str) -> Result<&'b str, String> {
    s.strip_prefix(starting_text)
        .ok_or(format!("expected {}", starting_text))
}

#[cfg(test)]
mod tests {
    use super::*;

    mod digit {
        use super::*;

        #[test]
        fn extract_on_digit() {
            assert_eq!(extract_digits("1+2"), Ok(("+2", "1")));
        }

        #[test]
        fn extract_multiple_digits() {
            assert_eq!(extract_digits("10-20"), Ok(("-20", "10")))
        }

        #[test]
        fn do_not_extract_digits_from_invalid_input() {
            assert_eq!(extract_digits("abcd"), Err("expected digits".to_string()));
        }

        #[test]
        fn extract_digits_with_no_remainder() {
            assert_eq!(extract_digits("100"), Ok(("", "100")))
        }
    }

    mod op {
        use super::*;

        #[test]
        fn extract_plus() {
            assert_eq!(extract_op("+2"), ("2", "+"))
        }

        #[test]
        fn extract_minus() {
            assert_eq!(extract_op("-10"), ("10", "-"))
        }

        #[test]
        fn extract_star() {
            assert_eq!(extract_op("*3"), ("3", "*"))
        }

        #[test]
        fn extract_slash() {
            assert_eq!(extract_op("/4"), ("4", "/"))
        }
    }

    mod whitespace {
        use super::*;

        #[test]
        fn extract_spaces() {
            assert_eq!(extract_whitespace("    1"), ("1", "    "))
        }

        #[test]
        fn extract_newlines_or_spaces() {
            assert_eq!(extract_whitespace(" \n   \n\nabc"), ("abc", " \n   \n\n"));
        }

        #[test]
        fn do_not_extract_spaces1_when_input_does_not_start_with_them() {
            assert_eq!(
                extract_whitespace1("blah"),
                Err("expected whitespace".to_string()),
            );
        }
    }

    mod ident {
        use super::*;

        #[test]
        fn extract_alphabetic_ident() {
            assert_eq!(extract_ident("abcdEFG stop"), Ok((" stop", "abcdEFG")))
        }

        #[test]
        fn extract_alphanumeric_ident() {
            assert_eq!(extract_ident("foobar1()"), Ok(("()", "foobar1")))
        }

        #[test]
        fn cannot_extract_ident_beginning_with_number() {
            assert_eq!(
                extract_ident("123abc"),
                Err("expected identifier".to_string())
            );
        }
    }

    mod tag {
        use super::*;

        #[test]
        fn tag_word() {
            assert_eq!(tag("let", "let a"), Ok(" a"))
        }
    }
}