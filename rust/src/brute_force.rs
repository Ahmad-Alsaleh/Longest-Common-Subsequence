pub fn lcs(text1: &str, text2: &str) -> Option<String> {
    let text1_subsequences = generate_subsequences(text1);
    let text2_subsequences = generate_subsequences(text2);

    let mut longest_subsequence = &String::new();

    for subsequence1 in text1_subsequences {
        for subsequence2 in &text2_subsequences {
            if subsequence1 == *subsequence2 && subsequence2.len() > longest_subsequence.len() {
                longest_subsequence = subsequence2;
            }
        }
    }

    if longest_subsequence.is_empty() {
        None
    } else {
        Some(longest_subsequence.clone())
    }
}

fn generate_subsequences(text: &str) -> Vec<String> {
    fn inner_generate_subsequences(
        chars: &[char],
        pos: usize,
        current: &mut String,
    ) -> Vec<String> {
        if pos == chars.len() {
            return vec![current.clone()];
        }
        // option 1: skip this character
        let mut without_curr_char = inner_generate_subsequences(chars, pos + 1, current);

        // option 2: include this character
        // kioiji
        current.push(chars[pos]);
        let mut with_curr_char = inner_generate_subsequences(chars, pos + 1, current);
        current.pop();

        without_curr_char.append(&mut with_curr_char);
        without_curr_char
    }

    inner_generate_subsequences(&text.chars().collect::<Vec<_>>(), 0, &mut String::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subsequence_works() {
        let res = lcs("ACADX", "CBDA");
        assert!(res.is_some());
        assert_eq!(res.unwrap().len(), 2);

        let res = lcs("CGATAATTGAGA", "GTTCCTAATA");
        assert!(res.is_some());
        assert_eq!(res.unwrap().len(), 6);

        let res = lcs("ABC", "XY");
        assert!(res.is_none());
    }
}
