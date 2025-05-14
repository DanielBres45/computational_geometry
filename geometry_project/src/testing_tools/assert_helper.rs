use std::fmt::{Debug, Display};

pub struct AssertHelper {}

impl AssertHelper {
    pub fn option_equal_debug<T>(expected: Option<T>, actual: Option<T>)
    where
        T: Debug + Display + PartialEq,
    {
        match (expected, actual) {
            (Some(e), Some(a)) => {
                assert_eq!(e, a, "Expected And Actual both Some() but values differ");
            }
            (None, Some(a)) => {
                panic!("Expected was None, but Actual was Some({})", a);
            }
            (Some(e), None) => {
                panic!("Expected was Some({}), but actual was None", e);
            }
            (None, None) => {}
        }
    }

    pub fn option_equal_vec<T>(expected: Option<Vec<T>>, actual: Option<Vec<T>>)
    where
        T: Debug + Display + PartialEq,
    {
        match (expected, actual) {
            (Some(e), Some(a)) => {
                Self::assert_vec_equal(e, a);
            }
            (None, Some(a)) => {
                panic!("Expected was None, but actual had {} items ", a.len());
            }
            (Some(e), None) => {
                panic!("Expected vec with {} items, but actual was None", e.len())
            }
            (None, None) => {}
        }
    }

    pub fn assert_vec_equal<T>(expected: Vec<T>, actual: Vec<T>)
    where
        T: Debug + Display + PartialEq,
    {
        let msg: String = format!(
            "Expected was: [{}] but actual is: [{}]",
            itertools::join(&expected, ", "),
            itertools::join(&actual, ", ")
        );

        if expected.len() != actual.len() {
            panic!("{}", msg);
        }

        for i in 0..expected.len() {
            assert_eq!(expected[i], actual[i], "{}", msg);
        }
    }
}
