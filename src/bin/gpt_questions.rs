// ============================================================
// EXERCISE 1 — OWNERSHIP TRANSFER
// ============================================================

fn print_text(s: String) {
    println!("{}", s);
}

fn exercise_1() {
    let string_var = String::from("Hello Rust");
    print_text(string_var);

    // This would cause an error because ownership
    // has moved to print_text():
    //
    // println!("{}", string_var);
}


// ============================================================
// EXERCISE 2 — BORROWING
// ============================================================

fn print_length(s: &String) -> u32 {
    let mut count: u32 = 0;
    let len = s.len();

    for _ in 0..len {
        count = count + 1;
    }

    return count;
}

fn exercise_2() {
    let string_var = String::from("Hello Rust");

    println!("{}", print_length(&string_var));
}


// ============================================================
// EXERCISE 3 — MUTABLE BORROWING
// ============================================================

fn add_exclamation(s: &mut String) {
    s.push_str("!");
}

fn exercise_3() {
    let mut string_var = String::from("Hello Rust");

    add_exclamation(&mut string_var);

    println!("{}", string_var);
}


// ============================================================
// EXERCISE 4 — MULTIPLE IMMUTABLE REFERENCES
// ============================================================

fn exercise_4() {
    let string_var = String::from("Rust");

    let a: &String = &string_var;
    let b: &String = &string_var;

    println!("{a}");
    println!("{b}");
    println!("{string_var}");
}


// ============================================================
// EXERCISE 5 — IMMUTABLE + MUTABLE BORROW
// ============================================================

fn exercise_5() {
    let mut string_var = String::from("Rust");

    let a: &String = &string_var;

    // This is intentionally invalid.
    // Rust doesn't allow an immutable and mutable
    // borrow to exist at the same time.
    //
    // let b: &mut String = &mut string_var;

    println!("{a}");

    // If the mutable borrow existed above, this would
    // also be part of the borrowing conflict.
    //
    // println!("{b}");
}


// ============================================================
// MAIN
// ============================================================

fn main() {

    // Run each exercise one at a time.
    //
    // Uncomment the one you want to practice.

    exercise_1();

    // exercise_2();
    // exercise_3();
    // exercise_4();
    // exercise_5();
}