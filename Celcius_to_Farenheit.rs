fn convert_temperature(value: f64, unit: char) {
    if unit == 'C' {
        let converted = (value * 9.0 / 5.0) + 32.0;
        println!("{:.2}F", converted);
        //{:.2} tell rust to have floating points with only 2 decimals
    } else if unit == 'F' {
        let converted = (value - 32.0) * 5.0 / 9.0;
        println!("{:.2}C", converted);
        //{:.2} tell rust to have floating points with only 2 decimals
    } else {
        println!("Invalid unit! Use 'C' for Celsius or 'F' for Fahrenheit.");
    }
}

fn main() {
    convert_temperature(100.0, 'C'); // 212.00F
    convert_temperature(212.0, 'F'); // 100.00C
    convert_temperature(50.0, 'X');  // Invalid unit
}
