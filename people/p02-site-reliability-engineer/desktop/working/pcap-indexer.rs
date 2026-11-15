use std::env;
use std::fs;

fn be_u32(bytes: &[u8]) -> u32 {
    u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

fn main() -> Result<(), String> {
    let path = env::args().nth(1).ok_or("usage: pcap-indexer <pcap-file>")?;
    let bytes = fs::read(path).map_err(|error| error.to_string())?;
    if bytes.len() < 24 { return Err("PCAP lacks a global header".into()); }
    let mut offset = 24usize;
    let mut packet_count = 0usize;
    while offset < bytes.len() {
        if offset + 16 > bytes.len() { return Err("truncated packet header".into()); }
        let length = be_u32(&bytes[offset + 8..offset + 12]) as usize;
        offset += 16;
        if offset + length > bytes.len() { return Err("truncated packet body".into()); }
        println!("packet={} offset={} captured_bytes={}", packet_count, offset, length);
        offset += length;
        packet_count += 1;
    }
    println!("packets_indexed={packet_count}");
    Ok(())
}

// Local-only helper for synthetic INC-260713 packet captures.
