//! This is a library crate named `phrases`.
//! 
//! # Examples
//! let en = phrases::greetings::english::hello();
//! println!("{}", en);

/*
    * This is a library crate named `phrases`.
    * It contains two modules: `greetings` and `farewells`.
    * Each module contains two functions: `hello` and `goodbye`.
    * The functions in `greetings` return English phrases.
    * The functions in `farewells` return French phrases.
*/

pub mod greetings {
    pub mod english {
        /// Applies to the next item
        /// Generates a 'Hello!' string.
        pub fn hello() -> String { "Hello!".to_string() }


        pub fn goodbye() -> String { "Goodbye.".to_string() }
    }

    pub mod french {
        pub fn hello() -> String { "Bonjour.".to_string() }
        pub fn goodbye() -> String { "Au revoir.".to_string() }
    }
}
