mod file_utils;
mod calculation_utils;

fn main() {
    let content = file_utils::load_content();
    let map = calculation_utils::get_value_map(content);
    let con1 = calculation_utils::iterate_for_trees(&map, 1, 1);
    let con2 = calculation_utils::iterate_for_trees(&map, 3, 1);
    let con3 = calculation_utils::iterate_for_trees(&map, 5, 1);
    let con4 = calculation_utils::iterate_for_trees(&map, 7, 1);
    let con5 = calculation_utils::iterate_for_trees(&map, 1, 2);
    println!("{}", con1 * con2 * con3 * con4 * con5);
}
