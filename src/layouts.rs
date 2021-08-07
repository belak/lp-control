use std::collections::BTreeMap;

use launchy::DeviceCanvas;

use crate::{Action, Color, Message, Pad};

pub struct Button {
    stale: bool,
    color: Color,
}

pub trait Layout<C>
where
    C: launchy::DeviceSpec,
{
    /// Show will be called right before switching to this layout.
    fn show(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    /// Hide will be called right before switching away from this layout.
    fn hide(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    /// Update is called when this layout is switched to, after every input, and
    /// every 1s. It should return true if any pad states changed.
    fn update(&mut self) -> anyhow::Result<bool> {
        Ok(false)
    }

    /// Input is called when a new message has arrived (generally when a pad was
    /// pressed).
    fn input(&mut self, _msg: Message) -> anyhow::Result<Action<C>> {
        Ok(Action::PopLayout)
    }

    /// Draw is called when the buttons should be updated. It should probably
    /// draw all buttons. The DeviceCanvas will properly cache buttons so setting
    /// all buttons will still be fairly fast. This is the only method which
    /// *must* be implemented.
    fn draw(&mut self, canvas: &mut DeviceCanvas<C>) -> anyhow::Result<()>;
}

pub struct ButtonLayout<C> {
    pads: BTreeMap<Pad, Action<C>>,
}

impl<C> Layout<C> for ButtonLayout<C>
where
    C: launchy::DeviceSpec,
{
    fn draw(&mut self, canvas: &mut DeviceCanvas<C>) -> anyhow::Result<()> {
        todo!()
    }
}

pub struct ValueLayout {}

impl<C> Layout<C> for ValueLayout
where
    C: launchy::DeviceSpec,
{
    fn draw(&mut self, canvas: &mut DeviceCanvas<C>) -> anyhow::Result<()> {
        todo!()
    }
}

// Tabbed Layout allows for switching between different layouts using the
// buttons on the top.
pub struct TabbedLayout<C>
where
    C: launchy::DeviceSpec,
{
    tabs: Vec<Box<dyn Layout<C>>>,
    selected: u8,
}

impl<C> Layout<C> for TabbedLayout<C>
where
    C: launchy::DeviceSpec,
{
    fn show(&mut self) -> anyhow::Result<()> {
        self.selected = 0;
        Ok(())
    }

    fn draw(&mut self, canvas: &mut DeviceCanvas<C>) -> anyhow::Result<()> {
        // Start by setting the top row to blank.
        for i in 0..8 {
            canvas[Pad::Top { index: i }.into()] = Color::Off.into();
        }

        for (i, _) in self.tabs.iter().take(8).enumerate() {
            if i == self.selected as usize {
                canvas[Pad::Top { index: i as u8 }.into()] = Color::Green.into();
            } else {
                canvas[Pad::Top { index: i as u8 }.into()] = Color::DimGreen.into();
            }
        }

        Ok(())
    }

    fn input(&mut self, msg: Message) -> anyhow::Result<Action<C>> {
        match msg {
            Message::Press {
                pad: Pad::Top { index },
            } => {
                if (index as usize) < self.tabs.len() {
                    self.selected = index
                }

                Ok(Action::None)
            }
            _ => self
                .tabs
                .get_mut(self.selected as usize)
                .map_or(Ok(Action::None), |layout| layout.input(msg)),
        }
    }
}
