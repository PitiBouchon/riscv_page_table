pub mod mode;

use core::marker::PhantomData;

use bit_field::BitField as _;
use mode::AddressingMode;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct VirtualAddr<M: AddressingMode>(usize, PhantomData<M>);

#[derive(Debug)]
pub(in super::super) struct VirtualPageNumber(pub(in super::super) u16);

#[derive(Debug)]
pub(in super::super) struct PageOffset(pub(super) u16);

impl<M: AddressingMode> VirtualAddr<M> {
    pub fn new(val: usize) -> Self {
        assert!(val <= M::MAX_ADDR);
        Self(val, PhantomData::default())
    }

    pub fn is_align(&self, align: usize) -> bool {
        self.0 % align == 0
    }

    pub const fn add_offset(self, offset: usize) -> Self {
        assert!(self.0 + offset <= M::MAX_ADDR);
        Self(self.0 + offset, self.1)
    }

    pub const fn sub_offset(self, offset: usize) -> Self {
        assert!(offset <= self.0);
        Self(self.0 - offset, self.1)
    }

    pub fn get(&self) -> &usize {
        &self.0
    }

    // pub fn page_round_down(self) -> Self {
    //     VirtualAddr(page_round_down(self.0), self.1)
    // }

    // pub fn page_round_up(self) -> Self {
    //     VirtualAddr(page_round_up(self.0), self.1)
    // }

    // pub(in super::super) fn virtual_page_numbers(&self) -> [VirtualPageNumber; 3] {
    //     [
    //         VirtualPageNumber(self.0.get_bits(12..21) as u16),
    //         VirtualPageNumber(self.0.get_bits(21..30) as u16),
    //         VirtualPageNumber(self.0.get_bits(30..39) as u16),
    //     ]
    // }

    pub(in super::super) fn page_offset(&self) -> PageOffset {
        PageOffset((self.0.get_bits(0..12)) as u16)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PhysicalAddr<M: AddressingMode>(pub(in super::super) u64);

impl PhysicalAddr {
    pub fn new(val: u64) -> Self {
        const MAX_PHYSICAL_ADDR: u64 = 1 << (26 + 9 + 9 + 12 - 1);

        assert!(val < MAX_PHYSICAL_ADDR);
        Self(val)
    }
}

#[derive(Debug)]
pub struct Ppn(pub(in super::super) u64);

impl Ppn {
    pub fn get(&self) -> u64 {
        self.0
    }
}

impl PhysicalAddr {
    pub fn is_align(&self, align: u64) -> bool {
        self.0 % align == 0
    }

    pub fn page_round_down(self) -> Self {
        PhysicalAddr(page_round_down(self.0))
    }

    pub fn page_round_up(self) -> Self {
        PhysicalAddr(page_round_up(self.0))
    }

    pub(in super::super) fn page_offset(&self) -> PageOffset {
        PageOffset((self.0.get_bits(0..12)) as u16)
    }

    pub fn ppn(&self) -> Ppn {
        Ppn(self.0.get_bits(12..54))
    }
}
