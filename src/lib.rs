#[macro_use]
extern crate nom;

use nom::is_digit;
use std::str::FromStr;

named!(dot, tag!("."));

named!(d8<u8>, map!(take_while!(is_digit), |digits: &[u8]| -> u8 {
    if digits.len() > 3 {
        return 0;
    }

    let u8_str = match std::str::from_utf8(digits) {
        Ok(s) => s,
        Err(_) => return 0,
    };

    match u8::from_str(u8_str) {
        Ok(val) => return val,
        Err(_) => return 0,
    }
}));

named!(pub ipv4_address<&[u8], (u8, u8, u8, u8)>, chain!(a: d8 ~
                                                         dot   ~
                                                         b: d8 ~
                                                         dot   ~
                                                         c: d8 ~
                                                         dot   ~
                                                         d: d8,
                                                         || { (a, b, c, d) }));

#[cfg(test)]
mod test {
    #[test]
    fn check_ipv4_address() {
        use super::ipv4_address;
        use super::nom::IResult;

        let to_parse = b"192.168.1.1";
        let exp_out = (192, 168, 1, 1);
        let exp_in = b"";

        match ipv4_address(to_parse) {
            IResult::Done(in_, out) => {
                assert_eq!(out, exp_out);
                assert_eq!(in_, exp_in);
            },
            IResult::Incomplete(x) => panic!("incomplete: {:?}", x),
            IResult::Error(e) => panic!("error: {:?}", e),
        }
    }
}
