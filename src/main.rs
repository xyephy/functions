fn main() {
    print_labeled_measurement(5, 'h');
}

// adding multiple parameters
fn print_labeled_measurement(value: 5, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}
