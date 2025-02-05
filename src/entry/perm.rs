use bit_field::BitField as _;
use core::ops::BitOr;

const PTE_BIT_VALID: usize = 0;
const PTE_BIT_READ: usize = 1;
const PTE_BIT_WRITE: usize = 2;
const PTE_BIT_EXECUTE: usize = 3;
const PTE_BIT_USER: usize = 4;

#[derive(Debug, Copy, Clone)]
pub struct PTEPermission(pub u8);

#[expect(clippy::derivable_impls, reason = "expliciteness")]
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

    // TODO : set_global (maybe set_dirty and set_access)
}

impl BitOr for PTEPermission {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
