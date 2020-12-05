pub struct seat {
    pub(crate) row: i32,
    pub(crate) column: i32,
    pub(crate) id: i32
}


pub fn get_seat(bin: String) -> seat {
    let chars: Vec<char> = bin.chars().collect();
    let mut row_chars: Vec<char> = vec![];
    let mut column_chars: Vec<char> = vec![];
    for i in 0..7 {
        row_chars.push(chars[i as usize]);
    }
    for i in 7..10 {
        column_chars.push(chars[i as usize]);
    }
    let row = get_row(&row_chars);
    let column = get_column(&column_chars);
    let id = row * 8 + column;
    seat {
        row,
        column,
        id
    }
}

fn get_row(arr: &Vec<char>) -> i32 {
    let mut min_value = 0;
    let mut max_value = 127;
    for char in arr {
        if char == &'F' {
            let diff = max_value - min_value;
            let cache = ((diff / 2) as f32).ceil() as i32;
            max_value -= cache;
        } else {
            let diff = max_value - min_value;
            let cache = ((diff / 2) as f32).floor() as i32;
            min_value += cache;
        }
    }
    if arr[0] == 'B' {
        min_value + 1
    } else {
        min_value
    }
}
fn get_column(arr: &Vec<char>) -> i32 {
    let mut min_value = 0;
    let mut max_value = 7;
    for char in arr {
        if char == &'L' {
            let diff = max_value - min_value;
            let cache = ((diff / 2) as f32).ceil() as i32;
            max_value -= cache;
        } else {
            let diff = max_value - min_value;
            let cache = ((diff / 2) as f32).floor() as i32;
            min_value += cache;
        }
    }
    if arr[0] == 'R' {
        min_value + 1
    } else {
        min_value
    }
}

pub fn get_highest_id(arr: &Vec<seat>) -> i32 {
    let mut cache = 0;
    for seat in arr {
        if seat.id > cache {
            cache = seat.id;
        }
    }
    cache
}

pub fn get_lowest_id(arr: &Vec<seat>) -> i32 {
    let mut cache = 0;
    for seat in arr {
        if seat.id < cache {
            cache = seat.id;
        }
    }
    cache
}

pub fn get_missing_ids(arr: &Vec<seat>, highest: i32, lowest: i32) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    let mut ids: Vec<i32> = vec![];
    for seat in arr {
        ids.push(seat.id);
    }
    for i in lowest..(highest + 1) {
        if !ids.contains(&i) {
            res.push(i);
        }
    }
    res
}