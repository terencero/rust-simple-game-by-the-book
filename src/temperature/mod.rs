use std::io;

pub fn convert_temperature() {
    let mut answer = String::new();

    println!("What is the temperature unit of measure? [f / c]?");
    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");

    let temperature_scale = answer.trim();

    let mut next_answer = String::new();

    println!("What is the current temperature measurement? i.e. 71");
    io::stdin()
        .read_line(&mut next_answer)
        .expect("Failed to read line");

    let og_temperature_unit: f64 = next_answer.trim().parse().expect("Please enter a number");

    #[derive(Debug)]
    struct TemperatureParts {
        original_unit: f64,
    }

    impl TemperatureParts {
        fn convert_celcius_to_fahrenheit(&self) -> f64 {
            (self.original_unit / 5.0) * 9.0 + 32.0
        }

        fn convert_fahrenheit_to_celcius(&self) -> f64 {
            (self.original_unit - 32.0) * 5.0 / 9.0
        }
    }

    #[derive(Debug)]
    enum ConvertedTemperature {
        Celcius(TemperatureParts),
        Fahrenheit(TemperatureParts),
    }

    let temperature_parts = TemperatureParts {
        original_unit: og_temperature_unit,
    };

    match temperature_scale {
        "c" => {
            if let ConvertedTemperature::Celcius(temperature) = ConvertedTemperature::Celcius(temperature_parts) {
                println!("{} degrees Celcius is {} degrees in Fahrenheit", temperature.original_unit, temperature.convert_celcius_to_fahrenheit());
            }
        },
        "f" => {
            if let ConvertedTemperature::Fahrenheit(temperature) = ConvertedTemperature::Fahrenheit(temperature_parts) {
                println!("{} degrees Fahrenheit is {} degrees in Celcius", temperature.original_unit, temperature.convert_fahrenheit_to_celcius());
            }
        },
        _ => println!("Not a valid unit of measure."),
    };
}
