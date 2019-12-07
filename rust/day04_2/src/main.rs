fn is_valid(num: isize) -> bool {
    let as_str = num.to_string();
    let mut password = as_str.chars();

    let mut pair_found = false;
    let mut group_length = 1;

    let mut a = password.next().unwrap().to_digit(10);

    loop {
        match password.next() {
            Some(raw) => {
                let b = raw.to_digit(10);
                if b < a {
                    return false;
                }
        
                if a == b {
                    group_length += 1;
                } else {
                    if group_length == 2 {
                        pair_found = true;
                    }
                    group_length = 1;
                }
                a = b;
            },
            None => break,
        }
    }
    if group_length != 2 && !pair_found {
        return false;
    }

    true
}

fn main() {
    let mut count = 0;

    for num in 125730..=579381 {
        if is_valid(num) {
            count += 1;
        }
    }

    println!("count: {}", count);
}

#[test]
fn tests() {
    assert_eq!(is_valid(112233), true);
    assert_eq!(is_valid(123444), false);
    assert_eq!(is_valid(111122), true);
    assert_eq!(is_valid(112345), true);
    assert_eq!(is_valid(123455), true);
}
