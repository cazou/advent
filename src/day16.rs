use std::cmp::{max, min};

fn read_n_bits_at(hex: &Vec<u8>, at: usize, n: usize) -> usize {
    let mut val = 0;
    for bit in at..at+n {
        if ((hex[bit / 8] >> (8 - (bit % 8) - 1)) & 1) == 1 {
            val += 1usize << (n - (bit-at) - 1)
        }
    }
    val
}

struct PacketHeader {
    version: u8,
    p_type: u8
}

impl PacketHeader {
    pub fn from_bytes(hex: &Vec<u8>, start_bit: usize) -> PacketHeader {
        PacketHeader {
            version: read_n_bits_at(hex, start_bit, 3) as u8,
            p_type: read_n_bits_at(hex, start_bit + 3, 3) as u8
        }
    }
}

struct LiteralPacket {
    header: PacketHeader,
    value: usize,
    bits: usize
}

impl LiteralPacket {
    // return the number and true if it is the last one
    fn get_group_at(hex: &Vec<u8>, bit: usize) -> (usize, bool) {
        (read_n_bits_at(hex, bit + 1, 4), read_n_bits_at(hex, bit, 1) != 1)
    }

    pub fn from_bytes(h: PacketHeader, hex: &Vec<u8>, start_bit: usize) -> LiteralPacket {
        let mut bit = start_bit;
        let mut values = vec![];
        let mut value = 0;
        let mut l = 0;

        loop {
            let (v, s) = LiteralPacket::get_group_at(hex, bit);
            bit += 5;
            values.push(v);
            if s {
                break;
            }
        }

        for v in values.iter().rev() {
            value += v << (4*l);
            l += 1;
        }

        LiteralPacket {
            header: h,
            bits: bit - start_bit,
            value
        }
    }

    pub fn bits_consumed(&self) -> usize {
        self.bits + 6
    }
}

struct OperatorPacket {
    header: PacketHeader,
    subs: Vec<PacketType>,
    size_bits: usize
}

impl OperatorPacket {
    pub fn from_bytes(h: PacketHeader, hex: &Vec<u8>, start_bit: usize) -> OperatorPacket {
        let i = read_n_bits_at(hex, start_bit, 1);
        let mut subs = vec![];
        let size_bits;

        if i == 0 {
            let mut sub_bits = read_n_bits_at(hex, start_bit+1, 15);
            size_bits = 16;
            let mut sub_start_bit = start_bit + size_bits;

            while sub_bits > 0 {
                let mut consumed = 0;
                subs.push(PacketType::from_hexa(hex, sub_start_bit, &mut consumed));
                sub_start_bit += consumed;
                sub_bits -= consumed;
            }
        } else {
            let sub_count = read_n_bits_at(hex, start_bit+1, 11);
            size_bits = 12;
            let mut sub_start_bit = start_bit + size_bits;

            for _ in 0..sub_count {
                let mut consumed = 0;
                subs.push(PacketType::from_hexa(hex, sub_start_bit, &mut consumed));
                sub_start_bit += consumed;
            }
        }

        OperatorPacket {
            header: h,
            subs,
            size_bits
        }
    }

    pub fn bits_consumed(&self) -> usize {
        let mut sub_size = 0;
        for s in &self.subs {
            sub_size += match s {
                PacketType::Operator(p) => p.bits_consumed(),
                PacketType::Literal(p) => p.bits_consumed(),
            };
        }
        self.size_bits + sub_size + 6
    }

    pub fn count_vers(&self) -> usize {
        let mut v: usize = self.header.version as usize;
        for c in &self.subs {
            v += match c {
                PacketType::Operator(p) => p.count_vers(),
                PacketType::Literal(p) => p.header.version as usize,
            }
        }
        v
    }

    pub fn resolve(&self) -> usize {
        let mut values = vec![];
        for s in &self.subs {
            values.push(match s {
                PacketType::Operator(p) => p.resolve(),
                PacketType::Literal(p) => p.value as usize,
            });
        }

        match self.header.p_type {
            0 => values.iter().sum(),
            1 => {
                let mut ret = 1;
                values.iter().for_each(|x| ret *= *x);
                ret
            },
            2 => {
                let mut ret = usize::MAX;
                values.iter().for_each(|x| ret = min(ret, *x));
                ret
            },
            3 => {
                let mut ret = 0;
                values.iter().for_each(|x| ret = max(ret, *x));
                ret
            },
            5 => {
                if values[0] > values[1] {
                    1
                } else {
                    0
                }
            }
            6 => {
                if values[0] < values[1] {
                    1
                } else {
                    0
                }
            }
            7 => {
                if values[0] == values[1] {
                    1
                } else {
                    0
                }
            }
            _ => 0
        }
    }
}

enum PacketType {
    Literal(LiteralPacket),
    Operator(OperatorPacket)
}

impl PacketType {
    pub fn from_hexa(bytes: &Vec<u8>, start_bit: usize, size_bits: &mut usize) -> PacketType {
        let header = PacketHeader::from_bytes(bytes, start_bit);
        match header.p_type {
            4 => {
                let p = LiteralPacket::from_bytes(header, bytes, start_bit + 6);
                *size_bits = p.bits_consumed();
                PacketType::Literal(p)
            },
            _ => {
                let p = OperatorPacket::from_bytes(header, bytes, start_bit + 6);
                *size_bits = p.bits_consumed();
                PacketType::Operator(p)
            },
        }
    }
}


pub fn run(contents: &str) -> Result<(), String> {
    print!("[BITS]...        ");

    let mut cases: Vec<PacketType> = vec![];
    let mut vers = 0;
    let mut last_pos= 0;

    let val = contents.trim();
    let mut bytes = vec![];
    for i in (0..val.len()).step_by(2) {
        bytes.push(u8::from_str_radix(&val[i..i+2], 16).unwrap());
    }

    while last_pos < bytes.len() {
        let mut consumed= 0;

        cases.push(PacketType::from_hexa(&bytes, last_pos, &mut consumed));
        if consumed == 0 {
            println!("No packet was consumed, Leaving");
            break;
        }

        last_pos += consumed;
        vers += match cases.last().unwrap() {
            PacketType::Literal(p) => p.header.version as usize,
            PacketType::Operator(p) => p.count_vers(),
        };
    }

    let r = match &cases[0] {
        PacketType::Literal(p) => p.value as usize,
        PacketType::Operator(p) => p.resolve(),
    };

    println!("{} {}", vers, r);

    Ok(())
}