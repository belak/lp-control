pub struct Canvas<CS>
where
    CS: launchy::DeviceSpec,
{
    inner: launchy::DeviceCanvas<CS>,
}

impl<CS> Canvas<CS>
where
    CS: launchy::DeviceSpec,
{
    fn set(&mut self, ) -> anyhow::Result<()> {
        Ok(())
    }
}
