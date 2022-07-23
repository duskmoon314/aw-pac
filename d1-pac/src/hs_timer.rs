#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - HS Timer IRQ Enable Register"]
    pub hs_tmr_irq_en: crate::Reg<hs_tmr_irq_en::HS_TMR_IRQ_EN_SPEC>,
    #[doc = "0x04 - HS Timer Status Register"]
    pub hs_tmr_irq_stas: crate::Reg<hs_tmr_irq_stas::HS_TMR_IRQ_STAS_SPEC>,
    _reserved2: [u8; 0x18],
    #[doc = "0x20 - HS Timer Control Register"]
    pub hs_tmr0_ctrl: crate::Reg<hs_tmr_ctrl::HS_TMR_CTRL_SPEC>,
    #[doc = "0x24 - HS Timer Interval Value Low Register"]
    pub hs_tmr0_intv_lo: crate::Reg<hs_tmr_intv_lo::HS_TMR_INTV_LO_SPEC>,
    #[doc = "0x28 - HS Timer Interval Value High Register"]
    pub hs_tmr0_intv_hi: crate::Reg<hs_tmr_intv_hi::HS_TMR_INTV_HI_SPEC>,
    #[doc = "0x2c - HS Timer Current Value Low Register"]
    pub hs_tmr0_curnt_lo: crate::Reg<hs_tmr_curnt_lo::HS_TMR_CURNT_LO_SPEC>,
    #[doc = "0x30 - HS Timer Current Value High Register"]
    pub hs_tmr0_curnt_hi: crate::Reg<hs_tmr_curnt_hi::HS_TMR_CURNT_HI_SPEC>,
    _reserved7: [u8; 0x0c],
    #[doc = "0x40 - HS Timer Control Register"]
    pub hs_tmr1_ctrl: crate::Reg<hs_tmr_ctrl::HS_TMR_CTRL_SPEC>,
    #[doc = "0x44 - HS Timer Interval Value Low Register"]
    pub hs_tmr1_intv_lo: crate::Reg<hs_tmr_intv_lo::HS_TMR_INTV_LO_SPEC>,
    #[doc = "0x48 - HS Timer Interval Value High Register"]
    pub hs_tmr1_intv_hi: crate::Reg<hs_tmr_intv_hi::HS_TMR_INTV_HI_SPEC>,
    #[doc = "0x4c - HS Timer Current Value Low Register"]
    pub hs_tmr1_curnt_lo: crate::Reg<hs_tmr_curnt_lo::HS_TMR_CURNT_LO_SPEC>,
    #[doc = "0x50 - HS Timer Current Value High Register"]
    pub hs_tmr1_curnt_hi: crate::Reg<hs_tmr_curnt_hi::HS_TMR_CURNT_HI_SPEC>,
}
#[doc = "hs_tmr_irq_en register accessor: an alias for `Reg<HS_TMR_IRQ_EN_SPEC>`"]
pub type HS_TMR_IRQ_EN = crate::Reg<hs_tmr_irq_en::HS_TMR_IRQ_EN_SPEC>;
#[doc = "HS Timer IRQ Enable Register"]
pub mod hs_tmr_irq_en;
#[doc = "hs_tmr_irq_stas register accessor: an alias for `Reg<HS_TMR_IRQ_STAS_SPEC>`"]
pub type HS_TMR_IRQ_STAS = crate::Reg<hs_tmr_irq_stas::HS_TMR_IRQ_STAS_SPEC>;
#[doc = "HS Timer Status Register"]
pub mod hs_tmr_irq_stas;
#[doc = "hs_tmr_ctrl register accessor: an alias for `Reg<HS_TMR_CTRL_SPEC>`"]
pub type HS_TMR_CTRL = crate::Reg<hs_tmr_ctrl::HS_TMR_CTRL_SPEC>;
#[doc = "HS Timer Control Register"]
pub mod hs_tmr_ctrl;
#[doc = "hs_tmr_intv_lo register accessor: an alias for `Reg<HS_TMR_INTV_LO_SPEC>`"]
pub type HS_TMR_INTV_LO = crate::Reg<hs_tmr_intv_lo::HS_TMR_INTV_LO_SPEC>;
#[doc = "HS Timer Interval Value Low Register"]
pub mod hs_tmr_intv_lo;
#[doc = "hs_tmr_intv_hi register accessor: an alias for `Reg<HS_TMR_INTV_HI_SPEC>`"]
pub type HS_TMR_INTV_HI = crate::Reg<hs_tmr_intv_hi::HS_TMR_INTV_HI_SPEC>;
#[doc = "HS Timer Interval Value High Register"]
pub mod hs_tmr_intv_hi;
#[doc = "hs_tmr_curnt_lo register accessor: an alias for `Reg<HS_TMR_CURNT_LO_SPEC>`"]
pub type HS_TMR_CURNT_LO = crate::Reg<hs_tmr_curnt_lo::HS_TMR_CURNT_LO_SPEC>;
#[doc = "HS Timer Current Value Low Register"]
pub mod hs_tmr_curnt_lo;
#[doc = "hs_tmr_curnt_hi register accessor: an alias for `Reg<HS_TMR_CURNT_HI_SPEC>`"]
pub type HS_TMR_CURNT_HI = crate::Reg<hs_tmr_curnt_hi::HS_TMR_CURNT_HI_SPEC>;
#[doc = "HS Timer Current Value High Register"]
pub mod hs_tmr_curnt_hi;
