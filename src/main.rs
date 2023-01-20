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
        // proto numbers in https://en.wikipedia.org/wiki/EtherType
        if proto != 0x0800 {
            // not ipv4
            continue;
        }

        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
            Ok(p) => {
                eprintln!("read {} bytes: (flags: {:x}, proto: {:x}) {:?}", flags, proto, nbytes - 4, p);
            },
            Err(e) => {
                eprintln!("ignoring packet: {:?}", e);
            }
        }
    }
    Ok(())
}
