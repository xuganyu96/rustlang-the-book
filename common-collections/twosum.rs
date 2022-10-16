use std::collections::HashMap;

/**
 * Given a list of numbers and a target, return the indices of two distinct
 * elements in the input list that sum to the target. It can be safely assumed
 * that the solution is unique. The order does not matter
 */
fn twosum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut lookup: HashMap<i32, usize> = HashMap::new(); // maps num to loc
    for (loc, &num) in nums.iter().enumerate() {
        lookup.insert(num, loc);
    }

    for (loc, &num) in nums.iter().enumerate() {
        let cmp = target - num;
        match lookup.get(&cmp) {
            Some(&cmploc) => {
                if cmploc != loc {
                    return vec![cmploc as i32, loc as i32];
                }
            },
            None => (),
        }
    }

    return Vec::new();
}

fn main() {
    let nums = vec![2, 7, 11, 15];

    twosum(nums, 9);
}

