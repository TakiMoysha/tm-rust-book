fn expensive_check() -> bool {
    println!("expensive check");
    return true;
}

// check only in debug mode
#[cfg(debug_assertions)]
fn another_expensive_check() -> bool {
    println!("Thoroughly performing some other expensive check");
    return true;
}

// check only in release mode
#[cfg(not(debug_assertions))]
fn another_expensive_check() -> bool {
    println!("Only superficially performing some other expensive  check");
    false
}

fn main() {
    println!("Release Checks:");
    assert!(expensive_check());
    assert!(
        expensive_check(),
        "Expensive check failed in Release Build!"
    );
    assert_eq!(expensive_check(), true);
    assert_ne!(expensive_check(), false);

    #[cfg(debug_assertions)]
    {
        println!("Debug Checks:");
    }

    debug_assert!(expensive_check());
    debug_assert!(expensive_check(), "Expensive check failed in Debug Build!");
    debug_assert_eq!(expensive_check(), true);
    debug_assert_ne!(expensive_check(), false);

    println!("Checks from conditional functions:");
    assert!(another_expensive_check());

    println!("All checks passed");
}
