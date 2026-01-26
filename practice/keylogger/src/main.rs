//
// Show current presset command in
//
// Show input devices: `cat /proc/bus/input/devices | less`
// we needed only where 'B: KEY=' have a '';
//

use argparse::{ArgumentParser, StoreTrue};

mod input {
    #[derive(Debug, Clone, PartialEq)]
    pub enum InputEvent {
        KeyPress(u32),
        KeyRelease(u32),
    }

    fn get_keyboards() -> Vec<()> {
        todo!()
    }

    mod devices_bus {
        use std::{
            fs::File,
            io::{BufRead, BufReader},
        };

        const EV_KEY: &str = "120013";

        #[derive(Debug, Default, Clone)]
        struct DeviceFromBus {
            name: Option<String>,
            sysfs: Option<String>,
            ev: Option<String>,
            handlers: Option<String>,
        }

        fn find_keyboards() -> Result<Vec<DeviceFromBus>, Box<dyn std::error::Error>> {
            let file = File::open("/proc/bus/input/devices")?;
            let reader = BufReader::new(file);

            let mut keyboards = Vec::new();
            let mut current_device: DeviceFromBus = DeviceFromBus::default();

            for line in reader.lines() {
                let line = line?;

                match line {
                    // H: Handlers=sysrq kbd event7
                    line if line.starts_with("H: Handlers=") => {
                        current_device.handlers = Some(line["H: Handlers=".len()..].split(' ').collect());
                    }
                    // N: Name="ZXZK YSX-68    USB Keyboard"
                    line if line.starts_with("N: Name=") => {
                        current_device.name = Some(line["N: NAME=".len()..].trim().to_string());
                    }
                    // S: Sysfs=/devices/pci0000:00/0000:00:14.0/usb1/1-2/1-2:1.1/0003:1C4F:0084.000D/input/input53
                    line if line.starts_with("S: Sysfs=") => {
                        current_device.sysfs = Some(line["S: Sysfs=".len()..].trim().to_string());
                    }
                    // B: EV=120013
                    line if line.starts_with("B: EV=") => {
                        current_device.ev = Some(line["B: EV=".len()..].trim().to_string());
                    }
                    line if line.trim().is_empty() => {
                        if current_device.ev == Some(EV_KEY.to_string()) {
                            keyboards.push(current_device.clone());
                            continue;
                        }

                        current_device = DeviceFromBus::default();
                    }
                    _ => continue,
                }
            }

            Ok(keyboards)
        }

        #[cfg(test)]
        mod test {
            use super::*;

            #[test]
            fn should_find_all_keyboards() {
                let keyboards = find_keyboards();
                println!("{:#?}", keyboards);
            }
        }
    }
}

fn main() {
    let mut verbose = false;

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Show current presset command in ");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue, "verbose mode");
        ap.parse_args_or_exit();
    }
}
