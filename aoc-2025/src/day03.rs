use anyhow::Result;

pub fn p1(input_text: &str) -> Result<i64> {
    let mut total = 0;
    Ok(total)
}

pub fn p2(input_text: &str) -> Result<i64> {
    let mut total = 0;
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_p1_example() {
        let result = p1(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn test_p2_example() {
        let result = p2(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, 4174379265);
    }
}
