#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod twain_dsm;

pub use twain_dsm::TwainDsm;

fn main<'a>() -> &'a str {
    "Hello world"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!("Hello world", main())
    }
}
