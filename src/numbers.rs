//! Converter functions that convert trits/trytes to numbers.
//!
//! NOTE: In the future it might be necessary to use `num-bigint`.
//!
//! Currently 11 trytes are enough to represent the IOTA supply
//! of (3^(3*11)-1)/2 = 2,779530283×10^15
//! (2^64-1)/2 = 9,223372037×10^18

use crate::{
    luts::ASCII_CODE_SEQ_TO_NUM,
    luts::ASCII_CODE_TO_TRITS,
    types::Trit,
    types::Tryte,
};

/// Converts up to 11 trytes to an `i64` by only using a LUT and addition.
/// This function will panic if the slice is bigger then 11 trytes.
//#[cfg(feature = "iota")]
pub fn from_trytes_max11(trytes: &[Tryte]) -> i64 {
    #[cfg(not(feature = "no_checks"))]
    {
        assert!(crate::util::is_trytes(&trytes));
        assert!(trytes.len() <= crate::constants::TRYTE_LENGTH_FOR_MAX_TOKEN_SUPPLY);
    }

    let mut number = 0;
    trytes.iter().enumerate().for_each(|(i, &t)| {
        number += ASCII_CODE_SEQ_TO_NUM[i][t as usize - 57];
    });

    number
}

/// Converts up to 13 trytes to an `i64` by using `i64::pow`.
pub fn from_trytes_max13(trytes: &[Tryte]) -> i64 {
    #[cfg(not(feature = "no_checks"))]
    {
        assert!(crate::util::is_trytes(&trytes));
        assert!(trytes.len() <= crate::constants::TRYTE_LENGTH_FOR_MAX_I64);
    }

    let mut number = 0;
    trytes.iter().rev().for_each(|tryte| {
        let trits = ASCII_CODE_TO_TRITS[tryte];
        for n in (0..trits.len()).rev() {
            number = number * 3 + i64::from(trits[n]);
        }
    });

    number
}

#[cfg(test)]
mod from_trytes_tests {
    use super::super::trytes;
    use super::*;
    use rand::*;

    #[test]
    fn test_from_trytes_max11() {
        let number = from_trytes_max11(&['9' as u8]);
        assert_eq!(0, number);

        let number = from_trytes_max11(&['A' as u8]);
        assert_eq!(1, number);

        let number = from_trytes_max11(&['M' as u8]);
        assert_eq!(13, number);

        let number = from_trytes_max11(&['A' as u8, '9' as u8]);
        assert_eq!(1, number);

        let number = from_trytes_max11(&['9' as u8, 'A' as u8]);
        assert_eq!(27, number);
    }

    #[test]
    fn test_from_trytes_max13() {
        let number = from_trytes_max13(&['9' as u8]);
        assert_eq!(0, number);

        let number = from_trytes_max13(&['A' as u8]);
        assert_eq!(1, number);

        let number = from_trytes_max13(&['M' as u8]);
        assert_eq!(13, number);

        let number = from_trytes_max13(&['A' as u8, '9' as u8]);
        assert_eq!(1, number);

        let number = from_trytes_max13(&['9' as u8, 'A' as u8]);
        assert_eq!(27, number);
    }

    #[test]
    fn test_from_trytes_max11_inverse() {
        for _ in 0..1000 {
            let a = i64::from(thread_rng().next_u32());
            let b = i64::from(thread_rng().next_u32());
            let c = a - b;
            assert_eq!(c, from_trytes_max11(&trytes::from_num_i64_to_11(c)));
        }
    }
    /*
    //#[test]
    fn test_from_i64() {
        let trytes = String::from_utf8(from_i64(26, 10)).unwrap();
        println!("{}", trytes);
        //TODO
        assert!(false);
    }

    */

}

/// Converts a slice of trits.
pub fn from_trits(trits: &[Trit]) -> i64 {
    #[cfg(not(feature = "no_checks"))]
    {
        assert!(crate::util::is_trits(&trits));
        assert!(trits.len() <= 20);
    }

    let mut number = 0;

    for n in (0..trits.len()).rev() {
        number = number * 3 + i64::from(trits[n]);
    }

    number
}

#[cfg(test)]
mod from_trits_tests {
    use super::*;

    #[test]
    fn test_from_trits() {
        let number = from_trits(&[0, 0, 0]);
        assert_eq!(0, number);

        let number = from_trits(&[1, 1, 1]);
        assert_eq!(13, number);

        let number = from_trits(&[-1, -1, -1]);
        assert_eq!(-13, number);

        let number = from_trits(&[-1, -1, -1, 1]);
        assert_eq!(14, number);
    }
}
