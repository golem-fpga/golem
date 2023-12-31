use byteorder::{LittleEndian, ReadBytesExt};
use mister_fpga::core::MisterFpgaCore;
use mister_fpga::fpga::MisterFpga;
use std::path::Path;

pub mod core;

pub struct CoreManager {
    fpga: MisterFpga,
}

impl CoreManager {
    pub fn new(fpga: MisterFpga) -> Self {
        Self { fpga }
    }

    pub fn fpga(&self) -> &MisterFpga {
        &self.fpga
    }

    pub fn fpga_mut(&mut self) -> &mut MisterFpga {
        &mut self.fpga
    }

    fn load(&mut self, program: &[u8]) -> Result<MisterFpgaCore, String> {
        let program = if &program[..6] != b"MiSTer" {
            program
        } else {
            let start = 16;
            let size = (&program[12..]).read_u32::<LittleEndian>().unwrap() as usize;
            &program[start..start + size]
        };

        self.fpga.wait_for_ready();
        self.fpga
            .load_rbf(program)
            .map_err(|e| format!("Could not load program: {e:?}"))?;
        self.fpga
            .core_reset()
            .map_err(|_| "Could not reset the Core".to_string())?;

        let core = MisterFpgaCore::new(self.fpga.clone())
            .map_err(|e| format!("Could not instantiate Core: {e}"))?;

        self.fpga_mut().osd_disable();

        unsafe {
            crate::file_io::FindStorage();
            crate::platform::de10::user_io::user_io_init(
                "\0".as_ptr() as *const _,
                std::ptr::null(),
            );
        }
        Ok(core)
    }
}

impl crate::platform::CoreManager for CoreManager {
    type Core = MisterFpgaCore;

    fn load_program(&mut self, path: impl AsRef<Path>) -> Result<MisterFpgaCore, String> {
        let bytes = std::fs::read(path.as_ref()).map_err(|e| e.to_string())?;
        let core = self.load(&bytes)?;
        Ok(core)
    }

    fn load_menu(&mut self) -> Result<Self::Core, String> {
        #[repr(align(4))]
        struct Aligned<T: ?Sized>(T);

        let bytes = Aligned(include_bytes!("../../../assets/menu.rbf"));

        let core = self.load(bytes.0)?;
        self.fpga_mut().osd_enable();
        Ok(core)
    }

    fn show_menu(&mut self) {
        self.fpga_mut().osd_enable();
    }

    fn hide_menu(&mut self) {
        self.fpga_mut().osd_disable();
    }
}
