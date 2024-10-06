use embedded_graphics::pixelcolor::Rgb888;
use std::process::Command;

use embedded_graphics::{image::Image, image::ImageRaw, prelude::*};
use embedded_graphics_framebuffer::FrameBufferDisplay;

fn main() -> Result<(), core::convert::Infallible> {
    let d = evdev::Device::open(
        "/dev/input/by-id/usb-QEMU_QEMU_USB_Keyboard_68284-0000:00:04.0-3-event-kbd",
    )
    .unwrap();

    let mut display = FrameBufferDisplay::new();

    let kodi_selected_image_bytes = include_bytes!("kodi-selected.rgb");
    let steam_selected_image_bytes = include_bytes!("steam-selected.rgb");

    println!("kodi: {:?}", kodi_selected_image_bytes.len());
    println!("steam: {:?}", steam_selected_image_bytes.len());

    let kodi_selected_raw: ImageRaw<Rgb888> = ImageRaw::new(kodi_selected_image_bytes, 3840);
    let steam_selected_raw: ImageRaw<Rgb888> = ImageRaw::new(steam_selected_image_bytes, 3840);

    let kodi_selected_image = Image::new(&kodi_selected_raw, Point::zero());
    let steam_selected_image = Image::new(&steam_selected_raw, Point::zero());

    println!("{:?}", kodi_selected_image.bounding_box());

    let mut kodi = true;

    'running: loop {
        display.clear(Rgb888::BLACK)?;

        let key_state = match d.get_key_state() {
            Ok(keys) => keys,
            Err(_e) => continue 'running,
        };

        if key_state.contains(evdev::Key::KEY_LEFT) {
            kodi = true;
        }
        if key_state.contains(evdev::Key::KEY_RIGHT) {
            kodi = false;
        }
        if key_state.contains(evdev::Key::KEY_ENTER) {
            let output = Command::new("systemctl")
                .arg("start")
                .arg(if kodi {
                    "kodi-gbm.service"
                } else {
                    "steam.service"
                })
                .output()
                .expect("failed to execute process");

            println!("{:?}", output);

            break 'running Ok(());
        }

        if kodi {
            kodi_selected_image.draw(&mut display)?
        } else {
            steam_selected_image.draw(&mut display)?;
        }

        display.flush().unwrap();
    }
}
