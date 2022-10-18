pub fn crash_and_burn() {
    panic!("Crash and burn!");
}

pub fn bad_index() {
    let nums = vec![0, 1, 2, 3];
    println!("{}", nums[4]);
}

