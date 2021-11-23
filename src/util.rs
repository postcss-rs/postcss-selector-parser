pub fn expected(mut description: Vec<&str>, index: usize, found: Option<&str>) {
    let last = description.pop();
    let description = format!("{} or {}", description.join(", "), last.unwrap_or(""));
    let an = if ['a', 'e', 'i', 'o', 'u'].contains(description[0]) {
        "an"
    } else {
        "a"
    };

    if let Some(founded) = found {
        panic!(
            "Expected {} {}, found \"{}\" instead. At #{}",
            an, description, founded, index
        );
    }

    panic!("Expected {} {}. At #{}", an, description, index);
}
