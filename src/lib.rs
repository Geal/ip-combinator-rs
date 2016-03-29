/**
 * The MIT License (MIT)
 * Copyright (c) 2016 Jean Pierre Dudey
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

#[macro_use]
extern crate nom;

use nom::is_digit;
use std::str::FromStr;
use std::net::Ipv4Addr;

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

named!(pub ipv4_address<&[u8], Ipv4Addr>, chain!(a: d8 ~
                                                         dot   ~
                                                         b: d8 ~
                                                         dot   ~
                                                         c: d8 ~
                                                         dot   ~
                                                         d: d8,
                                                         || { Ipv4Addr::new(a, b, c, d) }));

#[cfg(test)]
mod test {
    #[test]
    fn check_ipv4_address() {
        use super::ipv4_address;
        use super::nom::IResult;
        use std::net::Ipv4Addr;

        let to_parse = b"192.168.1.1";
        let exp_out = Ipv4Addr::new(192, 168, 1, 1);
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
