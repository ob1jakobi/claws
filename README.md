# claws
<p>
A Rust-based Terminal User Interface for packet analysis similar to Wireshark. Named "Claws" because there's no 
crab-based scary movie involving crabs that I know off the top of my head, and claws rhymes with "Jaws" - so why not?!
</p>
<p>
This project is primarily intended to help me understand the packet capture process. In order to build a rust-based clone
of Wireshark, this is also intended to help me learn Wireshark too.
</p>
<p>
<b>Note: This is a Debian-based Linux project, and won't work on Windows-based machines.</b>
</p>

## Dependencies:
* Rust Crates:
  * `pcap`: allows access to packet sniffing capabilities of `libpcap`.
    * `libpcap-dev`: installs the libraries and header files for the `libpcap` library.
  * `pnet`: allows for the parsing/information extraction of captured packets.
