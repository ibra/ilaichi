use ilaichi_chip8::Emulator;

fn main() {
    let emu: Emulator = Emulator::new();
    println!("{}", "running emulator, start instruction {emu.pc}");

}
