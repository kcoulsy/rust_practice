use std::io;

enum Base {
    Decimal = 10,
    Binary = 2,
    Octal = 8,
    Hexadecimal = 16,
}

impl Base {
    fn from_input_value(value: u32) -> Base {
        match value {
            1 => Base::Decimal,
            2 => Base::Binary,
            3 => Base::Octal,
            4 => Base::Hexadecimal,
            _ => Base::Decimal,
        }
    }

    fn to_label_string(self) -> String {
        match self {
            Base::Decimal => String::from("Decimal"),
            Base::Binary => String::from("Binary"),
            Base::Octal => String::from("Octal"),
            Base::Hexadecimal => String::from("Hexadecimal"),
        }
    }
}

fn main() {
    println!("Welcome to the base converter! \n");

    let number = get_input_number();
    let base = Base::from_input_value(get_input_base());
    let converted_number = convert_number(&number, &base);
    let base_label = base.to_label_string(); 
    println!("the number {number} in {base_label} is {converted_number}");
}

fn get_user_input(prompt: &str) -> String {
    println!("\n{prompt}");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn get_input_number() -> u32 {
    get_user_input("Enter a number:")
        .trim()
        .parse()
        .unwrap_or_else(|_| get_input_number())
}

fn get_input_base() -> u32 {
    let mut prompt = String::from("Enter the base of the number:");
    prompt.push_str("\n1. Decimal (10)");
    prompt.push_str("\n2. Binary (2)");
    prompt.push_str("\n3. Octal (8)");
    prompt.push_str("\n4. Hexadecimal (16)");
    let input = get_user_input(&prompt);
    let base_value: u32 = input.trim().parse().unwrap_or_else(|_| get_input_base());

    if base_value < 1 || base_value > 4 {
        println!("Please enter a valid option!");
        return get_input_base()
    }

    base_value
}

fn convert_number(number: &u32, base: &Base) -> String {
    match base {
        Base::Decimal => number.to_string(),
        Base::Binary => to_binary(number),
        Base::Octal => to_octal(number),
        Base::Hexadecimal => to_hexadecimal(number),
    }
}

fn to_binary(number: &u32) -> String {
    format!("{:b}", *number)
}

fn to_octal(number: &u32) -> String {
    format!("{:o}", *number)
}

fn to_hexadecimal(number: &u32) -> String {
    format!("{:X}", *number)
}
