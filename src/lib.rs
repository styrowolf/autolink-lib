mod private;

pub use private::{Plan, TimeDay};
pub use private::{import, export, check, check_all, open_link};

pub extern crate chrono;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
