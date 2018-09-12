

// extern crate pikelet;
// extern crate phrases;
// use phrases::english::greetings::hello;
// use phrases::english::farewells::goodbye;



#[no_mangle]
pub fn add(first: i32, second:i32) -> i32 {
  first + second
}

#[no_mangle]
pub fn subtract(first: i32, second:i32) -> i32 {
  first - second
}

#[no_mangle]
pub fn multiply(first: i32, second:i32) -> i32 {
  first * second
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add(1,1), 2);
        assert_eq!(subtract(1,1), 0);
        assert_eq!(multiply(1,1), 1);
    }
}

