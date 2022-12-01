use utils::read_input_file;

fn main() {
    let input = read_input_file();

    let mut elves = vec![0];
    for line in input.lines() {
        if line == "" {
            elves.push(0);
        } else {
            let calories = line.parse::<i32>().unwrap();
            let last = elves.last_mut().unwrap();
            *last += calories;
        }
    }

    // sort descending
    elves.sort_by(|a, b| b.cmp(a));

    let result = elves[0];
    println!("Part one result: {result}");

    let result: &i32 = &elves[0..3].iter().sum();
    println!("Part two result: {result}");
}
