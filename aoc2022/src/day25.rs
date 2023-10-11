fn snafu_to_dec(input: &str) -> i64 {
    let len = input.len() - 1;
    input
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let position_value = 5i64.pow(u32::try_from(len - i).unwrap());
            let position = match c {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                _ => panic!("Invalid SNAFU number: {c}"),
            };
            position_value * position
        })
        .sum()
}

fn dec_to_snafu(input: i64) -> String {
    let number_len = 20;
    let mut v = vec![0; number_len];

    let mut remainder = input;
    for i in (0..number_len).rev() {
        let position_value = 5i64.pow(u32::try_from(i).unwrap());
        v[i] = remainder / position_value;
        remainder %= position_value;
    }

    let mut carry = 0;
    for i in 0..number_len {
        let val = v[i] + carry;
        v[i] = ((val + 2) % 5) - 2;
        carry = (val + 2) / 5;
    }

    v.iter()
        .rev()
        .skip_while(|v| **v == 0)
        .map(|v| match v {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => panic!("Invalid decimal integer for SNAFU conversion!"),
        })
        .collect()
}

pub fn part1(input: &[String]) -> String {
    dec_to_snafu(input.iter().map(|s| snafu_to_dec(s)).sum())
}

pub fn part2(_input: &[String]) -> String {
    "".to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::read_input_from_file;

    #[test]
    fn test_snafu_to_dec() {
        assert_eq!(snafu_to_dec("1=-0-2"), 1747);
        assert_eq!(snafu_to_dec("12111"), 906);
        assert_eq!(snafu_to_dec("2=0="), 198);
        assert_eq!(snafu_to_dec("21"), 11);
        assert_eq!(snafu_to_dec("2=01"), 201);
        assert_eq!(snafu_to_dec("111"), 31);
        assert_eq!(snafu_to_dec("20012"), 1257);
        assert_eq!(snafu_to_dec("112"), 32);
        assert_eq!(snafu_to_dec("1=-1="), 353);
        assert_eq!(snafu_to_dec("1-12"), 107);
        assert_eq!(snafu_to_dec("12"), 7);
        assert_eq!(snafu_to_dec("1="), 3);
        assert_eq!(snafu_to_dec("122"), 37);
    }

    #[test]
    fn test_dec_to_snafu() {
        assert_eq!(dec_to_snafu(1747), "1=-0-2");
        assert_eq!(dec_to_snafu(906), "12111");
        assert_eq!(dec_to_snafu(198), "2=0=");
        assert_eq!(dec_to_snafu(11), "21");
        assert_eq!(dec_to_snafu(201), "2=01");
        assert_eq!(dec_to_snafu(31), "111");
        assert_eq!(dec_to_snafu(1257), "20012");
        assert_eq!(dec_to_snafu(32), "112");
        assert_eq!(dec_to_snafu(353), "1=-1=");
        assert_eq!(dec_to_snafu(107), "1-12");
        assert_eq!(dec_to_snafu(7), "12");
        assert_eq!(dec_to_snafu(3), "1=");
        assert_eq!(dec_to_snafu(37), "122");
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&read_input_from_file("sample/day25.txt")), "2=-1=0");
        assert_eq!(
            part1(&read_input_from_file("input/day25.txt")),
            "2-1-110-=01-1-0-0==2"
        );
    }
}
