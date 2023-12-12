#![cfg(test)] // only compile when running tests
mod tests {
    extern crate phrases;

    #[test]
    #[should_panic] // negative test case, expect panic/error
    fn english_greeting_correct() {
        assert_eq!("Hell!", phrases::greetings::english::hello());
    }
}

