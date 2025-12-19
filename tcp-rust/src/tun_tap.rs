use std::ffi::CStr;
use std::fs::{File, OpenOptions};
use std::io::{Error, Read, Result, Write};
use std::os::unix::io::{AsRawFd, RawFd};

use libc::{
    __c_anonymous_ifr_ifru, ifreq, ioctl, IFF_NO_PI, IFF_TAP, IFF_TUN, IFNAMSIZ, TUNSETIFF,
};

unsafe fn tuntap_setup(fd: RawFd, name: &str, mode: Mode, packet_info: bool) -> Result<String> {
    if name.len() > IFNAMSIZ {
        panic!(
            "Interface name cannot be greater than IFNAMSIZ = {}",
            IFNAMSIZ
        )
    }
    let mut ifr: ifreq = ifreq {
        ifr_name: [0; IFNAMSIZ],
        ifr_ifru: __c_anonymous_ifr_ifru { ifru_flags: 0 },
    };
    ifr.ifr_name[..name.len()]
        .copy_from_slice(unsafe { &*(name.as_bytes() as *const [u8] as *const [i8]) });

    match mode {
        Mode::Tun => ifr.ifr_ifru.ifru_flags = IFF_TUN as i16,
        Mode::Tap => ifr.ifr_ifru.ifru_flags = IFF_TAP as i16,
    }
    if !packet_info {
        unsafe {
            ifr.ifr_ifru.ifru_flags |= IFF_NO_PI as i16;
        }
    }
    let ioresult = unsafe { ioctl(fd, TUNSETIFF, &raw const ifr) };

    if ioresult < 0 {
        return Err(Error::last_os_error());
    }
    let new_name =
        unsafe { CStr::from_ptr(&raw const ifr.ifr_name as *const i8) }.to_string_lossy();
    Ok(new_name.to_string())
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Mode {
    Tun = 1,
    Tap = 2,
}

#[derive(Debug)]
pub struct Interface {
    fd: File,
    pub name: String,
}

impl Interface {
    pub fn new(ifname: &str, mode: Mode) -> Result<Self> {
        Interface::with_options(ifname, mode, true)
    }
    pub fn without_packet_info(ifname: &str, mode: Mode) -> Result<Self> {
        Interface::with_options(ifname, mode, false)
    }

    fn with_options(ifname: &str, mode: Mode, packet_info: bool) -> Result<Self> {
        let fd = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/net/tun")?;
        let name = unsafe { tuntap_setup(fd.as_raw_fd(), ifname, mode, packet_info) }?;
        Ok(Interface { fd, name })
    }
}

impl Write for Interface {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.fd.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.fd.flush()
    }
}

impl Read for Interface {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.fd.read(buf)
    }
}
