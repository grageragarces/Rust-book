pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        // if value < 1 {
        //     panic!(
        //         "Guess value must be greater than or equal to 1, got {value}."
        //     );
        // } else if value > 100 {
        //     panic!(
        //         "Guess value must be less than or equal to 100, got {value}."
        //     );
        // }
        if value < 1 { //BUGGY: should_panic test with an expected message fails
            panic!(
                "Guess value must be less than or equal to 100, got {value}."
            );
        } else if value > 100 {
            panic!(
                "Guess value must be greater than or equal to 1, got {value}."
            );
        }


        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // #[should_panic(expected = "less than or equal to 100")]
    // fn greater_than_100() {
    //     Guess::new(200);
    // }

    #[test] // not panick -> Err
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

}