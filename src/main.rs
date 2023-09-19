use std::fmt::Debug;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() -> Result<(), anyhow::Error> {
    println!("Supported hosts:\n  {:?}", cpal::ALL_HOSTS);
    let available_hosts = cpal::available_hosts();
    println!("Available hosts:\n  {:?}", available_hosts);

    let host = cpal::available_hosts()[0];
    let host = cpal::host_from_id(host)?;
    let device = host.default_input_device().unwrap();
    if let Ok(name) = device.name() {
        println!("using {}", name);
    } else {
        println!("using unnamed device");
    }

    let conf = device.default_input_config().unwrap();

    dbg!(&conf);

    let stream = device
        .build_input_stream(
            &conf.into(),
            |data, _: &_| write_input_data(data),
            |e| {
                println!("an error occured while streaming");
                dbg!(e);
            },
            None,
        )
        .unwrap();

    stream.play()?;

    // Let recording go for roughly three seconds.
    std::thread::sleep(std::time::Duration::from_secs(10));

    Ok(())
}

fn write_input_data(input: &[f32]) {
    let i = input[0];
    remap_print(i, -1.0, 1.0);
}

fn remap_print(value: f32, from: f32, to: f32) {
    let slider = "########################################";

    let value = value.clamp(from, to);

    // make value postive, and clamp to 1.0
    let value = (value - from) / (to - from);

    let index = slider.len() as f32 * value;
    let index = (index as usize).min(slider.len() - 1);

    let slider = &slider[..index];

    println!("{slider}");
}
