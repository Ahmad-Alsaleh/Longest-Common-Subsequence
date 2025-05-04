use std::cmp::max;

pub fn lcs(text1: &str, text2: &str) -> Option<u32> {
    fn inner_longest_common_subsequence(text1: &[u8], text2: &[u8]) -> u32 {
        if text1.is_empty() || text2.is_empty() {
            return 0;
        };

        let mut dp_table = vec![vec![0; text2.len() + 1]; text1.len() + 1];

        for i in 1..=text1.len() {
            for j in 1..=text2.len() {
                dp_table[i][j] = if text1[i - 1] == text2[j - 1] {
                    dp_table[i - 1][j - 1] + 1
                } else {
                    max(dp_table[i - 1][j], dp_table[i][j - 1])
                };
            }
        }

        *dp_table.last().unwrap().last().unwrap()
    }

    match inner_longest_common_subsequence(text1.as_bytes(), text2.as_bytes()) {
        0 => None,
        value => Some(value),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subsequence_works() {
        let res = lcs("ACADX", "CBDA");
        dbg!(&res);
        assert!(res.is_some());
        assert_eq!(res.unwrap(), 2);
        // assert_eq!(res.unwrap().len(), 2);

        let res = lcs("CGATAATTGAGA", "GTTCCTAATA");
        assert!(res.is_some());
        assert_eq!(res.unwrap(), 6);
        // assert_eq!(res.unwrap().len(), 6);

        let res = lcs("ABC", "XY");
        assert!(res.is_none());
    }
}
