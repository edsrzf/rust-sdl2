use std::ptr;
use std::mem;
use std::c_str::CString;
use std::c_vec::CVec;
use libc;
use libc::{c_int, size_t, c_void};
use libc::{uint8_t, uint16_t, uint32_t};
use std::raw::Slice;

use get_error;
use rwops::RWops;
use SdlResult;


#[allow(non_camel_case_types)]
pub mod ll {
    use libc::{c_int, c_uint, c_void, uint8_t, uint32_t};
    use libc::{uint16_t, c_double, c_char};
    use rwops::ll::SDL_RWops;

    // assume LSB
    pub type SDL_AudioFormat = uint16_t;
    pub const AUDIO_U8 : SDL_AudioFormat =         0x0008;
    pub const AUDIO_S8 : SDL_AudioFormat =         0x8008;
    pub const AUDIO_U16LSB : SDL_AudioFormat =     0x0010;
    pub const AUDIO_S16LSB : SDL_AudioFormat =     0x8010;
    pub const AUDIO_U16MSB : SDL_AudioFormat =     0x1010;
    pub const AUDIO_S16MSB : SDL_AudioFormat =     0x9010;
    pub const AUDIO_U16 : SDL_AudioFormat =        AUDIO_U16LSB;
    pub const AUDIO_S16 : SDL_AudioFormat =        AUDIO_S16LSB;
    pub const AUDIO_S32LSB : SDL_AudioFormat =     0x8020;
    pub const AUDIO_S32MSB : SDL_AudioFormat =     0x9020;
    pub const AUDIO_S32 : SDL_AudioFormat =        AUDIO_S32LSB;
    pub const AUDIO_F32LSB : SDL_AudioFormat =     0x8120;
    pub const AUDIO_F32MSB : SDL_AudioFormat =     0x9120;
    pub const AUDIO_F32 : SDL_AudioFormat =        AUDIO_F32LSB;
    pub const AUDIO_U16SYS : SDL_AudioFormat =     AUDIO_U16LSB;
    pub const AUDIO_S16SYS : SDL_AudioFormat =     AUDIO_S16LSB;
    pub const AUDIO_S32SYS : SDL_AudioFormat =     AUDIO_S32LSB;
    pub const AUDIO_F32SYS : SDL_AudioFormat =     AUDIO_F32LSB;

    pub type SDL_AudioCallback =
        ::std::option::Option<extern "C" fn
                                  (arg1: *const c_void, arg2: *const uint8_t,
                                   arg3: c_int)>;
    #[repr(C)]
    pub struct SDL_AudioSpec {
        pub freq: c_int,
        pub format: SDL_AudioFormat,
        pub channels: uint8_t,
        pub silence: uint8_t,
        pub samples: uint16_t,
        pub padding: uint16_t,
        pub size: uint32_t,
        pub callback: SDL_AudioCallback,
        pub userdata: *const c_void,
    }
    pub type SDL_AudioFilter =
        ::std::option::Option<extern "C" fn
                                  (arg1: *const SDL_AudioCVT,
                                   arg2: SDL_AudioFormat)>;
    #[allow(dead_code)]
    #[repr(C)]
    pub struct SDL_AudioCVT {
        pub needed: c_int,
        pub src_format: SDL_AudioFormat,
        pub dst_format: SDL_AudioFormat,
        pub rate_incr: c_double,
        pub buf: *mut uint8_t,
        pub len: c_int,
        pub len_cvt: c_int,
        pub len_mult: c_int,
        pub len_ratio: c_double,
        filters: [SDL_AudioFilter, ..10u],
        filter_index: c_int,
    }
    pub type SDL_AudioDeviceID = uint32_t;
    pub type SDL_AudioStatus = c_uint;
    pub const SDL_AUDIO_STOPPED: c_uint = 0;
    pub const SDL_AUDIO_PLAYING: c_uint = 1;
    pub const SDL_AUDIO_PAUSED: c_uint = 2;
    extern "C" {
        pub fn SDL_GetNumAudioDrivers() -> c_int;
        pub fn SDL_GetAudioDriver(index: c_int) -> *const c_char;
        pub fn SDL_AudioInit(driver_name: *const c_char) -> c_int;
        pub fn SDL_AudioQuit();
        pub fn SDL_GetCurrentAudioDriver() -> *const c_char;
        pub fn SDL_OpenAudio(desired: *const SDL_AudioSpec,
                             obtained: *const SDL_AudioSpec) -> c_int;
        pub fn SDL_GetNumAudioDevices(iscapture: c_int) -> c_int;
        pub fn SDL_GetAudioDeviceName(index: c_int, iscapture: c_int) -> *const c_char;
        pub fn SDL_OpenAudioDevice(device: *const c_char, iscapture: c_int,
                                   desired: *const SDL_AudioSpec,
                                   obtained: *const SDL_AudioSpec,
                                   allowed_changes: c_int) -> SDL_AudioDeviceID;
        pub fn SDL_GetAudioStatus() -> SDL_AudioStatus;
        pub fn SDL_GetAudioDeviceStatus(dev: SDL_AudioDeviceID) ->
            SDL_AudioStatus;
        pub fn SDL_PauseAudio(pause_on: c_int);
        pub fn SDL_PauseAudioDevice(dev: SDL_AudioDeviceID, pause_on: c_int);
        pub fn SDL_LoadWAV_RW(src: *const SDL_RWops, freesrc: c_int,
                              spec: *const SDL_AudioSpec,
                              audio_buf: *const *const uint8_t, audio_len: *const uint32_t) -> *const SDL_AudioSpec;
        pub fn SDL_FreeWAV(audio_buf: *const uint8_t);
        pub fn SDL_BuildAudioCVT(cvt: *mut SDL_AudioCVT,
                                 src_format: SDL_AudioFormat, src_channels: uint8_t,
                                 src_rate: c_int, dst_format: SDL_AudioFormat,
                                 dst_channels: uint8_t, dst_rate: c_int) -> c_int;
        pub fn SDL_ConvertAudio(cvt: *mut SDL_AudioCVT) -> c_int;
        pub fn SDL_MixAudio(dst: *const uint8_t, src: *const uint8_t, len: uint32_t,
                            volume: c_int);
        pub fn SDL_MixAudioFormat(dst: *const uint8_t, src: *const uint8_t,
                                  format: SDL_AudioFormat, len: uint32_t,
                                  volume: c_int);
        pub fn SDL_LockAudio();
        pub fn SDL_LockAudioDevice(dev: SDL_AudioDeviceID);
        pub fn SDL_UnlockAudio();
        pub fn SDL_UnlockAudioDevice(dev: SDL_AudioDeviceID);
        pub fn SDL_CloseAudio();
        pub fn SDL_CloseAudioDevice(dev: SDL_AudioDeviceID);
    }

}


pub type AudioFormat = ll::SDL_AudioFormat;

pub const AUDIOU8     : AudioFormat = ll::AUDIO_U8;
pub const AUDIOS8     : AudioFormat = ll::AUDIO_S8;
pub const AUDIOU16LSB : AudioFormat = ll::AUDIO_U16LSB;
pub const AUDIOS16LSB : AudioFormat = ll::AUDIO_S16LSB;
pub const AUDIOU16MSB : AudioFormat = ll::AUDIO_U16MSB;
pub const AUDIOS16MSB : AudioFormat = ll::AUDIO_S16MSB;
pub const AUDIOU16    : AudioFormat = ll::AUDIO_U16;
pub const AUDIOS16    : AudioFormat = ll::AUDIO_S16;
pub const AUDIOS32LSB : AudioFormat = ll::AUDIO_S32LSB;
pub const AUDIOS32MSB : AudioFormat = ll::AUDIO_S32MSB;
pub const AUDIOS32    : AudioFormat = ll::AUDIO_S32;
pub const AUDIOF32LSB : AudioFormat = ll::AUDIO_F32LSB;
pub const AUDIOF32MSB : AudioFormat = ll::AUDIO_F32MSB;
pub const AUDIOF32    : AudioFormat = ll::AUDIO_F32;
pub const AUDIOU16SYS : AudioFormat = ll::AUDIO_U16SYS;
pub const AUDIOS16SYS : AudioFormat = ll::AUDIO_S16SYS;
pub const AUDIOS32SYS : AudioFormat = ll::AUDIO_S32SYS;
pub const AUDIOF32SYS : AudioFormat = ll::AUDIO_F32SYS;

#[repr(C)]
#[deriving(Clone, PartialEq, Hash, Show, FromPrimitive)]
pub enum AudioStatus {
    Stopped = ll::SDL_AUDIO_STOPPED as int,
    Playing = ll::SDL_AUDIO_PLAYING as int,
    Paused  = ll::SDL_AUDIO_PAUSED  as int,
}

pub fn get_num_audio_drivers() -> int {
    unsafe { ll::SDL_GetNumAudioDrivers() as int }
}

pub fn get_audio_driver(index: int) -> String {
    unsafe {
        let buf = ll::SDL_GetAudioDriver(index as c_int);
        CString::new(buf, false).as_str().unwrap().into_string()
    }
}

pub fn get_num_audio_devices(iscapture: int) -> int {
    unsafe { ll::SDL_GetNumAudioDevices(iscapture as c_int) as int }
}

pub fn get_audio_device_name(index: int, iscapture: int) -> String {
    unsafe {
        let buf = ll::SDL_GetAudioDeviceName(index as c_int, iscapture as c_int);
        CString::new(buf, false).as_str().unwrap().into_string()
    }
}

pub fn audio_init(name: &str) -> SdlResult<()> {
    let ret = name.with_c_str(|buf| {
            unsafe { ll::SDL_AudioInit(buf) }
        });
    if ret == 0 {
        Ok(())
    } else {
        Err(get_error())
    }
}

pub fn audio_quit() {
    unsafe { ll::SDL_AudioQuit() }
}

pub fn get_current_audio_driver() -> String {
    unsafe {
        let buf = ll::SDL_GetCurrentAudioDriver();
        CString::new(buf, false).as_str().unwrap().into_string()
    }
}

pub struct AudioSpec<'a > {
    pub freq: c_int,
    pub format: AudioFormat,
    pub channels: uint8_t,
    pub silence: uint8_t,
    pub samples: uint16_t,
    pub padding: uint16_t,
    pub size: uint32_t,
    c_callback: ll::SDL_AudioCallback,
    pub callback: Option<&'a |&mut [u8]|:'a>, // same size as *c_void
}

extern "C" fn c_audio_callback(userdata: *const c_void, stream: *const uint8_t, len: c_int) {
    unsafe {
        let f : &mut |&mut [u8]| = mem::transmute(userdata);

        // FIXME: lifetime error in calling
        //slice::raw::mut_buf_as_slice(stream as *mut u8, len as uint, *f)
        (*f)(mem::transmute(Slice {
            data: stream,
            len: len as uint
        }))
    }
}


impl<'a> AudioSpec<'a> {
    pub fn load_wav(path: &Path) -> SdlResult<(AudioSpec<'a>, CVec<u8>)> {
        let ops = try!(RWops::from_file(path, "rb"));
        AudioSpec::load_wav_rw(&ops)
    }

    pub fn load_wav_rw(src: &RWops) -> SdlResult<(AudioSpec<'a>, CVec<u8>)> {
        assert_eq!(mem::size_of::<AudioSpec>(), mem::size_of::<ll::SDL_AudioSpec>());
        let mut spec = unsafe { mem::uninitialized::<AudioSpec>() };
        let audio_buf = ptr::null::<u8>();
        let audio_len = 0u32;
        unsafe {
            let ret = ll::SDL_LoadWAV_RW(src.raw(), 0, mem::transmute(&spec), &audio_buf, &audio_len);
            if ret.is_null() {
                Err(get_error())
            } else {
                let v = CVec::new_with_dtor(audio_buf as *mut u8, audio_len as uint, proc() {
                    ll::SDL_FreeWAV(audio_buf)
                });
                spec.c_callback = Some(c_audio_callback);
                Ok((spec, v))
            }
        }
    }
}

pub type AudioDeviceID = ll::SDL_AudioDeviceID;

// use rust's type system to make it right.
pub enum AudioDevice{
    PlaybackDevice(AudioDeviceID),
    RecordingDevice(AudioDeviceID),
}

impl AudioDevice {
    fn to_id(self) -> AudioDeviceID {
        match self {
            AudioDevice::PlaybackDevice(id)  => id,
            AudioDevice::RecordingDevice(id) => id
        }
    }

    pub fn open<'a>(device: Option<&str>, iscapture: int, spec: &AudioSpec)
        -> SdlResult<(AudioDevice, AudioSpec<'a>)> {
        //! SDL_OpenAudioDevice
        let obtained = unsafe { mem::uninitialized::<AudioSpec>() };
        unsafe {
            let device_c_str = match device {
                None => ptr::null(),
                Some(device) => device.to_c_str().unwrap(),
            };
            let ret = ll::SDL_OpenAudioDevice(device_c_str,
                                              iscapture as c_int,
                                              mem::transmute(spec),
                                              mem::transmute(&obtained),
                                              0);
            if ret == 0 {
                Err(get_error())
            } else {
                if iscapture == 0 { // plaback device
                    Ok((AudioDevice::PlaybackDevice(ret as AudioDeviceID), obtained))
                } else {
                    Ok((AudioDevice::RecordingDevice(ret as AudioDeviceID), obtained))
                }
            }
        }
    }

    pub fn get_status(self) -> AudioStatus {
        unsafe {
            let status = ll::SDL_GetAudioDeviceStatus(self.to_id());
            FromPrimitive::from_int(status as int).unwrap()
        }
    }

    pub fn pause(self) {
        unsafe { ll::SDL_PauseAudioDevice(self.to_id(), 1) }
    }

    pub fn resume(self) {
        unsafe { ll::SDL_PauseAudioDevice(self.to_id(), 0) }
    }

    pub fn lock(self) {
        unsafe { ll::SDL_LockAudioDevice(self.to_id()) }
    }

    pub fn unlock(self) {
        unsafe { ll::SDL_UnlockAudioDevice(self.to_id()) }
    }

    pub fn close(self) {
        //! Shut down audio processing and close the audio device.
        unsafe { ll::SDL_CloseAudioDevice(self.to_id()) }
    }
}

#[deriving(PartialEq)] #[allow(raw_pointer_deriving)]
pub struct AudioCVT {
    raw: *mut ll::SDL_AudioCVT,
    owned: bool,
}

impl_raw_accessors!(AudioCVT, *mut ll::SDL_AudioCVT)
impl_owned_accessors!(AudioCVT, owned)

impl Drop for AudioCVT {
    fn drop(&mut self) {
        if self.owned {
            unsafe { libc::free(self.raw as *mut c_void) }
        }
    }
}

impl AudioCVT {
    pub fn new(src_format: AudioFormat, src_channels: u8, src_rate: int,
               dst_format: AudioFormat, dst_channels: u8, dst_rate: int) -> SdlResult<AudioCVT> {
        unsafe {
            let c_cvt_p = libc::malloc(mem::size_of::<ll::SDL_AudioCVT>() as size_t) as *mut ll::SDL_AudioCVT;
            let ret = ll::SDL_BuildAudioCVT(c_cvt_p,
                                            src_format, src_channels, src_rate as c_int,
                                            dst_format, dst_channels, dst_rate as c_int);
            if ret == 1 || ret == 0 {
                Ok(AudioCVT { raw: c_cvt_p, owned: true })
            } else {
                Err(get_error())
            }
        }
    }

    pub fn convert(&self, src: CVec<u8>) -> SdlResult<CVec<u8>> {
        //! Convert audio data to a desired audio format.

        unsafe {
            if (*self.raw).needed != 1 {
                return Err("no convertion needed!".into_string())
            }
            // set len
            (*self.raw).len = src.len() as c_int;
            // alloc buf
            let size = (*self.raw).len * (*self.raw).len_mult;
            (*self.raw).buf = libc::malloc(size as size_t) as *mut u8;
            // set buf
            ptr::copy_memory::<u8>((*self.raw).buf, src.as_slice().as_ptr(), src.len());
            // convert
            let ret = ll::SDL_ConvertAudio(self.raw);
            // return
            let p = (*self.raw).buf as *mut c_void; // send to proc()
            if ret == 0 {
                Ok( CVec::new_with_dtor((*self.raw).buf as *mut u8, (*self.raw).len_cvt as uint,
                                        proc() { libc::free(p) })
                    )
            } else {
                Err(get_error())
            }
        }
    }
}
