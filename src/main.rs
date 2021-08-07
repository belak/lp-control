use std::collections::{BTreeSet, LinkedList};

use launchy::mini::{self, Button as LButton, Input, Message as LMessage};
use launchy::prelude::*;
use launchy::DeviceCanvas;

mod actions;
mod canvas;
mod layouts;
mod platform;

use actions::Action;
use layouts::Layout;

#[derive(Debug)]
pub enum Color {
    Off,
    DimRed,
    Red,
    Orange,
    Amber,
    Yellow,
    Green,
    DimGreen,
}

impl Into<launchy::Color> for Color {
    fn into(self) -> launchy::Color {
        use launchy::Color as LColor;

        match self {
            Color::Off => LColor::BLACK,
            Color::DimRed => LColor::new(0.3, 0.0, 0.0),
            Color::Red => LColor::RED,
            // Orange on the LP Mini doesn't line up with the LP S docs, so we define our own.
            Color::Orange => LColor::new(1.0, 0.3, 0.0),
            Color::Amber => LColor::YELLOW,
            // Yellow on the LP Mini doesn't line up with the LP S docs, so we define our own.
            Color::Yellow => LColor::new(0.3, 1.0, 0.0),
            Color::Green => LColor::GREEN,
            Color::DimGreen => LColor::new(0.0, 0.3, 0.0),
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum Pad {
    Top { index: u8 },
    Side { index: u8 },
    Grid { x: u8, y: u8 },
}

impl Into<launchy::Pad> for Pad {
    fn into(self) -> launchy::Pad {
        use launchy::Pad as LPad;

        match self {
            Pad::Top { index } => LPad {
                x: index.into(),
                y: 0,
            },
            Pad::Side { index } => LPad {
                x: 8,
                y: index.into(),
            },
            Pad::Grid { x, y } => LPad {
                x: x.into(),
                y: (y + 1).into(),
            },
        }
    }
}

#[derive(Debug)]
pub enum Message {
    Press { pad: Pad },
    Release { pad: Pad },
}

impl From<launchy::CanvasMessage> for Message {
    fn from(msg: launchy::CanvasMessage) -> Self {
        match msg {
            // Press events
            launchy::CanvasMessage::Press { x, y: 0 } => Message::Press {
                pad: Pad::Top { index: x as u8 },
            },
            launchy::CanvasMessage::Press { x: 8, y } => Message::Press {
                pad: Pad::Side { index: y as u8 - 1 },
            },
            launchy::CanvasMessage::Press { x, y } => Message::Press {
                pad: Pad::Grid {
                    x: x as u8,
                    y: y as u8 - 1,
                },
            },

            // Release events
            launchy::CanvasMessage::Release { x, y: 0 } => Message::Release {
                pad: Pad::Top { index: x as u8 },
            },
            launchy::CanvasMessage::Release { x: 8, y } => Message::Release {
                pad: Pad::Side { index: y as u8 - 1 },
            },
            launchy::CanvasMessage::Release { x, y } => Message::Release {
                pad: Pad::Grid {
                    x: x as u8,
                    y: y as u8 - 1,
                },
            },
        }
    }
}

fn setup_layouts<T>() -> Box<dyn Layout<T>>
where
    T: launchy::DeviceSpec,
{
    todo!()
}

fn main() -> anyhow::Result<()> {
    // Create a layout stack and push the root layout onto it.
    //let layout_stack: LinkedList<Box<dyn Layout<mini::Spec>>> = LinkedList::new();
    //layout_stack.push_back(setup_layouts::<mini::Spec>());

    let mut canvas: DeviceCanvas<mini::Spec> = DeviceCanvas::guess(|msg| {
        let msg: Message = msg.into();
        println!("Msg: {:?}", msg);
    })?;
    canvas.clear(); // TODO: this doesn't actually clear the board properly
    canvas.flush()?;

    let input = Input::guess_polling()?;
    input.drain();

    //let mut output = Output::guess()?;
    //output.reset()?;

    let pressed: BTreeSet<Pad> = BTreeSet::new();

    for item in input.iter() {
        let volume = platform::get_system_volume()
            .ok_or_else(|| anyhow::anyhow!("Failed to get system volume"))?;

        println!("Volume: {:?}", volume);

        // Ensure all lights are in the right state
        for x in 0..8 {
            for y in 0..8 {
                let idx = (y * 8 + x) as f32;

                // The currently selected pad should be within 0.01 of the
                // actual value. We can't use == because floats are a tiny bit
                // imprecise.
                let color = if (idx / 63.0 - volume).abs() < 0.01 {
                    Color::Yellow
                } else if idx / 63.0 <= volume {
                    Color::DimGreen
                } else {
                    Color::Off
                };

                canvas
                    .set(launchy::Pad { x, y: y + 1 }, color.into())
                    .ok_or_else(|| anyhow::anyhow!("Failed to set color"))?;
            }

            canvas
                .set(launchy::Pad { x: 8, y: 8 }, Color::Red.into())
                .ok_or_else(|| anyhow::anyhow!("Failed to set color"))?;

            canvas
                .set(launchy::Pad { x: 8, y: 7 }, Color::Green.into())
                .ok_or_else(|| anyhow::anyhow!("Failed to set color"))?;

            // Demo colors
            canvas
                .set(launchy::Pad { x: 1, y: 0 }, Color::DimGreen.into())
                .ok_or_else(|| anyhow::anyhow!("Failed to set color"))?;
            canvas
                .set(launchy::Pad { x: 2, y: 0 }, Color::Green.into())
                .ok_or_else(|| anyhow::anyhow!("Failed to set color"))?;
            canvas
                .set(launchy::Pad { x: 3, y: 0 }, Color::Yellow.into())
                .ok_or_else(|| anyhow::anyhow!("Failed to set color"))?;
            canvas
                .set(launchy::Pad { x: 4, y: 0 }, Color::Amber.into())
                .ok_or_else(|| anyhow::anyhow!("Failed to set color"))?;
            canvas
                .set(launchy::Pad { x: 5, y: 0 }, Color::Orange.into())
                .ok_or_else(|| anyhow::anyhow!("Failed to set color"))?;
            canvas
                .set(launchy::Pad { x: 6, y: 0 }, Color::Red.into())
                .ok_or_else(|| anyhow::anyhow!("Failed to set color"))?;
            canvas
                .set(launchy::Pad { x: 7, y: 0 }, Color::DimRed.into())
                .ok_or_else(|| anyhow::anyhow!("Failed to set color"))?;
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
