use winnow::PResult;
use winnow::stream::Stream;
use winnow::error::*;
use winnow::combinator::*;
use winnow::token::*;
use winnow::*;
use winnow::ascii::*;

mod tokens {
    use super::*;

    mod internal {
        use super::*;

        fn bin_digit1<'a>(input: &mut &'a str) -> PResult<&'a str> {
            take_while(1.., ('0', '1')).parse_next(input)
        }

        pub fn hex_or_bin_i64(input: &mut &str) -> PResult<i64> {
            dispatch!(take(2usize);
                "0x" => hex_digit1.try_map(|s| i64::from_str_radix(s, 16)),
                "0b" => bin_digit1.try_map(|s| i64::from_str_radix(s, 2)),
                _ => fail,
            ).parse_next( input)
        }

        fn dec_istr<'a>(input: &mut &'a str) -> PResult<&'a str> {
            preceded(opt(one_of(('+', '-'))), take_while(1.., '0'..='9'))
                .parse_next(input)
        }

        fn dec_ustr<'a>(input: &mut &'a str) -> PResult<&'a str> {
            take_while(1.., '0'..='9').parse_next(input)
        }

        pub fn dec_i64(input: &mut & str) -> PResult<i64> {
            alt((
                dec_istr.try_map(|s: &str| s.parse::<i64>()),
                dec_ustr.try_map(|s: &str| s.parse::<i64>())
            )).parse_next(input)
        }

        fn string<'a>(input: &mut &'a str) -> PResult<&'a str> {
            preceded('"', terminated(take_while(1.., |c: char| c != '"'), '"'))
                .parse_next(input)
        }

        fn code<'a>(input: &mut &'a str) -> PResult<&'a str> {
            preceded("[{", terminated(take_while(1.., |c: char| !"[{}]".contains(c)), "}]"))
                .parse_next(input)
        }

        fn identifier_chars<'a>(input: &mut &'a str) -> PResult<&'a str> {
            take_while(1.., ('a'..='z', 'A'..='Z', '0'..='9', '_')).parse_next(input)
        }

        fn identifier<'a>(input: &mut &'a str) -> PResult<&'a str> {
            preceded("[{", terminated(take_while(1.., |c: char| !"[{}]".contains(c)), "}]"))
                .parse_next(input)
        }
    }

    fn int (input: &mut &str) -> PResult<i64> {
        alt((
            internal::dec_i64,
            internal::hex_or_bin_i64,
        )).parse_next(input)
    }

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_numbers() {

        // println!("{:?}", r)
    }
}
