mod cpu;

use crate::cpu::CPU;

fn main() {
    let mut cpu = CPU::new();

    cpu.set_register(0, 5);
    cpu.set_register(1, 10);

    // 跳转到100，跳转到100，结束
    cpu.set_memory_by_slice(0x000..0x006, &[0x21, 0x00, 0x21, 0x00, 0x00, 0x00]);

    // 加0和1，加0和1，返回
    cpu.set_memory_by_slice(0x100..0x106, &[0x80, 0x14, 0x80, 0x14, 0x00, 0xEE]);

    cpu.run();

    println!("5 + [10 + 10] * 2 = {}", cpu.get_register(0))
}
