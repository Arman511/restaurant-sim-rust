pub enum PriceRange {
    Cheap,
    LowClass,
    MiddleClass,
    HighClass,
    WorldClass,
    None,
}
pub enum Type {
    Indian,
    Chinese,
    Japanese,
    American,
    French,
    Turkish,
    Seafood,
    None,
}

pub mod restaurant {
    use super::PriceRange;
    use super::Type;
    use crate::restaurants::restaurant::food::Food;
    use std::collections::HashMap;
    use std::io;

    pub struct Restaurant {
        pub name: String,
        pub address: String,
        pub price_range: PriceRange,
        pub food_type: Type,
        pub main_chef: String,
        pub owner: String,
        pub phone_number: String,
        pub menu: HashMap<u8, Food>,
    }

    impl Restaurant {
        pub fn new(
            name: String,
            address: String,
            food_type: Type,
            price_range: PriceRange,
            main_chef: String,
            owner: String,
            phone_number: String,
        ) -> Restaurant {
            Restaurant {
                name: name,
                address: address,
                food_type: food_type,
                price_range: price_range,
                main_chef: main_chef,
                owner: owner,
                phone_number: phone_number,
                menu: HashMap::new(),
            }
        }
        pub fn add_menu_item(&mut self, food: Food) {
            self.menu.insert((self.menu.len() + 1) as u8, food);
        }

        pub fn print_menu(&self) {
            println!("++++ MENU ++++");
            for (item_number, food) in &self.menu {
                println!("No.{}: {}, £{}", item_number, food.name, food.price)
            }
            println!();
        }

        pub fn take_order(&self) -> (u8, Vec<u8>) {
            println!("What is the seat number: ");

            let mut input: String = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let seat_number: u8 = input.trim().parse().expect("Input was not a number");
            self.print_menu();
            println!("What would you like (in the format: 3 5 7): ");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let mut foods: Vec<u8> = input
                .split_whitespace()
                .map(|s| s.parse().expect("Invalid input"))
                .collect();
            foods.remove(0);
            (
                seat_number,
                foods,
            )
        }
        pub fn serve(&self, food_num: u8, seat_number: u8) -> f32 {
            let food: &Food = self.menu.get(&food_num).unwrap();
            println!(
                "Here is the {}, for you seat number {}.",
                food.get_name(),
                seat_number
            );
            self.menu.get(&food_num).unwrap().get_price()
        }
        pub fn remove_menu_item(&mut self) {
            self.print_menu();
            println!("What is the food number: ");

            let mut input: String = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let food_number: u8 = input.trim().parse().expect("Input was not a number");
            self.menu.remove(&food_number);
        }
        pub fn change_food_price(&mut self, food_number: u8, new_price: f32) {
            if let Some(food) = self.menu.get_mut(&food_number) {
                food.change_price(new_price);
                println!("Food price changed successfully!");
            } else {
                println!("Food number {} not found in the menu.", food_number);
            }
        }
        pub fn show_ingredients(&self, food_number: u8) {
            if let Some(food) = self.menu.get(&food_number) {
                food.show_ingredients();
            } else {
                println!("Food number {} not found in the menu.", food_number);
            }
        }
    }
    pub mod food {
        pub struct Food {
            pub name: String,
            pub price: f32,
            pub ingredients: Vec<String>,
        }

        impl Food {
            pub fn new(name: String, price: f32, ingredients: Vec<String>) -> Food {
                Food {
                    name: name,
                    price: price,
                    ingredients: ingredients,
                }
            }

            pub fn change_price(&mut self, new_price: f32) {
                self.price = new_price;
            }

            pub fn show_ingredients(&self) {
                println!("\nIngredients for {}", self.name);
                for i in &self.ingredients {
                    println!("{}", i)
                }
                println!();
            }
            pub fn get_name(&self) -> String {
                self.name.clone()
            }
            pub fn get_price(&self) -> f32 {
                self.price.clone()
            }
        }
    }
}
