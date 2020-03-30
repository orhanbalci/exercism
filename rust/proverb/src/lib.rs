pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return "".to_string();
    }

    let mut resultv = Vec::new();

    for window in list.windows(2) {
        resultv.push(format!(
            "For want of a {} the {} was lost.\n",
            window[0], window[1]
        ));
    }

    resultv.push(format!("And all for the want of a {}.", list[0]));

    resultv.into_iter().collect()
}
