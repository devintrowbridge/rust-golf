pub fn largest_integer(num: i32) -> i32 {
    let mut vec = Vec::new();
    
    // split number into digits in an array
    let mut result = num;
    while result > 0 {
        vec.push(result % 10);
        result /= 10;
    }   

    // Do the actual swaps
    for i in 0..vec.len() {
        for j in i+1..vec.len() {
            if vec[i] % 2 == vec[j] % 2 && vec[i] > vec[j] {
                vec.swap(i, j);
            }
        }
    }

    // convert back to a number
    vec.iter().enumerate().fold(0, |acc, (index, x)| { acc + (x * 10_i32.pow(index as u32)) })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(largest_integer(1234),  3412);
        assert_eq!(largest_integer(65875), 87655);
    }
}