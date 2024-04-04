// Given a non-empty array of integers nums, every element appears twice except for one. Find that single one.
// You must implement a solution with a linear runtime complexity and use only constant extra space.

// Example 1:
// Input: nums = [2,2,1]
// Output: 1

// Example 2:
// Input: nums = [4,1,2,1,2]
// Output: 4

pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut ans = 0x0;
    for n in nums {
        ans ^= n; 
    }
    ans
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(single_number(vec![2,1,2]),  1);
        assert_eq!(single_number(vec![1,2,4,1,2]),  4);
        assert_eq!(single_number(vec![1]),  1);
    }
}