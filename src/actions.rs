use std::sync::Arc;

use crate::Layout;

pub enum Action<C> {
    None,
    PopLayout,
    PushLayout(Arc<dyn Layout<C>>),
    ReplaceLayout(Arc<dyn Layout<C>>),
    Fn(Box<Fn() -> anyhow::Result<()>>)
}
