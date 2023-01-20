use std::io;
extern crate tun_tap;

fn main() -> io::Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    // MTU of the interface is usually 1500, unless reconfigured + 4 for the header
    let mut buf = [0u8; 1504];
    loop {
        let nbytes = nic.recv(&mut buf[..])?;
        // 3.2 in https://www.kernel.org/doc/Documentation/networking/tuntap.txt # tuntap
        let flags = u16::from_be_bytes([buf[0], buf[1]]);
        let proto = u16::from_be_bytes([buf[2], buf[3]]);
        eprintln!("read {} bytes: (flags: {:x}, proto: {:x}) {:x?}", flags, proto, nbytes - 4, &buf[4..nbytes]);
    }
    Ok(())
}
