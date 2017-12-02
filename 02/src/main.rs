use std::cmp;


fn checksumer(input: &[u8]) -> usize {
    input.split(|&num| num == 13)
        .map(|row| {
            let it = row.iter()
                .filter(|&&byte| byte >= b'0' && byte <= b'9')
                .map(|&byte| byte - 48)
                .fold(None, |extremes, item| {
                    if let Some((min, max)) = extremes {
                        Some((cmp::min(min, item), cmp::max(max, item)))
                    } else {
                        Some((item, item))
                    }
                })
                .map(|extremes| {
                    extremes.unwrap_or(0)
                });

            
        });
        0
}

const INPUT: &[u8] = br#"5 1 9 5
7 5 3
2 4 6 8"#;

fn main() {
    println!("{}", checksumer(INPUT));
}
