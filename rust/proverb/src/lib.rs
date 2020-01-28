pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = vec![];

    for (index, verb) in list.iter().enumerate() {
        if index < list.len() - 1 {
            proverb.push(format!("For want of a {} the {} was lost.", list[index], list[index + 1]))
        } else {
            proverb.push(format!("And all for the want of a {}.", list[0]));
        }
    }

    return proverb.join("\n");
}

