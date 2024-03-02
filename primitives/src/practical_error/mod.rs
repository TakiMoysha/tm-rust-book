use chrono::NaiveDate;
use std::{env, fmt, fs};

#[derive(Debug)]
pub struct UserCommand {
    product: String,
    quantity: u32,
    delivery_date: NaiveDate,
}

impl fmt::Display for UserCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "UserCommand {{ product: {}, quantity: {}, delivery_date: {} }}",
            self.product, self.quantity, self.delivery_date
        )
    }
}

pub fn run() -> Result<Vec<UserCommand>, String> {
    let file_name = env::args().nth(1).ok_or("You need to pass a file name")?;
    let content = fs::read_to_string(&file_name)
        .map_err(|e| format!("Could not read the file, by reason: '{}'", e))?;

    let mut commands: Vec<UserCommand> = Vec::new();

    for (indx, line) in content.lines().enumerate() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let mut parts = line.split_whitespace();
        let product = parts
            .next()
            .ok_or(&format!("[InLine: <{}>] No product", indx))?
            .to_string();

        let quant = parts
            .next()
            .ok_or(&format!("[InLine: <{}>] No quantity", indx))?;
        let quantity = quant.trim().parse::<u32>().map_err(|e| {
            format!(
                "[InLine: <{}>;Value: <{}>] Quantity is not a number, by reason: '{}'",
                indx, quant, e
            )
        })?;

        let d_date = parts.next().ok_or("No delivery date")?;
        let delivery_date = NaiveDate::parse_from_str(d_date.trim(), "%d.%m.%Y").map_err(|e| {
            format!(
                "[InLine: <{}>;Value: <{}>] Invalid date (day.month.year), by reason: {}",
                indx, d_date, e
            )
        })?;

        let command = UserCommand {
            product,
            quantity,
            delivery_date,
        };
        commands.push(command);
    }

    Ok(commands)
}

pub fn run_demo() {
    let comands = run();

    if comands.is_err() {
        println!("{}", comands.err().unwrap());
        std::process::exit(1);
    }
    println!("Commands:");
    comands
        .unwrap()
        .iter()
        .for_each(|cmd| println!("* {}", cmd));
}

