//! A library aiming at facilitating handling of large amounts of data.

#![deny(missing_docs)]

pub mod entry;
pub mod series;
pub mod dataframe;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
