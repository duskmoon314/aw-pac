#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TP Control Register 0"]
    pub tp_ctrl0: crate::Reg<tp_ctrl0::TP_CTRL0_SPEC>,
    #[doc = "0x04 - TP Control Register 1"]
    pub tp_ctrl1: crate::Reg<tp_ctrl1::TP_CTRL1_SPEC>,
    #[doc = "0x08 - TP Control Register 2"]
    pub tp_ctrl2: crate::Reg<tp_ctrl2::TP_CTRL2_SPEC>,
    #[doc = "0x0c - TP Control Register 3"]
    pub tp_ctrl3: crate::Reg<tp_ctrl3::TP_CTRL3_SPEC>,
    #[doc = "0x10 - TP Interrupt FIFO Control Register"]
    pub tp_int_fifo_ctrl: crate::Reg<tp_int_fifo_ctrl::TP_INT_FIFO_CTRL_SPEC>,
    #[doc = "0x14 - TP Interrupt FIFO Status Register"]
    pub tp_int_fifo_stat: crate::Reg<tp_int_fifo_stat::TP_INT_FIFO_STAT_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - TP Calibration Data Register"]
    pub tp_cali_data: crate::Reg<tp_cali_data::TP_CALI_DATA_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x24 - TP Data Register"]
    pub tp_data: crate::Reg<tp_data::TP_DATA_SPEC>,
}
#[doc = "TP_CTRL0 register accessor: an alias for `Reg<TP_CTRL0_SPEC>`"]
pub type TP_CTRL0 = crate::Reg<tp_ctrl0::TP_CTRL0_SPEC>;
#[doc = "TP Control Register 0"]
pub mod tp_ctrl0;
#[doc = "TP_CTRL1 register accessor: an alias for `Reg<TP_CTRL1_SPEC>`"]
pub type TP_CTRL1 = crate::Reg<tp_ctrl1::TP_CTRL1_SPEC>;
#[doc = "TP Control Register 1"]
pub mod tp_ctrl1;
#[doc = "TP_CTRL2 register accessor: an alias for `Reg<TP_CTRL2_SPEC>`"]
pub type TP_CTRL2 = crate::Reg<tp_ctrl2::TP_CTRL2_SPEC>;
#[doc = "TP Control Register 2"]
pub mod tp_ctrl2;
#[doc = "TP_CTRL3 register accessor: an alias for `Reg<TP_CTRL3_SPEC>`"]
pub type TP_CTRL3 = crate::Reg<tp_ctrl3::TP_CTRL3_SPEC>;
#[doc = "TP Control Register 3"]
pub mod tp_ctrl3;
#[doc = "TP_INT_FIFO_CTRL register accessor: an alias for `Reg<TP_INT_FIFO_CTRL_SPEC>`"]
pub type TP_INT_FIFO_CTRL = crate::Reg<tp_int_fifo_ctrl::TP_INT_FIFO_CTRL_SPEC>;
#[doc = "TP Interrupt FIFO Control Register"]
pub mod tp_int_fifo_ctrl;
#[doc = "TP_INT_FIFO_STAT register accessor: an alias for `Reg<TP_INT_FIFO_STAT_SPEC>`"]
pub type TP_INT_FIFO_STAT = crate::Reg<tp_int_fifo_stat::TP_INT_FIFO_STAT_SPEC>;
#[doc = "TP Interrupt FIFO Status Register"]
pub mod tp_int_fifo_stat;
#[doc = "TP_CALI_DATA register accessor: an alias for `Reg<TP_CALI_DATA_SPEC>`"]
pub type TP_CALI_DATA = crate::Reg<tp_cali_data::TP_CALI_DATA_SPEC>;
#[doc = "TP Calibration Data Register"]
pub mod tp_cali_data;
#[doc = "TP_DATA register accessor: an alias for `Reg<TP_DATA_SPEC>`"]
pub type TP_DATA = crate::Reg<tp_data::TP_DATA_SPEC>;
#[doc = "TP Data Register"]
pub mod tp_data;
