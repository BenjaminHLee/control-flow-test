#![feature(register_tool)]
#![register_tool(paralegal_flow)]

#[paralegal_flow::marker(secret)]
struct PrivateUserData {
    key: String,
}

fn get_user_data() -> PrivateUserData {
    return PrivateUserData {
        key: "1234567890".to_string(),
    }
}

#[paralegal_flow::analyze]
fn main() {
    println!("Hello, world!");
    let user_data = get_user_data();
    println!("encrypt_safe_random: {}", encrypt_safe_random(&user_data));
    println!("encrypt_broken_basic: {}", encrypt_broken_basic(&user_data));
    println!("encrypt_broken_nested_one_level: {}", encrypt_broken_nested_one_level(&user_data));
    println!("encrypt_safe_side_effect: {}", encrypt_safe_side_effect(&user_data));
    // println!("encrypt_broken_side_effect: {}", encrypt_broken_side_effect(&user_data));
    println!("encrypt_safe_random_w_subcalls: {}", encrypt_safe_random_w_subcalls(&user_data));
    println!("encrypt_broken_basic_w_subcalls: {}", encrypt_broken_basic_w_subcalls(&user_data));
    // println!("encrypt_broken_side_effect_w_subcalls: {}", encrypt_broken_side_effect_w_subcalls(&user_data));
    // println!("encrypt_nested_timing: {}", encrypt_nested_timing());
}


// Control flow present, but not contingent upon a secret.
fn encrypt_safe_random(_: &PrivateUserData) -> bool {
    if rand::random() {
        return true;
    }
    return false;
}

// Control flow present, and contingent upon a secret...
// [BUG] ... but ctrl_flow isn't populated!
fn encrypt_broken_basic(user_data: &PrivateUserData) -> bool {
    if user_data.key.len() > 10 {
        return true;
    }
    return false;
}

// See encyrpt_broken_basic
fn encrypt_broken_nested_one_level(user_data: &PrivateUserData) -> bool {
    return encrypt_broken_basic(user_data);
}

// Safe, with a print statement
fn encrypt_safe_side_effect(user_data: &PrivateUserData) -> bool {
    println!("key: {}", user_data.key);
    if rand::random() {
        println!("random returned true!");
        return true;
    }
    return false;
}

// Broken, but with a printline — fails the test as expected!
fn encrypt_broken_side_effect(user_data: &PrivateUserData) -> bool {
    println!("key: {}", user_data.key);
    if user_data.key.len() > 10 {
        println!("key is longer than 10!");
        return true;
    }
    return false;
}

// Trying to see if side effects fix things...

fn print_and_return_true() -> bool {
    println!("print_and_return_true called!");
    return true;
}

fn print_and_return_false() -> bool {
    println!("print_and_return_false called!");
    return false;
}

// OK
fn encrypt_safe_random_w_subcalls(_: &PrivateUserData) -> bool {
    if rand::random() {
        return print_and_return_true();
    }
    return print_and_return_false();
}

// Should be flagged, but is not
fn encrypt_broken_basic_w_subcalls(user_data: &PrivateUserData) -> bool {
    if user_data.key.len() > 10 {
        return print_and_return_true();
    }
    return print_and_return_false();
}

// Is correctly flagged
fn encrypt_broken_side_effect_w_subcalls(user_data: &PrivateUserData) -> bool {
    if user_data.key.len() > 10 {
        println!("key is longer than 10!");
        return print_and_return_true();
    }
    return print_and_return_false();
}

// Is correctly flagged
fn encrypt_nested_timing() -> bool {
    let user_data = get_user_data();
    if rand::random() {
        encrypt_broken_side_effect_w_subcalls(&user_data);
        return true;
    }
    return false;
}
