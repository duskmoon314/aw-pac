#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    spinlock_systatus: SPINLOCK_SYSTATUS,
    _reserved1: [u8; 0x0c],
    spinlock_status: SPINLOCK_STATUS,
    _reserved2: [u8; 0x0c],
    spinlock_irq_en: SPINLOCK_IRQ_EN,
    _reserved3: [u8; 0x1c],
    spinlock_irq_sta: SPINLOCK_IRQ_STA,
    _reserved4: [u8; 0x3c],
    spinlock_lockid: [SPINLOCK_LOCKID; 5],
    _reserved5: [u8; 0x6c],
    spinlock_lock: [SPINLOCK_LOCK; 32],
}
impl RegisterBlock {
    #[doc = "0x00 - Spinlock System Status Register"]
    #[inline(always)]
    pub const fn spinlock_systatus(&self) -> &SPINLOCK_SYSTATUS {
        &self.spinlock_systatus
    }
    #[doc = "0x10 - Spinlock Status Register"]
    #[inline(always)]
    pub const fn spinlock_status(&self) -> &SPINLOCK_STATUS {
        &self.spinlock_status
    }
    #[doc = "0x20 - Spinlock Interrupt Enable Register"]
    #[inline(always)]
    pub const fn spinlock_irq_en(&self) -> &SPINLOCK_IRQ_EN {
        &self.spinlock_irq_en
    }
    #[doc = "0x40 - Spinlock Interrupt Status Register"]
    #[inline(always)]
    pub const fn spinlock_irq_sta(&self) -> &SPINLOCK_IRQ_STA {
        &self.spinlock_irq_sta
    }
    #[doc = "0x80..0x94 - Spinlock Lockid Register"]
    #[inline(always)]
    pub const fn spinlock_lockid(&self, n: usize) -> &SPINLOCK_LOCKID {
        &self.spinlock_lockid[n]
    }
    #[doc = "0x100..0x180 - Spinlock Register"]
    #[inline(always)]
    pub const fn spinlock_lock(&self, n: usize) -> &SPINLOCK_LOCK {
        &self.spinlock_lock[n]
    }
}
#[doc = "spinlock_systatus (r) register accessor: Spinlock System Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spinlock_systatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spinlock_systatus`] module"]
pub type SPINLOCK_SYSTATUS = crate::Reg<spinlock_systatus::SPINLOCK_SYSTATUS_SPEC>;
#[doc = "Spinlock System Status Register"]
pub mod spinlock_systatus;
#[doc = "spinlock_status (r) register accessor: Spinlock Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spinlock_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spinlock_status`] module"]
pub type SPINLOCK_STATUS = crate::Reg<spinlock_status::SPINLOCK_STATUS_SPEC>;
#[doc = "Spinlock Status Register"]
pub mod spinlock_status;
#[doc = "spinlock_irq_en (rw) register accessor: Spinlock Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spinlock_irq_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spinlock_irq_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spinlock_irq_en`] module"]
pub type SPINLOCK_IRQ_EN = crate::Reg<spinlock_irq_en::SPINLOCK_IRQ_EN_SPEC>;
#[doc = "Spinlock Interrupt Enable Register"]
pub mod spinlock_irq_en;
#[doc = "spinlock_irq_sta (rw) register accessor: Spinlock Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spinlock_irq_sta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spinlock_irq_sta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spinlock_irq_sta`] module"]
pub type SPINLOCK_IRQ_STA = crate::Reg<spinlock_irq_sta::SPINLOCK_IRQ_STA_SPEC>;
#[doc = "Spinlock Interrupt Status Register"]
pub mod spinlock_irq_sta;
#[doc = "spinlock_lockid (r) register accessor: Spinlock Lockid Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spinlock_lockid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spinlock_lockid`] module"]
pub type SPINLOCK_LOCKID = crate::Reg<spinlock_lockid::SPINLOCK_LOCKID_SPEC>;
#[doc = "Spinlock Lockid Register"]
pub mod spinlock_lockid;
#[doc = "spinlock_lock (rw) register accessor: Spinlock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spinlock_lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spinlock_lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spinlock_lock`] module"]
pub type SPINLOCK_LOCK = crate::Reg<spinlock_lock::SPINLOCK_LOCK_SPEC>;
#[doc = "Spinlock Register"]
pub mod spinlock_lock;
