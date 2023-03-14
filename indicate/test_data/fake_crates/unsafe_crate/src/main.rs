#[cfg(feature = "crazy_unsafe")]
union Kingdom {
    stay: u8,
    leave: u8,
}

#[cfg(feature = "crazy_unsafe")]
fn crazy_unsafe() {
    let mut uk = Kingdom { stay: 100 };

    uk.leave = 1;

    assert!(unsafe { uk.stay } < 50);

    println!("Independence restored!");
}

fn main() {
    let m = std::mem::size_of::<i64>();

    // Call to unsafe function
    let p: *const i64 = unsafe { libc::malloc(m) as *const i64 };

    // `p` is never checked against `libc::PT_NULL` to ensure
    // memory allocation succeeded

    // Unsafe dereference of raw pointer
    println!("Found {} at allocated memory", unsafe { *p });

    // `p` is never freed!

    #[cfg(feature = "crazy_unsafe")]
    crazy_unsafe();
}
