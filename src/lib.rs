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
    (++ $n:expr) => {
        n += 1;
        n
    };
    (-- $n:expr) => {
        n -= 1;
        n
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
