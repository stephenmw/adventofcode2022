use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let numbers = parse!(input);
    let mut list = List::new(numbers);
    list.mix();

    Ok(list.coordinate()?.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    const DECRIPTION_KEY: isize = 811589153;
    let numbers = parse!(input);
    let mut list = List::new(numbers.iter().copied().map(|x| x * DECRIPTION_KEY));
    for _ in 0..10 {
        list.mix();
    }

    Ok(list.coordinate()?.to_string())
}

struct List {
    nums: Vec<(usize, isize)>,
    index: Vec<usize>,
}

impl List {
    fn new(numbers: impl IntoIterator<Item = isize>) -> Self {
        let nums: Vec<_> = numbers.into_iter().enumerate().collect();
        let index = (0..nums.len()).collect();
        List { nums, index }
    }

    fn mix(&mut self) {
        for i in 0..self.nums.len() {
            self.move_number(i);
        }
    }

    fn move_number(&mut self, orig_index: usize) {
        let index = self.index[orig_index];
        let value = self.nums.remove(index);
        let new_index = add_offset(index, value.1, self.nums.len());

        if new_index == self.nums.len() {
            self.nums.push(value)
        } else {
            self.nums.insert(new_index, value);
        }

        if new_index > index {
            for i in index..=new_index {
                self.index[self.nums[i].0] = i;
            }
        } else {
            for i in new_index..=index {
                self.index[self.nums[i].0] = i;
            }
        }
    }

    fn get(&self, index: usize) -> isize {
        self.nums[index].1
    }

    fn coordinate(&self) -> anyhow::Result<isize> {
        let zero_index = self
            .nums
            .iter()
            .position(|x| x.1 == 0)
            .ok_or(anyhow!("value 0 not found"))?;

        let m = self.nums.len();

        let coordinate = self.get((zero_index + 1000) % m)
            + self.get((zero_index + 2000) % m)
            + self.get((zero_index + 3000) % m);

        Ok(coordinate)
    }
}

fn add_offset(a: usize, b: isize, modulus: usize) -> usize {
    let x = (a as isize + b) % modulus as isize;
    if x >= 0 {
        x as usize
    } else {
        (modulus as isize + x) as usize
    }
}

mod parser {
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<isize>> {
        ws_all_consuming(many1(ws_line(int)))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "
        1
        2
        -3
        3
        -2
        0
        4
    ";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "3")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "1623178306")
    }
}
