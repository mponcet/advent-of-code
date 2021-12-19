type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
enum Packet {
    Literal {
        version: u32,
        literal: u64,
    },
    Operator {
        version: u32,
        typeid: u32,
        subpackets: Vec<Packet>,
    },
}

fn parse_input(input: &'static str) -> impl Iterator<Item = u8> {
    input
        .chars()
        .filter_map(|c| c.to_digit(16))
        .flat_map(|d| (0..4).rev().map(move |i| (d as u8 >> i) & 1))
}

fn read_bits(nr_bits: usize, iter: &mut dyn Iterator<Item = u8>) -> Option<u32> {
    let mut ret: u32 = 0;

    for _ in 0..nr_bits {
        ret = ret << 1 | iter.next()? as u32;
    }

    Some(ret)
}

fn decode_packet(iter: &mut dyn Iterator<Item = u8>) -> Option<Packet> {
    let version = read_bits(3, iter)?;
    let typeid = read_bits(3, iter)?;

    match typeid {
        4 => {
            let mut literal = 0;
            loop {
                let msb = read_bits(1, iter)?;
                let group = read_bits(4, iter)? as u64;
                literal = literal << 4 | group;
                if msb == 0 {
                    break;
                }
            }
            Some(Packet::Literal { version, literal })
        }
        _ => {
            let len_typeid = read_bits(1, iter)?;
            let subpackets = if len_typeid == 0 {
                let len = read_bits(15, iter)? as usize;
                let mut pkts = Vec::new();
                let mut iter = iter.take(len).peekable();
                while iter.peek().is_some() {
                    pkts.push(decode_packet(&mut iter)?);
                }
                pkts
            } else {
                let nr_packets = read_bits(11, iter)? as usize;
                (0..nr_packets)
                    .map(|_| decode_packet(iter))
                    .collect::<Option<_>>()?
            };

            Some(Packet::Operator {
                version,
                typeid,
                subpackets,
            })
        }
    }
}

fn sum_versions(packet: &Packet) -> u32 {
    match packet {
        Packet::Literal { version, .. } => *version,
        Packet::Operator {
            version,
            subpackets,
            ..
        } => *version + subpackets.iter().map(sum_versions).sum::<u32>(),
    }
}

fn calc(packet: &Packet) -> Result<u64> {
    Ok(match packet {
        Packet::Literal { literal, .. } => *literal,
        Packet::Operator {
            typeid, subpackets, ..
        } => match typeid {
            0 => subpackets.iter().map(calc).sum::<Result<_>>()?,
            1 => subpackets.iter().map(calc).product::<Result<_>>()?,
            2 => subpackets
                .iter()
                .filter_map(|p| calc(p).ok())
                .min()
                .ok_or("no min")?,
            3 => subpackets
                .iter()
                .filter_map(|p| calc(p).ok())
                .max()
                .ok_or("no min")?,
            5 => match &subpackets[..] {
                [first, second] => (calc(first)? > calc(second)?) as u64,
                _ => return Err("too many packets".into()),
            },
            6 => match &subpackets[..] {
                [first, second] => (calc(first)? < calc(second)?) as u64,
                _ => return Err("too many packets".into()),
            },
            7 => match &subpackets[..] {
                [first, second] => (calc(first)? == calc(second)?) as u64,
                _ => return Err("too many packets".into()),
            },
            _ => return Err("unknown typeid".into()),
        },
    })
}

fn part1() -> Result<u32> {
    let iter = &mut parse_input(include_str!("../input.txt"));
    let packet = decode_packet(iter).ok_or("decode_packet failed")?;
    Ok(sum_versions(&packet))
}

fn part2() -> Result<u64> {
    let iter = &mut parse_input(include_str!("../input.txt"));
    let packet = decode_packet(iter).ok_or("decode_packet failed")?;
    Ok(calc(&packet)?)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        println!("answer: {}", part1().unwrap());
    }

    #[test]
    fn test_part2() {
        println!("answer: {}", part2().unwrap());
    }

    #[test]
    fn test_part1_samples() {
        for (sample, sum) in &[
            ("8A004A801A8002F478", 16),
            ("620080001611562C8802118E34", 12),
            ("C0015000016115A2E0802F182340", 23),
            ("A0016C880162017C3686B18A3D4780", 31),
        ] {
            let iter = &mut parse_input(sample);
            let packet = decode_packet(iter).unwrap();
            assert_eq!(sum_versions(&packet), *sum as u32);
        }
    }

    #[test]
    fn test_part2_samples() {
        for (sample, val) in &[
            ("C200B40A82", 3),
            ("04005AC33890", 54),
            ("880086C3E88112", 7),
            ("CE00C43D881120", 9),
            ("D8005AC2A8F0", 1),
            ("F600BC2D8F", 0),
            ("9C005AC2F8F0", 0),
            ("9C0141080250320F1802104A08", 1),
        ] {
            let iter = &mut parse_input(sample);
            let packet = decode_packet(iter).unwrap();
            assert_eq!(calc(&packet).unwrap(), *val as u64);
        }
    }
}
