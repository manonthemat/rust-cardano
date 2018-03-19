
// internal mobule to encode the address metadata in cbor to
// hash them.
//
pub mod spec {
    #[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Copy, Clone)]
    pub enum MajorType {
        UINT,
        NINT,
        BYTES,
        TEXT,
        ARRAY,
        MAP,
        TAG,
        T7
    }

    const INLINE_ENCODING : u8 = 24;

    impl MajorType {
        // serialize a major type in its highest bit form
        fn to_byte(self) -> u8 {
            use self::MajorType::*;
            match self {
                UINT  => 0b0000_0000,
                NINT  => 0b0010_0000,
                BYTES => 0b0100_0000,
                TEXT  => 0b0110_0000,
                ARRAY => 0b1000_0000,
                MAP   => 0b1010_0000,
                TAG   => 0b1100_0000,
                T7    => 0b1110_0000
            }
        }
    }

    pub fn cbor_header(ty: MajorType, r: u8) -> u8 {
        ty.to_byte() | r & 0x1f
    }

    pub fn cbor_uint_small(v: u8, buf: &mut Vec<u8>) {
        assert!(v < INLINE_ENCODING);
        buf.push(cbor_header(MajorType::UINT, v));
    }

    pub fn cbor_u8(v: u8, buf: &mut Vec<u8>) {
        buf.push(cbor_header(MajorType::UINT, 24));
        buf.push(v);
    }

    /// convenient macro to get the given bytes of the given value
    ///
    /// does all the job: Big Endian, bit shift and convertion
    macro_rules! byte_slice {
        ($value:ident, $shift:expr) => ({
            ($value.to_be() >> $shift) as u8
        });
    }

    pub fn write_u8(v: u8, buf: &mut Vec<u8>) {
        write_header_u8(MajorType::UINT, v, buf);
    }
    pub fn write_u16(v: u16, buf: &mut Vec<u8>) {
        write_header_u16(MajorType::UINT, v, buf);
    }
    pub fn write_u32(v: u32, buf: &mut Vec<u8>) {
        write_header_u32(MajorType::UINT, v, buf);
    }
    pub fn write_u64(v: u64, buf: &mut Vec<u8>) {
        write_header_u64(MajorType::UINT, v, buf);
    }
    pub fn write_header_u8(ty: MajorType, v: u8, buf: &mut Vec<u8>) {
        buf.push(cbor_header(ty, 24));
        buf.push(v);
    }
    pub fn write_header_u16(ty: MajorType, v: u16, buf: &mut Vec<u8>) {
        buf.push(cbor_header(ty, 25));
        buf.push(byte_slice!(v, 8));
        buf.push(byte_slice!(v, 0));
    }
    pub fn write_header_u32(ty: MajorType, v: u32, buf: &mut Vec<u8>) {
        buf.push(cbor_header(ty, 26));
        buf.push(byte_slice!(v, 24));
        buf.push(byte_slice!(v, 16));
        buf.push(byte_slice!(v,  8));
        buf.push(byte_slice!(v,  0));
    }
    pub fn write_header_u64(ty: MajorType, v: u64, buf: &mut Vec<u8>) {
        buf.push(cbor_header(ty, 27));
        buf.push(byte_slice!(v, 56));
        buf.push(byte_slice!(v, 48));
        buf.push(byte_slice!(v, 40));
        buf.push(byte_slice!(v, 32));
        buf.push(byte_slice!(v, 24));
        buf.push(byte_slice!(v, 16));
        buf.push(byte_slice!(v,  8));
        buf.push(byte_slice!(v,  0));
    }

    pub fn write_length_encoding(ty: MajorType, nb_elems: usize, buf: &mut Vec<u8>) {
        if nb_elems < (INLINE_ENCODING as usize) {
            buf.push(cbor_header(ty, nb_elems as u8));
        } else {
            if nb_elems < 0x100 {
                write_header_u8(ty, nb_elems as u8, buf);
            } else if nb_elems < 0x10000 {
                write_header_u16(ty, nb_elems as u16, buf);
            } else if nb_elems < 0x100000000 {
                write_header_u32(ty, nb_elems as u32, buf);
            } else {
                write_header_u64(ty, nb_elems as u64, buf);
            }
        }
    }

    pub fn cbor_tag(tag: usize, buf: &mut Vec<u8>) {
        write_length_encoding(MajorType::TAG, tag, buf);
    }

    pub fn cbor_bs(bs: &[u8], buf: &mut Vec<u8>) {
        write_length_encoding(MajorType::BYTES, bs.len(), buf);
        buf.extend_from_slice(bs)
    }

    pub fn cbor_array_start(nb_elems: usize, buf: &mut Vec<u8>) {
        write_length_encoding(MajorType::ARRAY, nb_elems, buf);
    }
    pub fn cbor_map_start(nb_elems: usize, buf: &mut Vec<u8>) {
        write_length_encoding(MajorType::MAP, nb_elems, buf);
    }

}
