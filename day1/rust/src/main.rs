use std::fs;

fn main() {
    let values = get_input();
    println!("{}", count_increasing(offset(&values)));
    println!("{}", count_increasing(offset(&chunks(values))));
}

fn get_input() -> Vec<u16> {
    let file = fs::read_to_string("input").unwrap();
    let values = file
        .split("\n")
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<u16>().unwrap())
        .collect::<Vec<u16>>();
    values
}

fn chunks(values: Vec<u16>) -> Vec<u16> {
    let mut chunks = Vec::with_capacity(values.len());
    let mut i = 0;

    while i < values.len() - 2 {
        chunks.push(values[i] + values[i + 1] + values[i + 2]);
        i += 1;
    }

    chunks
}

fn count_increasing(values: Vec<u16>) -> usize {
    let values_offset = offset(&values);
    let zipped = values.iter().zip(values_offset.clone());
    let is_increasing = zipped.map(|(a, b)| a > &b);
    is_increasing
        .clone()
        .filter(|a| *a == true)
        .collect::<Vec<bool>>()
        .len()
}

fn offset(values: &Vec<u16>) -> Vec<u16> {
    let peek = values.iter().next().unwrap();
    let offset = std::iter::once(peek + 1);
    let values_offset = offset.chain(values.clone());
    values_offset.collect()
}
