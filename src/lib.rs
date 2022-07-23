use std::time::Duration;


pub fn default_callback(item_name: &str, duration: Duration) {
    print_it(item_name, duration);
}

pub fn print_it(item_name: &str, duration: Duration) {
    println!("Function #{} took {:?}", item_name, duration);
}

#[cfg(test)]
mod test {
    use super::*;
    use lap_derive::time_it;

    // #[test]
    // fn default_without_response() {
    //     #[time_it]
    //     fn whatever() {}
    //     whatever();
    // }

    #[test]
    fn default_with_response() {
        #[time_it]
        fn whatever() -> String {
            "whatever".to_string()
        }
        whatever();
    }

    #[test]
    fn default_with_borrowed_response() {
        #[time_it]
        fn whatever() -> &'static str {
            "whatever"
        }
        whatever();
    }

    #[test]
    fn default_with_bounded_borrowed_response() {
        #[time_it]
        fn whatever<'a>() -> &'a str {
            "whatever"
        }
        whatever();
    }

    // #[test]
    // fn custom_without_response() {
    //     #[time_it]
    //     fn whatever() {}
    //     whatever();
    // }


    // #[test]
    // fn custom_with_response() {
    //     #[time_it]
    //     fn whatever() -> String {
    //         "whatever".to_string()
    //     }
    //     whatever();
    // }
}
