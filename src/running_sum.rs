#[allow(dead_code)]
struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
        if nums.is_empty() { return nums }

        for i in 1..nums.len() {
            nums[i] = nums[i] + nums[i - 1];
        }

        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![2,7,11,15];
        let target = vec![2, 9, 20, 35];
        assert_eq!(Solution::running_sum(nums), target)
    }
}