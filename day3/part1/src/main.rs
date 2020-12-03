mod file_utils;
mod calculation_utils;

fn main() {
    let content = file_utils::load_content();
    let map = calculation_utils::get_value_map(content);
    calculation_utils::iterate_for_trees(map);
}
