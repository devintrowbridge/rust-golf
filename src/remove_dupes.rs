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