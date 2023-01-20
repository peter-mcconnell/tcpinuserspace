use std::io;
extern crate tun_tap;

fn main() -> io::Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    loop {
        let mut buf = [0u8; 1504];
        let nbytes = nic.recv(&mut buf[..])?;
        eprintln!("read {} bytes: {:x?}", nbytes, &buf[..nbytes]);
    }
    Ok(())
}
