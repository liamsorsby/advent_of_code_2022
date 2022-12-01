use std::fs;


fn main() {
    let mut data: Vec<i64> = Vec::new();

    let content = fs::read_to_string("./inputs/day1_part1.txt")
        .expect("unable to find file");

    for item in content.split("\n\n") {
        let test: Vec<i64> = item.split_whitespace()
            .map(|s| -> i64 { s.parse().ok().iter().sum() })
            .collect();

        data.push(test.into_iter().sum::<i64>());
    }

    data.sort_by(|a, b| b.cmp(a)); // reverse sort

    println!("Results: {:?}", data);

    println!("Elf carrying the most Calories: {}", data.first().unwrap());
    println!("Top 3 Elves carrying the most calories: {}", data.get(..3).map(|s| s.iter().sum::<i64>()).unwrap());
}
