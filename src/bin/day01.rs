// TODO use include_str!("relative.txt") for importing inputs

pub fn process(input: &str) -> Result<String, std::io::Error> {
    let output = input
        .lines()
        .map(|line| {
            let mut it = line.chars();

            let first = it
            .find_map(|c| { c.to_digit(10) })
            .expect("should be a number");

            let last = it
                .rfind(|c| { c.is_ascii_digit() })
                .map(|c| c.to_digit(10).unwrap())
                .unwrap_or(first);

            first * 10 + last
        }).sum::<u32>();
    Ok(output.to_string())
}

pub fn main() -> Result<(), std::io::Error> {
    println!("Day 01");
    let d1p1 = include_str!("../../input/d1-p1.txt");
    println!("{}", process(d1p1)?);
    Ok(())
}

#[test]
fn test_process() -> Result<(), std::io::Error> {
    let test_input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    assert_eq!("142", process(test_input)?);
    Ok(())
}
