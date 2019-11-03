use core::{fmt, ops};
use periphery_sys::{
    spi_bit_order_LSB_FIRST, spi_bit_order_MSB_FIRST, spi_bit_order_t, spi_close, spi_errmsg,
    spi_error_code_SPI_ERROR_ARG, spi_error_code_SPI_ERROR_CLOSE,
    spi_error_code_SPI_ERROR_CONFIGURE, spi_error_code_SPI_ERROR_OPEN,
    spi_error_code_SPI_ERROR_QUERY, spi_error_code_SPI_ERROR_TRANSFER, spi_fd, spi_free,
    spi_get_bit_order, spi_get_bits_per_word, spi_get_extra_flags, spi_get_max_speed, spi_get_mode,
    spi_new, spi_open_advanced, spi_set_bit_order, spi_set_bits_per_word, spi_set_extra_flags,
    spi_set_max_speed, spi_set_mode, spi_t, spi_tostring, spi_transfer,
};
use std::ffi::{CStr, CString};
use std::os::raw::{c_int, c_uint};
use thiserror::Error;

#[derive(Debug)]
pub struct Spi(*mut spi_t);

impl Spi {
    fn check_result(&mut self, result: c_int) -> Option<SpiError> {
        match result {
            0 => None,
            res => {
                let res_str: String = unsafe { CStr::from_ptr(spi_errmsg(self.0)) }
                    .to_string_lossy()
                    .to_string();
                match res {
                    spi_error_code_SPI_ERROR_ARG => Some(SpiError::Arg(res_str)),
                    spi_error_code_SPI_ERROR_OPEN => Some(SpiError::Open(res_str)),
                    spi_error_code_SPI_ERROR_QUERY => Some(SpiError::Query(res_str)),
                    spi_error_code_SPI_ERROR_CONFIGURE => Some(SpiError::Configure(res_str)),
                    spi_error_code_SPI_ERROR_TRANSFER => Some(SpiError::Transfer(res_str)),
                    spi_error_code_SPI_ERROR_CLOSE => Some(SpiError::Close(res_str)),
                    res => Some(SpiError::OutOfRange(res)),
                }
            }
        }
    }

    pub fn try_new(path: &str, mode: c_uint, max_speed: u32) -> Result<Self, SpiError> {
        Self::try_new_advanced(path, mode, max_speed, BitOrder::MsbFirst, 8, 0)
    }

    pub fn try_new_advanced(
        path: &str,
        mode: c_uint,
        max_speed: u32,
        bit_order: BitOrder,
        bits_per_word: u8,
        extra_flags: u8,
    ) -> Result<Self, SpiError> {
        if mode > 4 {
            return Err(SpiError::ModeOutOfRange);
        }

        let spi = unsafe { spi_new() };
        if spi as usize == 0 {
            return Err(SpiError::AllocationFailed);
        }
        unsafe {
            let mut spi = Spi { 0: spi };
            let path = CString::new(path).unwrap();
            match spi.check_result(spi_open_advanced(
                spi.0,
                path.as_ptr(),
                mode,
                max_speed,
                bit_order as spi_bit_order_t,
                bits_per_word,
                extra_flags,
            )) {
                None => Ok(spi),
                Some(err) => Err(err),
            }
        }
    }

    pub fn transfer(&mut self, data: &[u8]) -> Result<Vec<u8>, SpiError> {
        let len = data.len();
        unsafe {
            let mut out = Vec::with_capacity(len);
            match self.check_result(spi_transfer(
                self.0,
                data.as_ptr(),
                (&mut out[..]).as_mut_ptr(),
                len,
            )) {
                Some(result) => Err(result),
                None => Ok(out),
            }
        }
    }

    pub fn get_bit_order(&mut self) -> Result<spi_bit_order_t, SpiError> {
        unsafe {
            let mut out: spi_bit_order_t = Default::default();
            match self.check_result(spi_get_bit_order(self.0, &mut out as *mut spi_bit_order_t)) {
                None => Ok(out),
                Some(err) => Err(err),
            }
        }
    }

    pub fn get_bits_per_word(&mut self) -> Result<u8, SpiError> {
        unsafe {
            let mut out: u8 = Default::default();
            match self.check_result(spi_get_bits_per_word(self.0, &mut out as *mut u8)) {
                None => Ok(out),
                Some(err) => Err(err),
            }
        }
    }

    pub fn get_extra_flags(&mut self) -> Result<u8, SpiError> {
        unsafe {
            let mut out: u8 = Default::default();
            match self.check_result(spi_get_extra_flags(self.0, &mut out as *mut u8)) {
                None => Ok(out),
                Some(err) => Err(err),
            }
        }
    }

    pub fn get_max_speed(&mut self) -> Result<u32, SpiError> {
        unsafe {
            let mut out: u32 = Default::default();
            match self.check_result(spi_get_max_speed(self.0, &mut out as *mut u32)) {
                None => Ok(out),
                Some(err) => Err(err),
            }
        }
    }

    pub fn get_mode(&mut self) -> Result<c_uint, SpiError> {
        unsafe {
            let mut out: c_uint = Default::default();
            match self.check_result(spi_get_mode(self.0, &mut out as *mut c_uint)) {
                None => Ok(out),
                Some(err) => Err(err),
            }
        }
    }

    pub fn set_bit_order(&mut self, bit_order: spi_bit_order_t) -> Result<(), SpiError> {
        unsafe {
            match self.check_result(spi_set_bit_order(self.0, bit_order)) {
                None => Ok(()),
                Some(err) => Err(err),
            }
        }
    }

    pub fn set_bits_per_word(&mut self, bits_per_word: u8) -> Result<(), SpiError> {
        unsafe {
            match self.check_result(spi_set_bits_per_word(self.0, bits_per_word)) {
                None => Ok(()),
                Some(err) => Err(err),
            }
        }
    }

    pub fn set_extra_flags(&mut self, extra_flags: u8) -> Result<(), SpiError> {
        unsafe {
            match self.check_result(spi_set_extra_flags(self.0, extra_flags)) {
                None => Ok(()),
                Some(err) => Err(err),
            }
        }
    }

    pub fn set_max_speed(&mut self, max_speed: u32) -> Result<(), SpiError> {
        unsafe {
            match self.check_result(spi_set_max_speed(self.0, max_speed)) {
                None => Ok(()),
                Some(err) => Err(err),
            }
        }
    }

    pub fn set_mode(&mut self, mode: c_uint) -> Result<(), SpiError> {
        unsafe {
            match self.check_result(spi_set_mode(self.0, mode)) {
                None => Ok(()),
                Some(err) => Err(err),
            }
        }
    }

    pub fn get_file_descriptor(&mut self) -> c_int {
        unsafe { spi_fd(self.0) }
    }
}

impl ops::Drop for Spi {
    fn drop(&mut self) {
        match self.check_result(unsafe { spi_close(self.0) }) {
            Some(SpiError::Close(_)) => { /*Don't do anything for now*/ }
            Some(_) => {}
            _ => {}
        };
        unsafe { spi_free(self.0) }
    }
}

impl fmt::Display for Spi {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        const BUF_SIZE: usize = 150;

        let c_str = CString::new(Vec::from(&[1u8; BUF_SIZE - 1][..]));
        if c_str.is_err() {
            return Err(fmt::Error);
        }
        let c_str = unsafe {
            let c_str = c_str.unwrap().into_raw();
            let len_without_buf_size = spi_tostring(self.0, c_str, BUF_SIZE);
            let c_str = CString::from_raw(c_str);
            if len_without_buf_size < 0 {
                return Err(fmt::Error);
            } else if len_without_buf_size >= BUF_SIZE as i32 {
                println!("Increase BUF_SIZE to {} + 1", len_without_buf_size);
            }
            c_str
        };
        let c_str = c_str.to_string_lossy().to_string();
        fmt::Display::fmt(&c_str, fmt)
    }
}

pub enum BitOrder {
    MsbFirst = spi_bit_order_MSB_FIRST as isize,
    LsbFirst = spi_bit_order_LSB_FIRST as isize,
}

#[derive(Error, Debug)]
pub enum SpiError {
    #[error("{0}")]
    Close(String),
    #[error("{0}")]
    Transfer(String),
    #[error("{0}")]
    Configure(String),
    #[error("{0}")]
    Query(String),
    #[error("{0}")]
    Open(String),
    #[error("{0}")]
    Arg(String),
    #[error("allocation failed")]
    AllocationFailed,
    #[error("0 <= mode <= 3")]
    ModeOutOfRange,
    #[error("i32 not in range specified by libperiphery! {0} received.")]
    OutOfRange(i32),
    #[error("another error occurred")]
    Unknown,
}

#[cfg(test)]
#[cfg(target_os = "linux")]
mod tests {
    use super::*;

    #[test]
    fn create_device() {
        let spi = Spi::try_new("/dev/spidev0.0", 0, 1000);
        assert!(spi.is_err())
    }
}
