use libc::c_char;
use std::ffi::CStr;
use tracing::info;

extern "C" {
    static mut bootcoretype: [u8; 64];
    static mut btimeout: u16;
}

#[no_mangle]
pub unsafe extern "C" fn bootcore_init(path: *const u8) {
    let path = CStr::from_ptr(path as *const c_char).to_str().ok();
    let path = if path == Some("") { None } else { Some(path) };
    info!("bootcore_init: path = {path:?}");

    // TODO: figure out why this is needed, and why is this here.
    let timeout = mister_fpga::config::cfg_bootcore_timeout() * 10;
    mister_fpga::config::cfg_set_bootcore_timeout(timeout);
    btimeout = mister_fpga::config::cfg_bootcore_timeout();

    info!("bootcore_init: timeout = {timeout}");

    let bootcore = mister_fpga::config::Config::bootcore();
    info!("bootcore = {bootcore:?}");

    if bootcore.is_last_core() {
        let bootcore_str = match bootcore {
            mister_fpga::config::BootCoreConfig::LastCore => "lastcore",
            mister_fpga::config::BootCoreConfig::ExactLastCore => "lastexactcore",
            _ => unreachable!(),
        };
        bootcoretype[..bootcore_str.len()].copy_from_slice(bootcore_str.as_bytes());
        bootcoretype[bootcore_str.len()] = 0; // Make sure we're NUL terminated.
    }
}
