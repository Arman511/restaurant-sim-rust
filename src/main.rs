use restaurants::restaurant::Restaurant;
use std::collections::HashMap;
use std::io;
mod restaurants;
use crate::restaurant::food::Food;
use restaurants::*;

fn main() {
    const PROFIT_MARGINS: f32 = 0.3;
    let mut my_restaurant = get_restaurant();
    let mut ans: char = ' ';
    let mut ordering: HashMap<u8, Vec<u8>> = HashMap::new();
    let mut profit: f32 = 0.0;

    while ans != 'q' {
        ans = input_string("++++ RESTAURANT SIMULATOR\nm to show the menu\ng for getting an order\ns for serving an order\na for adding menu item\ni to show ingredients of an item\np to see profits\nc to change the price of a food\nr to remove a food from the menu\nq to quit: ").to_lowercase().chars().next().unwrap();
        match ans {
            'q' => break,
            'm' => my_restaurant.print_menu(),
            'g' => {
                let (seat_number, food_numbers) = my_restaurant.take_order();
                ordering.insert(seat_number, food_numbers.clone());
                println!(
                    "Order received for seat number {}: {:?}",
                    seat_number, food_numbers
                );
            }
            'i' => {
                my_restaurant.print_menu();
                let food_number: u8 =
                    input_string("Enter the food number to see its ingredients: ")
                        .parse()
                        .unwrap();
                my_restaurant.show_ingredients(food_number);
            }
            's' => {
                let seat_number: u8 = input_string("Enter the seat number to serve: ")
                    .parse()
                    .unwrap();
                let mut found=false;
                for serving_item in ordering.get(&seat_number).unwrap() {
                    profit += my_restaurant.serve(*serving_item, seat_number) * PROFIT_MARGINS;
                    found = true;
                }
                if !found {
                    println!("No order found for seat number {}", seat_number);
                }
                ordering.remove(&seat_number);
            }
            'a' => {
                let food_name = input_string("Enter the food name: ");
                let food_price: f32 = input_string("Enter the food price: ").parse().unwrap();
                let ingredients =
                    input_string("Enter the food ingredients (separated by commas): ")
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect::<Vec<String>>();
                let new_food = Food::new(food_name, food_price, ingredients);
                my_restaurant.add_menu_item(new_food);
                println!("New menu item added successfully!");
            }
            'p' => {
                println!("Total profit: £{:.2}", profit);
            }
            'c' => {
                my_restaurant.print_menu();
                let food_number: u8 = input_string("Enter the food number to change its price: ")
                    .parse()
                    .unwrap();
                let new_price: f32 = input_string("Enter the new price: ").parse().unwrap();
                my_restaurant.change_food_price(food_number, new_price);
                println!("Food price updated successfully!");
            }
            'r' => {
                my_restaurant.remove_menu_item();
                println!("Food item removed successfully!");
            }

            _ => println!("Not a valid input"),
        }
    }
}

fn input_string(msg: &str) -> String {
    println!("{}", msg);
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().into()
}

fn get_restaurant() -> Restaurant {
    let name = input_string("What is the name of this restaurant: ");
    let address = input_string("What is the address of this restaurant: ");

    let price_range: String = input_string("What is the price range of this restaurant: ");

    let price_range = match price_range.trim().to_lowercase().as_str() {
        "cheap" => PriceRange::Cheap,
        "low class" => PriceRange::LowClass,
        "middle class" => PriceRange::MiddleClass,
        "high class" => PriceRange::HighClass,
        "world class" => PriceRange::WorldClass,
        _ => PriceRange::None,
    };

    let food_type: String = input_string("Please enter a cuisine type:");

    let food_type = match food_type.trim().to_lowercase().as_str() {
        "indian" => Type::Indian,
        "chinese" => Type::Chinese,
        "japanese" => Type::Japanese,
        "american" => Type::American,
        "french" => Type::French,
        "turkish" => Type::Turkish,
        "seafood" => Type::Seafood,
        _ => Type::None,
    };
    let main_chef: String = input_string("Who is the main Chef of this restaurant: ");
    let owner: String = input_string("Who is the owner of this restaurant: ");
    let phone_number: String = input_string("What is the phone number of this restaurant: ");

    Restaurant::new(
        name,
        address,
        food_type,
        price_range,
        main_chef,
        owner,
        phone_number,
    )
}
