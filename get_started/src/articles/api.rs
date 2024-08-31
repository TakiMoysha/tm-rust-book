const TAX_RATE: f64 = 0.05;
#[derive(Debug, Default)]
struct Accounts {
    slug: String,
    company_balance: f64,
    taxes_paid: f64,
}

#[derive(Clone, Copy)]
enum Place {
    Bratislava,
    London,
    Paris,
}

impl Place {
    fn match_tax_rate(self) -> f64 {
        match self {
            Place::Bratislava => 0.5,
            Place::London => 0.08,
            Place::Paris => 0.07,
        }
    }
}

#[derive(Clone, Copy)]
enum Products {
    Eggs,
    Apples,
}

impl Products {
    fn match_price(&self) -> f64 {
        match self {
            Products::Eggs => 1.0,
            Products::Apples => 2.0,
        }
    }
}

impl Accounts {
    fn log_purchase(&mut self, quantity: u64, place: Place, product: Products) {
        let is_collecting_taxes = matches!((place, product), (Place::London, Products::Eggs));

        let order_total = quantity as f64 * product.match_price();
        let taxes = if is_collecting_taxes {
            order_total * place.match_tax_rate()
        } else {
            0.0
        };

        self.taxes_paid += taxes;
        self.company_balance += order_total - taxes;
    }

    fn buy(&mut self, quantity: u64, place: Place, product: Products) {
        self.log_purchase(quantity, place, product);
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_init_default_accounts() {
        let accounts = Accounts::default();
        assert_eq!(accounts.company_balance, 0.0);
        assert_eq!(accounts.taxes_paid, 0.0);
    }

    #[test]
    fn should_after_purchase_balance_must_converge() {
        let mut accounts = Accounts::default();
        accounts.buy(2, Place::London, Products::Eggs);
        accounts.buy(1, Place::Bratislava, Products::Apples);

        assert_eq!(accounts.company_balance, 3.84);
        assert_eq!(accounts.taxes_paid, 0.16);
    }
}
