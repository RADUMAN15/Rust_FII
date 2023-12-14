//! # base64
//!
//! `base64` is a library crate that implements base64 functionalities.
//!
//! ## Examples
//!
//! ```
//! use base64::encode;
//!
//! let input = b"ILOVERUST";
//! let encoded = encode(input);
//!
//! assert_eq!(encoded, "SUxPVkVSVVNU");
//! ```

/// Encode a byte slice to base64.
///
/// # Examples
///
/// ```
/// use base64::encode;
///
/// let input = b"Hello, World!";
/// let encoded = encode(input);
///
/// assert_eq!(encoded, "SGVsbG8sIFdvcmxkIQ==");
/// ```
///
///
pub fn encode(input: &[u8]) -> String {
    let char_set: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
        .chars()
        .collect();

    let mut res_str = String::new();
    let mut val;
    let mut no_of_bits;
    let mut padding = 0;
    let mut count;
    let mut index;

    for chunk in input.chunks(3) {
        val = 0;
        count = 0;

        for &byte in chunk {
            val = val << 8;
            val = val | byte as u32;
            count += 1;
        }

        no_of_bits = count * 8;
        padding = no_of_bits % 3;

        while no_of_bits != 0 {
            if no_of_bits >= 6 {
                let temp = no_of_bits - 6;
                index = (val >> temp) & 63;
                no_of_bits -= 6;
            } else {
                let temp = 6 - no_of_bits;
                index = (val << temp) & 63;
                no_of_bits = 0;
            }
            res_str.push(char_set[index as usize]);
        }
    }
    for _ in 0..padding {
        res_str.push('=');
    }

    res_str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_no_padding() {
        // Test without padding
        let input = b"RADUMAN";
        let encoded = encode(input);
        assert_eq!(encoded, "UkFEVU1BTg==");
    }

    #[test]
    fn test_encode_half_padding() {
        // Test with padding
        let input = b"RADUMANDREA";
        let encoded = encode(input);
        assert_eq!(encoded, "UkFEVU1BTkRSRUE=");
    }
}
