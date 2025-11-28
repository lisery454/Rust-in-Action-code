use std::ops::Range;

/// 指令集 u16 分为四个u4，第一个u4指示当前的类型，后面的如何解释跟着这个类型变化
///
/// 0x8 x y 0x4 : 表示把x位置和y位置的寄存器的值加起来存储到x位置寄存器
/// 
/// 0x2 nnn: 表示当前的内存位置跳转到nnn这个12位的内存地址，CALL函数
/// 
/// 0x00EE: 表示返回上一次跳转前的内存地址，RETURN函数
///
pub struct CPU {
    registers: [u8; 16],       // 16个寄存器位置，用一个u4就可以表示位置
    memory: [u8; 0x1000],      // 2^12 个内存位置
    position_in_memory: usize, // usize只用12位，因为内存地址只有2^12个
    stack: [u16; 16],          // 表示16个u16数据的栈
    stack_pointer: usize,      // 用来表示当前栈在哪一层，实际上只用u4就行了，这里用usize方便索引
}



impl CPU {
    pub fn new() -> Self {
        Self {
            registers: [0; 16],
            memory: [0; 0x1000],
            position_in_memory: 0,
            stack: [0; 16],
            stack_pointer: 0,
        }
    }

    pub fn set_register(&mut self, index: usize, value: u8) {
        self.registers[index] = value;
    }

    pub fn get_register(&self, index: usize) -> u8 {
        self.registers[index]
    }

    fn get_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;
        // 把p位置和p+1位置的u8合成一个u16
        op_byte1 << 8 | op_byte2
    }

    pub fn set_memory(&mut self, index: usize, value: u8) {
        self.memory[index] = value;
    }

    pub fn set_memory_by_slice(&mut self, range: Range<usize>, slice: &[u8]) {
        self.memory[range].copy_from_slice(slice);
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.get_opcode();
            self.position_in_memory += 2; // 执行下一个内存位置

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            let nnn = opcode & 0x0FFF;

            match (c, x, y, d) {
                (0, 0, 0, 0) => {
                    return;
                }
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x2, _, _, _) => self.call(nnn),
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("stack overflow!");
        }

        // 存储当前的内存位置
        stack[sp] = self.position_in_memory as u16;
        self.stack_pointer += 1;

        // 跳转到新的内存位置
        self.position_in_memory = addr as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("stack underflow!");
        }

        self.stack_pointer -= 1;
        let call_addr = self.stack[self.stack_pointer];
        self.position_in_memory = call_addr as usize;
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let x = x as usize;
        let y = y as usize;
        let arg1 = self.get_register(x);
        let arg2 = self.get_register(y);

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.set_register(x, val);

        // 0xF位 判断是否溢出
        if overflow {
            self.set_register(0xF, 1);
        } else {
            self.set_register(0xF, 0);
        }
    }
}
