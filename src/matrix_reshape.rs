use std::collections::VecDeque;

#[allow(dead_code)]
pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
    let mut ans = Vec::new();
    
    // Flatten matrix into 1d queue
    let mut flat = VecDeque::new();
    for row in &mat {
        for col in row {
            flat.push_back(*col);
        }
    }
    if r * c != flat.len() as i32 { return mat }

    // Convert 1d queue to r x c matrix
    for _ in 0..r {
        let mut row = Vec::new();
        for _ in 0..c {
            row.push(flat.pop_front().unwrap())
        }
        ans.push(row);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mat = vec![vec![1,2],vec![3,4]];
        let r = 1;
        let c = 4;
        let target = vec![vec![1,2,3,4]];
        assert_eq!(matrix_reshape(mat, r, c), target)
    }

    #[test]
    fn reject() {
        let mat = vec![vec![1,2],vec![3,4]];
        let r = 2;
        let c = 4;
        let target = vec![vec![1,2],vec![3,4]];
        assert_eq!(matrix_reshape(mat, r, c), target)
    }
}