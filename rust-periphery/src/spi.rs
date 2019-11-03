use periphery_sys::{
    spi_bit_order_LSB_FIRST, spi_bit_order_MSB_FIRST, spi_bit_order_t, spi_close, spi_errmsg,
    spi_error_code_SPI_ERROR_ARG, spi_error_code_SPI_ERROR_CLOSE,
    spi_error_code_SPI_ERROR_CONFIGURE, spi_error_code_SPI_ERROR_OPEN,
    spi_error_code_SPI_ERROR_QUERY, spi_error_code_SPI_ERROR_TRANSFER, spi_free, spi_get_bit_order,
    spi_get_bits_per_word, spi_get_extra_flags, spi_get_max_speed, spi_get_mode, spi_new,
    spi_open_advanced, spi_set_bit_order, spi_set_bits_per_word, spi_set_extra_flags,
    spi_set_max_speed, spi_set_mode, spi_t, spi_transfer,
};
use std::ffi::{CStr, CString};
use std::os::raw::c_uint;
use thiserror::Error;

pub struct Spi(*mut spi_t);

impl Spi {
    fn check_result(&self, result: i32) -> Result<(), SpiError> {
        match result {
            0 => Ok(()),
            res => {
                let res_str: String = unsafe { CStr::from_ptr(spi_errmsg(self.0)) }
                    .to_string_lossy()
                    .to_string();
                match res {
                    spi_error_code_SPI_ERROR_ARG => Err(SpiError::Arg(res_str)),
                    spi_error_code_SPI_ERROR_OPEN => Err(SpiError::Open(res_str)),
                    spi_error_code_SPI_ERROR_QUERY => Err(SpiError::Query(res_str)),
                    spi_error_code_SPI_ERROR_CONFIGURE => Err(SpiError::Configure(res_str)),
                    spi_error_code_SPI_ERROR_TRANSFER => Err(SpiError::Transfer(res_str)),
                    spi_error_code_SPI_ERROR_CLOSE => Err(SpiError::Close(res_str)),
                    res => Err(SpiError::OutOfRange(res)),
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
        let spi = Spi { 0: spi };
        let path = CString::new(path).unwrap();
        match spi.check_result(unsafe {
            spi_open_advanced(
                spi.0,
                path.as_ptr(),
                mode,
                max_speed,
                bit_order as spi_bit_order_t,
                bits_per_word,
                extra_flags,
            )
        }) {
            Ok(_) => Ok(spi),
            Err(err) => Err(err),
        }
    }
}

impl core::ops::Drop for Spi {
    fn drop(&mut self) {
        match self.check_result(unsafe { spi_close(self.0) }) {
            Err(SpiError::Close(_)) => { /*Don't do anything for now*/ }
            Err(_) => {}
            _ => {}
        };
        unsafe { spi_free(self.0) }
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
        let err = Spi::try_new("/dev/spidev0.0", 0, 1000).err();
        assert!(err.is_some());
    }
}
