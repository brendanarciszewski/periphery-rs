#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

#[cfg(target_os = "linux")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(not(target_os = "linux"))]
mod not_linux {
    pub const gpio_error_code_GPIO_ERROR_ARG: gpio_error_code = -1;
    pub const gpio_error_code_GPIO_ERROR_OPEN: gpio_error_code = -2;
    pub const gpio_error_code_GPIO_ERROR_NOT_FOUND: gpio_error_code = -3;
    pub const gpio_error_code_GPIO_ERROR_QUERY: gpio_error_code = -4;
    pub const gpio_error_code_GPIO_ERROR_CONFIGURE: gpio_error_code = -5;
    pub const gpio_error_code_GPIO_ERROR_UNSUPPORTED: gpio_error_code = -6;
    pub const gpio_error_code_GPIO_ERROR_INVALID_OPERATION: gpio_error_code = -7;
    pub const gpio_error_code_GPIO_ERROR_IO: gpio_error_code = -8;
    pub const gpio_error_code_GPIO_ERROR_CLOSE: gpio_error_code = -9;
    pub type gpio_error_code = i32;
    pub const gpio_direction_GPIO_DIR_IN: gpio_direction = 0;
    pub const gpio_direction_GPIO_DIR_OUT: gpio_direction = 1;
    pub const gpio_direction_GPIO_DIR_OUT_LOW: gpio_direction = 2;
    pub const gpio_direction_GPIO_DIR_OUT_HIGH: gpio_direction = 3;
    pub type gpio_direction = u32;
    pub use self::gpio_direction as gpio_direction_t;
    pub const gpio_edge_GPIO_EDGE_NONE: gpio_edge = 0;
    pub const gpio_edge_GPIO_EDGE_RISING: gpio_edge = 1;
    pub const gpio_edge_GPIO_EDGE_FALLING: gpio_edge = 2;
    pub const gpio_edge_GPIO_EDGE_BOTH: gpio_edge = 3;
    pub type gpio_edge = u32;
    pub use self::gpio_edge as gpio_edge_t;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct gpio_handle {
        _unused: [u8; 0],
    }
    pub type gpio_t = gpio_handle;
    extern "C" {
        pub fn gpio_new() -> *mut gpio_t;
    }
    extern "C" {
        pub fn gpio_open(
            gpio: *mut gpio_t,
            path: *const ::std::os::raw::c_char,
            line: ::std::os::raw::c_uint,
            direction: gpio_direction_t,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn gpio_open_name(
            gpio: *mut gpio_t,
            path: *const ::std::os::raw::c_char,
            name: *const ::std::os::raw::c_char,
            direction: gpio_direction_t,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn gpio_open_sysfs(
            gpio: *mut gpio_t,
            line: ::std::os::raw::c_uint,
            direction: gpio_direction_t,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn gpio_read(gpio: *mut gpio_t, value: *mut bool) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn gpio_write(gpio: *mut gpio_t, value: bool) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn gpio_poll(
            gpio: *mut gpio_t,
            timeout_ms: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn gpio_close(gpio: *mut gpio_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn gpio_free(gpio: *mut gpio_t);
    }
    extern "C" {
        pub fn gpio_read_event(
            gpio: *mut gpio_t,
            edge: *mut gpio_edge_t,
            timestamp: *mut u64,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn gpio_get_direction(
            gpio: *mut gpio_t,
            direction: *mut gpio_direction_t,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn gpio_get_edge(gpio: *mut gpio_t, edge: *mut gpio_edge_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn gpio_set_direction(
            gpio: *mut gpio_t,
            direction: gpio_direction_t,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn gpio_set_edge(gpio: *mut gpio_t, edge: gpio_edge_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn gpio_line(gpio: *mut gpio_t) -> ::std::os::raw::c_uint;
    }
    extern "C" {
        pub fn gpio_fd(gpio: *mut gpio_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn gpio_name(
            gpio: *mut gpio_t,
            str: *mut ::std::os::raw::c_char,
            len: usize,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn gpio_chip_fd(gpio: *mut gpio_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn gpio_chip_name(
            gpio: *mut gpio_t,
            str: *mut ::std::os::raw::c_char,
            len: usize,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn gpio_chip_label(
            gpio: *mut gpio_t,
            str: *mut ::std::os::raw::c_char,
            len: usize,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn gpio_tostring(
            gpio: *mut gpio_t,
            str: *mut ::std::os::raw::c_char,
            len: usize,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn gpio_errno(gpio: *mut gpio_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn gpio_errmsg(gpio: *mut gpio_t) -> *const ::std::os::raw::c_char;
    }

    pub type __u8 = ::std::os::raw::c_uchar;
    pub type __u16 = ::std::os::raw::c_ushort;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct i2c_msg {
        pub addr: __u16,
        pub flags: __u16,
        pub len: __u16,
        pub buf: *mut __u8,
    }
    pub const i2c_error_code_I2C_ERROR_ARG: i2c_error_code = -1;
    pub const i2c_error_code_I2C_ERROR_OPEN: i2c_error_code = -2;
    pub const i2c_error_code_I2C_ERROR_QUERY: i2c_error_code = -3;
    pub const i2c_error_code_I2C_ERROR_NOT_SUPPORTED: i2c_error_code = -4;
    pub const i2c_error_code_I2C_ERROR_TRANSFER: i2c_error_code = -5;
    pub const i2c_error_code_I2C_ERROR_CLOSE: i2c_error_code = -6;
    pub type i2c_error_code = i32;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct i2c_handle {
        _unused: [u8; 0],
    }
    pub type i2c_t = i2c_handle;
    extern "C" {
        pub fn i2c_new() -> *mut i2c_t;
    }
    extern "C" {
        pub fn i2c_open(
            i2c: *mut i2c_t,
            path: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn i2c_transfer(
            i2c: *mut i2c_t,
            msgs: *mut i2c_msg,
            count: usize,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn i2c_close(i2c: *mut i2c_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn i2c_free(i2c: *mut i2c_t);
    }
    extern "C" {
        pub fn i2c_fd(i2c: *mut i2c_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn i2c_tostring(
            i2c: *mut i2c_t,
            str: *mut ::std::os::raw::c_char,
            len: usize,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn i2c_errno(i2c: *mut i2c_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn i2c_errmsg(i2c: *mut i2c_t) -> *const ::std::os::raw::c_char;
    }

    pub const mmio_error_code_MMIO_ERROR_ARG: mmio_error_code = -1;
    pub const mmio_error_code_MMIO_ERROR_OPEN: mmio_error_code = -2;
    pub const mmio_error_code_MMIO_ERROR_CLOSE: mmio_error_code = -3;
    pub type mmio_error_code = i32;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct mmio_handle {
        _unused: [u8; 0],
    }
    pub type mmio_t = mmio_handle;
    extern "C" {
        pub fn mmio_new() -> *mut mmio_t;
    }
    extern "C" {
        pub fn mmio_open(mmio: *mut mmio_t, base: usize, size: usize) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn mmio_ptr(mmio: *mut mmio_t) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn mmio_read32(
            mmio: *mut mmio_t,
            offset: usize,
            value: *mut u32,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn mmio_read16(
            mmio: *mut mmio_t,
            offset: usize,
            value: *mut u16,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn mmio_read8(
            mmio: *mut mmio_t,
            offset: usize,
            value: *mut u8,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn mmio_read(
            mmio: *mut mmio_t,
            offset: usize,
            buf: *mut u8,
            len: usize,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn mmio_write32(mmio: *mut mmio_t, offset: usize, value: u32) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn mmio_write16(mmio: *mut mmio_t, offset: usize, value: u16) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn mmio_write8(mmio: *mut mmio_t, offset: usize, value: u8) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn mmio_write(
            mmio: *mut mmio_t,
            offset: usize,
            buf: *const u8,
            len: usize,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn mmio_close(mmio: *mut mmio_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn mmio_free(mmio: *mut mmio_t);
    }
    extern "C" {
        pub fn mmio_base(mmio: *mut mmio_t) -> usize;
    }
    extern "C" {
        pub fn mmio_size(mmio: *mut mmio_t) -> usize;
    }
    extern "C" {
        pub fn mmio_tostring(
            mmio: *mut mmio_t,
            str: *mut ::std::os::raw::c_char,
            len: usize,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn mmio_errno(mmio: *mut mmio_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn mmio_errmsg(mmio: *mut mmio_t) -> *const ::std::os::raw::c_char;
    }

    pub const serial_error_code_SERIAL_ERROR_ARG: serial_error_code = -1;
    pub const serial_error_code_SERIAL_ERROR_OPEN: serial_error_code = -2;
    pub const serial_error_code_SERIAL_ERROR_QUERY: serial_error_code = -3;
    pub const serial_error_code_SERIAL_ERROR_CONFIGURE: serial_error_code = -4;
    pub const serial_error_code_SERIAL_ERROR_IO: serial_error_code = -5;
    pub const serial_error_code_SERIAL_ERROR_CLOSE: serial_error_code = -6;
    pub type serial_error_code = i32;
    pub const serial_parity_PARITY_NONE: serial_parity = 0;
    pub const serial_parity_PARITY_ODD: serial_parity = 1;
    pub const serial_parity_PARITY_EVEN: serial_parity = 2;
    pub type serial_parity = u32;
    pub use self::serial_parity as serial_parity_t;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct serial_handle {
        _unused: [u8; 0],
    }
    pub type serial_t = serial_handle;
    extern "C" {
        pub fn serial_new() -> *mut serial_t;
    }
    extern "C" {
        pub fn serial_open(
            serial: *mut serial_t,
            path: *const ::std::os::raw::c_char,
            baudrate: u32,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn serial_open_advanced(
            serial: *mut serial_t,
            path: *const ::std::os::raw::c_char,
            baudrate: u32,
            databits: ::std::os::raw::c_uint,
            parity: serial_parity_t,
            stopbits: ::std::os::raw::c_uint,
            xonxoff: bool,
            rtscts: bool,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn serial_read(
            serial: *mut serial_t,
            buf: *mut u8,
            len: usize,
            timeout_ms: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn serial_write(
            serial: *mut serial_t,
            buf: *const u8,
            len: usize,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn serial_flush(serial: *mut serial_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn serial_input_waiting(
            serial: *mut serial_t,
            count: *mut ::std::os::raw::c_uint,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn serial_output_waiting(
            serial: *mut serial_t,
            count: *mut ::std::os::raw::c_uint,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn serial_poll(
            serial: *mut serial_t,
            timeout_ms: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn serial_close(serial: *mut serial_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn serial_free(serial: *mut serial_t);
    }
    extern "C" {
        pub fn serial_get_baudrate(
            serial: *mut serial_t,
            baudrate: *mut u32,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn serial_get_databits(
            serial: *mut serial_t,
            databits: *mut ::std::os::raw::c_uint,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn serial_get_parity(
            serial: *mut serial_t,
            parity: *mut serial_parity_t,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn serial_get_stopbits(
            serial: *mut serial_t,
            stopbits: *mut ::std::os::raw::c_uint,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn serial_get_xonxoff(
            serial: *mut serial_t,
            xonxoff: *mut bool,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn serial_get_rtscts(serial: *mut serial_t, rtscts: *mut bool)
            -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn serial_set_baudrate(serial: *mut serial_t, baudrate: u32) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn serial_set_databits(
            serial: *mut serial_t,
            databits: ::std::os::raw::c_uint,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn serial_set_parity(
            serial: *mut serial_t,
            parity: serial_parity,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn serial_set_stopbits(
            serial: *mut serial_t,
            stopbits: ::std::os::raw::c_uint,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn serial_set_xonxoff(serial: *mut serial_t, enabled: bool) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn serial_set_rtscts(serial: *mut serial_t, enabled: bool) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn serial_fd(serial: *mut serial_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn serial_tostring(
            serial: *mut serial_t,
            str: *mut ::std::os::raw::c_char,
            len: usize,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn serial_errno(serial: *mut serial_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn serial_errmsg(serial: *mut serial_t) -> *const ::std::os::raw::c_char;
    }

    pub const spi_error_code_SPI_ERROR_ARG: spi_error_code = -1;
    pub const spi_error_code_SPI_ERROR_OPEN: spi_error_code = -2;
    pub const spi_error_code_SPI_ERROR_QUERY: spi_error_code = -3;
    pub const spi_error_code_SPI_ERROR_CONFIGURE: spi_error_code = -4;
    pub const spi_error_code_SPI_ERROR_TRANSFER: spi_error_code = -5;
    pub const spi_error_code_SPI_ERROR_CLOSE: spi_error_code = -6;
    pub type spi_error_code = i32;
    pub const spi_bit_order_MSB_FIRST: spi_bit_order = 0;
    pub const spi_bit_order_LSB_FIRST: spi_bit_order = 1;
    pub type spi_bit_order = u32;
    pub use self::spi_bit_order as spi_bit_order_t;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct spi_handle {
        _unused: [u8; 0],
    }
    pub type spi_t = spi_handle;
    extern "C" {
        pub fn spi_new() -> *mut spi_t;
    }
    extern "C" {
        pub fn spi_open(
            spi: *mut spi_t,
            path: *const ::std::os::raw::c_char,
            mode: ::std::os::raw::c_uint,
            max_speed: u32,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn spi_open_advanced(
            spi: *mut spi_t,
            path: *const ::std::os::raw::c_char,
            mode: ::std::os::raw::c_uint,
            max_speed: u32,
            bit_order: spi_bit_order_t,
            bits_per_word: u8,
            extra_flags: u8,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn spi_transfer(
            spi: *mut spi_t,
            txbuf: *const u8,
            rxbuf: *mut u8,
            len: usize,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn spi_close(spi: *mut spi_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn spi_free(spi: *mut spi_t);
    }
    extern "C" {
        pub fn spi_get_mode(
            spi: *mut spi_t,
            mode: *mut ::std::os::raw::c_uint,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn spi_get_max_speed(spi: *mut spi_t, max_speed: *mut u32) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn spi_get_bit_order(
            spi: *mut spi_t,
            bit_order: *mut spi_bit_order_t,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn spi_get_bits_per_word(
            spi: *mut spi_t,
            bits_per_word: *mut u8,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn spi_get_extra_flags(spi: *mut spi_t, extra_flags: *mut u8) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn spi_set_mode(spi: *mut spi_t, mode: ::std::os::raw::c_uint)
            -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn spi_set_max_speed(spi: *mut spi_t, max_speed: u32) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn spi_set_bit_order(
            spi: *mut spi_t,
            bit_order: spi_bit_order_t,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn spi_set_bits_per_word(spi: *mut spi_t, bits_per_word: u8) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn spi_set_extra_flags(spi: *mut spi_t, extra_flags: u8) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn spi_fd(spi: *mut spi_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn spi_tostring(
            spi: *mut spi_t,
            str: *mut ::std::os::raw::c_char,
            len: usize,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn spi_errno(spi: *mut spi_t) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn spi_errmsg(spi: *mut spi_t) -> *const ::std::os::raw::c_char;
    }

    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct periphery_version {
        pub major: ::std::os::raw::c_uint,
        pub minor: ::std::os::raw::c_uint,
        pub patch: ::std::os::raw::c_uint,
        pub commit_id: *const ::std::os::raw::c_char,
    }
    pub type periphery_version_t = periphery_version;
    extern "C" {
        pub fn periphery_version() -> *const ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn periphery_version_info() -> *const periphery_version_t;
    }
}
#[cfg(not(target_os = "linux"))]
pub use not_linux::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn function_available() {
        unsafe {
            spi_new();
        }
    }
}
