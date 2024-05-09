struct Emulator {
    eax: u32,
    ecx: u32,
    edx: u32,
    ebx: u32,
    esp: u32,
    ebp: u32,
    esi: u32,
    edi: u32,
    memory: Box<[u8]>,
    eip: u32,
}

impl Emulator {
    fn new(eip: u32, esp: u32, binary: Vec<u8>) -> Emulator {
        Emulator {
            eax: 0,
            ecx: 0,
            edx: 0,
            ebx: 0,
            esp,
            ebp: 0,
            esi: 0,
            edi: 0,
            memory: binary.into_boxed_slice(),
            eip,
        }
    }
    fn dump_registers(&self) {
        println!("EAX = 0x{:08x}", self.eax);
        println!("ECX = 0x{:08x}", self.ecx);
        println!("EDX = 0x{:08x}", self.edx);
        println!("EBX = 0x{:08x}", self.ebx);
        println!("ESP = 0x{:08x}", self.esp);
        println!("EBP = 0x{:08x}", self.ebp);
        println!("ESI = 0x{:08x}", self.esi);
        println!("EDI = 0x{:08x}", self.edi);
        println!("EIP = 0x{:08x}", self.eip);
    }
    fn run(&mut self) {
        loop {
            self.instruct();
            if self.eip == 0x00 {
                break;
            }
        }
    }
    fn instruct(&mut self) {
        let code = self.get_code8(0);
        match code {
            0xb8 => self.mov_r32_imm32(0),
            0xb9 => self.mov_r32_imm32(1),
            0xba => self.mov_r32_imm32(2),
            0xbb => self.mov_r32_imm32(3),
            0xbc => self.mov_r32_imm32(4),
            0xbd => self.mov_r32_imm32(5),
            0xbe => self.mov_r32_imm32(6),
            0xbf => self.mov_r32_imm32(7),
            0xeb => self.short_jump(),
            _ => {
                println!("Not implemented: 0x{:02x}", code);
                std::process::exit(1);
            }
        }
    }

    fn mov_r32_imm32(&mut self, register_index: u8) {
        let value = self.get_code32(1);
        match register_index {
            0 => self.eax = value,
            1 => self.ecx = value,
            2 => self.edx = value,
            3 => self.ebx = value,
            4 => self.esp = value,
            5 => self.ebp = value,
            6 => self.esi = value,
            7 => self.edi = value,
            _ => {
                println!("Not implemented: register_index = {}", register_index);
                std::process::exit(1);
            }
        }
        self.eip += 5;
    }
    fn short_jump(&mut self) {
        let diff = self.get_sign_code8(1);
        self.eip += 2;
        self.eip = (self.eip as i64 + diff as i64) as u32;
    }

    fn get_code8(&self, index: usize) -> u8 {
        self.memory[self.eip as usize + index]
    }
    fn get_sign_code8(&self, index: usize) -> i8 {
        self.get_code8(index) as i8
    }
    fn get_code32(&self, index: usize) -> u32 {
        let mut ret: u32 = 0;
        for i in 0..4 {
            ret |= (self.get_code8(index + i) as u32) << (i * 8);
        }
        ret
    }
}

fn main() {
    let binary = std::fs::read("test.bin").unwrap();
    let mut emu = Emulator::new(0x0000, 0x7c00, binary);
    emu.run();
    emu.dump_registers();
}
