// 1 byte, 4 bytes, 2 bytes -> |X|_|_|_| |X|X|X|X| |X|X|_|_| -> 12 bytes
#[repr(C)]
struct Demo(u8, u32, u16);

// 1 byte, 2 bytes, 4 bytes -> |X|X|X|_| |X|X|X|X| -> 8 bytes
#[repr(C)]
struct Demo2(u8, u16, u32);

fn main() {
    println!(
        "Size: {}, Alignment: {}",
        std::mem::size_of::<Demo>(),
        std::mem::align_of::<Demo>()
    );

    println!(
        "Size: {}, Alignment: {}",
        std::mem::size_of::<Demo2>(),
        std::mem::align_of::<Demo2>()
    );
}
