use hidapi::{HidApi, HidResult};

fn main() {
    println!("Printing all available hid devices:");

    match HidApi::new() {
        Ok(api) => {
            for device in api.device_list() {
                if device.vendor_id() == concept2::consts::CONCEPT2_VENDOR_ID {
                    println!("Found a device!");
                    let csafe_cmd = concept2::csafe::CSAFEFrame::new(vec![
                        concept2::concept2command::Concept2Command::ProprietaryCommand(vec![
                            concept2::concept2command::Concept2ProprietaryCommand::GetWorkTime,
                            concept2::concept2command::Concept2ProprietaryCommand::GetWorkDistance,
                            concept2::concept2command::Concept2ProprietaryCommand::GetWorkoutType,
                        ]),
                        concept2::concept2command::Concept2Command::GetSerialNumber,
                        concept2::concept2command::Concept2Command::GetSerialNumber,
                        concept2::concept2command::Concept2Command::GetSerialNumber,
                        concept2::concept2command::Concept2Command::GetSerialNumber,
                    ]);
                    let result: HidResult<Vec<u8>> = device.open_device(&api).and_then(|dev| {
                        concept2::hid_csafe::write_read_csafe_cmd(&dev, 2, &csafe_cmd)
                    });
                    match result {
                        Ok(v) => v.into_iter().for_each(|x| print!("{:x} ", x)),
                        Err(e) => print!("{}", e),
                    };
                    println!("");
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
