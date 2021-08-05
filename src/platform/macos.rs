use std::convert::TryInto;
use std::mem;
use std::ptr::null;

use coreaudio_sys::*;

fn default_output_device() -> Option<AudioDeviceID> {
    let property_address = AudioObjectPropertyAddress {
        mSelector: kAudioHardwarePropertyDefaultOutputDevice,
        mScope: kAudioObjectPropertyScopeGlobal,
        mElement: kAudioObjectPropertyElementMaster,
    };

    let audio_device_id: AudioDeviceID = 0;
    let data_size = mem::size_of::<AudioDeviceID>();
    let status = unsafe {
        AudioObjectGetPropertyData(
            kAudioObjectSystemObject,
            &property_address as *const _,
            0,
            null(),
            &data_size as *const _ as *mut _,
            &audio_device_id as *const _ as *mut _,
        )
    };
    if status != kAudioHardwareNoError as i32 {
        return None;
    }

    Some(audio_device_id)
}

pub(crate) fn get_system_volume() -> Option<f32> {
    let output_device = default_output_device()?;

    let property_address = AudioObjectPropertyAddress {
        mSelector: kAudioHardwareServiceDeviceProperty_VirtualMasterVolume,
        mScope: kAudioObjectPropertyScopeOutput,
        mElement: kAudioObjectPropertyElementMaster,
    };

    let has_volume_prop =
        unsafe { AudioObjectHasProperty(output_device, &property_address as *const _) == 1 };

    if !has_volume_prop {
        return None;
    }

    let data_size = mem::size_of::<f32>();
    let system_volume: f32 = 0.0;
    let os_status = unsafe {
        AudioObjectGetPropertyData(
            output_device,
            &property_address,
            0,
            null(),
            &data_size as *const _ as *mut _,
            &system_volume as *const _ as *mut _,
        )
    };
    if os_status != 0 {
        return None;
    }

    Some(system_volume.clamp(0.0, 1.0))
}

pub(crate) fn set_system_volume(volume: f32) -> anyhow::Result<()> {
    let output_device =
        default_output_device().ok_or_else(|| anyhow::anyhow!("Missing default output device"))?;

    let property_address = AudioObjectPropertyAddress {
        mSelector: kAudioHardwareServiceDeviceProperty_VirtualMasterVolume,
        mScope: kAudioObjectPropertyScopeOutput,
        mElement: kAudioObjectPropertyElementMaster,
    };

    let has_volume_prop =
        unsafe { AudioObjectHasProperty(output_device, &property_address as *const _) == 1 };
    if !has_volume_prop {
        anyhow::bail!("Output device missing volume property");
    }

    let is_settable = unsafe {
        let settable: u8 = 0;
        let os_error = AudioObjectIsPropertySettable(
            output_device,
            &property_address as *const _,
            &settable as *const _ as *mut _,
        );
        os_error == 0 && settable == 1
    };
    if !is_settable {
        anyhow::bail!("Output device missing volume property");
    }

    let data_size = mem::size_of_val::<f32>(&volume);
    let os_status = unsafe {
        AudioObjectSetPropertyData(
            output_device,
            &property_address,
            0,
            null(),
            data_size.try_into().unwrap(),
            &volume as *const _ as *mut _,
        )
    };
    if os_status != 0 {
        anyhow::bail!("Failed to set property");
    }

    Ok(())
}
