pub fn sum_base(n: i32, k: i32) -> i32 {
    let mut result = n;
    let mut ans = 0;
    
    loop {
        ans += result % k;
        result = result / k; 
        println!("ans {ans}, result {result}");
        if result <= 0 { break; }
    } 

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works1() {
        assert_eq!(sum_base(34, 6), 9);
    }

    #[test]
    fn it_works2() {
        assert_eq!(sum_base(10, 10), 1);
    }

    #[test]
    fn it_works3() {
        assert_eq!(sum_base(42, 2), 3);
    }
}