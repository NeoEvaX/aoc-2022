use std::collections::VecDeque;

fn main() {
    let input = include_str!("./input.txt");

    let mut marker = VecDeque::new();
    let mut marker_solution = false;
    let mut message = VecDeque::new();

    for (index, letter) in input.chars().enumerate() {
        marker.push_back(letter);
        message.push_back(letter);

        if marker.len() > 4 && !marker_solution {
            marker.pop_front();
            if !find_duplicates(Vec::from(marker.clone())) {
                println!("Marker: {:?}", index + 1);
                marker_solution = true;
            }
        }
        if message.len() > 14 {
            message.pop_front();
            if !find_duplicates(Vec::from(message.clone())) {
                println!("Start of Message: {:?}", index + 1);
                break;
            }
        }
    }
}

fn find_duplicates(mut chars: Vec<char>) -> bool {
    if chars.is_empty() {
        return false;
    }

    chars.sort();
    if (1..chars.len()).any(|i| chars[i].eq(&chars[i - 1])) {
        return true;
    }
    false
}
