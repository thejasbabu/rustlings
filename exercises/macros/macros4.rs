// macros4.rs
// Make me compile! Execute `rustlings hint macros4` for hints :)

// I AM NOT DONE

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
    ($val1:expr, $val2:expr) => {
        println!("Look at this other macro: {}, {}", $val1, $val2);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
    my_macro!(7, 7);
}
