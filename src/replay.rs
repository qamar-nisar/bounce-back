#[derive(Clone, Debug)]
pub struct ReplayEvent { pub frame: u16, pub event_type: i32, pub value: i8 }

#[derive(Default)]
pub struct ReplayReader {
    pub finished: bool,
    pub current_frame: i32,
    pub events: Vec<ReplayEvent>,
    pub cursor: usize,
}

impl ReplayReader {
    pub fn apply_events(&mut self, data: &[u8]) {
        self.events.clear(); self.cursor = 0; self.finished = false;
        // /res/r events are 7-byte records in the recovered binary map:
        // BE u16 frame, i32 type, i8 value.
        let mut p = 0usize;
        while p + 7 <= data.len() {
            let frame = u16::from_be_bytes([data[p], data[p+1]]); p += 2;
            let event_type = i32::from_be_bytes([data[p],data[p+1],data[p+2],data[p+3]]); p += 4;
            let value = data[p] as i8; p += 1;
            self.events.push(ReplayEvent{frame,event_type,value});
        }
    }
    pub fn events_for_frame(&mut self, frame: u32) -> Vec<ReplayEvent> {
        let mut out = Vec::new();
        while self.cursor < self.events.len() && self.events[self.cursor].frame as u32 <= frame {
            out.push(self.events[self.cursor].clone()); self.cursor += 1;
        }
        self.current_frame = frame as i32;
        if self.cursor >= self.events.len() { self.finished = true; }
        out
    }
}
