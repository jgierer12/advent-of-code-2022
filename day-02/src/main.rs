use utils::read_input_file;

fn main() {
    let input = read_input_file();

    let mut score_one = 0;
    let mut score_two = 0;
    for line in input.lines() {
        if line.len() != 3 {
            continue;
        }
        let chars = (line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap());

        score_one += get_score_one(chars);
        score_two += get_score_two(chars);
    }

    println!("Part one result: {score_one}");
    println!("Part two result: {score_two}");
}

fn get_score_one(chars: (char, char)) -> i32 {
    let shape_score = match chars.1 {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => 0,
    };

    let match_score = match chars {
        ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6, // win
        ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3, // tie
        _ => 0,                                    // loss
    };

    shape_score + match_score
}

fn get_score_two(chars: (char, char)) -> i32 {
    let shape_score = match chars {
        ('A', 'Y') | ('B', 'X') | ('C', 'Z') => 1, // X
        ('A', 'Z') | ('B', 'Y') | ('C', 'X') => 2, // Y
        ('A', 'X') | ('B', 'Z') | ('C', 'Y') => 3, // Z
        _ => 0,
    };

    let match_score = match chars.1 {
        'Y' => 3,
        'Z' => 6,
        _ => 0,
    };

    shape_score + match_score
}
