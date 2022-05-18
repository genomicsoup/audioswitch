/**
 * file: utility.rs
 * desc: Utility functions.
 */
use windows::core::PWSTR;

/**
 * Convert a PWSTR to a rust String. Read the PWSTR character by character until the null
 * terminator is found to determine the length, then slice up to the length and convert into
 * a string.
 *
 * args
 *  ptr: a wide string pointer
 *
 * returns
 *  a rust String
 */
pub unsafe fn pwstr_to_string(ptr: PWSTR) -> String {
    let mut len: usize = 0;
    let mut cursor = ptr;

    loop {
        let val = cursor.0.read();

        // Breaks on the null terminator
        if val == 0 {
            break;
        }

        len += 1;
        cursor = PWSTR(cursor.0.add(1));
    }

    let slice = std::slice::from_raw_parts(ptr.0, len);

    String::from_utf16(slice).unwrap()
}

#[cfg(test)]
#[cfg(target_os = "windows")]
#[path = "tests/utility_tests.rs"]
mod utility_tests;
