/**
 * file: utility_tests.rs
 * desc: Test functions in the utility module.
 */
use crate::utility::pwstr_to_string;

use windows::core::PWSTR;

#[test]
fn test_pwstr_to_string() {
    let mut s1 = String::from("nice string you got there\0")
        .encode_utf16()
        .collect::<Vec<u16>>();
    let mut s2 = String::from("would be a shame if\0")
        .encode_utf16()
        .collect::<Vec<u16>>();
    let mut s3 = String::from("something happened to it\0")
        .encode_utf16()
        .collect::<Vec<u16>>();
    let p1 = PWSTR(s1.as_mut_ptr());
    let p2 = PWSTR(s2.as_mut_ptr());
    let p3 = PWSTR(s3.as_mut_ptr());

    unsafe {
        assert_eq!("nice string you got there", pwstr_to_string(p1));
        assert_eq!("would be a shame if", pwstr_to_string(p2));
        assert_eq!("something happened to it", pwstr_to_string(p3));
    }
}
