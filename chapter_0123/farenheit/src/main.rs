// Converts temperatures between farenheit and degrees or vice versa

// Celsius = (Farenheit - 32) * 5/9
// Farenheit = 32 + Celsius * 9/5

fn main() {

    //input float and units i.e c or f
    let c_to_f_conversion: f64 = celcius_to_farenheit(10.5);
    let f_to_c_conversion: f64 = farenheit_to_celcuis(50.9);


    println!("Celcius to farenheit conversion: {}", c_to_f_conversion);
    println!("Farenheit to celcius conversion: {}", f_to_c_conversion);

}

fn celcius_to_farenheit(deg_celcius: f64) -> f64 {
    32.0 + f64::from(deg_celcius) * 9.0 / 5.0
}

fn farenheit_to_celcuis(deg_farenheit: f64) -> f64 {
    (f64::from(deg_farenheit) - 32.0) * 5.0 / 9.0
}

fn user_input() {
    // TODO include user input to make decisions on which conversion
    loop {
        println!("")
    }
}