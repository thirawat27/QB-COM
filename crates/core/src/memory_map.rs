use std::sync::{Arc, Mutex};
use crate::errors::{QError, QErrorCode, QResult};

/// DOS Memory emulation (1 MB)
/// Memory Map:
/// - 0x00000 - 0x003FF: Interrupt Vectors
/// - 0x00400 - 0x004FF: BIOS Data Area
/// - 0xA0000 - 0xAFFFF: VGA Video RAM (Mode 13h)
/// - 0xB0000 - 0xB7FFF: Monochrome Text Mode
/// - 0xB8000 - 0xBFFFF: Color Text Mode
pub struct DosMemory {
    buffer: Vec<u8>,
    size: usize,
}

impl DosMemory {
    pub const SIZE: usize = 1_048_576; // 1 MB
    
    // Memory regions
    pub const INTERRUPT_VECTORS_START: usize = 0x00000;
    pub const INTERRUPT_VECTORS_END: usize = 0x003FF;
    pub const BIOS_DATA_START: usize = 0x00400;
    pub const BIOS_DATA_END: usize = 0x004FF;
    pub const CONVENTIONAL_START: usize = 0x00500;
    pub const CONVENTIONAL_END: usize = 0x9FFFF;
    pub const VGA_RAM_START: usize = 0xA0000;
    pub const VGA_RAM_END: usize = 0xAFFFF;
    pub const MONO_TEXT_START: usize = 0xB0000;
    pub const MONO_TEXT_END: usize = 0xB7FFF;
    pub const COLOR_TEXT_START: usize = 0xB8000;
    pub const COLOR_TEXT_END: usize = 0xBFFFF;
    pub const VIDEO_BIOS_START: usize = 0xC0000;
    pub const VIDEO_BIOS_END: usize = 0xC7FFF;
    pub const BIOS_ROM_START: usize = 0xF0000;
    pub const BIOS_ROM_END: usize = 0xFFFFF;

    pub fn new() -> Self {
        let mut mem = Self {
            buffer: vec![0; Self::SIZE],
            size: Self::SIZE,
        };
        mem.initialize();
        mem
    }

    /// Initialize memory with default values
    fn initialize(&mut self) {
        // Set up interrupt vectors (minimal)
        for i in 0..256 {
            let addr = i * 4;
            // Default: all interrupts point to IRET at FFFF:0000
            self.buffer[addr] = 0x00;
            self.buffer[addr + 1] = 0x00;
            self.buffer[addr + 2] = 0x00;
            self.buffer[addr + 3] = 0xF0;
        }

        // Set up BIOS data area (minimal)
        // Keyboard buffer at 0040:001E
        self.buffer[0x041E] = 0x00;
        self.buffer[0x041F] = 0x00;

        // Equipment word at 0040:0010
        self.buffer[0x0410] = 0x21; // Video mode: color 80x25
        self.buffer[0x0411] = 0x00;

        // Video mode at 0040:0049
        self.buffer[0x0449] = 0x03; // Text mode 80x25 color

        // Screen columns at 0040:004A
        self.buffer[0x044A] = 0x50; // 80 columns
        self.buffer[0x044B] = 0x00;

        // Active display page at 0040:004E
        self.buffer[0x044E] = 0x00;
    }

    /// Calculate physical address from segment:offset
    pub fn absolute_address(segment: u16, offset: u16) -> usize {
        ((segment as usize) << 4) + (offset as usize)
    }

    /// Check if address is valid
    pub fn is_valid_address(&self, addr: usize) -> bool {
        addr < self.size
    }

    /// Read byte from memory
    pub fn read_byte(&self, segment: u16, offset: u16) -> QResult<u8> {
        let addr = Self::absolute_address(segment, offset);
        if addr >= self.size {
            return Err(QError::runtime(QErrorCode::SubscriptOutOfRange, 0, 0));
        }
        Ok(self.buffer[addr])
    }

    /// Write byte to memory
    pub fn write_byte(&mut self, segment: u16, offset: u16, value: u8) -> QResult<()> {
        let addr = Self::absolute_address(segment, offset);
        if addr >= self.size {
            return Err(QError::runtime(QErrorCode::SubscriptOutOfRange, 0, 0));
        }
        self.buffer[addr] = value;
        Ok(())
    }

    /// Read word (16-bit) from memory (little-endian)
    pub fn read_word(&self, segment: u16, offset: u16) -> QResult<u16> {
        let low = self.read_byte(segment, offset)? as u16;
        let high = self.read_byte(segment, offset.wrapping_add(1))? as u16;
        Ok((high << 8) | low)
    }

    /// Write word (16-bit) to memory (little-endian)
    pub fn write_word(&mut self, segment: u16, offset: u16, value: u16) -> QResult<()> {
        self.write_byte(segment, offset, (value & 0xFF) as u8)?;
        self.write_byte(segment, offset.wrapping_add(1), ((value >> 8) & 0xFF) as u8)?;
        Ok(())
    }

    /// Read double word (32-bit) from memory
    pub fn read_dword(&self, segment: u16, offset: u16) -> QResult<u32> {
        let low = self.read_word(segment, offset)? as u32;
        let high = self.read_word(segment, offset.wrapping_add(2))? as u32;
        Ok((high << 16) | low)
    }

    /// Write double word (32-bit) to memory
    pub fn write_dword(&mut self, segment: u16, offset: u16, value: u32) -> QResult<()> {
        self.write_word(segment, offset, (value & 0xFFFF) as u16)?;
        self.write_word(segment, offset.wrapping_add(2), ((value >> 16) & 0xFFFF) as u16)?;
        Ok(())
    }

    /// Read bytes from memory region
    pub fn read_bytes(&self, segment: u16, offset: u16, count: usize) -> QResult<Vec<u8>> {
        let mut result = Vec::with_capacity(count);
        let mut off = offset;
        for _ in 0..count {
            result.push(self.read_byte(segment, off)?);
            off = off.wrapping_add(1);
        }
        Ok(result)
    }

    /// Write bytes to memory region
    pub fn write_bytes(&mut self, segment: u16, offset: u16, data: &[u8]) -> QResult<()> {
        let mut off = offset;
        for &byte in data {
            self.write_byte(segment, off, byte)?;
            off = off.wrapping_add(1);
        }
        Ok(())
    }

    /// Read string from memory (Pascal string: length byte + chars)
    pub fn read_pascal_string(&self, segment: u16, offset: u16) -> QResult<String> {
        let len = self.read_byte(segment, offset)? as usize;
        let bytes = self.read_bytes(segment, offset.wrapping_add(1), len)?;
        String::from_utf8(bytes).map_err(|_| QError::runtime(QErrorCode::SyntaxError, 0, 0))
    }

    /// Write string to memory (Pascal string format)
    pub fn write_pascal_string(&mut self, segment: u16, offset: u16, s: &str) -> QResult<()> {
        let bytes = s.as_bytes();
        if bytes.len() > 255 {
            return Err(QError::runtime(QErrorCode::OutOfStringSpace, 0, 0));
        }
        self.write_byte(segment, offset, bytes.len() as u8)?;
        self.write_bytes(segment, offset.wrapping_add(1), bytes)?;
        Ok(())
    }

    /// Get direct access to VGA video memory (0xA0000)
    pub fn get_vga_buffer(&self) -> &[u8] {
        &self.buffer[Self::VGA_RAM_START..=Self::VGA_RAM_END]
    }

    /// Get mutable access to VGA video memory
    pub fn get_vga_buffer_mut(&mut self) -> &mut [u8] {
        &mut self.buffer[Self::VGA_RAM_START..=Self::VGA_RAM_END]
    }

    /// Get direct access to text video memory (0xB8000)
    pub fn get_text_buffer(&self) -> &[u8] {
        &self.buffer[Self::COLOR_TEXT_START..=Self::COLOR_TEXT_END]
    }

    /// Get mutable access to text video memory
    pub fn get_text_buffer_mut(&mut self) -> &mut [u8] {
        &mut self.buffer[Self::COLOR_TEXT_START..=Self::COLOR_TEXT_END]
    }

    /// Set video mode (affects memory layout)
    pub fn set_video_mode(&mut self, mode: u8) -> QResult<()> {
        // Update BIOS video mode byte
        self.buffer[0x0449] = mode;
        
        // Clear appropriate video memory
        match mode {
            0x00..=0x03 => {
                // Text modes - clear text buffer
                for i in Self::COLOR_TEXT_START..=Self::COLOR_TEXT_END {
                    self.buffer[i] = 0;
                }
            }
            0x13 => {
                // Mode 13h - 320x200 256 colors
                for i in Self::VGA_RAM_START..=Self::VGA_RAM_END {
                    self.buffer[i] = 0;
                }
            }
            _ => {}
        }
        
        Ok(())
    }

    /// Get current video mode
    pub fn get_video_mode(&self) -> u8 {
        self.buffer[0x0449]
    }

    /// Read from absolute address
    pub fn peek(&self, addr: usize) -> QResult<u8> {
        if addr >= self.size {
            return Err(QError::runtime(QErrorCode::SubscriptOutOfRange, 0, 0));
        }
        Ok(self.buffer[addr])
    }

    /// Write to absolute address
    pub fn poke(&mut self, addr: usize, value: u8) -> QResult<()> {
        if addr >= self.size {
            return Err(QError::runtime(QErrorCode::SubscriptOutOfRange, 0, 0));
        }
        self.buffer[addr] = value;
        Ok(())
    }

    /// Peek a word (16-bit) from absolute address
    pub fn peek_word(&self, addr: usize) -> QResult<u16> {
        let low = self.peek(addr)? as u16;
        let high = self.peek(addr + 1)? as u16;
        Ok((high << 8) | low)
    }

    /// Poke a word (16-bit) to absolute address
    pub fn poke_word(&mut self, addr: usize, value: u16) -> QResult<()> {
        self.poke(addr, (value & 0xFF) as u8)?;
        self.poke(addr + 1, ((value >> 8) & 0xFF) as u8)?;
        Ok(())
    }
}

impl Default for DosMemory {
    fn default() -> Self {
        Self::new()
    }
}

/// Thread-safe shared memory
pub type SharedMemory = Arc<Mutex<DosMemory>>;

pub fn create_shared_memory() -> SharedMemory {
    Arc::new(Mutex::new(DosMemory::new()))
}

/// Memory segment constants
pub mod segments {
    pub const VIDEO_VGA: u16 = 0xA000;
    pub const VIDEO_MONO: u16 = 0xB000;
    pub const VIDEO_COLOR: u16 = 0xB800;
    pub const VIDEO_BIOS: u16 = 0xC000;
    pub const BIOS_ROM: u16 = 0xF000;
}
