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

struct Emulator {
    pc: u16,
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

        (_, _, _, _) => unimplemented!("Unimplemented opcode: {}", opcode),
    }
}
}

