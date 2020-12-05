

pub fn get_binary_array(content: String) -> Vec<String> {
    let cache = content.split("\n").collect::<Vec<&str>>();
    let mut res: Vec<String> = vec![];
    for el in cache {
        res.push(el.to_string());
    }
    res
}



