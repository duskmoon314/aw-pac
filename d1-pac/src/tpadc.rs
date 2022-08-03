#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TP Control Register 0"]
    pub tp_ctrl0: TP_CTRL0,
    #[doc = "0x04 - TP Control Register 1"]
    pub tp_ctrl1: TP_CTRL1,
    #[doc = "0x08 - TP Control Register 2"]
    pub tp_ctrl2: TP_CTRL2,
    #[doc = "0x0c - TP Control Register 3"]
    pub tp_ctrl3: TP_CTRL3,
    #[doc = "0x10 - TP Interrupt FIFO Control Register"]
    pub tp_int_fifo_ctrl: TP_INT_FIFO_CTRL,
    #[doc = "0x14 - TP Interrupt FIFO Status Register"]
    pub tp_int_fifo_stat: TP_INT_FIFO_STAT,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - TP Calibration Data Register"]
    pub tp_cali_data: TP_CALI_DATA,
    _reserved7: [u8; 0x04],
    #[doc = "0x24 - TP Data Register"]
    pub tp_data: TP_DATA,
}
#[doc = "tp_ctrl0 (rw) register accessor: an alias for `Reg<TP_CTRL0_SPEC>`"]
pub type TP_CTRL0 = crate::Reg<tp_ctrl0::TP_CTRL0_SPEC>;
#[doc = "TP Control Register 0"]
pub mod tp_ctrl0;
#[doc = "tp_ctrl1 (rw) register accessor: an alias for `Reg<TP_CTRL1_SPEC>`"]
pub type TP_CTRL1 = crate::Reg<tp_ctrl1::TP_CTRL1_SPEC>;
#[doc = "TP Control Register 1"]
pub mod tp_ctrl1;
#[doc = "tp_ctrl2 (rw) register accessor: an alias for `Reg<TP_CTRL2_SPEC>`"]
pub type TP_CTRL2 = crate::Reg<tp_ctrl2::TP_CTRL2_SPEC>;
#[doc = "TP Control Register 2"]
pub mod tp_ctrl2;
#[doc = "tp_ctrl3 (rw) register accessor: an alias for `Reg<TP_CTRL3_SPEC>`"]
pub type TP_CTRL3 = crate::Reg<tp_ctrl3::TP_CTRL3_SPEC>;
#[doc = "TP Control Register 3"]
pub mod tp_ctrl3;
#[doc = "tp_int_fifo_ctrl (rw) register accessor: an alias for `Reg<TP_INT_FIFO_CTRL_SPEC>`"]
pub type TP_INT_FIFO_CTRL = crate::Reg<tp_int_fifo_ctrl::TP_INT_FIFO_CTRL_SPEC>;
#[doc = "TP Interrupt FIFO Control Register"]
pub mod tp_int_fifo_ctrl;
#[doc = "tp_int_fifo_stat (rw) register accessor: an alias for `Reg<TP_INT_FIFO_STAT_SPEC>`"]
pub type TP_INT_FIFO_STAT = crate::Reg<tp_int_fifo_stat::TP_INT_FIFO_STAT_SPEC>;
#[doc = "TP Interrupt FIFO Status Register"]
pub mod tp_int_fifo_stat;
#[doc = "tp_cali_data (rw) register accessor: an alias for `Reg<TP_CALI_DATA_SPEC>`"]
pub type TP_CALI_DATA = crate::Reg<tp_cali_data::TP_CALI_DATA_SPEC>;
#[doc = "TP Calibration Data Register"]
pub mod tp_cali_data;
#[doc = "tp_data (r) register accessor: an alias for `Reg<TP_DATA_SPEC>`"]
pub type TP_DATA = crate::Reg<tp_data::TP_DATA_SPEC>;
#[doc = "TP Data Register"]
pub mod tp_data;
