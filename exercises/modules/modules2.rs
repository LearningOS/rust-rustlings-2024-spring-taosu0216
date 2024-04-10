// modules2.rs
//
// You can bring module paths into scopes and provide new names for them with
// the 'use' and 'as' keywords. Fix these 'use' statements to make the code
// compile.
//
// Execute `rustlings hint modules2` or use the `hint` watch subcommand for a
// hint.


pub mod delicious_snacks {
    // TODO: Fix these use statements
    // use self::fruits::PEAR as fruit;
    // use self::veggies::CUCUMBER as veggie;

    pub mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    pub mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    use delicious_snacks::fruits::PEAR as fruit;
    use delicious_snacks::veggies::CUCUMBER as veggie;
    println!(
        "favorite snacks: {} and {}",
        fruit,
        veggie
    );
}
