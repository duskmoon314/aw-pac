#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Spinlock System Status Register"]
    pub spinlock_systatus_reg: crate::Reg<spinlock_systatus_reg::SPINLOCK_SYSTATUS_REG_SPEC>,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - Spinlock Status Register"]
    pub spinlock_status_reg: crate::Reg<spinlock_status_reg::SPINLOCK_STATUS_REG_SPEC>,
    _reserved2: [u8; 0x0c],
    #[doc = "0x20 - Spinlock Interrupt Enable Register"]
    pub spinlock_irq_en_reg: crate::Reg<spinlock_irq_en_reg::SPINLOCK_IRQ_EN_REG_SPEC>,
    _reserved3: [u8; 0x1c],
    #[doc = "0x40 - Spinlock Interrupt Status Register"]
    pub spinlock_irq_sta_reg: crate::Reg<spinlock_irq_sta_reg::SPINLOCK_IRQ_STA_REG_SPEC>,
    _reserved4: [u8; 0x3c],
    #[doc = "0x80 - Spinlock Lockid0 Register"]
    pub spinlock_lockid0_reg: crate::Reg<spinlock_lockid0_reg::SPINLOCK_LOCKID0_REG_SPEC>,
    #[doc = "0x84 - Spinlock Lockid1 Register"]
    pub spinlock_lockid1_reg: crate::Reg<spinlock_lockid1_reg::SPINLOCK_LOCKID1_REG_SPEC>,
    #[doc = "0x88 - Spinlock Lockid2 Register"]
    pub spinlock_lockid2_reg: crate::Reg<spinlock_lockid2_reg::SPINLOCK_LOCKID2_REG_SPEC>,
    #[doc = "0x8c - Spinlock Lockid3 Register"]
    pub spinlock_lockid3_reg: crate::Reg<spinlock_lockid3_reg::SPINLOCK_LOCKID3_REG_SPEC>,
    #[doc = "0x90 - Spinlock Lockid4 Register"]
    pub spinlock_lockid4_reg: crate::Reg<spinlock_lockid4_reg::SPINLOCK_LOCKID4_REG_SPEC>,
    _reserved9: [u8; 0x6c],
    #[doc = "0x100..0x180 - Spinlock Register"]
    pub spinlock_lock_reg: [crate::Reg<spinlock_lock_reg::SPINLOCK_LOCK_REG_SPEC>; 32],
}
#[doc = "SPINLOCK_SYSTATUS_REG register accessor: an alias for `Reg<SPINLOCK_SYSTATUS_REG_SPEC>`"]
pub type SPINLOCK_SYSTATUS_REG = crate::Reg<spinlock_systatus_reg::SPINLOCK_SYSTATUS_REG_SPEC>;
#[doc = "Spinlock System Status Register"]
pub mod spinlock_systatus_reg;
#[doc = "SPINLOCK_STATUS_REG register accessor: an alias for `Reg<SPINLOCK_STATUS_REG_SPEC>`"]
pub type SPINLOCK_STATUS_REG = crate::Reg<spinlock_status_reg::SPINLOCK_STATUS_REG_SPEC>;
#[doc = "Spinlock Status Register"]
pub mod spinlock_status_reg;
#[doc = "SPINLOCK_IRQ_EN_REG register accessor: an alias for `Reg<SPINLOCK_IRQ_EN_REG_SPEC>`"]
pub type SPINLOCK_IRQ_EN_REG = crate::Reg<spinlock_irq_en_reg::SPINLOCK_IRQ_EN_REG_SPEC>;
#[doc = "Spinlock Interrupt Enable Register"]
pub mod spinlock_irq_en_reg;
#[doc = "SPINLOCK_IRQ_STA_REG register accessor: an alias for `Reg<SPINLOCK_IRQ_STA_REG_SPEC>`"]
pub type SPINLOCK_IRQ_STA_REG = crate::Reg<spinlock_irq_sta_reg::SPINLOCK_IRQ_STA_REG_SPEC>;
#[doc = "Spinlock Interrupt Status Register"]
pub mod spinlock_irq_sta_reg;
#[doc = "SPINLOCK_LOCKID0_REG register accessor: an alias for `Reg<SPINLOCK_LOCKID0_REG_SPEC>`"]
pub type SPINLOCK_LOCKID0_REG = crate::Reg<spinlock_lockid0_reg::SPINLOCK_LOCKID0_REG_SPEC>;
#[doc = "Spinlock Lockid0 Register"]
pub mod spinlock_lockid0_reg;
#[doc = "SPINLOCK_LOCKID1_REG register accessor: an alias for `Reg<SPINLOCK_LOCKID1_REG_SPEC>`"]
pub type SPINLOCK_LOCKID1_REG = crate::Reg<spinlock_lockid1_reg::SPINLOCK_LOCKID1_REG_SPEC>;
#[doc = "Spinlock Lockid1 Register"]
pub mod spinlock_lockid1_reg;
#[doc = "SPINLOCK_LOCKID2_REG register accessor: an alias for `Reg<SPINLOCK_LOCKID2_REG_SPEC>`"]
pub type SPINLOCK_LOCKID2_REG = crate::Reg<spinlock_lockid2_reg::SPINLOCK_LOCKID2_REG_SPEC>;
#[doc = "Spinlock Lockid2 Register"]
pub mod spinlock_lockid2_reg;
#[doc = "SPINLOCK_LOCKID3_REG register accessor: an alias for `Reg<SPINLOCK_LOCKID3_REG_SPEC>`"]
pub type SPINLOCK_LOCKID3_REG = crate::Reg<spinlock_lockid3_reg::SPINLOCK_LOCKID3_REG_SPEC>;
#[doc = "Spinlock Lockid3 Register"]
pub mod spinlock_lockid3_reg;
#[doc = "SPINLOCK_LOCKID4_REG register accessor: an alias for `Reg<SPINLOCK_LOCKID4_REG_SPEC>`"]
pub type SPINLOCK_LOCKID4_REG = crate::Reg<spinlock_lockid4_reg::SPINLOCK_LOCKID4_REG_SPEC>;
#[doc = "Spinlock Lockid4 Register"]
pub mod spinlock_lockid4_reg;
#[doc = "SPINLOCK_LOCK_REG register accessor: an alias for `Reg<SPINLOCK_LOCK_REG_SPEC>`"]
pub type SPINLOCK_LOCK_REG = crate::Reg<spinlock_lock_reg::SPINLOCK_LOCK_REG_SPEC>;
#[doc = "Spinlock Register"]
pub mod spinlock_lock_reg;
