use std::io::{self, Cursor, Read};

#[derive(Clone, Debug)]
pub struct LevelMeta {
    pub unknown0: i8,
    pub unknown1: i8,
    pub unknown2: i8,
    pub unknown3: i8,
    pub spawn_variant: i8,
    pub spawn_x_cell: i8,
    pub object_count: i8,
}

#[derive(Clone, Debug)]
pub struct LevelData {
    pub meta: LevelMeta,
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<u8>,
}

impl LevelData {
    /// Exact high-level layout recovered from h.b(int): the selected `/res/lf`
    /// entry is a 7-byte header followed by a map entry. The map entry starts
    /// with height/width bytes and then contains height*width tile bytes.
    pub fn decode_meta(bytes: &[u8]) -> Option<LevelMeta> {
        if bytes.len() < 7 { return None; }
        Some(LevelMeta {
            unknown0: bytes[0] as i8,
            unknown1: bytes[1] as i8,
            unknown2: bytes[2] as i8,
            unknown3: bytes[3] as i8,
            spawn_variant: bytes[4] as i8,
            spawn_x_cell: bytes[5] as i8,
            object_count: bytes[6] as i8,
        })
    }

    pub fn decode_map(bytes: &[u8]) -> io::Result<Self> {
        let mut cur = Cursor::new(bytes);
        let height = read_u8(&mut cur)? as usize;
        let width = read_u8(&mut cur)? as usize;
        let count = width.checked_mul(height).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "level dimensions overflow"))?;
        if bytes.len() < 2 + count { return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "level map truncated")); }
        let mut tiles = vec![0u8; count];
        cur.read_exact(&mut tiles)?;
        Ok(Self { meta: LevelMeta{unknown0:0,unknown1:0,unknown2:0,unknown3:0,spawn_variant:0,spawn_x_cell:0,object_count:0}, width, height, tiles })
    }

    pub fn set_meta(&mut self, meta: LevelMeta) { self.meta = meta; }
}
fn read_u8(r:&mut Cursor<&[u8]>) -> io::Result<u8> { let mut b=[0]; r.read_exact(&mut b)?; Ok(b[0]) }
