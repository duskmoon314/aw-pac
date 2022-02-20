#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - LRADC Control Register"]
    pub lradc_ctrl: crate::Reg<lradc_ctrl::LRADC_CTRL_SPEC>,
    #[doc = "0x04 - LRADC Interrupt Control Register"]
    pub lradc_intc: crate::Reg<lradc_intc::LRADC_INTC_SPEC>,
    #[doc = "0x08 - LRADC Interrupt Status Register"]
    pub lradc_ints: crate::Reg<lradc_ints::LRADC_INTS_SPEC>,
    #[doc = "0x0c - LRADC Data Register"]
    pub lradc_data: crate::Reg<lradc_data::LRADC_DATA_SPEC>,
}
#[doc = "LRADC_CTRL register accessor: an alias for `Reg<LRADC_CTRL_SPEC>`"]
pub type LRADC_CTRL = crate::Reg<lradc_ctrl::LRADC_CTRL_SPEC>;
#[doc = "LRADC Control Register"]
pub mod lradc_ctrl;
#[doc = "LRADC_INTC register accessor: an alias for `Reg<LRADC_INTC_SPEC>`"]
pub type LRADC_INTC = crate::Reg<lradc_intc::LRADC_INTC_SPEC>;
#[doc = "LRADC Interrupt Control Register"]
pub mod lradc_intc;
#[doc = "LRADC_INTS register accessor: an alias for `Reg<LRADC_INTS_SPEC>`"]
pub type LRADC_INTS = crate::Reg<lradc_ints::LRADC_INTS_SPEC>;
#[doc = "LRADC Interrupt Status Register"]
pub mod lradc_ints;
#[doc = "LRADC_DATA register accessor: an alias for `Reg<LRADC_DATA_SPEC>`"]
pub type LRADC_DATA = crate::Reg<lradc_data::LRADC_DATA_SPEC>;
#[doc = "LRADC Data Register"]
pub mod lradc_data;
