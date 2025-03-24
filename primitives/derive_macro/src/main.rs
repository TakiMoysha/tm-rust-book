use std::error::Error;

pub use derive_lib::*;

type Row = Vec<String>;

// it should automatically derive the FromRow trait
pub struct DiveMetric {
    pub time: i32,
    pub depth: f32,
    pub pressure: Option<f32>,
    pub temperature: Option<f32>,
}

pub trait FromRow {
    fn from_row(row: Row) -> Result<Self, Box<dyn Error>>
    where
        Self: std::marker::Sized;
}

mod demo_impl {
    use super::*;

    impl FromRow for DiveMetric {
        fn from_row(row: Row) -> Result<Self, Box<dyn Error>> {
            Ok(Self {
                time: row[0].parse()?,
                depth: row[1].parse()?,
                pressure: row[2].parse::<f32>().ok(),
                temperature: row[3].parse::<f32>().ok(),
            })
        }
    }
}

mod dervie_impl {
    use derive_lib::*;

    #[derive(FromRow)]
    pub struct DiveMetricNamed {
        pub time: i32,
        pub depth: f32,
        pub pressure: Option<f32>,
        pub temperature: Option<f32>,
    }

    // #[derive(FromRow)]
    // pub struct DiveMetricUnnamed(i32, f32, Option<f32>, Option<f32>);
    //
    // #[derive(FromRow)]
    // pub struct DiveMetricUnit;
}

fn main() {}
