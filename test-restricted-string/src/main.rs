#[macro_use]
extern crate restricted_string_derive;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    #[derive(RestrictedString)]
    #[RestrictedStringRegex = ".*"]
    struct AnyString {
        s: String
    }

    impl AnyString {
        pub fn new(string: String) -> AnyString {
            AnyString {
                s: string
            }
        }
    }
    
    #[test]
    fn it_works() {
    }
}
