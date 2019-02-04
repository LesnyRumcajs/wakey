//! Library for managing Wake-on-LAN packets.
//! # Example
//! ```
//! extern crate wakey;
//! let wol = wakey::WolPacket::from_string("01:02:03:04:05:06", ':');
//! match wol.send_magic() {
//!     Ok(_) => println!("Sent the magic packet!"),
//!     Err(_) => println!("Failed to send the magic packet!")
//! }
//! ```

extern crate hex;

use std::iter;
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};

const MAC_SIZE: usize = 6;
const MAC_PER_MAGIC: usize = 16;
static HEADER: [u8; 6] = [0xFF; 6];

/// Wake-on-LAN packet
pub struct WolPacket {
    /// WOL packet bytes
    packet: Vec<u8>,
}

impl WolPacket {
    /// Creates WOL packet from byte MAC representation
    /// # Example
    /// ```
    /// extern crate wakey;
    /// let wol = wakey::WolPacket::from_bytes(&vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05]);
    /// ```
    pub fn from_bytes(mac: &[u8]) -> WolPacket {
        WolPacket { packet: WolPacket::create_packet_bytes(mac) }
    }

    /// Creates WOL packet from string MAC representation (e.x. 00:01:02:03:04:05)
    /// # Example
    /// ```
    /// extern crate wakey;
    /// let wol = wakey::WolPacket::from_string("00:01:02:03:04:05", ':');
    /// ```
    pub fn from_string(data: &str, sep: char) -> WolPacket {
        WolPacket::from_bytes(&WolPacket::mac_to_byte(data, sep))
    }

    /// Broadcasts the magic packet from / to default address
    /// Source: 0.0.0.0:0
    /// Destination 255.255.255.255:9
    /// # Example
    /// ```
    /// extern crate wakey;
    /// let wol = wakey::WolPacket::from_bytes(&vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05]);
    /// wol.send_magic();
    /// ```
    pub fn send_magic(&self) -> std::io::Result<()> {
        self.send_magic_to(
            SocketAddr::from(([0, 0, 0, 0], 0)),
            SocketAddr::from(([255, 255, 255, 255], 9)),
        )
    }

    /// Broadcasts the magic packet from / to specified address.
    /// # Example
    /// ```
    /// extern crate wakey;
    /// use std::net::SocketAddr;
    /// let wol = wakey::WolPacket::from_bytes(&vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05]);
    /// let src = SocketAddr::from(([0,0,0,0], 0));
    /// let dst = SocketAddr::from(([255,255,255,255], 9));
    /// wol.send_magic_to(src, dst);
    /// ```
    pub fn send_magic_to<A: ToSocketAddrs>(&self, src: A, dst: A) -> std::io::Result<()> {
        let udp_sock = UdpSocket::bind(src)?;
        udp_sock.set_broadcast(true)?;
        udp_sock.send_to(&self.packet, dst)?;

        Ok(())
    }

    /// Converts string representation of MAC address (e.x. 00:01:02:03:04:05) to raw bytes.
    /// Panics when input MAC is invalid (i.e. contains non-byte characters)
    fn mac_to_byte(data: &str, sep: char) -> Vec<u8> {
        data.split(sep)
            .map(|x| hex::decode(x).expect("Invalid mac!"))
            .flatten()
            .collect()
    }

    /// Extends the MAC address to fill the magic packet
    fn extend_mac(mac: &[u8]) -> Vec<u8> {
        iter::repeat(mac)
            .take(MAC_PER_MAGIC)
            .flatten()
            .cloned()
            .collect()
    }

    /// Creates bytes of the magic packet from MAC address
    fn create_packet_bytes(mac: &[u8]) -> Vec<u8> {
        let mut packet = Vec::with_capacity(HEADER.len() + MAC_SIZE * MAC_PER_MAGIC);

        packet.extend(HEADER.iter());
        packet.extend(WolPacket::extend_mac(mac));

        packet
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn extend_mac_test() {
        let mac = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06];

        let extended_mac = super::WolPacket::extend_mac(&mac);

        assert_eq!(extended_mac.len(), super::MAC_PER_MAGIC * super::MAC_SIZE);
        assert_eq!(&extended_mac[(super::MAC_PER_MAGIC - 1) * super::MAC_SIZE..], &mac[..]);
    }

    #[test]
    fn mac_to_byte_test() {
        let mac = "01:02:03:04:05:06";
        let result = super::WolPacket::mac_to_byte(mac, ':');

        assert_eq!(result, vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);
    }

    #[test]
    #[should_panic]
    fn mac_to_byte_invalid_chars_test() {
        let mac = "ZZ:02:03:04:05:06";
        super::WolPacket::mac_to_byte(mac, ':');
    }

    #[test]
    #[should_panic]
    fn mac_to_byte_invalid_separator_test() {
        let mac = "01002:03:04:05:06";
        super::WolPacket::mac_to_byte(mac, ':');
    }

    #[test]
    fn create_packet_bytes_test() {
        let bytes = super::WolPacket::create_packet_bytes(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);

        assert_eq!(bytes.len(), super::MAC_SIZE * super::MAC_PER_MAGIC + super::HEADER.len());
        assert!(bytes.iter().all(|&x| x == 0xFF));
    }
}
