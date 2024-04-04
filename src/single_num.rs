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