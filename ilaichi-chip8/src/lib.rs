use rand::random;

const FONTSET_SIZE: usize = 80;
const FONTSET: [u8; FONTSET_SIZE] = [
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
0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

const RAM_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;

const NUM_REGS: usize = 16;
const NUM_KEYS: usize = 16;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

const START_ADDR: u16 = 0x200;

pub struct Emulator {
    pub pc: u16,

    ram: [u8;  RAM_SIZE],
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],

    v_registers: [u8; NUM_REGS], 
    i_register: u16,

    stack: [u16; STACK_SIZE],
    stack_pointer: u16,

    delay_timer: u8,
    sound_timer: u8,

    keys: [bool; NUM_KEYS] 
}

impl Emulator {
 pub fn new() -> Self {
      let mut instance =  Self {
            pc:  START_ADDR,
            ram: [0; RAM_SIZE], 
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],

            v_registers: [0;NUM_REGS],
            i_register: 0,

            stack : [0; STACK_SIZE],
            stack_pointer: 0,

            delay_timer: 0,
            sound_timer: 0,

            keys: [false; NUM_KEYS]
        };

        instance.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
        instance
}

pub fn reset(&mut self) {
            self.pc =  START_ADDR;
            self.ram = [0; RAM_SIZE]; 
            self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];

            self.v_registers = [0;NUM_REGS];
            self.i_register = 0;

            self.stack = [0; STACK_SIZE];
            self.stack_pointer = 0;

            self.delay_timer =  0;
            self.sound_timer = 0;

            self.keys = [false; NUM_KEYS];
}

fn push(&mut self, val: u16) {
  self.stack[self.stack_pointer as usize] = val;
  self.stack_pointer += 1;
}

fn pop(&mut self) -> u16 {
  self.stack_pointer -= 1;
  self.stack[self.stack_pointer as usize]
}

pub fn tick(&mut self) {
 let opcode = self.fetch();
 self.execute(opcode);
}

pub fn tick_timers(&mut self) {
    if self.delay_timer > 0 {
        self.delay_timer -= 1;    
    }

    if self.sound_timer > 0 {
        if self.sound_timer == 1 {
            // TODO: beep noise    
        }

        self.sound_timer -= 1;
    }   
}

fn fetch(&mut self) -> u16 {
    let higher_byte = self.ram[self.pc as usize] as u16;
    let lower_byte = self.ram[(self.pc+1) as usize] as u16;
    
    self.pc += 2;
    
    let op = (higher_byte << 8) | lower_byte;

    op 
}

fn execute(&mut self, opcode: u16) {
    let digit1 = (opcode & 0xF000) >> 12; 
    let digit2 = (opcode & 0x0F00) >> 8; 
    let digit3 = (opcode & 0x00F0) >> 4; 
    let digit4 = opcode & 0x000F; 

    match (digit1,digit2,digit3,digit4) {
        (0,0,0,0) => return,
        (0,0,0xE,0) => { 
                self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT]
        }, 
        (0,0,0xE,0xE) => {
               let ret = self.pop();
               self.pc = ret;
        },
        (1,_,_,_) => {
                let nnn = opcode & 0xFFF; 
                self.pc = nnn; 
        },
        (2,_,_,_) => {
                let nnn = opcode & 0xFFF;

                self.push(self.pc);
                self.pc = nnn;
        },
        (3,_,_,_) => {
                let x = digit2 as usize;
                let nn = (opcode & 0xFF) as u8;

                if self.v_registers[x] == nn {
                    self.pc += 2;
                }
        },
        (4,_,_,_) => {
                let x = digit2 as usize;
                let nn = (opcode & 0xFF) as u8;

                if self.v_registers[x] != nn {
                    self.pc += 2;
                }
        },
        (5,_,_,0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                if self.v_registers[x] == self.v_registers[y] {
                    self.pc += 2;
                }
        },
        (6,_,_,_) => {
                let x = digit2 as usize;
                let kk = (opcode & 0xFF) as u8;

                self.v_registers[x] = kk;
        },
        (7,_,_,_) => {
                let x = digit2 as usize;
                let kk = (opcode & 0xFF) as u8;

                self.v_registers[x] = self.v_registers[x] + kk;
        },
        (8,_,_,0) => {
                let x = digit2 as usize;
                let y = digit3 as usize; 

                self.v_registers[x] = self.v_registers[y];
        },
        (8,_,_,1) => {
                let x = digit2 as usize;
                let y = digit3 as usize; 

                self.v_registers[x] = self.v_registers[x] | self.v_registers[y];
        },
        (8,_,_,2) => { 
                let x = digit2 as usize;
                let y = digit3 as usize; 

                self.v_registers[x] = self.v_registers[x] & self.v_registers[y];
        },
        (8,_,_,3) => { 
                let x = digit2 as usize;
                let y = digit3 as usize; 

                self.v_registers[x] = self.v_registers[x] ^ self.v_registers[y];
        },
        (8,_,_,4) => { 
                let x = digit2 as usize;
                let y = digit3 as usize; 

                let (vx_new,carry) = self.v_registers[x].overflowing_add(self.v_registers[y]);
                let vf_new = if carry {1} else {0}; 
                
                self.v_registers[x] = vx_new;
                self.v_registers[0xF] = vf_new;
        },
        (8,_,_,5) => { 
                let x = digit2 as usize;
                let y = digit3 as usize; 

                let(vx_new,borrow) = self.v_registers[x].overflowing_sub(self.v_registers[y]);
                let vf_new = if !borrow {1} else {0};

                self.v_registers[x] = vx_new;
                self.v_registers[0xF] = vf_new;
                
        },
        (8,_,_,6) => { 
                let x = digit2 as usize;
                let lsb = self.v_registers[x] & 1;

                self.v_registers[x] >>= 1;
                self.v_registers[0xF] = lsb;
        },
        (8,_,_,7) => { 
                let x = digit2 as usize;
                let y = digit3 as usize; 

                let(vx_new,borrow) = self.v_registers[y].overflowing_sub(self.v_registers[x]);
                let vf_new = if !borrow {1} else {0};

                self.v_registers[x] = vx_new;
                self.v_registers[0xF] = vf_new;
                
        },
        (8,_,_,0xE) => { 
                let x = digit2 as usize;
                let msb = (self.v_registers[x] >> 7) & 1;
                
                self.v_registers[x] <<= 1;
                self.v_registers[0xF] = msb;
        },
        (9,_,_,0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                if self.v_registers[x] != self.v_registers[y] {
                    self.pc += 2;
                }
        },
        (0xA,_,_,_) => {
                let nnn = opcode & 0xFFF;
                self.i_register = nnn;
        },
        (0xB,_,_,_) => {
                let nnn = opcode & 0xFFF;
                self.pc = nnn + (self.v_registers[0] as u16);
        },
        (0xC,_,_,_) => {
                let x = digit2 as usize;
                let kk = (opcode & 0xFF) as u8;
                let rnd: u8 = random();

                self.v_registers[x] = rnd & kk; 
        },
        (0xD,_,_,_) => {
                //TODO: implement drawing sprites
                let x_c = self.v_registers[digit2 as usize];
                let y_c = self.v_registers[digit3 as usize];

                let n = digit4 as usize;
        },
        (0xE,_,9,0xE) => {
                let x = digit2 as usize;
                let v_x = self.v_registers[x];
                let key_pressed = self.keys[v_x as usize];               

                if key_pressed {
                        self.pc += 2
                }
        },
        (0xE,_,0xA,1) => {
                let x = digit2 as usize;
                let v_x = self.v_registers[x];
                let key_pressed = self.keys[v_x as usize];               

                if !key_pressed {
                        self.pc += 2
                }
        },
        (0xF,_,0,7) => {
                let x = digit2 as usize;
                self.v_registers[x] = self.delay_timer;
        },
        (0xF,_,0,0xA) => {
                let x = digit2 as usize;
                // TODO: implement waiting for keypress
        }, 
        (0xF,_,1,5) => {
                let x = digit2 as usize;
                self.delay_timer = self.v_registers[x];
        }, 
        (0xF,_,1,8) => {
                let x = digit2 as usize;
                self.sound_timer = self.v_registers[x];
        },
        (0xF,_,1,0xE) => {
                let x = digit2 as usize;
                let v_x = self.v_registers[x];
                self.i_register = self.i_register.wrapping_add(v_x as u16);
        },
        (0xF,_,2,9) => {
                let x = digit2 as usize;
                self.i_register = (self.v_registers[x] as u16) * 5;
        },
        (0xF,_,3,3) => {
                let x = digit2 as usize;
                let v_x = self.v_registers[x] as f32;

                self.ram[self.i_register as usize] = (v_x / 100.0).floor() as u8;
                self.ram[(self.i_register+1) as usize] = ((v_x / 10.0) % 10.0).floor() as u8;
                self.ram[(self.i_register+2) as usize] = (v_x % 10.0) as u8; 
        },
        (0xF,_,5,5) => {
                let x = digit2 as usize;
                for i in 1..=x {
                       self.ram[i+x] = self.v_registers[i];   
                }
        },
        (0xF,_,6,5) => {
                let x = digit2 as usize;
                for i in 1..=x {
                       self.v_registers[i] = self.ram[i+x];   
                }
        }

        (_, _, _, _) => unimplemented!("Unimplemented opcode: {}", opcode),
    }
}
}

