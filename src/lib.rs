
use std::ffi::{c_int, c_void};
use std::slice;
//use std::arch::x86_64::*;

pub use crate::validate_ascii_number as other_validate_ascii_number;

// #[cfg(target_arch = "x86_64")]
// pub unsafe fn validate_ascii_number_avx(input: &[u8], allow_negative: bool) -> bool {
//     if input.len() > 32 {
//         return false;
//     }
//
//     // Initialize buffer with spaces
//     let mut buffer = [b' '; 32];
//     buffer[..input.len()].copy_from_slice(input);
//
//     let data = _mm256_loadu_si256(buffer.as_ptr() as *const __m256i);
//
//     // Check for '-'
//     let dash = _mm256_set1_epi8(b'-' as i8);
//     let is_dash = _mm256_cmpeq_epi8(data, dash);
//     if _mm256_movemask_epi8(is_dash) != 0 {
//         return false;
//     }
//
//     // Check for spaces
//     let space = _mm256_set1_epi8(b' ' as i8);
//     let is_space = _mm256_cmpeq_epi8(data, space);
//
//     // Check for digits
//     let zero = _mm256_set1_epi8(b'0' as i8);
//     let nine = _mm256_set1_epi8(b'9' as i8);
//     let is_digit = _mm256_and_si256(
//         _mm256_cmpgt_epi8(data, _mm256_sub_epi8(zero, _mm256_set1_epi8(1))),
//         _mm256_cmpgt_epi8(_mm256_add_epi8(nine, _mm256_set1_epi8(1)), data),
//     );
//
//     // Combine spaces and digits
//     let is_space_or_digit = _mm256_or_si256(is_space, is_digit);
//
//     // Check for letters 'p' to 'y' in the last position
//     let is_valid_letter_last_char = if allow_negative && input.len() > 0 {
//         let p_char = _mm256_set1_epi8(b'p' as i8);
//         let y_char = _mm256_set1_epi8(b'y' as i8);
//
//         let is_letter = _mm256_and_si256(
//             _mm256_cmpgt_epi8(data, _mm256_sub_epi8(p_char, _mm256_set1_epi8(1))),
//             _mm256_cmpgt_epi8(_mm256_add_epi8(y_char, _mm256_set1_epi8(1)), data),
//         );
//
//         // Mask for the last character
//         let mut last_char_mask_arr = [0i8; 32];
//         last_char_mask_arr[input.len() - 1] = -1;
//         let last_char_mask = _mm256_loadu_si256(last_char_mask_arr.as_ptr() as *const __m256i);
//
//         _mm256_and_si256(is_letter, last_char_mask)
//     } else {
//         _mm256_setzero_si256()
//     };
//
//     // Combine all valid characters
//     let is_valid_char = _mm256_or_si256(is_space_or_digit, is_valid_letter_last_char);
//
//     // Mask for input length
//     let mut length_mask_arr = [0i8; 32];
//     for i in 0..input.len() {
//         length_mask_arr[i] = -1;
//     }
//     let length_mask = _mm256_loadu_si256(length_mask_arr.as_ptr() as *const __m256i);
//
//     // Invalidate characters beyond input length
//     let invalid_chars = _mm256_andnot_si256(is_valid_char, length_mask);
//
//     // Check if any invalid characters exist
//     _mm256_testz_si256(invalid_chars, invalid_chars) != 0
// }
//
// #[target_feature(enable = "sse2")]
// unsafe fn validate_number_128bit(input: &[u8], allow_negative: bool) -> bool {
//     if input.len() > 16 {
//         return false;
//     }
//
//     // Initialize buffer with spaces
//     let mut buffer = [b' '; 16];
//     buffer[..input.len()].copy_from_slice(input);
//
//     let data = _mm_loadu_si128(buffer.as_ptr() as *const __m128i);
//
//     // Check for '-'
//     let dash = _mm_set1_epi8(b'-' as i8);
//     let is_dash = _mm_cmpeq_epi8(data, dash);
//     if _mm_movemask_epi8(is_dash) != 0 {
//         return false;
//     }
//
//     // Check for spaces
//     let space = _mm_set1_epi8(b' ' as i8);
//     let is_space = _mm_cmpeq_epi8(data, space);
//
//     // Check for digits
//     let zero = _mm_set1_epi8(b'0' as i8);
//     let nine = _mm_set1_epi8(b'9' as i8);
//     let is_digit = _mm_and_si128(
//         _mm_cmpgt_epi8(data, _mm_sub_epi8(zero, _mm_set1_epi8(1))),
//         _mm_cmpgt_epi8(_mm_add_epi8(nine, _mm_set1_epi8(1)), data),
//     );
//
//     // Combine spaces and digits
//     let is_space_or_digit = _mm_or_si128(is_space, is_digit);
//
//     // Check for letters 'p' to 'y' in the last position
//     let is_valid_letter_last_char = if allow_negative && input.len() > 0 {
//         let p_char = _mm_set1_epi8(b'p' as i8);
//         let y_char = _mm_set1_epi8(b'y' as i8);
//
//         let is_letter = _mm_and_si128(
//             _mm_cmpgt_epi8(data, _mm_sub_epi8(p_char, _mm_set1_epi8(1))),
//             _mm_cmpgt_epi8(_mm_add_epi8(y_char, _mm_set1_epi8(1)), data),
//         );
//
//         // Mask for the last character
//         let mut last_char_mask_arr = [0i8; 16];
//         last_char_mask_arr[input.len() - 1] = -1;
//         let last_char_mask = _mm_loadu_si128(last_char_mask_arr.as_ptr() as *const __m128i);
//
//         _mm_and_si128(is_letter, last_char_mask)
//     } else {
//         _mm_setzero_si128()
//     };
//
//     // Combine all valid characters
//     let is_valid_char = _mm_or_si128(is_space_or_digit, is_valid_letter_last_char);
//
//     // Mask for input length
//     let mut length_mask_arr = [0i8; 16];
//     for i in 0..input.len() {
//         length_mask_arr[i] = -1;
//     }
//     let length_mask = _mm_loadu_si128(length_mask_arr.as_ptr() as *const __m128i);
//
//     // Invalidate characters beyond input length
//     let invalid_chars = _mm_andnot_si128(is_valid_char, length_mask);
//
//     // Check if any invalid characters exist
//     _mm_testz_si128(invalid_chars, invalid_chars) != 0
// }
//
// #[target_feature(enable = "sse2")]
// unsafe fn validate_number_64bit(input: &[u8], allow_negative: bool) -> bool {
//     if input.len() > 8 {
//         return false;
//     }
//
//     // Initialize buffer with spaces
//     let mut buffer = [b' '; 8];
//     buffer[..input.len()].copy_from_slice(input);
//
//     let data = _mm_loadl_epi64(buffer.as_ptr() as *const __m128i);
//
//     // Similar checks as above, adjusted for 64 bits
//     let dash = _mm_set1_epi8(b'-' as i8);
//     let is_dash = _mm_cmpeq_epi8(data, dash);
//     if _mm_movemask_epi8(is_dash) != 0 {
//         return false;
//     }
//
//     let space = _mm_set1_epi8(b' ' as i8);
//     let is_space = _mm_cmpeq_epi8(data, space);
//
//     let zero = _mm_set1_epi8(b'0' as i8);
//     let nine = _mm_set1_epi8(b'9' as i8);
//     let is_digit = _mm_and_si128(
//         _mm_cmpgt_epi8(data, _mm_sub_epi8(zero, _mm_set1_epi8(1))),
//         _mm_cmpgt_epi8(_mm_add_epi8(nine, _mm_set1_epi8(1)), data),
//     );
//
//     let is_space_or_digit = _mm_or_si128(is_space, is_digit);
//
//     let is_valid_letter_last_char = if allow_negative && input.len() > 0 {
//         let p_char = _mm_set1_epi8(b'p' as i8);
//         let y_char = _mm_set1_epi8(b'y' as i8);
//
//         let is_letter = _mm_and_si128(
//             _mm_cmpgt_epi8(data, _mm_sub_epi8(p_char, _mm_set1_epi8(1))),
//             _mm_cmpgt_epi8(_mm_add_epi8(y_char, _mm_set1_epi8(1)), data),
//         );
//
//         let mut last_char_mask_arr = [0i8; 8];
//         last_char_mask_arr[input.len() - 1] = -1;
//         let last_char_mask = _mm_loadl_epi64(last_char_mask_arr.as_ptr() as *const __m128i);
//
//         _mm_and_si128(is_letter, last_char_mask)
//     } else {
//         _mm_setzero_si128()
//     };
//
//     let is_valid_char = _mm_or_si128(is_space_or_digit, is_valid_letter_last_char);
//
//     let mut length_mask_arr = [0i8; 8];
//     for i in 0..input.len() {
//         length_mask_arr[i] = -1;
//     }
//     let length_mask = _mm_loadl_epi64(length_mask_arr.as_ptr() as *const __m128i);
//
//     let invalid_chars = _mm_andnot_si128(is_valid_char, length_mask);
//
//     _mm_testz_si128(invalid_chars, invalid_chars) != 0
// }
//
// #[target_feature(enable = "sse2")]
// unsafe fn validate_number_32bit(input: &[u8], allow_negative: bool) -> bool {
//     if input.len() > 4 {
//         return false;
//     }
//
//     // Initialize buffer with spaces
//     let mut buffer = [b' '; 4];
//     buffer[..input.len()].copy_from_slice(input);
//
//     // Load 32 bits
//     let data = _mm_cvtsi32_si128(*(buffer.as_ptr() as *const i32));
//
//     let dash = _mm_set1_epi8(b'-' as i8);
//     let is_dash = _mm_cmpeq_epi8(data, dash);
//     if _mm_movemask_epi8(is_dash) & 0x0F != 0 {
//         return false;
//     }
//
//     let space = _mm_set1_epi8(b' ' as i8);
//     let is_space = _mm_cmpeq_epi8(data, space);
//
//     let zero = _mm_set1_epi8(b'0' as i8);
//     let nine = _mm_set1_epi8(b'9' as i8);
//     let is_digit = _mm_and_si128(
//         _mm_cmpgt_epi8(data, _mm_sub_epi8(zero, _mm_set1_epi8(1))),
//         _mm_cmpgt_epi8(_mm_add_epi8(nine, _mm_set1_epi8(1)), data),
//     );
//
//     let is_space_or_digit = _mm_or_si128(is_space, is_digit);
//
//     let is_valid_letter_last_char = if allow_negative && input.len() > 0 {
//         let p_char = _mm_set1_epi8(b'p' as i8);
//         let y_char = _mm_set1_epi8(b'y' as i8);
//
//         let is_letter = _mm_and_si128(
//             _mm_cmpgt_epi8(data, _mm_sub_epi8(p_char, _mm_set1_epi8(1))),
//             _mm_cmpgt_epi8(_mm_add_epi8(y_char, _mm_set1_epi8(1)), data),
//         );
//
//         let mut last_char_mask_arr = [0i8; 4];
//         last_char_mask_arr[input.len() - 1] = -1;
//         let last_char_mask = _mm_cvtsi32_si128(*(last_char_mask_arr.as_ptr() as *const i32));
//
//         _mm_and_si128(is_letter, last_char_mask)
//     } else {
//         _mm_setzero_si128()
//     };
//
//     let is_valid_char = _mm_or_si128(is_space_or_digit, is_valid_letter_last_char);
//
//     let mut length_mask_arr = [0i8; 4];
//     for i in 0..input.len() {
//         length_mask_arr[i] = -1;
//     }
//     let length_mask = _mm_cvtsi32_si128(*(length_mask_arr.as_ptr() as *const i32));
//
//     let invalid_chars = _mm_andnot_si128(is_valid_char, length_mask);
//
//     _mm_testz_si128(invalid_chars, invalid_chars) & 0x0F != 0
// }

#[no_mangle]
pub extern "C" fn validate_ascii_number(ptr: *const c_void, length: c_int, allow_negative: c_int) -> bool {
    let length = length as usize;
    let bytes = unsafe { slice::from_raw_parts(ptr as *const u8, length) };
    let allow_negative = allow_negative != 0;

    // unsafe {
    //     match bytes.len() {
    //         0..=4 => validate_number_32bit(bytes, allow_negative),
    //         5..=8 => validate_number_64bit(bytes, allow_negative),
    //         9..=16 => validate_number_128bit(bytes, allow_negative),
    //         17..=32 => validate_ascii_number_avx(bytes, allow_negative),
    //         _ => false,
    //     }
    // }
    //the fallback implementation is faster than the SIMD implementation for the data sizes we are testing
    fallback_validate_ascii_number(bytes, allow_negative)
}

fn fallback_validate_ascii_number(bytes: &[u8], allow_negative: bool) -> bool {
    // Track if we've encountered any valid non-space character
    let mut found_digit = false;

    for (i, &byte) in bytes.iter().enumerate() {
        match byte {
            b'0'..=b'9' => {
                found_digit = true;
            }
            b'p'..=b'y' if allow_negative && i == bytes.len() - 1 => {
                found_digit = true; // Valid at the last position for negative cases
            }
            b' ' => {
                // Continue for spaces, but they should not be followed by invalid characters
                continue;
            }
            _ => return false, // Early exit on any invalid character
        }
    }

    // If we haven't found any valid digits or allowed letter, it's invalid
    found_digit
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_validation(input: &[u8], allow_negative: bool, expected: bool) {
        let result = validate_ascii_number(input.as_ptr() as *const c_void, input.len() as c_int, if allow_negative { 1 } else { 0 });
        assert_eq!(result, expected, "Failed on input: {:?}, allow_negative: {}", String::from_utf8_lossy(input), allow_negative);
    }

    #[test]
    fn test_valid_positive_numbers() {
        test_validation(b"123", false, true);
        test_validation(b"0", false, true);
        test_validation(b"9999", false, true);
    }

    #[test]
    fn test_valid_negative_numbers() {
        test_validation(b"123p", true, true);
        test_validation(b"0y", true, true);
        test_validation(b"-9999", true, false);
    }

    #[test]
    fn test_invalid_numbers() {
        test_validation(b"", false, false);
        test_validation(b"abc", false, false);
        test_validation(b"12a3", false, false);
        test_validation(b"12.3", false, false);
    }

    #[test]
    fn test_negative_numbers_when_not_allowed() {
        test_validation(b"-123", false, false);
        test_validation(b"123p", false, false);
    }

    #[test]
    fn test_edge_cases() {
        test_validation(b"0", true, true);
        test_validation(b"-0", true, false);
        test_validation(b"0y", true, true);
        test_validation(b"-", true, false);
        test_validation(b"p", true, true);
    }

    #[test]
    fn test_invalid_negative_encodings() {
        test_validation(b"123y", true, true);
        test_validation(b"123p", true, true);
        test_validation(b"123o", true, false); //'o' is out of range
        test_validation(b"123z", true, false); // 'z' is out of range
        test_validation(b"y123", true, false); // encoding in wrong position
    }

    #[test]
    fn test_multiple_minus_signs() {
        test_validation(b"-123-", true, false);
        test_validation(b"--123", true, false);
    }

    #[test]
    fn test_whitespace() {
        test_validation(b" 123 ", true, true);
        test_validation(b"  123", true, true);
        test_validation(b"  12 3", true, true);
    }
}
