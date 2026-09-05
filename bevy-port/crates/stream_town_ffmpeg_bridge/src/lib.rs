//! Audited `FFmpeg` allocation helpers that cannot be expressed through
//! `ffmpeg-next`'s safe output constructors.

#[cfg(target_os = "windows")]
use std::{ffi::CString, ptr};

#[cfg(target_os = "windows")]
use anyhow::{Context, Result, bail};
#[cfg(target_os = "windows")]
use ffmpeg_next as ffmpeg;

/// Allocate an `FFmpeg` FIFO pseudo-muxer without opening an `AVIO` socket on the
/// calling thread. The FIFO muxer opens its child output during header writing.
#[cfg(target_os = "windows")]
pub fn allocate_fifo_output(url: &str) -> Result<ffmpeg::format::context::Output> {
    let url = CString::new(url).context("FFmpeg output URL contains a null byte")?;
    let muxer = CString::new("fifo").context("static FIFO muxer name contains a null byte")?;
    let mut context = ptr::null_mut();
    // SAFETY: Both C strings are alive for the call, the output pointer starts
    // null, and ownership of a successful allocation is transferred exactly
    // once into ffmpeg-next's Output wrapper below.
    let result = unsafe {
        ffmpeg::ffi::avformat_alloc_output_context2(
            &raw mut context,
            ptr::null_mut(),
            muxer.as_ptr(),
            url.as_ptr(),
        )
    };
    if result < 0 || context.is_null() {
        if !context.is_null() {
            // SAFETY: A non-null context on this error path is still owned here
            // and has not been wrapped or freed elsewhere.
            unsafe { ffmpeg::ffi::avformat_free_context(context) };
        }
        bail!(
            "linked FFmpeg runtime cannot allocate the FIFO muxer: {}",
            ffmpeg::Error::from(result)
        );
    }
    // SAFETY: FFmpeg returned a unique, initialized AVFormatContext and
    // ffmpeg-next's wrapper assumes ownership of precisely that pointer.
    Ok(unsafe { ffmpeg::format::context::Output::wrap(context) })
}
