fn main() {
    let test_num = 1023232;
    println!("{}", to_words(test_num));
}

const BASE: [&'static str; 10] = ["", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
const TEEN: [&'static str; 10] = ["ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"];
const TENS: [&'static str; 10] = ["", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];
const UNITS: [&'static str; 8] = ["", "thousand", "million", "billion", "trillion", "quadrillion", "quintillion", "sextillion"];

fn to_words(num: usize) -> String {

    // Special case for zero
    if num == 0 {
        return "Zero".to_string();
    }

    // Special case for less than 10
    if num < 10 {
        return BASE[num].to_string();
    }

    // Special case for teens
    if num < 20 {
        return TEEN[num - 10].to_string();
    }

    let mut num_str = String::new();
    let mut current_depth: u8 = 0;
    let mut current_num = num;

    loop {
        let ones_digit = current_num % 10;
        let tens_digit = current_num / 10 % 10;
        let hundreds_digit = current_num / 100 % 10;
        let has_units = current_depth != 0 && std::cmp::max(std::cmp::max(ones_digit, tens_digit), hundreds_digit) > 0;

        // Get the ones string representation
        let ones_str: String =  match tens_digit {
            1 => "".to_string(),
            _ => BASE[ones_digit].to_string(),
        };

        // Add the unit if needed
        let ones_str_with_unit: String = match current_depth {
            0 => ones_str,
            _ => {
                match has_units {
                    true => format!("{} {}", ones_str, UNITS[current_depth as usize]),
                    false => ones_str,
                }
            },
        };

        // Get the tens string representation
        let tens_str: String = match tens_digit {
            0 => "".to_string(),
            1 => TEEN[ones_digit].to_string(),
            _ => TENS[tens_digit].to_string(),
        };

        // Get the hundreds string representation
        let hundred_str: String = match hundreds_digit {
            0 => "".to_string(),
            _ => format!("{} hundred", BASE[hundreds_digit]),
        };

        num_str = format!("{} {} {} {}", hundred_str, tens_str, ones_str_with_unit, num_str);

        current_num /= 1000;

        if current_num <= 0 {
            break;
        }

        current_depth += 1;
    }

    num_str.trim().split_whitespace().collect::<Vec<&str>>().join(" ")
}