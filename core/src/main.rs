extern crate linen_core;

use linen_core::Platform;

fn main() {
    let platforms = Platform::all();

    for platform in platforms {
        println!("name: {}", platform.name());
        println!("profile: {:?}", platform.profile());
        println!("vendor: {}", platform.vendor());

        println!("extensions:");
        for extension in platform.extensions().iter() {
            println!("\t{}", extension);
        }

        println!("devices:");
        for device in platform.all_devices() {
            println!("\tname: {}", device.name());
            println!("\tprofile: {:?}", device.profile());
            println!("\tvendor: {}", device.vendor());

            for extension in device.extensions().iter() {
                println!("\t\t{}", extension);
            }
        }
    }
}
