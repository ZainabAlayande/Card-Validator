fn main() {
    let card_number = "4388576018410707".to_string();
    let card_array = pass_card_number_into_array(card_number.clone());

    let result_one = double_every_second_digit_from_behind(card_array.clone());
    let result_two = add_digit_in_odd_places_from_right_to_left(card_array);

    let validity_status = validate_card_number_is_valid(result_one + result_two);
    let card_type = check_card_type("4388576018410707");

    let card_output = card_output(card_type, &card_number, 16, &validity_status);

    println!("{:?}", card_output);
}



fn pass_card_number_into_array(card_number: String) -> Vec<i32> {
    let mut result = Vec::new();
    for number in card_number.chars() {
        if let Some(digit) = number.to_digit(10) {
            result.push(digit as i32);
        }
    }

    println!("{:?}", result);
    result
}



fn double_every_second_digit_from_behind(array: Vec<i32>) -> i32 {
    let mut count = 0;
    for (index, number) in array.iter().rev().enumerate() {
        if index % 2 == 1 {
            let mut answer = number * 2;

            if answer > 9 {
                answer = (answer/10) + (answer % 10);
                count += answer;
            } else {
                count += answer;
            }
        }
    }
    count
}



fn add_digit_in_odd_places_from_right_to_left(array: Vec<i32>) -> i32 {
    let mut count = 0;

    for (index, &number) in array.iter().rev().enumerate() {
        if index % 2 == 0 {
            count += number;
        }
    }
    count
}


fn validate_card_number_is_valid(result: i32) -> String {
    if result % 10 == 0 {"Valid".to_string()} else {"Invalid".to_string()}
}


fn check_card_type(card_number: &str) -> &str {
    if card_number.starts_with("4") {
        "Visa Card"
    } else if card_number.starts_with("5") {
        "MasterCard"
    } else if card_number.starts_with("6") {
        "Discovery Card"
    } else if card_number.starts_with("37") {
        "American Express Card"
    } else {
        "Unknown Card Type"
    }
}



fn card_output(type_: &str, card_number: &str, length: i32, validity_status: &str) -> String {
    format!(
        "*********************************************************\n\
        **Credit Card Type: {}\n\
        **Credit Card Number: {}\n\
        **Credit Card Digit Length: {}\n\
        **Credit Card Validity Status: {}\n\
        *********************************************************",
        type_, card_number, length, validity_status
    )
}
