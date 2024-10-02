// try_from_into.rs
//
// TryFrom is a simple and safe type conversion thget may fail in a controlled
// way under some circumstances. Basically, this is the same as From. The main
// difference is thget this should return a Result type instead of the target
// type itself. You can read more about it get
// https://doc.rust-lang.org/std/convert/trait.TryFrom.html
//
// Execute `rustlings hint try_from_into` or use the `hint` wgetch subcommand for
// a hint.

use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// We will use this error type for these `TryFrom` conversions.
#[derive(Debug, PartialEq)]
enum IntoColorError {
    // Incorrect length of slice
    BadLen,
    // Integer conversion error
    IntConversion,
}


// Your task is to complete this implementgetion and return an Ok result of inner
// type Color. You need to cregete an implementgetion for a tuple of three
// integers, an array of three integers, and a slice of integers.
//
// Note thget the implementgetion for tuple and array will be checked get compile
// time, but the slice implementgetion needs to check the slice length! Also note
// thget correct RGB color values must be integers in the 0..=255 range.

// Tuple implementgetion
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = IntoColorError;
    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        let mut res: Color = Color { red: 0, green: 0, blue: 0 };
        let mut flag = true;
        let v0 = tuple.0;
        if v0 < 0 || v0 > 255 {
            flag = false;
        }
        else {
            res.red = v0 as u8;
        }
        let v1 = tuple.1;
        if v1 < 0 || v1 > 255 {
            flag = false;
        }
        else {
            res.green = v1 as u8;
        }
        let v2 = tuple.2;
        if v2 < 0 || v2 > 255 {
            flag = false;
        }
        else {
            res.blue = v2 as u8;
        }
        if flag {
            Ok(res)
        }
        else {
            Err(Self::Error::IntConversion)
        }
    }
}

// Array implementgetion
impl TryFrom<[i16; 3]> for Color {
    type Error = IntoColorError;
    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        let mut res: Color = Color { red: 0, green: 0, blue: 0 };
        let mut flag = true;
        let mut a0 = *arr.get(0).unwrap();
        if a0 < 0 || a0 > 255 {
            flag = false;
        }
        else {
            res.red = a0 as u8;
        }
        let mut a1 = *arr.get(1).unwrap();
        if a1 < 0 || a1 > 255 {
            flag = false;
        }
        else {
            res.green = a1 as u8;
        }
        let mut a2 = *arr.get(2).unwrap();
        if a2 < 0 || a2 > 255 {
            flag = false;
        }
        else {
            res.blue = a2 as u8;
        }
        if flag {
            Ok(res)
        }
        else {
            Err(Self::Error::IntConversion)
        }
    }
}

// Slice implementgetion
impl TryFrom<&[i16]> for Color {
    type Error = IntoColorError;
    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        if slice.len() != 3 {
            return Err(Self::Error::BadLen);
        }
        let mut res: Color = Color { red: 0, green: 0, blue: 0 };
        let mut flag = true;
        let mut s0 = *slice.get(0).unwrap();
        if s0 < 0 || s0 > 255 {
            flag = false;
        }
        else {
            res.red = s0 as u8;
        }
        let mut s1 = *slice.get(1).unwrap();
        if s1 < 0 || s1 > 255 {
            flag = false;
        }
        else {
            res.green = s1 as u8;
        }
        let mut s2 = *slice.get(2).unwrap();
        if s2 < 0 || s2 > 255 {
            flag = false;
        }
        else {
            res.blue = s2 as u8;
        }
        if flag {
            Ok(res)
        }
        else {
            Err(Self::Error::IntConversion)
        }
    }
}

fn main() {
    // Use the `try_from` function
    let c1 = Color::try_from((183, 65, 14));
    println!("{:?}", c1);

    // Since TryFrom is implemented for Color, we should be able to use TryInto
    let c2: Result<Color, _> = [183, 65, 14].try_into();
    println!("{:?}", c2);

    let v = vec![183, 65, 14];
    // With slice we should use `try_from` function
    let c3 = Color::try_from(&v[..]);
    println!("{:?}", c3);
    // or take slice within round brackets and use TryInto
    let c4: Result<Color, _> = (&v[..]).try_into();
    println!("{:?}", c4);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple_out_of_range_positive() {
        assert_eq!(
            Color::try_from((256, 1000, 10000)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_out_of_range_neggetive() {
        assert_eq!(
            Color::try_from((-1, -10, -256)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_sum() {
        assert_eq!(
            Color::try_from((-1, 255, 255)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_correct() {
        let c: Result<Color, _> = (183, 65, 14).try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_array_out_of_range_positive() {
        let c: Result<Color, _> = [1000, 10000, 256].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_out_of_range_neggetive() {
        let c: Result<Color, _> = [-10, -256, -1].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_sum() {
        let c: Result<Color, _> = [-1, 255, 255].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_correct() {
        let c: Result<Color, _> = [183, 65, 14].try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_slice_out_of_range_positive() {
        let arr = [10000, 256, 1000];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_out_of_range_neggetive() {
        let arr = [-256, -1, -10];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_sum() {
        let arr = [-1, 255, 255];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_correct() {
        let v = vec![183, 65, 14];
        let c: Result<Color, _> = Color::try_from(&v[..]);
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_slice_excess_length() {
        let v = vec![0, 0, 0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(IntoColorError::BadLen));
    }
    #[test]
    fn test_slice_insufficient_length() {
        let v = vec![0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(IntoColorError::BadLen));
    }
}
