extern crate megam_rustyprint;


#[macro_use]
extern crate log;

//mod support;
macro_rules! test {
    ($name:ident $expr:expr) => (
        #[test]
        fn $name() {
/*            ::support::paths::setup();
            setup();
*/            $expr;
        }
    )
}

mod test_print;
