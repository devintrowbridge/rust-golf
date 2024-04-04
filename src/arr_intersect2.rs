use std::collections::HashMap;

// O(nums1 + nums2) time complexity
// Space complexity is O(nums1) in the worst case where every element in nums1 is unique
pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut nums1_cnt: HashMap<i32, i32> = HashMap::new();
    let mut ans = Vec::new();

    for n in nums1 {
        let mut cnt = *nums1_cnt.get(&n).unwrap_or(&0);
        cnt += 1;
        nums1_cnt.insert(n, cnt);
    }

    for n in nums2 {
        if let Some( v) = nums1_cnt.get_mut(&n) {
            if *v > 0 {
                *v -= 1;
                ans.push(n);
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
        assert_eq!(intersect(vec![1,2,2,1], vec![2,2]), vec![2,2]);
        assert_eq!(intersect(vec![4,9,5], vec![9,4,9,8,4]), vec![9,4]);
    }
}