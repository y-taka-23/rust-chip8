use log::debug;

const MEMORY_SIZE: usize = 4096;
const FONT_SIZE: u8 = 5;

const FONT: [u8; FONT_SIZE as usize * 16] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Memory {
    at: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn with_rom(rom: Vec<u8>) -> Self {
        debug!("Loading ROM: {:?}", rom);

        let mut memory = Memory {
            at: [0x00; MEMORY_SIZE],
        };

        for (font_addr, &b) in FONT.iter().enumerate() {
            memory.at[font_addr] = b;
        }

        let rom_from = 0x200;
        for (offset, &b) in rom.iter().enumerate() {
            memory.at[rom_from + offset] = b;
        }

        memory
    }

    pub fn load(&self, addr: u16) -> u8 {
        self.at[addr as usize]
    }

    pub fn store(&mut self, addr: u16, value: u8) {
        self.at[addr as usize] = value;
    }

    pub fn load_sprite(&self, from: u16, size: u8) -> &[u8] {
        let from = from as usize;
        let size = size as usize;
        &self.at[from..from + size]
    }

    pub fn font_addr(font: u8) -> u16 {
        (font * FONT_SIZE) as u16
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {

        #[test]
        fn load_stored_value(addr in 0x200u16..MEMORY_SIZE as u16 - 1, value: u8) {
            let mut memory = Memory::with_rom(vec![]);

            memory.store(addr, value);
            let result = memory.load(addr);
            assert_eq!(result, value);
        }

        #[test]
        fn load_overwritten_value(addr in 0x200u16..MEMORY_SIZE as u16 - 1, old: u8, new: u8) {
            let mut memory = Memory::with_rom(vec![]);

            memory.store(addr, old);
            memory.store(addr, new);
            let result = memory.load(addr);
            assert_eq!(result, new);
        }

        #[test]
        fn load_stored_sprite(from in 0x200u16..MEMORY_SIZE as u16 - 9, value: u8) {
            let mut memory = Memory::with_rom(vec![]);
            let sprite = &[value; 8];

            for offset in 0..8 {
                memory.store(from + offset, value);
            }
            let result = memory.load_sprite(from, 8);
            assert_eq!(result, sprite);
        }

        #[test]
        fn load_font_sprite(font in 0x0u8..0xFu8) {
            let memory = Memory::with_rom(vec![]);
            let from = font as usize * FONT_SIZE as usize;
            let sprite = &FONT[from..from + FONT_SIZE as usize];

            let result = memory.load_sprite(Memory::font_addr(font), FONT_SIZE);
            assert_eq!(result, sprite);
        }
    }
}
