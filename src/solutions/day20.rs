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
}

impl List {
    fn new(numbers: impl IntoIterator<Item = isize>) -> Self {
        let nums: Vec<_> = numbers.into_iter().enumerate().collect();
        List { nums }
    }

    fn mix(&mut self) {
        for i in 0..self.nums.len() {
            self.move_number(i);
        }
    }

    fn move_number(&mut self, orig_index: usize) {
        let index = self.find(orig_index);
        let value = self.nums[index];
        let new_index = add_offset(index, value.1, self.nums.len() - 1);

        if index < new_index {
            self.nums.copy_within((index + 1)..=new_index, index);
        } else {
            self.nums.copy_within(new_index..index, new_index + 1);
        }

        self.nums[new_index] = value;
    }

    fn find(&self, orig_index: usize) -> usize {
        self.nums
            .iter()
            .map(|(i, _)| *i)
            .enumerate()
            .find(|(_, i)| *i == orig_index)
            .unwrap()
            .0
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
