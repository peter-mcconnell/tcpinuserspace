use std::io;
use std::collections::HashMap;
use std::net::Ipv4Addr;
extern crate tun_tap;

mod tcp;

struct Quad {
    src: (Ipv4Addr, u16),
    dest: (Ipv4Addr, u16),
}


fn main() -> io::Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    // MTU of the interface is usually 1500, unless reconfigured + 4 for the header
    let mut buf = [0u8; 1504];
    loop {
        let nbytes = nic.recv(&mut buf[..])?;
        // 3.2 in https://www.kernel.org/doc/Documentation/networking/tuntap.txt # tuntap
        let eth_proto = u16::from_be_bytes([buf[2], buf[3]]);
        // proto numbers in https://en.wikipedia.org/wiki/EtherType
        if eth_proto != 0x0800 {
            // not ipv4
            continue;
        }

        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
            Ok(iph) => {
                let src = iph.source_addr();
                let dest = iph.destination_addr();
                if iph.protocol() != 0x06 {
                    // not tcp
                    continue;
                }

                match etherparse::TcpHeaderSlice::from_slice(&buf[4 + iph.slice().len()..]) {
                    Ok(tcph) => {
                        let data = 4 + iph.slice().len() + tcph.slice().len();
                        connections.entry(Quad {
                            src: (src, tcph.source_port()),
                            dst: (dest, tcph.destination_port()),
                        }).or_default().on_packet(iph, tcph, &buf[p]);
                        eprintln!(
                            "{} -> {} {}b of tcp to port {}",
                            src,
                            dest,
                            tcph.slice().len(),
                            tcph.destination_port()
                        );
                    }
                    Err(e) => {
                        eprintln!("ignoring weird tcp packet {:?}", e);
                    }
            },
            Err(e) => {
                eprintln!("ignoring packet: {:?}", e);
            }
        }
    }
}
