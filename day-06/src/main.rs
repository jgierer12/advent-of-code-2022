use utils::read_input_file;

fn main() {
    let input = read_input_file();
    let input = input.trim();

    let mut result_one: Option<usize> = None;
    let mut result_two: Option<usize> = None;
    let mut last_dupe: usize = 0;
    for (i, char) in input.chars().enumerate() {
        if let Some(it) = &input[0..i].rfind(char) {
            last_dupe = last_dupe.max(*it);
        };

        if i - last_dupe >= 4 && result_one.is_none() {
            result_one = Some(i + 1);
        }
        if i - last_dupe >= 14 && result_two.is_none() {
            result_two = Some(i + 1);
            break;
        }
    }

    println!("Part one result: {:?}", result_one);
    println!("Part two result: {:?}", result_two);
}
