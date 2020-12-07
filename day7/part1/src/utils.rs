use std::collections::{VecDeque, HashMap};
use crate::parser;

const needed: &str = "shiny gold";




pub fn get_num<'a, I, S>(lines: I) -> usize
    where
        I: IntoIterator<Item = &'a S>,
        S: AsRef<str> + 'a,
{
    let bunte_bags = parser::parse(lines);
    let mut golden_bags = HashMap::new();
    let mut vec = Vec::new();
    bunte_bags.keys()
        .filter(|key| {
            golden_bags.get(*key).copied().unwrap_or_else(|| {
                vec.push((*key, 0));
                'stack: loop {
                    match vec.pop() {
                        Some((key, i)) => {
                            for (j, (_, item)) in bunte_bags[key][i..].iter().enumerate() {
                                if item == needed {
                                    golden_bags.insert(key, true);
                                    continue 'stack;
                                }
                                match golden_bags.get(item) {
                                    None => {
                                        vec.push((key, i + j));
                                        vec.push((item, 0));
                                        continue 'stack;
                                    }
                                    Some(true) => {
                                        golden_bags.insert(key, true);
                                        continue 'stack;
                                    }
                                    Some(false) => {}
                                }
                            }
                            golden_bags.insert(key, false);
                        }
                        None => break golden_bags[*key],
                    }
                }
            })
        })
        .count()
}
