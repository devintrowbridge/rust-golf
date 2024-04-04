use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut unique = HashSet::new();
    for n in nums {
        if !unique.insert(n) {
            return true;
        }
    }

    false
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(contains_duplicate(vec![1,1,1,3,3,4,3,2,4,2]),  true);
        assert_eq!(contains_duplicate(vec![1,2,3,1]),  true);
        assert_eq!(contains_duplicate(vec![1,2,3,4]),  false);
    }
}