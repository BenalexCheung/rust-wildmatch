#[derive(Debug, Clone, PartialEq, Default)]
pub struct WildMatch<'a> {
    pattern: &'a str,
}

impl WildMatch<'_> {
    pub fn new(pattern: &str) -> WildMatch {
        println!("pattern:{}", pattern);
        WildMatch { pattern: pattern }
    }
    pub fn matches(&self, input: &str) -> bool {
        if self.pattern.is_empty() {
            return input.is_empty();
        }

        let mut m = 0;
        for _ in input.chars() {
            m = m + 1;
        }
        let mut n = 0;
        for _ in self.pattern.chars() {
            n = n + 1;
        }

        println!("input:{}", input);
        println!("m:{},n:{}", m, n);

        use ndarray::Array2;
        let mut dp = Array2::zeros((m + 1, n + 1));
        // let mut dp: Vec<usize,usize> = vec![];
        dp[[0, 0]] = 1;
        println!("{:?}", dp);

        let mut index = 1;
        for ch in self.pattern.chars() {
            if ch == '*' {
                dp[[0, index]] = 1;
            } else {
                break;
            }
            index = index + 1;
        }

        let mut i = 0;
        let mut j = 0;
        for input_char in input.chars() {
            i = i + 1;
            j = 0;
            for ch in self.pattern.chars() {
                j = j + 1;
                if ch == '*' {
                    dp[[i, j]] = dp[[i, j - 1]] | dp[[i - 1, j]];
                } else if ch == '?' || input_char == ch {
                    dp[[i, j]] = dp[[i - 1, j - 1]];
                }
            }
        }

        return dp[[i, j]] == 1;
    }
}

#[cfg(test)]
mod tests {
    use super::WildMatch;
    #[test]
    fn it_works() {
        let a = WildMatch::new("*pple?好呀").matches("apple你好呀");
        assert_eq!(a, true);
    }

    #[test]
    fn it_does_not_works() {
        let a = WildMatch::new("*pple?好呀").matches("apple你你好呀");
        assert_ne!(a, true);
    }

    #[test]
    fn it_does_works() {
        let a = WildMatch::new("*pple?好呀").matches("aaaaaaaaaapple你好呀");
        assert_eq!(a, true);
    }
}
