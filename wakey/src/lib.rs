//! Library for managing Wake-on-LAN packets.
//! # Example
//! ```
//! let wol = wakey::WolPacket::from_string("01:02:03:04:05:06", ':').unwrap();
//! if wol.send_magic().is_ok() {
//!     println!("Sent the magic packet!");
//! } else {
//!     println!("Failed to send the magic packet!");
//! }
//! ```

use std::error::Error;
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use std::{fmt, iter};

use arrayvec::ArrayVec;

const MAC_SIZE: usize = 6;
const MAC_PER_MAGIC: usize = 16;
const HEADER: [u8; 6] = [0xFF; 6];
const PACKET_LEN: usize = HEADER.len() + MAC_SIZE * MAC_PER_MAGIC;

type Packet = ArrayVec<u8, PACKET_LEN>;
type Mac = ArrayVec<u8, MAC_SIZE>;

/// Wrapper `Result` for the module errors.
pub type Result<T> = std::result::Result<T, WakeyError>;

/// Wrapper Error for fiascoes occuring in this module.
#[derive(Debug)]
pub enum WakeyError {
    /// The provided MAC address has invalid length.
    InvalidMacLength,
    /// The provided MAC address has invalid format.
    InvalidMacFormat,
    /// There was an error sending the WoL packet.
    SendFailure(std::io::Error),
}

impl Error for WakeyError {}

impl fmt::Display for WakeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WakeyError::InvalidMacLength => {
                write!(f, "Invalid MAC address length")
            }
            WakeyError::InvalidMacFormat => write!(f, "Invalid MAC address format"),
            WakeyError::SendFailure(e) => write!(f, "Couldn't send WoL packet: {e}"),
        }
    }
}

impl From<std::io::Error> for WakeyError {
    fn from(error: std::io::Error) -> Self {
        WakeyError::SendFailure(error)
    }
}

/// Wake-on-LAN packet
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WolPacket {
    /// WOL packet bytes
    packet: Packet,
}

impl WolPacket {
    /// Creates WOL packet from byte MAC representation
    /// # Example
    /// ```
    /// let wol = wakey::WolPacket::from_bytes(&vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05]);
    /// ```
    pub fn from_bytes(mac: &[u8]) -> Result<WolPacket> {
        Ok(WolPacket {
            packet: WolPacket::create_packet_bytes(mac)?,
        })
    }

    /// Creates WOL packet from string MAC representation (e.x. 00:01:02:03:04:05)
    /// # Example
    /// ```
    /// let wol = wakey::WolPacket::from_string("00:01:02:03:04:05", ':');
    /// ```
    /// # Panic
    ///  Panics when input MAC is invalid (i.e. contains non-byte characters)
    pub fn from_string<T: AsRef<str>>(data: T, sep: char) -> Result<WolPacket> {
        WolPacket::from_bytes(&WolPacket::mac_to_byte(data, sep)?)
    }

    /// Broadcasts the magic packet from / to default address
    /// Source: 0.0.0.0:0
    /// Destination 255.255.255.255:9
    /// # Example
    /// ```
    /// let wol = wakey::WolPacket::from_bytes(&vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05]).unwrap();
    /// wol.send_magic();
    /// ```
    pub fn send_magic(&self) -> Result<()> {
        self.send_magic_to(
            SocketAddr::from(([0, 0, 0, 0], 0)),
            SocketAddr::from(([255, 255, 255, 255], 9)),
        )
    }

    /// Broadcasts the magic packet from / to specified address.
    /// # Example
    /// ```
    /// use std::net::SocketAddr;
    /// let wol = wakey::WolPacket::from_bytes(&vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05]).unwrap();
    /// let src = SocketAddr::from(([0,0,0,0], 0));
    /// let dst = SocketAddr::from(([255,255,255,255], 9));
    /// wol.send_magic_to(src, dst);
    /// ```
    pub fn send_magic_to<A: ToSocketAddrs>(&self, src: A, dst: A) -> Result<()> {
        let udp_sock = UdpSocket::bind(src)?;
        udp_sock.set_broadcast(true)?;
        udp_sock.send_to(&self.packet, dst)?;

        Ok(())
    }

    /// Returns the underlying WoL packet bytes
    #[must_use]
    pub fn into_inner(self) -> Packet {
        self.packet
    }

    /// Converts string representation of MAC address (e.x. 00:01:02:03:04:05) to raw bytes.
    /// # Panic
    /// Panics when input MAC is invalid (i.e. contains non-byte characters)
    fn mac_to_byte<T: AsRef<str>>(data: T, sep: char) -> Result<Mac> {
        // hex-encoded bytes * 2 plus separators
        if data.as_ref().len() != MAC_SIZE * 3 - 1 {
            return Err(WakeyError::InvalidMacLength);
        }

        let bytes = data
            .as_ref()
            .split(sep)
            .map(|x| hex::decode(x).map_err(|_| WakeyError::InvalidMacFormat))
            .collect::<Result<ArrayVec<_, MAC_SIZE>>>()?
            .into_iter()
            .flatten()
            .collect::<Mac>();

        debug_assert_eq!(MAC_SIZE, bytes.len());

        Ok(bytes)
    }

    /// Extends the MAC address to fill the magic packet
    fn extend_mac(mac: &[u8]) -> ArrayVec<u8, { MAC_SIZE * MAC_PER_MAGIC }> {
        let magic = iter::repeat(mac)
            .take(MAC_PER_MAGIC)
            .flatten()
            .copied()
            .collect::<ArrayVec<u8, { MAC_SIZE * MAC_PER_MAGIC }>>();

        debug_assert_eq!(MAC_SIZE * MAC_PER_MAGIC, magic.len());

        magic
    }

    /// Creates bytes of the magic packet from MAC address
    fn create_packet_bytes(mac: &[u8]) -> Result<Packet> {
        if mac.len() != MAC_SIZE {
            return Err(WakeyError::InvalidMacLength);
        }
        let mut packet = Packet::new();

        packet.extend(HEADER);
        packet.extend(WolPacket::extend_mac(mac));

        debug_assert_eq!(PACKET_LEN, packet.len());

        Ok(packet)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extend_mac_test() {
        let mac = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06];

        let extended_mac = WolPacket::extend_mac(&mac);

        assert_eq!(extended_mac.len(), MAC_PER_MAGIC * MAC_SIZE);
        assert_eq!(&extended_mac[(MAC_PER_MAGIC - 1) * MAC_SIZE..], &mac[..]);
    }

    #[test]
    #[should_panic]
    fn extend_mac_mac_too_long_test() {
        let mac = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
        WolPacket::extend_mac(&mac);
    }

    #[test]
    #[should_panic]
    fn extend_mac_mac_too_short_test() {
        let mac = vec![0x01, 0x02, 0x03, 0x04, 0x05];
        WolPacket::extend_mac(&mac);
    }

    #[test]
    fn mac_to_byte_test() {
        let mac = "01:02:03:04:05:06";
        let result = WolPacket::mac_to_byte(mac, ':');

        assert_eq!(
            result.unwrap().into_inner().unwrap(),
            [0x01, 0x02, 0x03, 0x04, 0x05, 0x06]
        );
    }

    #[test]
    fn mac_to_byte_invalid_chars_test() {
        let mac = "ZZ:02:03:04:05:06";
        assert!(matches!(
            WolPacket::mac_to_byte(mac, ':'),
            Err(WakeyError::InvalidMacFormat)
        ));
    }

    #[test]
    fn mac_to_byte_invalid_separator_test() {
        let mac = "01002:03:04:05:06";
        assert!(matches!(
            WolPacket::mac_to_byte(mac, ':'),
            Err(WakeyError::InvalidMacFormat)
        ));
    }

    #[test]
    fn mac_to_byte_mac_too_long_test() {
        let mac = "01:02:03:04:05:06:07";
        assert!(matches!(
            WolPacket::mac_to_byte(mac, ':'),
            Err(WakeyError::InvalidMacLength)
        ));
    }

    #[test]
    fn mac_to_byte_mac_too_short_test() {
        let mac = "01:02:03:04:05";
        assert!(matches!(
            WolPacket::mac_to_byte(mac, ':'),
            Err(WakeyError::InvalidMacLength)
        ));
    }

    #[test]
    fn create_packet_bytes_test() {
        let bytes = WolPacket::create_packet_bytes(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]).unwrap();

        assert_eq!(bytes.len(), MAC_SIZE * MAC_PER_MAGIC + HEADER.len());
        assert!(bytes.iter().all(|&x| x == 0xFF));
    }

    #[test]
    fn create_wol_packet() {
        let mac = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05];
        let wol = WolPacket::from_bytes(&mac).unwrap();
        let packet = wol.into_inner();

        assert_eq!(packet.len(), PACKET_LEN);
        assert_eq!(
            [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF],
            &packet[0..HEADER.len()]
        );
        for offset in (HEADER.len()..PACKET_LEN).step_by(MAC_SIZE) {
            assert_eq!(&mac, &packet[offset..offset + MAC_SIZE]);
        }
    }
}
