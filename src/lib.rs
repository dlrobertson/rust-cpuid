use std::ffi::CString;
use std::os::raw::c_char;

#[link(name="cpuid", kind="static")]
extern {
    pub fn get_name(bytes: *mut c_char) -> u16;
}

#[derive(Debug)]
pub enum CPUIdErr {
    NameError(String)
}

pub type CPUIdResult<T> = Result<T, CPUIdErr>;

#[derive(Debug)]
pub struct CPUId {
    vendor_str: String,
    high_value: u16
}

impl CPUId {
    pub fn new() -> CPUId {
        let (high_value, vendor_str) = unsafe {
            let res = CString::new("             ").unwrap().into_raw();
            let value = get_name(res);
            let vendor_str = String::from_raw_parts(res as *mut u8, 12, 12);
            (value, vendor_str)
        };
        CPUId {
            vendor_str: vendor_str,
            high_value: high_value
        }
    }

    pub fn vendor(&self) -> String {
        self.vendor_str.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::CPUId;
    #[test]
    fn it_works() {
        println!("{:?}", CPUId::new());
    }
}
