use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");

    let mut tls_count = 0;
    let mut ssl_count = 0;
    for line in input.lines() {
        let parts = line.split(|c| c == '[' || c == ']');

        let mut has_abba_outside_brackets = false;
        let mut has_abba_inside_brackets = false;
        let mut ab_set = HashSet::new();
        let mut ba_set = HashSet::new();

        for (i, part) in parts.enumerate() {
            if i % 2 == 0 {
                if has_abba(part) {
                    has_abba_outside_brackets = true
                }
                if let Some(abas) = get_abs(part) {
                    for aba in abas {
                        ab_set.insert(aba);
                    }
                }
            } else {
                if has_abba(part) {
                    has_abba_inside_brackets = true
                }
                if let Some(bas) = get_bas(part) {
                    for ba in bas {
                        ba_set.insert(ba);
                    }
                }
            }
        }

        if ab_set.intersection(&ba_set).next().is_some() {
            ssl_count += 1;
        }

        if !has_abba_inside_brackets && has_abba_outside_brackets {
            tls_count += 1;
        }
    }

    println!("TLS count: {}", tls_count);
    println!("SSL count: {}", ssl_count);
}

fn has_abba(str: &str) -> bool {
    let mut chars = str.chars();
    let mut a = chars.next();
    let mut b = chars.next();
    let mut c = chars.next();
    let mut d = chars.next();

    while a.is_some() && b.is_some() && c.is_some() && d.is_some() {
        if a.unwrap() == d.unwrap() && b.unwrap() == c.unwrap() && a.unwrap() != b.unwrap() {
            return true;
        };
        a = b;
        b = c;
        c = d;
        d = chars.next();
    }

    false
}

fn get_abs(str: &str) -> Option<Vec<String>> {
    let mut chars = str.chars();
    let mut a = chars.next();
    let mut b = chars.next();
    let mut c = chars.next();

    let mut result = vec![];

    while a.is_some() && b.is_some() && c.is_some() {
        if a.unwrap() == c.unwrap() && a.unwrap() != b.unwrap() {
            result.push(format!("{}{}", a.unwrap(), b.unwrap()));
        };
        a = b;
        b = c;
        c = chars.next();
    }

    if result.len() > 0 {
        Some(result)
    } else {
        None
    }
}

fn get_bas(str: &str) -> Option<Vec<String>> {
    let mut chars = str.chars();
    let mut a = chars.next();
    let mut b = chars.next();
    let mut c = chars.next();

    let mut result = vec![];

    while a.is_some() && b.is_some() && c.is_some() {
        if a.unwrap() == c.unwrap() && a.unwrap() != b.unwrap() {
            result.push(format!("{}{}", b.unwrap(), a.unwrap()));
        };
        a = b;
        b = c;
        c = chars.next();
    }

    if result.len() > 0 {
        Some(result)
    } else {
        None
    }
}
