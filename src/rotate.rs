pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let k = (k as usize) % nums.len();
    let split_at = nums.len() - k;

    let mut vec2 = nums.split_off(split_at);
    vec2.append(nums);
    nums.append(&mut vec2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut nums = vec![1,2,3,4,5,6,7];
        let k = 10;
        rotate(&mut nums, k);
        assert_eq!(nums, vec![5,6,7,1,2,3,4])
    }

    #[test]
    fn it_also_works() {
        let mut nums = vec![-1,-100,3,99];
        let k = 2;
        rotate(&mut nums, k);
        assert_eq!(nums, vec![3,99,-1,-100])
    }

    #[test]
    fn it_also_works_again() {
        let mut nums = vec![1,2];
        let k = 1;
        rotate(&mut nums, k);
        assert_eq!(nums, vec![2,1])
    }
}