
use emulator::BusInterface;

use emulator::memory::RAM;
use emulator::memory::ROM;
use emulator::bus::BUS;
use emulator::dma::DMA;



fn main() {

    let ram = RAM::new(0,1024);
    let rom = ROM::new(1024,1024,"rom.bin");

    let mut bus = BUS::new(0,2048);

    bus.add_device(ram);
    bus.add_device(rom);

    println!("Writting to RAM 2");
    bus.write(2,2);
    println!("RAM 2 is {}",bus.read(2).unwrap());

    println!("Read ROM 1 {}",bus.read(1024).unwrap());

    let mut dma = DMA::new(Box::new(bus),3000,2);

    println!("Writting to RAM 2");
    dma.write(2,3);
    println!("RAM 2 is {}",dma.read(2).unwrap());

    println!("Read ROM 1 {}",dma.read(1024).unwrap());


}
