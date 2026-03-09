// types.rs — BASS 2.4 Rust type definitions
// Updated for BASS 2.4.17+ / 2.4.18.3
//
// ─── Breaking layout changes vs the 2.4.16 binding ───────────────────────────
// BassInfo:    fields were 2.3-era; replaced with the correct 2.4 layout
//              (flags + reserved[7] + minbuf/dsver/latency/initflags/speakers/freq).
// BassSample:  `voice_allocation_flags` + `priority` replaced by `reserved: [DWORD; 2]`
//              to match the current BASS_SAMPLE definition in bass.h.
//
// ─── New types ────────────────────────────────────────────────────────────────
// BassFileOpenProcs  — extended file-proc table with an `open` callback
// TagBinary          — binary tag data (BASS_TAG_ID3V2_BINARY etc.)
// TagApeBinary       — binary APE tag entry
// FILEOPENPROC       — file-open callback (used with BassFileOpenProcs)
// DEVICENOTIFYPROC   — device change notification callback
// ─────────────────────────────────────────────────────────────────────────────

use std::os::raw::{c_char, c_int, c_void};

// ─── Primitive types ──────────────────────────────────────────────────────────
pub type BYTE  = u8;
pub type WORD  = u16;
pub type DWORD = u32;
pub type QWORD = u64;
pub type BOOL  = i32;

// ─── Handle types ─────────────────────────────────────────────────────────────
pub type HMUSIC  = DWORD;
pub type HSAMPLE = DWORD;
pub type HCHANNEL = DWORD;
pub type HSTREAM = DWORD;
pub type HRECORD = DWORD;
pub type HSYNC   = DWORD;
pub type HDSP    = DWORD;
pub type HFX     = DWORD;
pub type HPLUGIN = DWORD;

// ─── Callback types ───────────────────────────────────────────────────────────
pub type STREAMPROC    = unsafe extern "system" fn(HSTREAM, *mut c_void, DWORD, *mut c_void) -> DWORD;
pub type FILECLOSEPROC = unsafe extern "system" fn(*mut c_void);
pub type FILELENPROC   = unsafe extern "system" fn(*mut c_void) -> QWORD;
pub type FILEREADPROC  = unsafe extern "system" fn(*mut c_void, DWORD, *mut c_void) -> DWORD;
pub type FILESEEKPROC  = unsafe extern "system" fn(QWORD, *mut c_void) -> BOOL;
/// File-open callback used with [`BassFileOpenProcs`] (added in 2.4.17).
/// Returns an opaque user pointer, or NULL on failure.
pub type FILEOPENPROC  = unsafe extern "system" fn(*const c_char, DWORD) -> *mut c_void;
pub type DOWNLOADPROC  = unsafe extern "system" fn(*mut c_void, DWORD, *mut c_void);
pub type SYNCPROC      = unsafe extern "system" fn(HSYNC, DWORD, DWORD, *mut c_void);
pub type DSPPROC       = unsafe extern "system" fn(HDSP, DWORD, *mut c_void, DWORD, *mut c_void);
pub type RECORDPROC    = unsafe extern "system" fn(HRECORD, *mut c_void, DWORD, *mut c_void) -> BOOL;
pub type IOSNOTIFYPROC = unsafe extern "system" fn(DWORD);
/// Device change notification callback (added in 2.4.17).
pub type DEVICENOTIFYPROC = unsafe extern "system" fn(DWORD);

// ─── Structures ───────────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Debug, Clone)]
pub struct BassDeviceInfo {
    pub name:   *const c_void,
    pub driver: *const c_void,
    pub flags:  DWORD,
}

impl BassDeviceInfo {
    pub fn new(name: *const c_void, driver: *const c_void, flags: DWORD) -> Self {
        Self { name, driver, flags }
    }
}

/// Output device info.
///
/// # Breaking change vs 2.4.16 binding
/// The old binding used legacy 2.3-era field names. The struct now matches
/// the actual `BASS_INFO` layout in bass.h 2.4:
/// `flags + reserved[7] + minbuf + dsver + latency + initflags + speakers + freq`.
#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct BassInfo {
    /// DirectSound capability flags (DSCAPS_xxx).
    pub flags:     DWORD,
    pub reserved:  [DWORD; 7],
    /// Recommended minimum buffer length (ms).
    pub minbuf:    DWORD,
    /// DirectSound version.
    pub dsver:     DWORD,
    /// Average playback latency (ms).
    pub latency:   DWORD,
    /// The `flags` value passed to `BASS_Init`.
    pub initflags: DWORD,
    /// Number of speakers available.
    pub speakers:  DWORD,
    /// Current output sample rate.
    pub freq:      DWORD,
}

#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct BassRecordInfo {
    pub flags:              DWORD,
    pub supported_formats:  DWORD,
    pub number_of_inputs:   DWORD,
    pub single_in:          BOOL,
    pub current_frequency:  DWORD,
}

impl BassRecordInfo {
    pub fn new(
        flags: DWORD,
        supported_formats: DWORD,
        number_of_inputs: DWORD,
        single_in: BOOL,
        current_frequency: DWORD,
    ) -> Self {
        Self { flags, supported_formats, number_of_inputs, single_in, current_frequency }
    }
}

/// Sample info / default attributes.
///
/// # Breaking change vs 2.4.16 binding
/// `voice_allocation_flags` and `priority` removed. They were legacy 2.3 fields
/// not present in bass.h 2.4. Replaced by `reserved: [DWORD; 2]` to preserve
/// the correct ABI layout.
#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct BassSample {
    pub default_frequency:                     DWORD,
    pub volume:                                f32,
    pub pan:                                   f32,
    pub flags:                                 DWORD,
    pub length:                                DWORD,
    pub maximum_simultaneous_playbacks:        DWORD,
    pub original_resolution:                   DWORD,
    pub number_of_channels:                    DWORD,
    pub minimum_gap:                           DWORD,
    pub mode_3d:                               DWORD,
    pub minimum_distance:                      f32,
    pub maximum_distance:                      f32,
    pub angle_of_inside_projection_cone:       DWORD,
    pub angle_of_outside_projection_cone:      DWORD,
    pub volume_delta_of_outside_projection_cone: f32,
    /// Reserved — do not use (replaces legacy voice_allocation_flags + priority).
    pub reserved:                              [DWORD; 2],
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct BassChannelInfo {
    pub default_frequency:  DWORD,
    pub channels:           DWORD,
    pub flags:              DWORD,
    pub type_of_channel:    DWORD,
    pub original_resolution: DWORD,
    pub plugin:             HPLUGIN,
    pub sample:             HSAMPLE,
    pub file_name:          *const c_char,
}

impl BassChannelInfo {
    pub fn new(
        default_frequency: DWORD,
        channels: DWORD,
        flags: DWORD,
        type_of_channel: DWORD,
        original_resolution: DWORD,
        plugin: HPLUGIN,
        sample: HSAMPLE,
        file_name: *const c_char,
    ) -> Self {
        Self {
            default_frequency, channels, flags, type_of_channel,
            original_resolution, plugin, sample, file_name,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct BassPluginForm {
    pub name:                   *const c_void,
    pub file_extension_filter:  *const c_void,
}

impl BassPluginForm {
    pub fn new(name: *const c_void, file_extension_filter: *const c_void) -> Self {
        Self { name, file_extension_filter }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct BassPluginInfo {
    pub version:      DWORD,
    pub format_count: DWORD,
    pub formats:      *mut BassPluginForm,
}

impl BassPluginInfo {
    pub fn new(version: DWORD, format_count: DWORD, formats: *mut BassPluginForm) -> Self {
        Self { version, format_count, formats }
    }
}

#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct Bass3DVector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Bass3DVector {
    pub fn new(x: f32, y: f32, z: f32) -> Self { Self { x, y, z } }
}

/// Standard file-procedure callbacks (close / length / read / seek).
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BassFileProcs {
    pub close_handle:  *mut FILECLOSEPROC,
    pub length_handle: *mut FILELENPROC,
    pub read_handle:   *mut FILEREADPROC,
    pub seek_handle:   *mut FILESEEKPROC,
}

impl BassFileProcs {
    pub fn new(
        close_handle:  *mut FILECLOSEPROC,
        length_handle: *mut FILELENPROC,
        read_handle:   *mut FILEREADPROC,
        seek_handle:   *mut FILESEEKPROC,
    ) -> Self {
        Self { close_handle, length_handle, read_handle, seek_handle }
    }
}

/// Extended file-procedure callbacks including an `open` callback (added in 2.4.17).
/// Used with `BASS_CONFIG_FILEOPENPROCS`.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BassFileOpenProcs {
    pub close_handle:  *mut FILECLOSEPROC,
    pub length_handle: *mut FILELENPROC,
    pub read_handle:   *mut FILEREADPROC,
    pub seek_handle:   *mut FILESEEKPROC,
    pub open_handle:   *mut FILEOPENPROC,
}

impl BassFileOpenProcs {
    pub fn new(
        close_handle:  *mut FILECLOSEPROC,
        length_handle: *mut FILELENPROC,
        read_handle:   *mut FILEREADPROC,
        seek_handle:   *mut FILESEEKPROC,
        open_handle:   *mut FILEOPENPROC,
    ) -> Self {
        Self { close_handle, length_handle, read_handle, seek_handle, open_handle }
    }
}

/// Binary tag data — returned by `BASS_TAG_ID3V2_BINARY`, `BASS_TAG_ID3V2_2_BINARY` (added 2.4.17).
///
/// ⚠ `data` is owned by BASS — do not free it.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TagBinary {
    pub data:   *const c_void,
    pub length: DWORD,
}

impl TagBinary {
    pub fn new(data: *const c_void, length: DWORD) -> Self { Self { data, length } }
}

/// Binary APE tag entry — returned by `BASS_TAG_APE_BINARY + index` (added 2.4.17).
///
/// ⚠ `key` and `data` are owned by BASS — do not free them.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TagApeBinary {
    pub key:    *const c_char,
    pub data:   *const c_void,
    pub length: DWORD,
}

impl TagApeBinary {
    pub fn new(key: *const c_char, data: *const c_void, length: DWORD) -> Self {
        Self { key, data, length }
    }
}

// ─── WAVE format ──────────────────────────────────────────────────────────────

#[repr(C, packed)]
#[derive(Default, Debug, Clone)]
pub struct WaveFormatEx {
    pub format_tag:               WORD,
    pub channels_number:          WORD,
    pub samples_per_second:       DWORD,
    pub average_bytes_per_second: DWORD,
    pub block_align:              WORD,
    pub bits_per_sample:          WORD,
    pub size:                     WORD,
}

impl WaveFormatEx {
    pub fn new(
        format_tag: WORD,
        channels_number: WORD,
        samples_per_second: DWORD,
        average_bytes_per_second: DWORD,
        block_align: WORD,
        bits_per_sample: WORD,
        size: WORD,
    ) -> Self {
        Self {
            format_tag, channels_number, samples_per_second,
            average_bytes_per_second, block_align, bits_per_sample, size,
        }
    }
}

// ─── DX8 FX parameter structures ─────────────────────────────────────────────

#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct BassDx8Chorus {
    pub wet_dry_mix: f32, pub depth: f32, pub feedback: f32,
    pub frequency: f32,   pub waveform: DWORD, pub delay: f32, pub phase: DWORD,
}
impl BassDx8Chorus {
    pub fn new(wet_dry_mix: f32, depth: f32, feedback: f32, frequency: f32, waveform: DWORD, delay: f32, phase: DWORD) -> Self {
        Self { wet_dry_mix, depth, feedback, frequency, waveform, delay, phase }
    }
}

#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct BassDx8Compressor {
    pub gain: f32, pub attack: f32, pub release: f32,
    pub threshold: f32, pub ratio: f32, pub predelay: f32,
}
impl BassDx8Compressor {
    pub fn new(gain: f32, attack: f32, release: f32, threshold: f32, ratio: f32, predelay: f32) -> Self {
        Self { gain, attack, release, threshold, ratio, predelay }
    }
}

#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct BassDx8Distortion {
    pub gain: f32, pub edge: f32, pub post_eq_center_frequency: f32,
    pub post_eq_bandwidth: f32, pub pre_lowpass_cutoff: f32,
}
impl BassDx8Distortion {
    pub fn new(gain: f32, edge: f32, post_eq_center_frequency: f32, post_eq_bandwidth: f32, pre_lowpass_cutoff: f32) -> Self {
        Self { gain, edge, post_eq_center_frequency, post_eq_bandwidth, pre_lowpass_cutoff }
    }
}

#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct BassDx8Echo {
    pub wet_dry_mix: f32, pub feedback: f32,
    pub left_delay: f32, pub right_delay: f32, pub pan_delay: BOOL,
}
impl BassDx8Echo {
    pub fn new(wet_dry_mix: f32, feedback: f32, left_delay: f32, right_delay: f32, pan_delay: BOOL) -> Self {
        Self { wet_dry_mix, feedback, left_delay, right_delay, pan_delay }
    }
}

#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct BassDx8Flanger {
    pub wet_dry_mix: f32, pub depth: f32, pub feedback: f32,
    pub frequency: f32, pub waveform: DWORD, pub delay: f32, pub phase: DWORD,
}
impl BassDx8Flanger {
    pub fn new(wet_dry_mix: f32, depth: f32, feedback: f32, frequency: f32, waveform: DWORD, delay: f32, phase: DWORD) -> Self {
        Self { wet_dry_mix, depth, feedback, frequency, waveform, delay, phase }
    }
}

#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct BassDx8Gargle {
    pub rate_hz: DWORD, pub wave_shape: DWORD,
}
impl BassDx8Gargle {
    pub fn new(rate_hz: DWORD, wave_shape: DWORD) -> Self { Self { rate_hz, wave_shape } }
}

#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct BassDx8I3Dl2Reverb {
    pub room: c_int, pub room_hf: c_int, pub room_rolloff_factor: f32,
    pub decay_time: f32, pub decay_hf_ratio: f32,
    pub reflections: c_int, pub reflections_delay: f32,
    pub reverb: c_int, pub reverb_delay: f32,
    pub diffusion: f32, pub density: f32, pub hf_reference: f32,
}
impl BassDx8I3Dl2Reverb {
    pub fn new(
        room: c_int, room_hf: c_int, room_rolloff_factor: f32, decay_time: f32,
        decay_hf_ratio: f32, reflections: c_int, reflections_delay: f32,
        reverb: c_int, reverb_delay: f32, diffusion: f32, density: f32, hf_reference: f32,
    ) -> Self {
        Self {
            room, room_hf, room_rolloff_factor, decay_time, decay_hf_ratio,
            reflections, reflections_delay, reverb, reverb_delay, diffusion, density, hf_reference,
        }
    }
}

#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct BassDx8ParamEq {
    pub center: f32, pub bandwidth: f32, pub gain: f32,
}
impl BassDx8ParamEq {
    pub fn new(center: f32, bandwidth: f32, gain: f32) -> Self { Self { center, bandwidth, gain } }
}

#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct BassDx8Reverb {
    pub gain: f32, pub reverb_mix: f32, pub reverb_time: f32, pub high_frequency_rt_ratio: f32,
}
impl BassDx8Reverb {
    pub fn new(gain: f32, reverb_mix: f32, reverb_time: f32, high_frequency_rt_ratio: f32) -> Self {
        Self { gain, reverb_mix, reverb_time, high_frequency_rt_ratio }
    }
}

#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct BassFxVolumeParam {
    pub target: f32, pub current: f32, pub time: f32, pub curve: DWORD,
}
impl BassFxVolumeParam {
    pub fn new(target: f32, current: f32, time: f32, curve: DWORD) -> Self {
        Self { target, current, time, curve }
    }
}
