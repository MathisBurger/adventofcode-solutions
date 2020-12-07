use std::collections::HashMap;
use regex::Regex;

lazy_static! {
    static ref line_regex: Regex = Regex::new(r"(\w+ \w+) bags contain (.*)").unwrap();
    static ref item_regex: Regex = Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();
}

pub(crate) fn parse<'a, I, S>(lines: I) -> HashMap<String, Vec<(usize, String)>>
    where
        I: IntoIterator<Item = &'a S>,
        S: AsRef<str> + 'a,
{
    let mut bags = HashMap::<String, Vec<(usize, String)>>::new();
    for line in lines.into_iter() {
        if let Some((item, items)) = line_regex
            .captures(line.as_ref())
            .and_then(|captures| Some((captures.get(1)?.as_str(), captures.get(2)?.as_str())))
        {
            bags.insert(
                item.to_string(),
                item_regex
                    .captures_iter(items)
                    .filter_map(|captures| {
                        Some((
                            captures.get(1)?.as_str().parse().ok()?,
                            captures.get(2)?.as_str().to_string(),
                        ))
                    })
                    .collect(),
            );
        }
    }
    bags
}