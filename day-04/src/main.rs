use regex::Regex;
use utils::read_input_file;

#[derive(Debug)]
struct Range {
    start: u32,
    end: u32,
}
impl Range {
    fn from_str(start: &str, end: &str) -> Self {
        Self {
            start: start.parse().expect("start should be a number"),
            end: end.parse().expect("end should be a number"),
        }
    }

    fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Range) -> bool {
        !(self.start > other.end || self.end < other.start)
    }
}

fn main() {
    let input = read_input_file();

    let mut result_one = 0;
    let mut result_two = 0;
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    for cap in re.captures_iter(&input) {
        let a = Range::from_str(&cap[1], &cap[2]);
        let b = Range::from_str(&cap[3], &cap[4]);

        if a.contains(&b) || b.contains(&a) {
            result_one += 1;
        }

        if a.overlaps(&b) {
            result_two += 1;
        }
    }

    println!("Part one result: {:?}", result_one);
    println!("Part two result: {:?}", result_two);
}
