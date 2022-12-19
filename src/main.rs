use minifb::{Scale, Window, WindowOptions};
use crate::emulator::{ArithmeticTarget, CPU, FlagsRegister, Instruction, Registers};

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

mod emulator;

fn main() {
    let mut cpu = CPU{
        registers: Registers{
            a: 0,
            b: 0b10010001,
            c: 0,
            d: 0,
            e: 0,
            f: FlagsRegister{
                zero: false,
                subtract: false,
                half_carry: false,
                carry: false,
            },
            h: 0,
            l: 0,
        },
    };

    cpu.execute(Instruction::SRL(ArithmeticTarget::B));
    cpu.print();

    /*let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Emulator",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    ).unwrap();
    window.set_position(450, 120);

    while window.is_open() {
        /*buffer[((200 /*y*/ as usize) * (WIDTH)) + 200 /*x*/ as usize] = 0x00ffffff;*/

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).expect("Oops!");
        buffer.clear();
        buffer.resize(WIDTH*HEIGHT,0);
    }*/
}