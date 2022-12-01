use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day-01.txt").unwrap_or_default();
    println!("Part 1: {}", max_calories(&input));
    println!("Part 2: {}", top(&input, 3));
}

fn elf_calories<'a>(calories: &'a str) -> impl Iterator<Item = usize> + 'a {
    calories.split("\n\n").map(|calories_list| {
        calories_list
            .lines()
            .map(|cal| cal.trim())
            .filter_map(|cal| cal.parse::<usize>().ok())
            .sum::<usize>()
    })
}

fn max_calories(calories: &str) -> usize {
    elf_calories(calories).max().unwrap_or_default()
}

fn top(calories: &str, n: usize) -> usize {
    elf_calories(calories)
        .fold(Vec::with_capacity(n), |mut top, elf| {
            top.push(elf);
            top.sort();
            top.reverse();
            top.resize(n, 0);
            top
        })
        .into_iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_file() {
        assert_eq!(max_calories(""), 0);
    }

    #[test]
    fn test_ignores_invalid_values() {
        let cals = "
            10
            20
            lol
            30
        ";
        assert_eq!(max_calories(cals), 60);
    }

    #[test]
    fn test_max_calories() {
        let cals = "
            10
            20
            30
        ";
        assert_eq!(max_calories(cals), 60);

        let cals = "
            10
            20
            30

            70
        ";
        assert_eq!(max_calories(cals), 70);

        let cals = "
            10
            20
            30

            70

            40
        ";
        assert_eq!(max_calories(cals), 70);
    }

    #[test]
    fn test_top() {
        let cals = "
            10

            20

            30

            40
        ";
        assert_eq!(top(cals, 2), 70);

        let cals = "
            10

            20
        ";
        assert_eq!(top(cals, 3), 30);

        let cals = "
            10
            20

            40

            50
            60

            70
            80
        ";
        assert_eq!(top(cals, 3), 40 + 50 + 60 + 70 + 80);
    }
}
