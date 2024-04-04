// Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place such that each unique element appears only once. The relative order of the elements should be kept the same. Then return the number of unique elements in nums.
// Consider the number of unique elements of nums to be k, to get accepted, you need to do the following things:
// Change the array nums such that the first k elements of nums contain the unique elements in the order they were present in nums initially. The remaining elements of nums are not important as well as the size of nums.
// Return k.

// Example
// Input: nums = [0,0,1,1,1,2,2,3,3,4]
// Output: 5, nums = [0,1,2,3,4,_,_,_,_,_]
// Explanation: Your function should return k = 5, with the first five elements of nums being 0, 1, 2, 3, and 4 respectively.
// It does not matter what you leave beyond the returned k (hence they are underscores).

pub fn find_smallest(nums: &Vec<i32>, min: i32) -> Option<i32> {
    for num in nums {
        if *num > min {
            return Some(*num);
        }
    }

    None
}

pub fn remove_duplicates(nums: &mut Vec<i32>) -> usize {
    let mut idx = 1;
    let mut highest = nums[0];
    
    for _ in 1..nums.len() {
        let smallest = find_smallest(nums, highest);
        if let Some(smallest) = smallest {
            if smallest > highest {
                nums[idx] = smallest;
                highest = smallest;
                idx += 1;
            }
        } 
    }

    idx
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut nums = vec![0,0,1,1,1,2,2,3,3,4]; // Input array
        let expected_nums = [0,1,2,3,4]; // The expected answer with correct length

        let k = remove_duplicates(&mut nums); // Calls your implementation

        assert_eq!(k, expected_nums.len(), "HELLO {nums:?} {expected_nums:?}");
        for i in 0..k {
            assert_eq!(nums[i], expected_nums[i], "HELLO {nums:?} {expected_nums:?}");
        }
    }
}