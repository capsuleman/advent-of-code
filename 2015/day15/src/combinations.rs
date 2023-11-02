#[derive(Debug)]
pub struct CombinationsRestrictedBySum {
    size: usize,
    sum_value: u32,

    current_value: u32,
    sub_combinations: Option<Box<Self>>,
}

impl CombinationsRestrictedBySum {
    pub fn new(size: usize, sum_value: u32) -> CombinationsRestrictedBySum {
        let mut current_sub_combinations = CombinationsRestrictedBySum {
            size: 1,
            sum_value,
            current_value: sum_value,
            sub_combinations: None,
        };

        for sub_size in 2..size + 1 {
            current_sub_combinations = CombinationsRestrictedBySum {
                size: sub_size,
                sum_value,
                current_value: 0,
                sub_combinations: Some(Box::new(current_sub_combinations)),
            };
        }

        current_sub_combinations
    }
}

impl Iterator for CombinationsRestrictedBySum {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Vec<u32>> {
        if self.size == 1 {
            if self.current_value > self.sum_value {
                return None;
            }

            let current_value = self.current_value;
            self.current_value += 1;
            return Some(vec![current_value]);
        };

        let sub_combinations = self.sub_combinations.as_mut().unwrap();
        if let Some(mut next_sub_combination_value) = sub_combinations.next() {
            next_sub_combination_value.push(self.current_value);
            Some(next_sub_combination_value)
        } else {
            if self.current_value == self.sum_value {
                return None;
            }

            self.current_value += 1;
            self.sub_combinations = Some(Box::new(Self::new(
                self.size - 1,
                self.sum_value - self.current_value,
            )));

            let mut next_sub_combination_value = self
                .sub_combinations
                .as_mut()
                .expect("to have a sub combination for size > 1")
                .next()
                .expect("to have at least one element in new sub combination");
            next_sub_combination_value.push(self.current_value);
            Some(next_sub_combination_value)
        }
    }
}
