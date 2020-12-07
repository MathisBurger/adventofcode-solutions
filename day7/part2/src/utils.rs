use crate::parser::parse;
use std::collections::VecDeque;

const needed: &str = "shiny gold";

pub fn get_sum<'a, I, S>(lines: I) -> usize
    where
        I: IntoIterator<Item = &'a S>,
        S: AsRef<str> + 'a,
{
    let bunte_bags = parse(lines);
    let mut sum = 0;
    let mut queue = bunte_bags.get(needed).map_or_else(VecDeque::new, |items| {
        items
            .iter()
            .map(|(count, item)| (*count, item.as_str()))
            .collect()
    });
    loop {
        match queue.pop_front() {
            Some((count, item)) => {
                sum += count;
                if let Some(items) = bunte_bags.get(item) {
                    for (count2, item2) in items {
                        queue.push_back((count * count2, item2));
                    }
                }
            }
            None => break,
        }
    }
    sum
}