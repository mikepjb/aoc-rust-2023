fn process_line(line: &str) -> u32 {
    let mut it = (0..line.len()).filter_map(|index| {
        let reduced_line = &line[index..];
        let result = if reduced_line.starts_with("one") {
            Some(1)
        } else if reduced_line.starts_with("two") {
            Some(2)
        } else if reduced_line.starts_with("three") {
            Some(3)
        } else if reduced_line.starts_with("four") {
            Some(4)
        } else if reduced_line.starts_with("five") {
            Some(5)
        } else if reduced_line.starts_with("six") {
            Some(6)
        } else if reduced_line.starts_with("seven") {
            Some(7)
        } else if reduced_line.starts_with("eight") {
            Some(8)
        } else if reduced_line.starts_with("nine") {
            Some(9)
        } else {
            reduced_line.chars().next().unwrap().to_digit(10)
        };

        result
    });
    let first = it.next().expect("should be a number");

    match it.last() {
        Some(num) => first * 10 + num,
        None => first * 10 + first,
    }
}

pub fn process_original(input: &str) -> Result<String, std::io::Error> {
    let output = input
        .lines()
        .map(|line| {
            let line = line.replace("one", "1")
                .replace("two", "2")
                .replace("three", "3")
                .replace("four", "4")
                .replace("five", "5")
                .replace("six", "6")
                .replace("seven", "7")
                .replace("eight", "8")
                .replace("nine", "9");
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

pub fn process(input: &str) -> Result<String, std::io::Error> {
    let output = input
        .lines().map(process_line).sum::<u32>();
    Ok(output.to_string())
}

pub fn main() -> Result<(), std::io::Error> {
    println!("Day 01 Part 01");
    // Day 01 has the same input for both parts
    let d1p1 = include_str!("../../input/d1-p1.txt");
    println!("{}", process_original(d1p1)?);
    println!("Day 01 Part 02");
    println!("{}", process(d1p1)?);
    Ok(())
}

#[test]
fn test_process() -> Result<(), std::io::Error> {
    let test_input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    assert_eq!("142", process_original(test_input)?);

let test_input_2 = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    assert_eq!("281", process(test_input_2)?);

    Ok(())
}
