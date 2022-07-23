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
    #[doc = "0x80..0x94 - Spinlock Lockid Register"]
    pub spinlock_lockid_reg: [crate::Reg<spinlock_lockid_reg::SPINLOCK_LOCKID_REG_SPEC>; 5],
    _reserved5: [u8; 0x6c],
    #[doc = "0x100..0x180 - Spinlock Register"]
    pub spinlock_lock_reg: [crate::Reg<spinlock_lock_reg::SPINLOCK_LOCK_REG_SPEC>; 32],
}
#[doc = "spinlock_systatus_reg register accessor: an alias for `Reg<SPINLOCK_SYSTATUS_REG_SPEC>`"]
pub type SPINLOCK_SYSTATUS_REG = crate::Reg<spinlock_systatus_reg::SPINLOCK_SYSTATUS_REG_SPEC>;
#[doc = "Spinlock System Status Register"]
pub mod spinlock_systatus_reg;
#[doc = "spinlock_status_reg register accessor: an alias for `Reg<SPINLOCK_STATUS_REG_SPEC>`"]
pub type SPINLOCK_STATUS_REG = crate::Reg<spinlock_status_reg::SPINLOCK_STATUS_REG_SPEC>;
#[doc = "Spinlock Status Register"]
pub mod spinlock_status_reg;
#[doc = "spinlock_irq_en_reg register accessor: an alias for `Reg<SPINLOCK_IRQ_EN_REG_SPEC>`"]
pub type SPINLOCK_IRQ_EN_REG = crate::Reg<spinlock_irq_en_reg::SPINLOCK_IRQ_EN_REG_SPEC>;
#[doc = "Spinlock Interrupt Enable Register"]
pub mod spinlock_irq_en_reg;
#[doc = "spinlock_irq_sta_reg register accessor: an alias for `Reg<SPINLOCK_IRQ_STA_REG_SPEC>`"]
pub type SPINLOCK_IRQ_STA_REG = crate::Reg<spinlock_irq_sta_reg::SPINLOCK_IRQ_STA_REG_SPEC>;
#[doc = "Spinlock Interrupt Status Register"]
pub mod spinlock_irq_sta_reg;
#[doc = "spinlock_lockid_reg register accessor: an alias for `Reg<SPINLOCK_LOCKID_REG_SPEC>`"]
pub type SPINLOCK_LOCKID_REG = crate::Reg<spinlock_lockid_reg::SPINLOCK_LOCKID_REG_SPEC>;
#[doc = "Spinlock Lockid Register"]
pub mod spinlock_lockid_reg;
#[doc = "spinlock_lock_reg register accessor: an alias for `Reg<SPINLOCK_LOCK_REG_SPEC>`"]
pub type SPINLOCK_LOCK_REG = crate::Reg<spinlock_lock_reg::SPINLOCK_LOCK_REG_SPEC>;
#[doc = "Spinlock Register"]
pub mod spinlock_lock_reg;
