use anyhow::{self};

fn action() -> anyhow::Result<i32, Box<dyn std::error::Error>> {
    let result = 10/0;
    let res = anyhow::anyhow!(|v| {
        let r = 10/0;
        Ok(r)
    });

    Ok(result)
}

pub fn run() {
    let _ = action();
}

#[test]
fn name() {
    run();
}
