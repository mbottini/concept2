use hidapi::HidApi;

fn main() {
    println!("Printing all available hid devices:");

    match HidApi::new() {
        Ok(api) => {
            for device in api.device_list() {
                println!("{:04x}:{:04x}", device.vendor_id(), device.product_id());
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
        },
    }
}