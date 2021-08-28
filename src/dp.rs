use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DP {
    array2: Vec<Vec<i32>>,
}

impl fmt::Display for DP {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use std::fmt::Write;

        for array in &self.array2 {
            f.write_fmt(format_args!("{:?}", array))?;
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl DP {
    pub fn new(x: usize, y: usize) -> DP {
        let array2 = vec![vec![0; y]; x];
        DP { array2 }
    }
}

impl Index<[usize; 2]> for DP {
    type Output = i32;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.array2[index[0]][index[1]]
    }
}

impl IndexMut<[usize; 2]> for DP {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        &mut self.array2[index[0]][index[1]]
    }
}

#[cfg(test)]
mod tests {
    use super::DP;

    #[test]
    fn it_works_dp() {
        let mut a = DP::new(10, 12);
        assert_ne!(a[[1, 2]], 2);
        a[[1, 2]] = 2;
        assert_eq!(a[[1, 2]], 2);
    }
}
