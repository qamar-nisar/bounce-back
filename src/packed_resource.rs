use std::io::{self, Cursor, Read};

/// Original J2ME resource container: BE u16 chunk count, then BE u16 lengths,
/// followed by concatenated chunk payloads. This mirrors class `c`.
pub struct PackedResource {
    chunks: Vec<Vec<u8>>,
}

impl PackedResource {
    pub fn parse(data: &[u8]) -> io::Result<Self> {
        let mut cur = Cursor::new(data);
        let count = read_u16(&mut cur)? as usize;
        let mut lengths = Vec::with_capacity(count);
        for _ in 0..count { lengths.push(read_u16(&mut cur)? as usize); }
        let mut chunks = Vec::with_capacity(count);
        for len in lengths {
            let mut buf = vec![0u8; len];
            cur.read_exact(&mut buf)?;
            chunks.push(buf);
        }
        Ok(Self { chunks })
    }

    pub fn len(&self) -> usize { self.chunks.len() }
    pub fn chunk(&self, index: usize) -> Option<&[u8]> { self.chunks.get(index).map(Vec::as_slice) }
    pub fn png_chunks(&self) -> impl Iterator<Item=(usize, &[u8])> {
        self.chunks.iter().enumerate().filter(|(_, c)| c.starts_with(b"\x89PNG\r\n\x1a\n")).map(|(i,c)| (i,c.as_slice()))
    }
}

fn read_u16(r: &mut Cursor<&[u8]>) -> io::Result<u16> {
    let mut b = [0;2]; r.read_exact(&mut b)?; Ok(u16::from_be_bytes(b))
}
