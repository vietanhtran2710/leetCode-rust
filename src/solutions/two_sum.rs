use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut checked: HashMap<i32, i32> = HashMap::new();
    for index in 0..nums.len() {
        if checked.contains_key(&(target - nums[index])) {
            return vec![
                index.try_into().unwrap(),
                *checked.get(&(target - nums[index])).unwrap(),
            ];
        }
        checked.insert(nums[index], index as i32);
    }
    return vec![];
}
