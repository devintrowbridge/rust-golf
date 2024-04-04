pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut ans = Vec::new();
    
    for (ipos, i) in nums.iter().enumerate() {
        for jpos in (ipos+1)..nums.len() {
            if i + nums.get(jpos).unwrap() == target {
                ans.push(ipos as i32);
                ans.push(jpos as i32);
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![2,7,11,15];
        let target = 9;
        assert_eq!(two_sum(nums, target), vec![0, 1])
    }
}