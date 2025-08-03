pub fn edit_distance(source: &str, target: &str) -> usize {
    let m = source.len();
    let n = target.len();
    let source_bytes = source.as_bytes();
    let target_bytes = target.as_bytes();

    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 0..=m {
        dp[i][0] = i;
    }

    for j in 0..=n {
        dp[0][j] = j;
    }

    for i in 1..=m {
        for j in 1..=n {
            if source_bytes[i - 1] == target_bytes[j - 1] {
                dp[i][j] = dp[i - 1][j - 1]; 
            } else {
                let insert = dp[i][j - 1] + 1;
                let delete = dp[i - 1][j] + 1;
                let replace = dp[i - 1][j - 1] + 1;
                
                let mut min = insert;
                if delete < min {
                    min = delete;
                }
                if replace < min {
                    min = replace;
                }

                dp[i][j] = min;
            }
        }
    }

    dp[m][n]
}
