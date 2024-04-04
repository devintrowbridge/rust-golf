pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut k = 0; // idx of non-zero array
    let mut num_zeros = 0;
    let l = nums.len();
    for i in 0..l {
        if nums[i] != 0 {
            nums[k] = nums[i];
            k += 1;
        } else {
            num_zeros += 1;
        }
    }

    for i in 0..num_zeros {
        nums[(l - 1) - i] = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut nums = vec![0,1,0,3,12];
        move_zeroes(&mut nums);
        assert_eq!(nums,  vec![1,3,12,0,0]);

        let mut nums = vec![0,1,0];
        move_zeroes(&mut nums);
        assert_eq!(nums,  vec![1,0,0]);
    }
}