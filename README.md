tcp in rust; in userspace
=========================

Just a bit of fun to learn Rust.

References
----------

 - https://www.youtube.com/watch?v=bzja9fQWzdA&ab_channel=JonGjengset
 - https://www.rfc-editor.org/rfc/rfc793  # tcp
 - https://www.rfc-editor.org/rfc/rfc791  # ip
 - https://www.rfc-editor.org/rfc/rfc7414.html
 - https://www.rfc-editor.org/rfc/rfc1180
 - https://www.rfc-editor.org/rfc/rfc1122
 - https://www.rfc-editor.org/rfc/rfc2398

Debug tools
-----------

```sh
sudo apt install tshark

# monitor our interface
sudo tshark -i tun0
```

Quick Play
----------

```sh
./run.sh
```

Send packets to some address on our interface

```sh
ping -I tun0 192.168.0.2
```

Manual approach
---------------

Set capabilities

```sh
sudo setcap CAP_NET_ADMIN=eip $CARGO_TARGET_DIR/release/tcpinuserspace
```

Run binary

```sh
$CARGO_TARGET_DIR/release/tcpinuserspace
```

Check devices

```sh
# look for tun0
ip addr
```

Set address

```sh
sudo ip addr add 192.168.0.1/24 dev tun0
# recheck tun0
ip addr
```

Bring it up

```sh
sudo ip link set dev tun0
```
