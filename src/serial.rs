use std::{io, time::Duration};

use crate::keys;

const CONFIG_PACKET_SIZE: usize = 67;
const REQUEST_PACKET_SIZE: usize = 4;

const PACKET_START: [u8; 2] = [0x4D, 0x39]; // M9

// Response packets with precompiled CRC
const SET_ACK: [u8; 4] = [0x4D, 0x39, 0x4B, 0xA5];
const SET_NACK: [u8; 4] = [0x4D, 0x39, 0x45, 0x8F];

const GET_REQ: [u8; 4] = [0x4D, 0x39, 0x47, 0x81];

#[repr(u8)]
enum Type {
    GetRequest = 0x47,  // G
    GetResponse = 0x52, // R
    SetRequest = 0x53,  // S
    SetAck = 0x4B,      // K
    SetError = 0x45,    // E
}

impl TryFrom<u8> for Type {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x47 => Ok(Type::GetRequest),
            0x52 => Ok(Type::GetResponse),
            0x53 => Ok(Type::SetRequest),
            0x4B => Ok(Type::SetAck),
            0x45 => Ok(Type::SetError),
            _ => Err(()),
        }
    }
}

fn calculate_crc(data: &[u8]) -> u8 {
    simple_crc::simple_crc8(data, 0x07, 0x00, false, false, 0x00)
}

pub fn get_config(port: &str) -> io::Result<keys::KeypadConfig> {
    let mut port = serialport::new(port, 9600)
        .timeout(Duration::from_millis(25))
        .open()?;

    port.write_all(&GET_REQ)?;

    let mut buf = [0u8; CONFIG_PACKET_SIZE];

    port.read_exact(&mut buf)?;

    if buf[0..2] != PACKET_START {
        println!("ERROR: Invalid packet received (GET)");
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Invalid packet received (GET)",
        ));
    }

    println!("{:?}", buf);
    if let Ok(Type::GetResponse) = Type::try_from(buf[2]) {
        if calculate_crc(&buf[0..66]) != buf[66] {
            println!("ERROR: GET CRC failed");
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "GET CRC failed",
            ));
        }

        let key_struct = match keys::KeypadConfig::deserialize(&buf[3..66]) {
            Some(k) => k,
            None => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Deserialization failed",
                ));
            }
        };

        return Ok(key_struct);
    }

    println!("ERROR: Unexpected packet (GET)");
    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Unexpected packet (GET)",
    ))
}

pub fn set_config(port: &str, key_struct: &keys::KeypadConfig) -> io::Result<()> {
    let mut port = serialport::new(port, 9600)
        .timeout(Duration::from_millis(25))
        .open()?;

    let mut buf = [0u8; CONFIG_PACKET_SIZE];
    buf[0..2].copy_from_slice(&PACKET_START);
    buf[2] = 0x53;

    key_struct
        .serialize(&mut buf[3..66])
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Serialization failed"))?;

    buf[66] = calculate_crc(&buf[0..66]);

    port.write_all(&buf)?;

    let mut response = [0u8; 4];

    port.read_exact(&mut response)?;

    if response == SET_ACK {
        return Ok(());
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Error setting config",
    ))
}
