pub struct SparseTable {
    pub dp: Vec<Vec<i32>>,
    op: fn(i32, i32) -> i32,
}
impl SparseTable {
    pub fn init(v: &Vec<i32>, op: fn(i32, i32) -> i32) -> Self {
        let len = v.len();
        let wid = (v.len() as f64).log2().ceil() as usize + 1;
        let mut dp = vec![vec![0; wid]; len];
        for i in 0..len {
            dp[i][0] = v[i];
        }
        for j in 1..wid {
            for i in 0..len {
                if i + (1 << j) - 1 < len {
                    dp[i][j] = op(dp[i][j - 1], dp[i + (1 << (j - 1))][j - 1]);
                }
            }
        }
        SparseTable { dp, op }
    }
    pub fn query(&self, l: usize, r: usize) -> i32 {
        if l > r || l > self.dp.len() || r > self.dp.len() {
            panic!("Array Index Out Of Bounds Exception!!")
        }
        if l == r {
            self.dp[l][0]
        } else {
            let s = ((r - l + 1) as f64).log2().ceil() as usize - 1;
            (self.op)(self.dp[l][s], self.dp[r - (1 << s) + 1][s])
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::max;
    use super::*;

    #[test]
    fn it_works() {
        let mut v = vec![0i32; 100];
        for i in 0..100 {
            v[i] = i as i32;
        }
        let spt = SparseTable::init(&v, max);
        for i in 0..100 {
            for j in i..100 {
                assert_eq!(spt.query(i, j), j as i32);
            }
        }
    }
}
