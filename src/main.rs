/**
 * file: main.rs
 * desc: main function.
 */
mod audio;
mod policyconfig;
mod utility;

use audio::{AudioDevice, AudioDeviceCollection};
use clap::{arg, ArgGroup, Command};
use windows::Win32::System::Com::CoInitialize;

/**
 * Prints the list of active, available devices to stdout.
 *
 * args
 *  device_collection: the list of available devices
 */
fn list_devices(device_collection: &AudioDeviceCollection) {
    for device in device_collection.devices.iter() {
        println!("{}", device);
    }
}

/**
 * Changes the current audio device by string matching against the given identifier.
 *
 * args
 *  device_collection: the list of available devices
 *  identifier:        audio device identifier to use when finding the correct device
 *  use_id:            if true, matches against the device ID otherwise uses the name
 */
fn change_audio_device(
    device_collection: &AudioDeviceCollection,
    identifier: String,
    use_id: bool,
) -> Result<(), String> {
    let found: Vec<AudioDevice> = if use_id {
        device_collection
            .devices
            .iter()
            .filter(|d| d.id == identifier)
            .cloned()
            .collect()
    } else {
        device_collection
            .devices
            .iter()
            .filter(|d| d.name == identifier)
            .cloned()
            .collect()
    };

    // We can't change the audio device if none exist with the given identifier or if there
    // are multiple devices with the same identifier. Duplicate identifiers should ONLY happen
    // if matching against names.
    if found.is_empty() {
        return Err(format!(
            "Failed to find a device with the identifier '{}'",
            identifier
        ));
    } else if found.len() > 1 {
        return Err(format!(
            "There are {} devices with the same identifier, '{}'",
            found.len(),
            identifier
        ));
    }

    // Try to change the audio device
    found[0].clone().change_audio_device()
}

fn cli() {
    let matches = Command::new("audioswitch")
        .version("0.1.0")
        .author("genomicsoup")
        .about("Switch default audio devices on Win10")
        .arg(arg!(-l --list "List available audio devices").required(false))
        .arg(
            arg!(-i --"device-id" "Switch audio output using a device ID")
                .required(false)
                .takes_value(true),
        )
        .arg(
            arg!(-n --"device-name" "Switch audio output using a device name")
                .required(false)
                .takes_value(true),
        )
        .group(
            ArgGroup::new("command")
                .args(&["list", "device-id", "device-name"])
                .required(true),
        )
        .get_matches();

    // Initialize the COM API
    let init_result = unsafe { CoInitialize(std::ptr::null_mut()) };

    if init_result.is_err() {
        panic!("Failed to initialize '{:?}'", init_result.err());
    }

    // Try to grab all available audio devices
    let device_collection = AudioDeviceCollection::default();

    // We're listing audio devices
    if matches.is_present("list") {
        println!("--- Listing available audio devices");

        list_devices(&device_collection);

        std::process::exit(0);
    }

    let change_result = if matches.is_present("device-id") {
        let device_id = matches.value_of("device-id").unwrap().to_string();

        change_audio_device(&device_collection, device_id, true)
    } else {
        let device_name = matches.value_of("device-name").unwrap().to_string();

        change_audio_device(&device_collection, device_name, false)
    };

    if change_result.is_ok() {
        println!("--- Successfully changed audio devices")
    } else {
        println!("!!! {}", change_result.unwrap_err());
        std::process::exit(1);
    }
}

fn main() {
    cli();
}
