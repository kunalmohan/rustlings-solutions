// test4.rs
// This test covers the sections:
// - Modules
// - Macros

// Write a macro that passes the test! No hints this time, you can do it!



mod macros {
	#[macro_export]
	macro_rules! my_macro {
		($a:expr) => {
			format!("Hello {}", $a)
		}
	}
}

fn main() {
    if my_macro!("world!") != "Hello world!" {
        panic!("Oh no! Wrong output!");
    }
}