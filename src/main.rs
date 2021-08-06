mod actions;
mod layouts;
mod platform;

use launchy::mini::{self, Button as LButton, Input, Message as LMessage};
use launchy::prelude::*;
use launchy::DeviceCanvas;

#[derive(Debug)]
pub enum Color {
    Off,
    Red,
    Orange,
    Amber,
    Yellow,
    Green,
}

impl Into<launchy::Color> for Color {
    fn into(self) -> launchy::Color {
        use launchy::Color as LColor;

        match self {
            Color::Off => LColor::BLACK,
            Color::Red => LColor::RED,
            Color::Orange => LColor::new(1.0, 0.3, 0.0),
            Color::Amber => LColor::YELLOW,
            Color::Yellow => LColor::new(0.3, 1.0, 0.0),
            Color::Green => LColor::GREEN,
        }
    }
}

#[derive(Debug)]
pub enum Button {
    TopButton { index: u8 },
    SideButton { index: u8 },
    GridButton { x: u8, y: u8 },
}

#[derive(Debug)]
pub enum Message {
    Press { button: Button },
    Release { button: Button },
}

impl From<launchy::CanvasMessage> for Message {
    fn from(msg: launchy::CanvasMessage) -> Self {
        match msg {
            // Press events
            launchy::CanvasMessage::Press { x, y: 0 } => Message::Press {
                button: Button::TopButton { index: x as u8 },
            },
            launchy::CanvasMessage::Press { x: 8, y } => Message::Press {
                button: Button::SideButton { index: y as u8 - 1 },
            },
            launchy::CanvasMessage::Press { x, y } => Message::Press {
                button: Button::GridButton {
                    x: x as u8,
                    y: y as u8 - 1,
                },
            },

            // Release events
            launchy::CanvasMessage::Release { x, y: 0 } => Message::Release {
                button: Button::TopButton { index: x as u8 },
            },
            launchy::CanvasMessage::Release { x: 8, y } => Message::Release {
                button: Button::SideButton { index: y as u8 - 1 },
            },
            launchy::CanvasMessage::Release { x, y } => Message::Release {
                button: Button::GridButton {
                    x: x as u8,
                    y: y as u8 - 1,
                },
            },
        }
    }
}

fn main() -> anyhow::Result<()> {
    let mut canvas: DeviceCanvas<mini::Spec> = DeviceCanvas::guess(|msg| {
        let msg: Message = msg.into();
        println!("Msg: {:?}", msg);
    })?;
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

                let color = if (idx / 63.0 - volume).abs() < 0.01 {
                    Color::Yellow
                } else if idx / 63.0 <= volume {
                    Color::Green
                } else {
                    Color::Off
                };

                canvas
                    .set(Pad { x, y: y + 1 }, color.into())
                    .ok_or_else(|| anyhow::anyhow!("Failed to set color"))?;
            }
        }

        match item {
            LMessage::Press { button } => match button {
                LButton::ControlButton { index: 0 } => {
                    //println!("Volume: {:?}", platform::get_system_volume());
                }
                LButton::GridButton { x, y } if x <= 7 && y <= 7 => {
                    let idx = (y * 8 + x) as f32;
                    //println!("Grid button press: {}, {} ({})", x, y, idx);

                    platform::set_system_volume(idx / 63.0)?;
                }
                _ => {
                    //println!("Button Press: {:?}", button);
                }
            },
            LMessage::Release { button } => {
                //println!("Button Release: {:?}", button);
            }
            _ => {
                println!("Message: {:?}", item);
            }
        }

        canvas.flush()?;
    }

    Ok(())
}
