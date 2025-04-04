use bit_field::BitField as _;

use super::VirtualPageNumber;

#[cfg(target_pointer_width = "32")]
pub struct Sv32;
#[cfg(target_pointer_width = "64")]
pub struct Sv39;
#[cfg(target_pointer_width = "64")]
pub struct Sv48;
#[cfg(target_pointer_width = "64")]
pub struct Sv57;

pub trait AddressingMode {
    const PAGE_COUNT: usize;
    const MAX_ADDR: usize = 1 << (9 * Self::PAGE_COUNT + 12 - 1);
    fn virtual_page_numbers(addr: usize) -> [VirtualPageNumber; Self::PAGE_COUNT];
}

#[cfg(target_pointer_width = "32")]
impl AddressingMode for Sv32 {
    const PAGE_COUNT: usize = 2;

    fn virtual_page_numbers(addr: usize) -> [VirtualPageNumber; Self::PAGE_COUNT] {
        [
            VirtualPageNumber(addr.get_bits(12..=21) as u16),
            VirtualPageNumber(addr.get_bits(22..=31) as u16),
        ]
    }
}

#[cfg(target_pointer_width = "64")]
impl AddressingMode for Sv39 {
    const PAGE_COUNT: usize = 3;

    fn virtual_page_numbers(addr: usize) -> [VirtualPageNumber; Self::PAGE_COUNT] {
        [
            VirtualPageNumber(addr.get_bits(12..=20) as u16),
            VirtualPageNumber(addr.get_bits(21..=29) as u16),
            VirtualPageNumber(addr.get_bits(30..=38) as u16),
        ]
    }
}

#[cfg(target_pointer_width = "64")]
impl AddressingMode for Sv48 {
    const PAGE_COUNT: usize = 4;

    fn virtual_page_numbers(addr: usize) -> [VirtualPageNumber; Self::PAGE_COUNT] {
        [
            VirtualPageNumber(addr.get_bits(12..=20) as u16),
            VirtualPageNumber(addr.get_bits(21..=29) as u16),
            VirtualPageNumber(addr.get_bits(30..=38) as u16),
            VirtualPageNumber(addr.get_bits(39..=47) as u16),
        ]
    }
}

#[cfg(target_pointer_width = "64")]
impl AddressingMode for Sv57 {
    const PAGE_COUNT: usize = 5;

    fn virtual_page_numbers(addr: usize) -> [VirtualPageNumber; Self::PAGE_COUNT] {
        [
            VirtualPageNumber(addr.get_bits(12..=20) as u16),
            VirtualPageNumber(addr.get_bits(21..=29) as u16),
            VirtualPageNumber(addr.get_bits(30..=38) as u16),
            VirtualPageNumber(addr.get_bits(39..=47) as u16),
            VirtualPageNumber(addr.get_bits(48..=56) as u16),
        ]
    }
}
