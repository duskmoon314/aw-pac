#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TVD MODULE CONTROL Register"]
    pub tvd_en: crate::Reg<tvd_en::TVD_EN_SPEC>,
    #[doc = "0x04 - TVD MODE CONTROL Register"]
    pub tvd_mode: crate::Reg<tvd_mode::TVD_MODE_SPEC>,
    #[doc = "0x08 - TVD CLAMP And AGC CONTROL Register1"]
    pub tvd_clamp_agc1: crate::Reg<tvd_clamp_agc1::TVD_CLAMP_AGC1_SPEC>,
    #[doc = "0x0c - TVD CLAMP And AGC CONTROL Register2"]
    pub tvd_clamp_agc2: crate::Reg<tvd_clamp_agc2::TVD_CLAMP_AGC2_SPEC>,
    #[doc = "0x10 - TVD HLOCK CONTROL Register1"]
    pub tvd_hlock1: crate::Reg<tvd_hlock1::TVD_HLOCK1_SPEC>,
    #[doc = "0x14 - TVD HLOCK CONTROL Register2"]
    pub tvd_hlock2: crate::Reg<tvd_hlock2::TVD_HLOCK2_SPEC>,
    #[doc = "0x18 - TVD HLOCK CONTROL Register3"]
    pub tvd_hlock3: crate::Reg<tvd_hlock3::TVD_HLOCK3_SPEC>,
    #[doc = "0x1c - TVD HLOCK CONTROL Register4"]
    pub tvd_hlock4: crate::Reg<tvd_hlock4::TVD_HLOCK4_SPEC>,
    #[doc = "0x20 - TVD HLOCK CONTROL Register5"]
    pub tvd_hlock5: crate::Reg<tvd_hlock5::TVD_HLOCK5_SPEC>,
    #[doc = "0x24 - TVD VLOCK CONTROL Register1"]
    pub tvd_vlock1: crate::Reg<tvd_vlock1::TVD_VLOCK1_SPEC>,
    #[doc = "0x28 - TVD VLOCK CONTROL Register2"]
    pub tvd_vlock2: crate::Reg<tvd_vlock2::TVD_VLOCK2_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0x30 - TVD CHROMA LOCK CONTROL Register1"]
    pub tvd_clock1: crate::Reg<tvd_clock1::TVD_CLOCK1_SPEC>,
    #[doc = "0x34 - TVD CHROMA LOCK CONTROL Register2"]
    pub tvd_clock2: crate::Reg<tvd_clock2::TVD_CLOCK2_SPEC>,
    _reserved13: [u8; 0x08],
    #[doc = "0x40 - TVD YC SEPERATION CONROL Register1"]
    pub tvd_yc_sep1: crate::Reg<tvd_yc_sep1::TVD_YC_SEP1_SPEC>,
    #[doc = "0x44 - TVD YC SEPERATION CONROL Register2"]
    pub tvd_yc_sep2: crate::Reg<tvd_yc_sep2::TVD_YC_SEP2_SPEC>,
    _reserved15: [u8; 0x08],
    #[doc = "0x50 - TVD ENHANCEMENT CONTROL Register1"]
    pub tvd_enhance1: crate::Reg<tvd_enhance1::TVD_ENHANCE1_SPEC>,
    #[doc = "0x54 - TVD ENHANCEMENT CONTROL Register2"]
    pub tvd_enhance2: crate::Reg<tvd_enhance2::TVD_ENHANCE2_SPEC>,
    #[doc = "0x58 - TVD ENHANCEMENT CONTROL Register3"]
    pub tvd_enhance3: crate::Reg<tvd_enhance3::TVD_ENHANCE3_SPEC>,
    _reserved18: [u8; 0x04],
    #[doc = "0x60 - TVD WB DMA CONTROL Register1"]
    pub tvd_wb1: crate::Reg<tvd_wb1::TVD_WB1_SPEC>,
    #[doc = "0x64 - TVD WB DMA CONTROL Register2"]
    pub tvd_wb2: crate::Reg<tvd_wb2::TVD_WB2_SPEC>,
    #[doc = "0x68 - TVD WB DMA CONTROL Register3"]
    pub tvd_wb3: crate::Reg<tvd_wb3::TVD_WB3_SPEC>,
    #[doc = "0x6c - TVD WB DMA CONTROL Register4"]
    pub tvd_wb4: crate::Reg<tvd_wb4::TVD_WB4_SPEC>,
    _reserved22: [u8; 0x10],
    #[doc = "0x80 - TVD DMA Interrupt Control Register"]
    pub tvd_irq_ctl: crate::Reg<tvd_irq_ctl::TVD_IRQ_CTL_SPEC>,
    _reserved23: [u8; 0x0c],
    #[doc = "0x90 - TVD DMA Interrupt Status Register"]
    pub tvd_irq_status: crate::Reg<tvd_irq_status::TVD_IRQ_STATUS_SPEC>,
    _reserved24: [u8; 0x6c],
    #[doc = "0x100 - TVD DEBUG CONTROL Register1"]
    pub tvd_debug1: crate::Reg<tvd_debug1::TVD_DEBUG1_SPEC>,
    _reserved25: [u8; 0x7c],
    #[doc = "0x180 - TVD DEBUG STATUS Register1"]
    pub tvd_status1: crate::Reg<tvd_status1::TVD_STATUS1_SPEC>,
    #[doc = "0x184 - TVD DEBUG STATUS Register2"]
    pub tvd_status2: crate::Reg<tvd_status2::TVD_STATUS2_SPEC>,
    #[doc = "0x188 - TVD DEBUG STATUS Register3"]
    pub tvd_status3: crate::Reg<tvd_status3::TVD_STATUS3_SPEC>,
    #[doc = "0x18c - TVD DEBUG STATUS Register4"]
    pub tvd_status4: crate::Reg<tvd_status4::TVD_STATUS4_SPEC>,
    #[doc = "0x190 - TVD DEBUG STATUS Register5"]
    pub tvd_status5: crate::Reg<tvd_status5::TVD_STATUS5_SPEC>,
    #[doc = "0x194 - TVD DEBUG STATUS Register6"]
    pub tvd_status6: crate::Reg<tvd_status6::TVD_STATUS6_SPEC>,
}
#[doc = "TVD_EN register accessor: an alias for `Reg<TVD_EN_SPEC>`"]
pub type TVD_EN = crate::Reg<tvd_en::TVD_EN_SPEC>;
#[doc = "TVD MODULE CONTROL Register"]
pub mod tvd_en;
#[doc = "TVD_MODE register accessor: an alias for `Reg<TVD_MODE_SPEC>`"]
pub type TVD_MODE = crate::Reg<tvd_mode::TVD_MODE_SPEC>;
#[doc = "TVD MODE CONTROL Register"]
pub mod tvd_mode;
#[doc = "TVD_CLAMP_AGC1 register accessor: an alias for `Reg<TVD_CLAMP_AGC1_SPEC>`"]
pub type TVD_CLAMP_AGC1 = crate::Reg<tvd_clamp_agc1::TVD_CLAMP_AGC1_SPEC>;
#[doc = "TVD CLAMP And AGC CONTROL Register1"]
pub mod tvd_clamp_agc1;
#[doc = "TVD_CLAMP_AGC2 register accessor: an alias for `Reg<TVD_CLAMP_AGC2_SPEC>`"]
pub type TVD_CLAMP_AGC2 = crate::Reg<tvd_clamp_agc2::TVD_CLAMP_AGC2_SPEC>;
#[doc = "TVD CLAMP And AGC CONTROL Register2"]
pub mod tvd_clamp_agc2;
#[doc = "TVD_HLOCK1 register accessor: an alias for `Reg<TVD_HLOCK1_SPEC>`"]
pub type TVD_HLOCK1 = crate::Reg<tvd_hlock1::TVD_HLOCK1_SPEC>;
#[doc = "TVD HLOCK CONTROL Register1"]
pub mod tvd_hlock1;
#[doc = "TVD_HLOCK2 register accessor: an alias for `Reg<TVD_HLOCK2_SPEC>`"]
pub type TVD_HLOCK2 = crate::Reg<tvd_hlock2::TVD_HLOCK2_SPEC>;
#[doc = "TVD HLOCK CONTROL Register2"]
pub mod tvd_hlock2;
#[doc = "TVD_HLOCK3 register accessor: an alias for `Reg<TVD_HLOCK3_SPEC>`"]
pub type TVD_HLOCK3 = crate::Reg<tvd_hlock3::TVD_HLOCK3_SPEC>;
#[doc = "TVD HLOCK CONTROL Register3"]
pub mod tvd_hlock3;
#[doc = "TVD_HLOCK4 register accessor: an alias for `Reg<TVD_HLOCK4_SPEC>`"]
pub type TVD_HLOCK4 = crate::Reg<tvd_hlock4::TVD_HLOCK4_SPEC>;
#[doc = "TVD HLOCK CONTROL Register4"]
pub mod tvd_hlock4;
#[doc = "TVD_HLOCK5 register accessor: an alias for `Reg<TVD_HLOCK5_SPEC>`"]
pub type TVD_HLOCK5 = crate::Reg<tvd_hlock5::TVD_HLOCK5_SPEC>;
#[doc = "TVD HLOCK CONTROL Register5"]
pub mod tvd_hlock5;
#[doc = "TVD_VLOCK1 register accessor: an alias for `Reg<TVD_VLOCK1_SPEC>`"]
pub type TVD_VLOCK1 = crate::Reg<tvd_vlock1::TVD_VLOCK1_SPEC>;
#[doc = "TVD VLOCK CONTROL Register1"]
pub mod tvd_vlock1;
#[doc = "TVD_VLOCK2 register accessor: an alias for `Reg<TVD_VLOCK2_SPEC>`"]
pub type TVD_VLOCK2 = crate::Reg<tvd_vlock2::TVD_VLOCK2_SPEC>;
#[doc = "TVD VLOCK CONTROL Register2"]
pub mod tvd_vlock2;
#[doc = "TVD_CLOCK1 register accessor: an alias for `Reg<TVD_CLOCK1_SPEC>`"]
pub type TVD_CLOCK1 = crate::Reg<tvd_clock1::TVD_CLOCK1_SPEC>;
#[doc = "TVD CHROMA LOCK CONTROL Register1"]
pub mod tvd_clock1;
#[doc = "TVD_CLOCK2 register accessor: an alias for `Reg<TVD_CLOCK2_SPEC>`"]
pub type TVD_CLOCK2 = crate::Reg<tvd_clock2::TVD_CLOCK2_SPEC>;
#[doc = "TVD CHROMA LOCK CONTROL Register2"]
pub mod tvd_clock2;
#[doc = "TVD_YC_SEP1 register accessor: an alias for `Reg<TVD_YC_SEP1_SPEC>`"]
pub type TVD_YC_SEP1 = crate::Reg<tvd_yc_sep1::TVD_YC_SEP1_SPEC>;
#[doc = "TVD YC SEPERATION CONROL Register1"]
pub mod tvd_yc_sep1;
#[doc = "TVD_YC_SEP2 register accessor: an alias for `Reg<TVD_YC_SEP2_SPEC>`"]
pub type TVD_YC_SEP2 = crate::Reg<tvd_yc_sep2::TVD_YC_SEP2_SPEC>;
#[doc = "TVD YC SEPERATION CONROL Register2"]
pub mod tvd_yc_sep2;
#[doc = "TVD_ENHANCE1 register accessor: an alias for `Reg<TVD_ENHANCE1_SPEC>`"]
pub type TVD_ENHANCE1 = crate::Reg<tvd_enhance1::TVD_ENHANCE1_SPEC>;
#[doc = "TVD ENHANCEMENT CONTROL Register1"]
pub mod tvd_enhance1;
#[doc = "TVD_ENHANCE2 register accessor: an alias for `Reg<TVD_ENHANCE2_SPEC>`"]
pub type TVD_ENHANCE2 = crate::Reg<tvd_enhance2::TVD_ENHANCE2_SPEC>;
#[doc = "TVD ENHANCEMENT CONTROL Register2"]
pub mod tvd_enhance2;
#[doc = "TVD_ENHANCE3 register accessor: an alias for `Reg<TVD_ENHANCE3_SPEC>`"]
pub type TVD_ENHANCE3 = crate::Reg<tvd_enhance3::TVD_ENHANCE3_SPEC>;
#[doc = "TVD ENHANCEMENT CONTROL Register3"]
pub mod tvd_enhance3;
#[doc = "TVD_WB1 register accessor: an alias for `Reg<TVD_WB1_SPEC>`"]
pub type TVD_WB1 = crate::Reg<tvd_wb1::TVD_WB1_SPEC>;
#[doc = "TVD WB DMA CONTROL Register1"]
pub mod tvd_wb1;
#[doc = "TVD_WB2 register accessor: an alias for `Reg<TVD_WB2_SPEC>`"]
pub type TVD_WB2 = crate::Reg<tvd_wb2::TVD_WB2_SPEC>;
#[doc = "TVD WB DMA CONTROL Register2"]
pub mod tvd_wb2;
#[doc = "TVD_WB3 register accessor: an alias for `Reg<TVD_WB3_SPEC>`"]
pub type TVD_WB3 = crate::Reg<tvd_wb3::TVD_WB3_SPEC>;
#[doc = "TVD WB DMA CONTROL Register3"]
pub mod tvd_wb3;
#[doc = "TVD_WB4 register accessor: an alias for `Reg<TVD_WB4_SPEC>`"]
pub type TVD_WB4 = crate::Reg<tvd_wb4::TVD_WB4_SPEC>;
#[doc = "TVD WB DMA CONTROL Register4"]
pub mod tvd_wb4;
#[doc = "TVD_IRQ_CTL register accessor: an alias for `Reg<TVD_IRQ_CTL_SPEC>`"]
pub type TVD_IRQ_CTL = crate::Reg<tvd_irq_ctl::TVD_IRQ_CTL_SPEC>;
#[doc = "TVD DMA Interrupt Control Register"]
pub mod tvd_irq_ctl;
#[doc = "TVD_IRQ_STATUS register accessor: an alias for `Reg<TVD_IRQ_STATUS_SPEC>`"]
pub type TVD_IRQ_STATUS = crate::Reg<tvd_irq_status::TVD_IRQ_STATUS_SPEC>;
#[doc = "TVD DMA Interrupt Status Register"]
pub mod tvd_irq_status;
#[doc = "TVD_DEBUG1 register accessor: an alias for `Reg<TVD_DEBUG1_SPEC>`"]
pub type TVD_DEBUG1 = crate::Reg<tvd_debug1::TVD_DEBUG1_SPEC>;
#[doc = "TVD DEBUG CONTROL Register1"]
pub mod tvd_debug1;
#[doc = "TVD_STATUS1 register accessor: an alias for `Reg<TVD_STATUS1_SPEC>`"]
pub type TVD_STATUS1 = crate::Reg<tvd_status1::TVD_STATUS1_SPEC>;
#[doc = "TVD DEBUG STATUS Register1"]
pub mod tvd_status1;
#[doc = "TVD_STATUS2 register accessor: an alias for `Reg<TVD_STATUS2_SPEC>`"]
pub type TVD_STATUS2 = crate::Reg<tvd_status2::TVD_STATUS2_SPEC>;
#[doc = "TVD DEBUG STATUS Register2"]
pub mod tvd_status2;
#[doc = "TVD_STATUS3 register accessor: an alias for `Reg<TVD_STATUS3_SPEC>`"]
pub type TVD_STATUS3 = crate::Reg<tvd_status3::TVD_STATUS3_SPEC>;
#[doc = "TVD DEBUG STATUS Register3"]
pub mod tvd_status3;
#[doc = "TVD_STATUS4 register accessor: an alias for `Reg<TVD_STATUS4_SPEC>`"]
pub type TVD_STATUS4 = crate::Reg<tvd_status4::TVD_STATUS4_SPEC>;
#[doc = "TVD DEBUG STATUS Register4"]
pub mod tvd_status4;
#[doc = "TVD_STATUS5 register accessor: an alias for `Reg<TVD_STATUS5_SPEC>`"]
pub type TVD_STATUS5 = crate::Reg<tvd_status5::TVD_STATUS5_SPEC>;
#[doc = "TVD DEBUG STATUS Register5"]
pub mod tvd_status5;
#[doc = "TVD_STATUS6 register accessor: an alias for `Reg<TVD_STATUS6_SPEC>`"]
pub type TVD_STATUS6 = crate::Reg<tvd_status6::TVD_STATUS6_SPEC>;
#[doc = "TVD DEBUG STATUS Register6"]
pub mod tvd_status6;
