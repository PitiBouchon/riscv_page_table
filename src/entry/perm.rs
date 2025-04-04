use bit_field::BitField as _;
use core::ops::BitOr;

const PTE_BIT_VALID: usize = 0;
const PTE_BIT_READ: usize = 1;
const PTE_BIT_WRITE: usize = 2;
const PTE_BIT_EXECUTE: usize = 3;
const PTE_BIT_USER: usize = 4;
const PTE_BIT_GLOBAL: usize = 5;
const PTE_BIT_ACCESS: usize = 6;
const PTE_BIT_DIRTY: usize = 7;

#[derive(Debug, Copy, Clone)]
pub struct PTEPermission(pub u8);

impl Default for PTEPermission {
    fn default() -> Self {
        Self(0)
    }
}

impl PTEPermission {
    // See section 4.3.1 Addressing and Memory Protection (RiscV privileged manual)
    #[must_use]
    pub fn valid() -> Self {
        let mut res = 0;
        let _res = res.set_bit(PTE_BIT_VALID, true);
        Self(res)
    }

    #[must_use]
    pub fn read() -> Self {
        let mut res = 0;
        let _res = res.set_bit(PTE_BIT_READ, true);
        Self(res)
    }

    #[must_use]
    pub fn write() -> Self {
        let mut res = 0;
        let _res = res.set_bit(PTE_BIT_WRITE, true);
        Self(res)
    }

    #[must_use]
    pub fn execute() -> Self {
        let mut res = 0;
        let _res = res.set_bit(PTE_BIT_EXECUTE, true);
        Self(res)
    }

    #[must_use]
    pub fn user() -> Self {
        let mut res = 0;
        let _res = res.set_bit(PTE_BIT_USER, true);
        Self(res)
    }

    #[must_use]
    pub fn global() -> Self {
        let mut res = 0;
        let _res = res.set_bit(PTE_BIT_GLOBAL, true);
        Self(res)
    }

    #[must_use]
    pub fn access() -> Self {
        let mut res = 0;
        let _res = res.set_bit(PTE_BIT_ACCESS, true);
        Self(res)
    }

    #[must_use]
    pub fn dirty() -> Self {
        let mut res = 0;
        let _res = res.set_bit(PTE_BIT_DIRTY, true);
        Self(res)
    }
}

impl BitOr for PTEPermission {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
