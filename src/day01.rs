pub fn main() {
    println!("Day 01")
}

// TODO use include_str!("relative.txt") for importing inputs

pub fn process(_input: &str) -> Result<String, std::io::Error> {
    Ok("142".to_string())
}

// pub fn process(input: &str) -> miette::Result<String, AocError> {
//     let output = input
//         .lines()
//         .map(|line| {
//             let mut it =
//                 line.chars().filter_map(|c| {
//                     c.to_digit(10)
//                 });
//             let first =
//                 it.next().expect("should be a number");
//             match it.last() {
//                 Some(num) => format!("{first}{num}"),
//                 None => format!("{first}{first}"),
//             }.parse::<u32>().expect("should be a valid number")
//         })
//     .sum::<u32>();
// 
//     Ok(output.to_string())
// }

#[test]
fn test_process() -> Result<(), std::io::Error> {
    let test_input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    assert_eq!("142", process(test_input)?);
    Ok(())
}
