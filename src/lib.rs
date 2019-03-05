fn dec2_fact_string(nb: u64) -> String {
    let mut a: String = "".to_string();
    let mut step = 0;
    let mut div = nb;
    'a: loop {
        if div == 0 {
            break 'a;
        }
        step += 1;
        let lef = div % step;
        div = div / step;
        a.push_str(&(lef.to_string()));
    }
    a.chars().rev().collect()
}

fn fact_string_2dec(s: String) -> u64 {
    let len = s.len() as u64;
    let mut sum = 0;
    for (i, c) in s.chars().enumerate() {
        let k: u64 = match c.to_string().parse::<u64>() {
            Ok(v) => v,
            Err(_) => 0,
        };
        sum = sum * (len - i as u64) + k;
    }
    sum
}

fn testing1(nb: u64, exp: &str) -> () {
    assert_eq!(&dec2_fact_string(nb), exp)
}

fn testing2(s: &str, exp: u64) -> () {
    assert_eq!(fact_string_2dec(s.to_string()), exp)
}

#[test]
fn basics_dec2_fact_string() {
    testing1(2982, "4041000");
    testing1(463, "341010");
}

#[test]
fn basics_fact_string_2dec() {
    testing2("4041000", 2982);
    testing2("341010", 463);
}