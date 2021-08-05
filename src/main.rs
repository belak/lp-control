mod platform;

use launchy::mini::{self, Button, Input, Message};
use launchy::prelude::*;
use launchy::DeviceCanvas;

fn main() -> anyhow::Result<()> {
    let mut canvas: DeviceCanvas<mini::Spec> = DeviceCanvas::guess(|_msg| {})?;
    canvas.clear();
    canvas.flush()?;

    let input = Input::guess_polling()?;
    input.drain();

    //let mut output = Output::guess()?;
    //output.reset()?;

    for item in input.iter() {
        let volume = platform::get_system_volume()
            .ok_or_else(|| anyhow::anyhow!("Failed to get system volume"))?;

        println!("Volume: {:?}", volume);

        // Ensure all lights are in the right state
        for x in 0..8 {
            for y in 0..8 {
                let idx = (y * 8 + x) as f32;

                let color = if idx / 64.0 <= volume || idx / 64.0 - volume < 0.01 {
                    Color::RED
                } else {
                    Color::BLACK
                };

                canvas
                    .set(Pad { x, y: y + 1 }, color)
                    .ok_or_else(|| anyhow::anyhow!("Failed to set color"))?;
            }
        }

        match item {
            Message::Press { button } => {
                println!("Button Press: {:?}", button);
                match button {
                    Button::ControlButton { index: 0 } => {
                        println!("Volume: {:?}", platform::get_system_volume());
                    }
                    Button::GridButton { x, y } if x <= 7 && y <= 7 => {
                        let idx = (y * 8 + x) as f32;
                        println!("Grid button: {}, {} ({})", x, y, idx);
                        platform::set_system_volume(idx / 64.0)?;
                    }
                    _ => {}
                }
            }
            Message::Release { button } => println!("Button Press: {:?}", button),
            _ => println!("Message: {:?}", item),
        }

        canvas.flush()?;
    }

    Ok(())
}
