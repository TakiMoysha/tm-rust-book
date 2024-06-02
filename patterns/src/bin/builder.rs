#[derive(Debug, Default, Clone)]
pub enum Equipped {
    #[default]
    Silver,
    Gold,
    Platinum,
}

#[derive(Debug, Default, Clone)]
pub enum Engine {
    #[default]
    Petrol,
    Diesel,
    Hybrid,
}

pub mod consuming_method {
    use crate::*;

    use anyhow::{anyhow, Result};
    use std::fmt;
    #[derive(Debug, Default, Clone)]
    pub struct Bike {
        equip: Equipped,
        color: Option<String>,
        engine: Option<Engine>,
        _weel_front: Option<String>,
        _weel_back: Option<String>,
    }
    #[derive(Debug, Clone)]
    pub struct BikeBuilder {
        bike: Bike,
    }

    impl BikeBuilder {
        pub fn new(equip: Equipped) -> BikeBuilder {
            BikeBuilder {
                bike: Bike {
                    equip,
                    ..Default::default()
                },
            }
        }
        pub fn set_color(mut self, color: impl Into<String>) -> Self {
            self.bike.color = Some(color.into());
            self
        }
        pub fn set_engine(mut self, engine: Engine) -> Self {
            self.bike.engine = Some(engine);
            self
        }
        pub fn build(self) -> Result<Bike> {
            let Some(engine) = self.bike.engine else {
                return Err(anyhow!("No engine"));
            };

            Ok(Bike {
                equip: self.bike.equip,
                color: Some(self.bike.color).unwrap_or(Some("White".to_owned())),
                engine: Some(engine),
                ..Default::default()
            })
        }
    }

    impl fmt::Display for Bike {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Bike ")?;
            write!(f, "equipped with {:?}, ", self.equip)?;

            if let Some(ref engine) = self.engine {
                write!(f, "having a {:?} engine, ", engine)?;
            } else {
                write!(f, "bike_engine: None")?;
            }

            if let Some(ref color) = self.color {
                write!(f, "and {} color.", color)?;
            } else {
                write!(f, "color: None")?;
            }

            write!(f, "")
        }
    }

    pub fn run_demo() -> Result<()> {
        let equip = Equipped::Gold;

        let bike = BikeBuilder::new(equip)
            .set_color("Metallic")
            .set_engine(Engine::Hybrid)
            .build()?;

        println!("Consuming Builder Patter:");
        println!("You have a {}", bike);

        let bike_builder = BikeBuilder::new(Equipped::Platinum);

        let bike_1 = bike_builder
            .clone()
            .set_color("Purple")
            .set_engine(Engine::Petrol)
            .build()?;
        let bike_2 = bike_builder
            .clone()
            .set_color("Leopard")
            .set_engine(Engine::Diesel)
            .build()?;

        println!("Consuming Builder Patter:");
        println!("You have a {}", bike_1);
        println!("You have a {}", bike_2);
        Ok(())
    }
}

pub mod non_consuming_method {
    use crate::*;
    use anyhow::{anyhow, Result};
    use std::fmt;

    #[derive(Debug, Default, Clone)]
    pub struct Bike {
        equip: Equipped,
        color: Option<String>,
        engine: Option<Engine>,
        _weel_front: Option<String>,
        _weel_back: Option<String>,
    }
    #[derive(Debug, Clone)]
    pub struct BikeBuilder {
        bike: Bike,
    }

    impl BikeBuilder {
        pub fn new(equip: Equipped) -> BikeBuilder {
            BikeBuilder {
                bike: Bike {
                    equip,
                    ..Default::default()
                },
            }
        }

        pub fn set_color(mut self, color: impl Into<String>) -> Self {
            self.bike.color = Some(color.into());
            self
        }

        pub fn set_engine(mut self, engine: Engine) -> Self {
            self.bike.engine = Some(engine);
            self
        }

        pub fn build(&mut self) -> Result<Bike, anyhow::Error> {
            let bike_engine = self
                .bike
                .engine
                .clone()
                .ok_or_else(|| anyhow!("No engine"))?;

            Ok(Bike {
                equip: self.bike.equip.clone(),
                color: Some(self.bike.color.clone().unwrap_or("white".to_owned())),
                engine: Some(bike_engine),
                ..Default::default()
            })
        }
    }

    impl fmt::Display for Bike {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Bike ")?;
            write!(f, "equipped with {:?}, ", self.equip)?;

            if let Some(ref engine) = self.engine {
                write!(f, "having a {:?} engine, ", engine)?;
            } else {
                write!(f, "bike_engine: None")?;
            }

            if let Some(ref color) = self.color {
                write!(f, "and {} color.", color)?;
            } else {
                write!(f, "color: None")?;
            }

            write!(f, "")
        }
    }

    pub fn run() -> Result<()> {
        let equip = Equipped::Gold;
        let bike = BikeBuilder::new(equip)
            .set_color("Dust")
            .set_engine(Engine::Hybrid)
            .build()?;

        println!("Consuming Builder Patter:");
        println!("You have a {}", bike);

        let equip = Equipped::Platinum;
        let bike_builder = BikeBuilder::new(equip);

        let bike_1 = bike_builder
            .clone()
            .set_color("Space")
            .set_engine(Engine::Petrol)
            .build()?;

        let bike_2 = bike_builder
            .clone()
            .set_color("Sunset")
            .set_engine(Engine::Diesel)
            .build()?;

        println!("Consuming Builder Patter:");
        println!("You have a {}", bike_1);
        println!("You have a {}", bike_2);

        Ok(())
    }
}

fn main() {
    println!("CONSUMING METHOD");
    consuming_method::run_demo().unwrap();
    println!("NON CONSUMING METHOD");
    non_consuming_method::run().unwrap();
}
