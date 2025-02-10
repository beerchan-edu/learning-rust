pub fn convert_temperatures() {
    let celsius_result = fahrenheit_to_celsius(25.0);

    println!("{celsius_result:.2}");

    let fahrenheit_result = celsius_to_fahrenheit(50.0);

    println!("{fahrenheit_result:.2}");

}

fn fahrenheit_to_celsius(degree: f64) -> f64 {
    (degree - 32.0) * (5.0 / 9.0)
}

fn celsius_to_fahrenheit(degree: f64) -> f64 {
    (degree * 9.0 / 5.0) + 32.0
}