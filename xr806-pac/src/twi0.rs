#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TWI Slave Address Register"]
    pub twi_addr: crate::Reg<twi_addr::TWI_ADDR_SPEC>,
    #[doc = "0x04 - TWI Extended Slave Address Register"]
    pub twi_xaddr: crate::Reg<twi_xaddr::TWI_XADDR_SPEC>,
    #[doc = "0x08 - TWI Data Byte Register"]
    pub twi_data: crate::Reg<twi_data::TWI_DATA_SPEC>,
    #[doc = "0x0c - TWI Control Register"]
    pub twi_cntr: crate::Reg<twi_cntr::TWI_CNTR_SPEC>,
    #[doc = "0x10 - TWI Status Register"]
    pub twi_stat: crate::Reg<twi_stat::TWI_STAT_SPEC>,
    #[doc = "0x14 - TWI Clock Control Register"]
    pub twi_ccr: crate::Reg<twi_ccr::TWI_CCR_SPEC>,
    #[doc = "0x18 - TWI Software Reset Register"]
    pub twi_srst: crate::Reg<twi_srst::TWI_SRST_SPEC>,
    #[doc = "0x1c - TWI Enhance Feature Register"]
    pub twi_efr: crate::Reg<twi_efr::TWI_EFR_SPEC>,
    #[doc = "0x20 - TWI Line Control Register"]
    pub twi_lcr: crate::Reg<twi_lcr::TWI_LCR_SPEC>,
}
#[doc = "TWI_ADDR register accessor: an alias for `Reg<TWI_ADDR_SPEC>`"]
pub type TWI_ADDR = crate::Reg<twi_addr::TWI_ADDR_SPEC>;
#[doc = "TWI Slave Address Register"]
pub mod twi_addr;
#[doc = "TWI_XADDR register accessor: an alias for `Reg<TWI_XADDR_SPEC>`"]
pub type TWI_XADDR = crate::Reg<twi_xaddr::TWI_XADDR_SPEC>;
#[doc = "TWI Extended Slave Address Register"]
pub mod twi_xaddr;
#[doc = "TWI_DATA register accessor: an alias for `Reg<TWI_DATA_SPEC>`"]
pub type TWI_DATA = crate::Reg<twi_data::TWI_DATA_SPEC>;
#[doc = "TWI Data Byte Register"]
pub mod twi_data;
#[doc = "TWI_CNTR register accessor: an alias for `Reg<TWI_CNTR_SPEC>`"]
pub type TWI_CNTR = crate::Reg<twi_cntr::TWI_CNTR_SPEC>;
#[doc = "TWI Control Register"]
pub mod twi_cntr;
#[doc = "TWI_STAT register accessor: an alias for `Reg<TWI_STAT_SPEC>`"]
pub type TWI_STAT = crate::Reg<twi_stat::TWI_STAT_SPEC>;
#[doc = "TWI Status Register"]
pub mod twi_stat;
#[doc = "TWI_CCR register accessor: an alias for `Reg<TWI_CCR_SPEC>`"]
pub type TWI_CCR = crate::Reg<twi_ccr::TWI_CCR_SPEC>;
#[doc = "TWI Clock Control Register"]
pub mod twi_ccr;
#[doc = "TWI_SRST register accessor: an alias for `Reg<TWI_SRST_SPEC>`"]
pub type TWI_SRST = crate::Reg<twi_srst::TWI_SRST_SPEC>;
#[doc = "TWI Software Reset Register"]
pub mod twi_srst;
#[doc = "TWI_EFR register accessor: an alias for `Reg<TWI_EFR_SPEC>`"]
pub type TWI_EFR = crate::Reg<twi_efr::TWI_EFR_SPEC>;
#[doc = "TWI Enhance Feature Register"]
pub mod twi_efr;
#[doc = "TWI_LCR register accessor: an alias for `Reg<TWI_LCR_SPEC>`"]
pub type TWI_LCR = crate::Reg<twi_lcr::TWI_LCR_SPEC>;
#[doc = "TWI Line Control Register"]
pub mod twi_lcr;
