use std::collections::BTreeMap;
use std::sync::Arc;

use launchy::DeviceCanvas;

use crate::{Action, Message, Pad};

pub enum LayoutAction<C> {
    None,
    Pop,
    Push(Arc<Box<dyn Layout<C>>>),
    Replace(Arc<Box<dyn Layout<C>>>),
}

pub trait Layout<C>
where
    C: launchy::DeviceSpec,
{
    // Update is called when this layout is switched to and after every input.
    // It should return true if any pad states changed.
    fn update(&mut self) -> anyhow::Result<bool>;

    // Draw is called when the buttons should be updated. It should probably
    // draw all buttons.
    fn draw(&mut self, canvas: &mut DeviceCanvas<C>) -> anyhow::Result<()>;

    // Input is called when a new message has arrived (generally when a pad was
    // pressed).
    fn input(&mut self, msg: Message) -> anyhow::Result<LayoutAction<C>>;
}

pub struct ButtonLayout {
    pads: BTreeMap<Pad, Box<dyn Action>>,
}

impl<C> Layout<C> for ButtonLayout
where
    C: launchy::DeviceSpec,
{
    fn update(&mut self) -> anyhow::Result<bool> {
        todo!()
    }

    fn draw(&mut self, canvas: &mut DeviceCanvas<C>) -> anyhow::Result<()> {
        todo!()
    }

    fn input(&mut self, msg: Message) -> anyhow::Result<LayoutAction<C>> {
        todo!()
    }
}

#[derive(Debug)]
pub struct ValueLayout {}

impl<C> Layout<C> for ValueLayout
where
    C: launchy::DeviceSpec,
{
    fn update(&mut self) -> anyhow::Result<bool> {
        todo!()
    }

    fn draw(&mut self, canvas: &mut DeviceCanvas<C>) -> anyhow::Result<()> {
        todo!()
    }

    fn input(&mut self, msg: Message) -> anyhow::Result<LayoutAction<C>> {
        todo!()
    }
}
