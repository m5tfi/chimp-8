#![warn(clippy::pedantic, clippy::all)]
// #![allow(unused_variables, dead_code)] // TODO: remove this later.
#![feature(stmt_expr_attributes)]

use rand::random;

pub struct Vm {
    pc: u16,
    memory: [u8; Self::MEMORY_SIZE],
    display: [bool; Self::SCREEN_WIDTH * Self::SCREEN_HEIGHT],
    v_reg: [u8; Self::REG_COUNT],
    i_reg: u16,
    stack: [u16; Self::STACK_SIZE],
    sp: u16,
    delay_timer: u8,
    sound_timer: u8,
    keys: [bool; Self::KEYS_COUNT],
}

// --- Constants ---
impl Vm {
    pub const SCREEN_WIDTH: usize = 0x40;
    pub const SCREEN_HEIGHT: usize = 0x20;

    const MEMORY_SIZE: usize = 4 * 1024;
    const REG_COUNT: usize = 16;
    const STACK_SIZE: usize = 16;
    const KEYS_COUNT: usize = 16;
    const START_ADDR: u16 = 0x200;

    const FONT_SET_SIZE: usize = 80;
    const FONT_SET: [u8; Self::FONT_SET_SIZE] = [
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
}

impl Default for Vm {
    fn default() -> Self {
        let mut memory = [0; Self::MEMORY_SIZE];
        memory[..Self::FONT_SET_SIZE].copy_from_slice(&Self::FONT_SET);
        Self {
            pc: Self::START_ADDR,
            memory,
            display: [false; Self::SCREEN_WIDTH * Self::SCREEN_HEIGHT],
            v_reg: [0; Self::REG_COUNT],
            i_reg: 0,
            stack: [0; Self::STACK_SIZE],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            keys: [false; Self::KEYS_COUNT],
        }
    }
}

// --- Public Methods ---
impl Vm {
    /// Resets the Vm to its initial state without creating new object.
    pub fn reset(&mut self) {
        self.pc = Self::START_ADDR;
        self.memory = [0; Self::MEMORY_SIZE];
        self.memory[..Self::FONT_SET_SIZE].copy_from_slice(&Self::FONT_SET);
        self.display = [false; Self::SCREEN_WIDTH * Self::SCREEN_HEIGHT];
        self.v_reg = [0; Self::REG_COUNT];
        self.i_reg = 0;
        self.stack = [0; Self::STACK_SIZE];
        self.sp = 0;
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.keys = [false; Self::KEYS_COUNT];
    }

    /// Load program bytes into the Vm memory, starting at addr 0x200.
    pub fn load_program(&mut self, data: &[u8]) {
        let start = usize::from(Self::START_ADDR);
        let end = usize::from(Self::START_ADDR) + data.len();
        self.memory[start..end].copy_from_slice(data);
    }

    #[must_use]
    pub fn get_display(&self) -> &[bool] {
        &self.display
    }

    pub fn keypress(&mut self, idx: usize, pressed: bool) {
        self.keys[idx] = pressed;
    }

    /// Read and execute a single Opcode.
    pub fn tick(&mut self) {
        let opcode = self.fetch_next_opcode();
        self.execute_opcode(opcode);
    }

    /// Updates the delay and sound timers.
    pub fn tick_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                // TODO: Emit sound
            }
            self.sound_timer -= 1;
        }
    }
}

// --- Private Methods ---
impl Vm {
    /// Pushes an address to the stack.
    fn push_stack(&mut self, addr: u16) {
        self.stack[self.sp as usize] = addr;
        self.sp += 1;
    }

    /// Pops an address from the stack and returns the last address.
    fn pop_stack(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    /// Returns a single Opcode, based on the current Program Counter.
    fn fetch_next_opcode(&mut self) -> u16 {
        let higher_byte = u16::from(self.memory[self.pc as usize]);
        let lower_byte = u16::from(self.memory[self.pc as usize]);
        let opcode = (higher_byte << 8) | lower_byte;
        self.pc += 2;
        opcode
    }

    /// Executes a single Opcode.
    fn execute_opcode(&mut self, opcode: u16) {
        let digit_1 = (opcode & 0xF000) >> 12;
        let digit_2 = (opcode & 0x0F00) >> 8;
        let digit_3 = (opcode & 0x00F0) >> 4;
        let digit_4 = opcode & 0x000F;

        #[rustfmt::skip]
        match (digit_1, digit_2, digit_3, digit_4) {
            (0x0, 0x0, 0x0, 0x0) => { /* Do nothing */ },
            (0x0, 0x0, 0xE, 0x0) => self.clear_display(),
            (0x0, 0x0, 0xE, 0xE) => self.return_from_subroutine(),
            (0x1,   _,   _,   _) => self.jump_to_address(opcode),
            (0x2,   _,   _,   _) => self.call_subroutine(opcode),
            (0x3,   _,   _,   _) => self.skip_if_vx_equal_nn(digit_2, opcode),
            (0x4,   _,   _,   _) => self.skip_if_vx_not_equal_nn(digit_2, opcode),
            (0x5,   _,   _, 0x0) => self.skip_if_vx_equals_vy(digit_2, digit_3),
            (0x6,   _,   _,   _) => self.set_vx_to_nn(digit_2, opcode),
            (0x7,   _,   _,   _) => self.increment_vx_by_nn(digit_2, opcode),
            (0x8,   _,   _, 0x0) => self.set_vx_to_vy(digit_2, digit_3),
            (0x8,   _,   _, 0x1) => self.set_vx_to_bit_or_vy(digit_2, digit_3),
            (0x8,   _,   _, 0x2) => self.set_vx_to_bit_and_vy(digit_2, digit_3),
            (0x8,   _,   _, 0x3) => self.set_vx_to_bit_xor_vy(digit_2, digit_3),
            (0x8,   _,   _, 0x4) => self.increment_vx_by_vy(digit_2, digit_3),
            (0x8,   _,   _, 0x5) => self.decrement_vx_by_vy(digit_2, digit_3),
            (0x8,   _,   _, 0x6) => self.right_shift_vx(digit_2),
            (0x8,   _,   _, 0x7) => self.set_vx_to_vy_minus_vx(digit_2, digit_3),
            (0x8,   _,   _, 0xE) => self.left_shift_vx(digit_2),
            (0x9,   _,   _, 0x0) => self.skip_if_vx_not_equal_vy(digit_2, digit_3),
            (0xA,   _,   _,   _) => self.set_i_to_nnn(opcode),
            (0xB,   _,   _,   _) => self.jump_v0_plus_nnn(opcode),
            (0xC,   _,   _,   _) => self.set_vx_to_bit_and_rand_nn(digit_2, opcode),
            (0xD,   _,   _,   _) => self.draw_sprite(digit_2, digit_3, digit_4),
            (0xE,   _, 0x9, 0xE) => self.skip_if_key_pressed(digit_2),
            (0xE,   _, 0xA, 0x1) => self.skip_if_key_not_pressed(digit_2),
            (0xF,   _, 0x0, 0x7) => self.set_vx_to_dt(digit_2),
            (0xF,   _, 0x0, 0xA) => self.wait_key_press(digit_2),
            (0xF,   _, 0x1, 0x5) => self.set_dt_to_vx(digit_2),
            (0xF,   _, 0x1, 0x8) => self.set_st_to_vx(digit_2),
            (0xF,   _, 0x1, 0xE) => self.increment_i_by_vx(digit_2),
            (0xF,   _, 0x2, 0x9) => self.set_i_to_font_address(digit_2),
            (0xF,   _, 0x3, 0x3) => self.load_i_bcd_vx(digit_2),
            (0xF,   _, 0x5, 0x5) => self.store_v0_vx_into_i(digit_2),
            (0xF,   _, 0x6, 0x5) => self.load_i_into_v0_vx(digit_2),
            (  _,   _,   _,   _) => unimplemented!("Unimplemented opcode: {}", opcode),
        }
    }

    /// 00E0
    fn clear_display(&mut self) {
        self.display = [false; Self::SCREEN_WIDTH * Self::SCREEN_HEIGHT];
    }

    /// 00EE
    fn return_from_subroutine(&mut self) {
        self.pc = self.pop_stack();
    }

    /// 1NNN
    fn jump_to_address(&mut self, opcode: u16) {
        let nnn = opcode & 0xFFF;
        self.pc = nnn;
    }

    /// 2NNN
    fn call_subroutine(&mut self, opcode: u16) {
        let nnn = opcode & 0xFFF;
        self.push_stack(nnn);
        self.pc = nnn;
    }

    /// 3XNN
    #[allow(clippy::cast_possible_truncation)]
    fn skip_if_vx_equal_nn(&mut self, digit_2: u16, opcode: u16) {
        let x = digit_2 as usize;
        let nn = (opcode & 0xFF) as u8;
        if self.v_reg[x] == nn {
            self.pc += 2;
        }
    }

    /// 4XNN
    #[allow(clippy::cast_possible_truncation)]
    fn skip_if_vx_not_equal_nn(&mut self, digit_2: u16, opcode: u16) {
        let x = digit_2 as usize;
        let nn = (opcode & 0xFF) as u8;
        if self.v_reg[x] != nn {
            self.pc += 2;
        }
    }

    /// 5XY0
    fn skip_if_vx_equals_vy(&mut self, digit_2: u16, digit_3: u16) {
        let x = digit_2 as usize;
        let y = digit_3 as usize;
        if self.v_reg[x] == self.v_reg[y] {
            self.pc += 2;
        }
    }

    /// 6XNN
    #[allow(clippy::cast_possible_truncation)]
    fn set_vx_to_nn(&mut self, digit_2: u16, opcode: u16) {
        let x = digit_2 as usize;
        let nn = (opcode & 0xFF) as u8;
        self.v_reg[x] = nn;
    }

    /// 7XNN
    #[allow(clippy::cast_possible_truncation)]
    fn increment_vx_by_nn(&mut self, digit_2: u16, opcode: u16) {
        let x = digit_2 as usize;
        let nn = (opcode & 0xFF) as u8;
        self.v_reg[x] = self.v_reg[x].wrapping_add(nn);
    }

    /// 8XY0
    fn set_vx_to_vy(&mut self, digit_2: u16, digit_3: u16) {
        let x = digit_2 as usize;
        let y = digit_3 as usize;
        self.v_reg[x] = self.v_reg[y];
    }

    /// 8XY1
    fn set_vx_to_bit_or_vy(&mut self, digit_2: u16, digit_3: u16) {
        let x = digit_2 as usize;
        let y = digit_3 as usize;
        self.v_reg[x] |= self.v_reg[y];
    }

    /// 8XY2
    fn set_vx_to_bit_and_vy(&mut self, digit_2: u16, digit_3: u16) {
        let x = digit_2 as usize;
        let y = digit_3 as usize;
        self.v_reg[x] &= self.v_reg[y];
    }
    /// 8XY3
    fn set_vx_to_bit_xor_vy(&mut self, digit_2: u16, digit_3: u16) {
        let x = digit_2 as usize;
        let y = digit_3 as usize;
        self.v_reg[x] ^= self.v_reg[y];
    }

    /// 8XY4
    fn increment_vx_by_vy(&mut self, digit_2: u16, digit_3: u16) {
        let x = digit_2 as usize;
        let y = digit_3 as usize;

        let (new_v_x, carry) = self.v_reg[x].overflowing_add(self.v_reg[y]);
        let new_v_f = if carry { 1 } else { 0 };

        self.v_reg[x] = new_v_x;
        self.v_reg[0xF] = new_v_f;
    }

    /// 8XY5
    fn decrement_vx_by_vy(&mut self, digit_2: u16, digit_3: u16) {
        let x = digit_2 as usize;
        let y = digit_3 as usize;

        let (new_v_x, borrow) = self.v_reg[x].overflowing_sub(self.v_reg[y]);
        let new_v_f = if borrow { 0 } else { 1 };

        self.v_reg[x] = new_v_x;
        self.v_reg[0xF] = new_v_f;
    }

    /// 8XY6
    fn right_shift_vx(&mut self, digit_2: u16) {
        let x = digit_2 as usize;
        let lsb = self.v_reg[x] & 1;
        self.v_reg[x] >>= 1;
        self.v_reg[0xF] = lsb;
    }

    /// 8XY7
    fn set_vx_to_vy_minus_vx(&mut self, digit_2: u16, digit_3: u16) {
        let x = digit_2 as usize;
        let y = digit_3 as usize;

        let (new_v_x, borrow) = self.v_reg[y].overflowing_sub(self.v_reg[x]);
        let new_v_f = if borrow { 0 } else { 1 };

        self.v_reg[x] = new_v_x;
        self.v_reg[0xF] = new_v_f;
    }

    /// 8XYE
    fn left_shift_vx(&mut self, digit_2: u16) {
        let x = digit_2 as usize;
        let msb = (self.v_reg[x] >> 7) & 1;
        self.v_reg[x] <<= 1;
        self.v_reg[0xF] = msb;
    }

    /// 9XY0
    fn skip_if_vx_not_equal_vy(&mut self, digit_2: u16, digit_3: u16) {
        let x = digit_2 as usize;
        let y = digit_3 as usize;
        if self.v_reg[x] != self.v_reg[y] {
            self.pc += 2;
        }
    }

    /// ANNN
    fn set_i_to_nnn(&mut self, opcode: u16) {
        let nnn = opcode & 0xFFF;
        self.i_reg = nnn;
    }

    /// BNNN
    fn jump_v0_plus_nnn(&mut self, opcode: u16) {
        let nnn = opcode & 0xFFF;
        self.pc = u16::from(self.v_reg[0]) + nnn;
    }

    /// CXNN
    #[allow(clippy::cast_possible_truncation)]
    fn set_vx_to_bit_and_rand_nn(&mut self, digit_2: u16, opcode: u16) {
        let x = digit_2 as usize;
        let nn = (opcode & 0xFF) as u8;
        let rng = random::<u8>();
        self.v_reg[x] = rng & nn;
    }

    /// DXYN
    fn draw_sprite(&mut self, digit_2: u16, digit_3: u16, digit_4: u16) {
        let x_coord = u16::from(self.v_reg[digit_2 as usize]);
        let y_coord = u16::from(self.v_reg[digit_3 as usize]);
        let num_rows = digit_4;

        let mut flipped = false;
        for y_line in 0..num_rows {
            let addr = self.i_reg + y_line;
            let pixels = self.memory[addr as usize];

            for x_line in 0..8 {
                if (pixels & (0b_1000_0000 >> x_line)) != 0 {
                    let x = usize::from(x_coord + x_line) % Self::SCREEN_WIDTH;
                    let y = usize::from(y_coord + y_line) % Self::SCREEN_HEIGHT;

                    let idx = x + Self::SCREEN_WIDTH * y;

                    flipped |= self.display[idx];
                    self.display[idx] ^= true;
                }
            }
        }

        self.v_reg[0xF] = if flipped { 1 } else { 0 };
    }

    /// EX9E
    fn skip_if_key_pressed(&mut self, digit_2: u16) {
        let x = digit_2 as usize;
        let vx = self.v_reg[x];
        let key = self.keys[usize::from(vx)];
        if key {
            self.pc += 2;
        }
    }

    /// EXA1
    fn skip_if_key_not_pressed(&mut self, digit_2: u16) {
        let x = digit_2 as usize;
        let vx = self.v_reg[x];
        let key = self.keys[usize::from(vx)];
        if !key {
            self.pc += 2;
        }
    }

    /// FX07
    fn set_vx_to_dt(&mut self, digit_2: u16) {
        let x = digit_2 as usize;
        self.v_reg[x] = self.delay_timer;
    }

    /// FX0A
    #[allow(clippy::cast_possible_truncation)]
    fn wait_key_press(&mut self, digit_2: u16) {
        let x = digit_2 as usize;
        let mut pressed = false;
        for i in 0..self.keys.len() {
            if self.keys[i] {
                self.v_reg[x] = i as u8;
                pressed = true;
                break;
            }
        }
        if !pressed {
            self.pc -= 2;
        }
    }

    /// FX15
    fn set_dt_to_vx(&mut self, digit_2: u16) {
        let x = digit_2 as usize;
        self.delay_timer = self.v_reg[x];
    }

    /// FX18
    fn set_st_to_vx(&mut self, digit_2: u16) {
        let x = digit_2 as usize;
        self.sound_timer = self.v_reg[x];
    }

    /// FX1E
    fn increment_i_by_vx(&mut self, digit_2: u16) {
        let x = digit_2 as usize;
        let vx = u16::from(self.v_reg[x]);
        self.i_reg = self.i_reg.wrapping_add(vx);
    }

    /// FX29
    fn set_i_to_font_address(&mut self, digit_2: u16) {
        let x = digit_2 as usize;
        let c = u16::from(self.v_reg[x]);
        self.i_reg = c * 5;
    }

    /// FX33
    fn load_i_bcd_vx(&mut self, digit_2: u16) {
        let x = digit_2 as usize;
        let vx = self.v_reg[x];

        let hundreds = vx / 100;
        let tens = (vx - hundreds * 100) / 10;
        let ones = vx - hundreds * 100 - tens * 10;

        let i = self.i_reg as usize;
        self.memory[i] = hundreds;
        self.memory[i + 1] = tens;
        self.memory[i + 2] = ones;
    }

    /// FX55
    fn store_v0_vx_into_i(&mut self, digit_2: u16) {
        let x = digit_2 as usize;
        let i = self.i_reg as usize;
        for idx in 0..x {
            self.memory[i + idx] = self.v_reg[idx];
        }
    }

    /// FX65
    fn load_i_into_v0_vx(&mut self, digit_2: u16) {
        let x = digit_2 as usize;
        let i = self.i_reg as usize;
        for idx in 0..x {
            self.v_reg[idx] = self.memory[i + idx];
        }
    }
}
