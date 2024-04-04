use std::collections::VecDeque;
pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut carry = true;
    let mut ans = VecDeque::from(digits);

    for n in ans.iter_mut().rev() {
        if carry {
            *n = (*n + 1) % 10;
            carry = *n == 0;
        }
    }
    if carry {
        ans.push_front(1);
    }

    ans.into()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(plus_one(vec![1,2,3]),  vec![1,2,4]);
        assert_eq!(plus_one(vec![4,3,2,1]), vec![4,3,2,2]);
        assert_eq!(plus_one(vec![9]), vec![1,0]);
        assert_eq!(plus_one(vec![9,8,7,6,5,4,3,2,1,0]), vec![9,8,7,6,5,4,3,2,1,1]);
    }
}