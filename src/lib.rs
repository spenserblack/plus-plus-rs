//! `++` and `--` for Rust.
//!
//! # `++`
//!
//! ```
//! # use plus_plus::ppmm;
//! let mut x = 0;
//!
//! assert_eq!(ppmm!(x++), 0);
//! assert_eq!(x, 1);
//! assert_eq!(ppmm!(++x), 2);
//! assert_eq!(x, 2);
//! ```
//!
//! # `--`
//!
//! ```
//! # use plus_plus::ppmm;
//! let mut x = 10;
//!
//! assert_eq!(ppmm!(x--), 10);
//! assert_eq!(x, 9);
//! assert_eq!(ppmm!(--x), 8);
//! assert_eq!(x, 8);
//! ```

#[macro_export]
macro_rules! ppmm {
    (++ $n:ident) => {
        {
            $n += 1;
            $n
        }
    };
    (-- $n:ident) => {
        {
            $n -= 1;
            $n
        }
    };
    ($n:ident ++) => {
        {
            let temp = $n;
            $n += 1;
            temp
        }
    };
    ($n:ident --) => {
        {
            let temp = $n;
            $n -= 1;
            temp
        }
    };
}
