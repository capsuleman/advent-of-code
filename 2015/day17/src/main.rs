use std::collections::VecDeque;

fn main() {
    println!(
        "{:?}",
        get_combination_count(25, VecDeque::from([20, 15, 10, 5, 5]))
    );

    println!(
        "{:?}",
        get_combination_count(
            150,
            VecDeque::from([
                50, 49, 47, 46, 44, 43, 42, 40, 40, 36, 32, 26, 24, 22, 21, 18, 18, 11, 10, 7,
            ])
        )
    );
}

fn get_combination_count(target_size: u32, containers_size: VecDeque<u32>) -> usize {
    let combinations = get_combinations(target_size, 0, containers_size);

    let minimum_container_count = combinations
        .iter()
        .min_by_key(|combination| combination.len())
        .unwrap()
        .len();

    combinations
        .iter()
        .filter(|combination| combination.len() == minimum_container_count)
        .count()
}

fn get_combinations(
    target_size: u32,
    current_size: u32,
    mut containers_size: VecDeque<u32>,
) -> Vec<Vec<u32>> {
    if target_size == current_size {
        return vec![vec![]];
    }

    let mut new_combinations = vec![];

    while let Some(container_size) = containers_size.pop_front() {
        if current_size + container_size > target_size {
            continue;
        }

        let sub_container_sizes = containers_size.clone();

        let sub_combinations = get_combinations(
            target_size,
            current_size + container_size,
            sub_container_sizes,
        );

        for mut sub_combination in sub_combinations {
            sub_combination.push(container_size);
            new_combinations.push(sub_combination);
        }
    }

    new_combinations
}
