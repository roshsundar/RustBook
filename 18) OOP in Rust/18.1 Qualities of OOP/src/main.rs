/* Summary:
There isn't a consensus about what features a language must have to be considered object oriented.
However, OOP shares some commonalities - i.e. objects, encapsulation, and inheritance.
Rust takes inspiration from OOP.
*/

use std::fmt::Display;

fn main() {
    // Objects.
    {
        //$ An object contains data and behavior.
        // Rust has structs and enums that contain data, and impl blocks provide methods on them.

        struct Object<T> {
            data: T,
        }

        impl<T: Display> Object<T> {
            fn new(data: T) -> Self {
                Object {
                    data
                }
            }

            fn print_data(&self) {
                println!("{}", self.data);
            }
        }
    }

    // Encapsulation.
    {
        //$ Encapsulation is when the implementation details of an object aren't available to code using that object.
        // The only way to interact with an object is through its public API.
        // The 'pub' keyword decides with modules, types, functions, and methods should be public.

        // The struct itself is public, so other code can use it.
        // However, the fields w/in are private.
        pub struct AveragedCollection {
            list: Vec<i32>,
            average: f64,
        }

        impl AveragedCollection {
            pub fn add(&mut self, value: i32) {
                self.list.push(value);
                self.update_average();
            }

            pub fn remove(&mut self) -> Option<i32> {
                let result = self.list.pop();
                match result {
                    Some(value) => {
                        self.update_average();
                        Some(value)
                    },
                    None => None,
                }
            }

            pub fn average(&self) -> f64 {
                self.average
            }

            fn update_average(&mut self) {
                let total: i32 = self.list.iter().sum();
                self.average = total as f64 / self.list.len() as f64;
            }
        }

        // The public methods add(), remove(), and average() are the only ways to access or modify data in the AveragedCollection.
        // The list and average fields are private, so there's no way for external code to change them directly.
    }

    // Inheritance.
    {
        //$ Inheritance is when an object can inherit elements from another object's definition.
        // The object gets the parent's data and behavior w/o having to redefine them again.

        //$ Rust doesn't technically have inheritance.

        // Traits kind of do this. Creating a trait w/ a method that has a default implementation, and then impl'ing the trait for the type,
        // gives the type that functionality.

        trait Hello {
            fn say_hello(&self) {
                println!("Hello!");
            }
        }

        struct Person;
        impl Hello for Person {}

        let o = Person;
        o.say_hello();
    }
}
