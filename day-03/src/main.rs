use utils::read_input_file;

fn priority(char: char) -> usize {
    const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let index = ALPHABET
        .chars()
        .position(|c| c == char)
        .expect("character should be in alphabet");
    index + 1
}

fn find_duplicate(strings: Vec<&str>) -> char {
    for char in strings[0].chars() {
        if strings[1..].iter().all(|s| s.contains(char)) {
            return char;
        }
    }
    panic!("strings should contain a duplicate character");
}

fn main() {
    let input = read_input_file();

    let lines: Vec<&str> = input.lines().collect();
    let groups = lines.chunks(3);

    let mut result_one = 0;
    let mut result_two = 0;
    for group in groups {
        for line in group {
            let split_index = line.len() / 2;
            let strings = vec![&line[..split_index], &line[split_index..]];
            let duplicate = priority(find_duplicate(strings));
            result_one += duplicate;
        }

        let badge = priority(find_duplicate(group.to_vec()));
        result_two += badge;
    }

    println!("Part one result: {:?}", result_one);
    println!("Part two result: {:?}", result_two);
}
