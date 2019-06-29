pub fn raindrops(n: u32) -> String {
    format!(
        "{}{}{}{}",
        if n % 3 == 0 { "Pling" } else { "" },
        if n % 5 == 0 { "Plang" } else { "" },
        if n % 7 == 0 { "Plong" } else { "" },
        if n % 3 != 0 && n % 5 != 0 && n % 7 != 0 {
            n.to_string()
        } else {
            "".to_string()
        }
    )
}
