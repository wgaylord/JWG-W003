
use crate::BusInterface;

#[derive(Clone)]
pub struct DMAEntry{
    src_addr: u32,
    src_length: u16,
    dst_addr: u32,
    dst_length: u16,
    flags: u8,
    transfer_counter: u8,
    clock_div: u16,
}

pub struct DMA {
    bus: Box<dyn BusInterface>,
    dma_channels: Vec<DMAEntry>,
    dma_base_addr: u32,
    dma_channel_count: u8,
}


impl DMA {
    pub fn new(bus: Box<dyn BusInterface>, base_addr: u32,channel_count: u8) -> DMA {

        let mut dma = DMA{ bus:bus,dma_channels: Vec::new() , dma_base_addr:base_addr,dma_channel_count:channel_count };
        dma.dma_channels.resize(channel_count as usize,DMAEntry{src_addr:0,src_length:0,dst_addr:0,dst_length:0,flags:0,transfer_counter:0,clock_div:0});
        dma
    }
    fn is_dma_register(&self,addr:u32)-> bool {

        if (addr as i64 - self.dma_base_addr as i64) > 0 && (addr as i64 - self.dma_base_addr as i64 ) < (self.dma_channel_count as i64 *16){
            return true
        }
        return false
    }
 
}


impl BusInterface for DMA {
    fn get_start_address(&self) -> u32 {
        return self.bus.get_start_address();    
    }

    fn get_end_address(&self) -> u32{
        return self.bus.get_end_address();    
    }


    fn read(&self,addr: u32) -> Result<u8, ()>{
        if self.is_dma_register(addr){
                 return Ok(0);
            }else{

                return self.bus.read(addr);
            }

    }
    fn write(&mut self,addr:u32,data:u8) -> () {
       if self.is_dma_register(addr){
               
            }else{

            self.bus.write(addr,data);
            }
    }

  fn tick(&mut self) -> Result<(),u8>{
        return self.bus.tick()
    }
}


