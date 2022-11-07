
mod front_of_house;

mod customer {
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        front_of_house::hosting::add_to_waitlist();
    }

    #[test]
    fn import_hosting() {}
}