use std::io;

#[allow(dead_code)]
extern "C" {
    fn aici_host_print(ptr: *const u8, len: u32);
}

#[cfg(not(target_arch = "wasm32"))]
pub type Printer = std::io::Stdout;

#[cfg(target_arch = "wasm32")]
pub struct Printer {}

#[cfg(target_arch = "wasm32")]
impl io::Write for Printer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        unsafe { aici_host_print(buf.as_ptr(), buf.len() as u32) };
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub fn init_panic() {
    #[cfg(target_arch = "wasm32")]
    std::panic::set_hook(Box::new(|info| {
        let file = info.location().unwrap().file();
        let line = info.location().unwrap().line();
        let col = info.location().unwrap().column();

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &s[..],
                None => "Box<Any>",
            },
        };

        let err_info = format!("Panicked at '{}', {}:{}:{}\n", msg, file, line, col);
        _print(&err_info);
    }))
}

pub fn stdout() -> Printer {
    #[cfg(target_arch = "wasm32")]
    {
        Printer {}
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        io::stdout()
    }
}

pub fn _print(msg: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        let vec: Vec<u8> = msg.into();
        unsafe { aici_host_print(vec.as_ptr(), vec.len() as u32) };
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::io::Write;
        std::io::stdout().write_all(msg.as_bytes()).unwrap();
    }
}

#[no_mangle]
pub extern "C" fn aici_init() {
    init_panic();
}

#[allow(dead_code)]
extern "C" {
    fn aici_host_read_arg(ptr: *mut u8, len: u32) -> u32;
}

pub fn arg_bytes() -> Vec<u8> {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        let size = aici_host_read_arg(0 as _, 0);
        let mut buffer = vec![0u8; size as usize];
        aici_host_read_arg(buffer.as_mut_ptr(), size);
        return buffer;
    }

    #[cfg(not(target_arch = "wasm32"))]
    std::fs::read("arg.json").unwrap()
}