use std::num::ParseIntError;

fn main() {
    let mac_address = "00:11:22:33:44:55";
    let magic_packet: Vec<u8> = match create_magic_packet(mac_address) {
        Ok(packet) => packet,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };
    println!("Magic packet: {:?}", magic_packet);
}

// MAC アドレスを受け取り、その MAC アドレスに対応する Magic Packet を生成する
fn create_magic_packet(mac_address: &str) -> Result<Vec<u8>, ParseIntError> {
    let mut magic_packet: Vec<u8> = vec![0xff; 6];
    for _ in 0..16 {
        let parsed_address: Vec<u8> = parse_mac_address(mac_address)?;
        magic_packet.extend_from_slice(&parsed_address);
    }
    Ok(magic_packet)
}

// MAC アドレスを 16 進数のベクタに変換する
fn parse_mac_address(mac_address: &str) -> Result<Vec<u8>, ParseIntError> {
    let mut parsed: Vec<u8> = Vec::new();
    for octet in mac_address.split(":") {
        parsed.push(u8::from_str_radix(octet, 16)?);
    }
    Ok(parsed)
}

// --- テストコード ---

#[cfg(test)]
mod tests {
    #[test]
    fn test_create_magic_packet() {
        let mac_address = "00:11:22:33:44:55";
        let magic_packet = super::create_magic_packet(mac_address).unwrap();
        assert_eq!(magic_packet.len(), 102);

        let expected: Vec<u8> = vec![
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
        ];
        assert_eq!(magic_packet, expected);
    }

    #[test]
    fn test_parse_mac_address() {
        let mac_address = "00:11:22:33:44:55";
        let parsed = super::parse_mac_address(mac_address).unwrap();
        assert_eq!(parsed.len(), 6);

        let expected: Vec<u8> = vec![0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        assert_eq!(parsed, expected);
    }
}