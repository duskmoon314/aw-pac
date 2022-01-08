#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TP Control Register 0"]
    pub tp_ctrl_reg0: crate::Reg<tp_ctrl_reg0::TP_CTRL_REG0_SPEC>,
    #[doc = "0x04 - TP Control Register 1"]
    pub tp_ctrl_reg1: crate::Reg<tp_ctrl_reg1::TP_CTRL_REG1_SPEC>,
    #[doc = "0x08 - TP Control Register 2"]
    pub tp_ctrl_reg2: crate::Reg<tp_ctrl_reg2::TP_CTRL_REG2_SPEC>,
    #[doc = "0x0c - TP Control Register 3"]
    pub tp_ctrl_reg3: crate::Reg<tp_ctrl_reg3::TP_CTRL_REG3_SPEC>,
    #[doc = "0x10 - TP Interrupt FIFO Control Register"]
    pub tp_int_fifo_ctrl_reg: crate::Reg<tp_int_fifo_ctrl_reg::TP_INT_FIFO_CTRL_REG_SPEC>,
    #[doc = "0x14 - TP Interrupt FIFO Status Register"]
    pub tp_int_fifo_stat_reg: crate::Reg<tp_int_fifo_stat_reg::TP_INT_FIFO_STAT_REG_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - TP Calibration Data Register"]
    pub tp_cali_data_reg: crate::Reg<tp_cali_data_reg::TP_CALI_DATA_REG_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x24 - TP Data Register"]
    pub tp_data_reg: crate::Reg<tp_data_reg::TP_DATA_REG_SPEC>,
}
#[doc = "TP_CTRL_REG0 register accessor: an alias for `Reg<TP_CTRL_REG0_SPEC>`"]
pub type TP_CTRL_REG0 = crate::Reg<tp_ctrl_reg0::TP_CTRL_REG0_SPEC>;
#[doc = "TP Control Register 0"]
pub mod tp_ctrl_reg0;
#[doc = "TP_CTRL_REG1 register accessor: an alias for `Reg<TP_CTRL_REG1_SPEC>`"]
pub type TP_CTRL_REG1 = crate::Reg<tp_ctrl_reg1::TP_CTRL_REG1_SPEC>;
#[doc = "TP Control Register 1"]
pub mod tp_ctrl_reg1;
#[doc = "TP_CTRL_REG2 register accessor: an alias for `Reg<TP_CTRL_REG2_SPEC>`"]
pub type TP_CTRL_REG2 = crate::Reg<tp_ctrl_reg2::TP_CTRL_REG2_SPEC>;
#[doc = "TP Control Register 2"]
pub mod tp_ctrl_reg2;
#[doc = "TP_CTRL_REG3 register accessor: an alias for `Reg<TP_CTRL_REG3_SPEC>`"]
pub type TP_CTRL_REG3 = crate::Reg<tp_ctrl_reg3::TP_CTRL_REG3_SPEC>;
#[doc = "TP Control Register 3"]
pub mod tp_ctrl_reg3;
#[doc = "TP_INT_FIFO_CTRL_REG register accessor: an alias for `Reg<TP_INT_FIFO_CTRL_REG_SPEC>`"]
pub type TP_INT_FIFO_CTRL_REG = crate::Reg<tp_int_fifo_ctrl_reg::TP_INT_FIFO_CTRL_REG_SPEC>;
#[doc = "TP Interrupt FIFO Control Register"]
pub mod tp_int_fifo_ctrl_reg;
#[doc = "TP_INT_FIFO_STAT_REG register accessor: an alias for `Reg<TP_INT_FIFO_STAT_REG_SPEC>`"]
pub type TP_INT_FIFO_STAT_REG = crate::Reg<tp_int_fifo_stat_reg::TP_INT_FIFO_STAT_REG_SPEC>;
#[doc = "TP Interrupt FIFO Status Register"]
pub mod tp_int_fifo_stat_reg;
#[doc = "TP_CALI_DATA_REG register accessor: an alias for `Reg<TP_CALI_DATA_REG_SPEC>`"]
pub type TP_CALI_DATA_REG = crate::Reg<tp_cali_data_reg::TP_CALI_DATA_REG_SPEC>;
#[doc = "TP Calibration Data Register"]
pub mod tp_cali_data_reg;
#[doc = "TP_DATA_REG register accessor: an alias for `Reg<TP_DATA_REG_SPEC>`"]
pub type TP_DATA_REG = crate::Reg<tp_data_reg::TP_DATA_REG_SPEC>;
#[doc = "TP Data Register"]
pub mod tp_data_reg;
