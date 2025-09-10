fn main() {
    let celsius = 23.0;
    let fahrenheit = celsius_to_fahrenheit(celsius);

    assert_eq!(fahrenheit, 73.4);
    println!("Test passed!");
}

fn celsius_to_fahrenheit(temp: f64) -> f64 {
    (temp * 1.8 + 32.0)
}
