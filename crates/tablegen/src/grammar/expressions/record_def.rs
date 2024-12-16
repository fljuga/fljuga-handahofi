/*
   Copyright (C) 2022-2024 Yuriy Yarosh.

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use winnow::PResult;
use winnow::ascii::*;
use winnow::combinator::*;
use winnow::combinator::*;
use winnow::error::*;
use winnow::error::*;
use winnow::stream::{AsChar, Stream};
use winnow::token::*;
use winnow::*;

use crate::grammar::tokens::internal;

#[cfg(test)]
mod tests {
    use super::*;
    use internal::tests::*;

    #[test]
    fn should_parse_record_def() {
        // let output56 = Some((5, 6));
        // test_parser(
        //     vec![
        //         ("5 ... 6", output56, ""),    // Valid range, fully consumed
        //         ("5...6xx", output56, "xx"),  // Partially valid range input, stops before 'x'
        //         ("5-6", output56, ""),    // Valid range, fully consumed
        //         ("5 -6", Some((5, -6)), ""),  // Invalid range, fully consumed
        //         ("5 6xx", output56, "xx"),  // Partially valid range input, stops before 'x'
        //         ("5 6", output56, ""),    // Valid range, fully consumed
        //         ("5   6xx", output56, "xx"),  // Partially valid range input, stops before 'x'
        //         ("5   -6xx", Some((5, -6)), "xx"),  // Partially valid range input with neg exclusion, stops before 'x'\
        //         ("5--6xx", Some((5,-6)), "xx"),  // Partially valid range input with neg exclusion, stops before 'x'
        //         ("", None, ""),                   // Empty input should fail
        //     ],
        //     range_int_piece,
        // );
    }
}
