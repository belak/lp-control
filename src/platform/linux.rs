// Note that we use alsa rather than pulse because pulse has a wrapper which
// acts as an alsa mixer. Because of this we can support both by just supporting
// alsa.
use alsa::*;

pub(crate) fn get_system_volume() -> Option<f32> {
    let mixer = Mixer::new("default", true).ok()?;
    let sid = mixer::SelemId::new("Master", 0);
    let selem = mixer.find_selem(&sid)?;
    let (minv, maxv) = selem.get_playback_volume_range();
    let raw_vol = selem
        .get_playback_volume(mixer::SelemChannelId::mono())
        .ok()?;

    Some(((raw_vol as f32) - minv as f32) / maxv as f32)
}

pub(crate) fn set_system_volume(volume: f32) -> anyhow::Result<()> {
    let mixer = Mixer::new("default", true)?;
    let sid = mixer::SelemId::new("Master", 0);
    let selem = mixer
        .find_selem(&sid)
        .ok_or_else(|| anyhow::anyhow!("Failed to get selem"))?;
    let (minv, maxv) = selem.get_playback_volume_range();

    let final_volume = (volume * (maxv - minv) as f32 + minv as f32) as i64;
    selem.set_playback_volume_all(final_volume)?;

    Ok(())
}
