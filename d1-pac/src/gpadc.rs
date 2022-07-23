#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPADC Sample Rate Configure Register"]
    pub gp_sr_con: crate::Reg<gp_sr_con::GP_SR_CON_SPEC>,
    #[doc = "0x04 - GPADC Control Register"]
    pub gp_ctrl: crate::Reg<gp_ctrl::GP_CTRL_SPEC>,
    #[doc = "0x08 - GPADC Compare and Select Enable Register"]
    pub gp_cs_en: crate::Reg<gp_cs_en::GP_CS_EN_SPEC>,
    #[doc = "0x0c - GPADC FIFO Interrupt Control Register"]
    pub gp_fifo_intc: crate::Reg<gp_fifo_intc::GP_FIFO_INTC_SPEC>,
    #[doc = "0x10 - GPADC FIFO Interrupt Status Register"]
    pub gp_fifo_ints: crate::Reg<gp_fifo_ints::GP_FIFO_INTS_SPEC>,
    #[doc = "0x14 - GPADC FIFO Data Register"]
    pub gp_fifo_data: crate::Reg<gp_fifo_data::GP_FIFO_DATA_SPEC>,
    #[doc = "0x18 - GPADC Calibration Data Register"]
    pub gp_cdata: crate::Reg<gp_cdata::GP_CDATA_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - GPADC Data Low Interrupt Configure Register"]
    pub gp_datal_intc: crate::Reg<gp_datal_intc::GP_DATAL_INTC_SPEC>,
    #[doc = "0x24 - GPADC Data High Interrupt Configure Register"]
    pub gp_datah_intc: crate::Reg<gp_datah_intc::GP_DATAH_INTC_SPEC>,
    #[doc = "0x28 - GPADC Data Interrupt Configure Register"]
    pub gp_data_intc: crate::Reg<gp_data_intc::GP_DATA_INTC_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x30 - GPADC Data Low Interrupt Status Register"]
    pub gp_datal_ints: crate::Reg<gp_datal_ints::GP_DATAL_INTS_SPEC>,
    #[doc = "0x34 - GPADC Data High Interrupt Status Register"]
    pub gp_datah_ints: crate::Reg<gp_datah_ints::GP_DATAH_INTS_SPEC>,
    #[doc = "0x38 - GPADC Data Interrupt Status Register"]
    pub gp_data_ints: crate::Reg<gp_data_ints::GP_DATA_INTS_SPEC>,
    _reserved13: [u8; 0x04],
    #[doc = "0x40 - GPADC CH0 Compare Data Register"]
    pub gp_ch0_cmp_data: crate::Reg<gp_ch0_cmp_data::GP_CH0_CMP_DATA_SPEC>,
    #[doc = "0x44 - GPADC CH1 Compare Data Register"]
    pub gp_ch1_cmp_data: crate::Reg<gp_ch1_cmp_data::GP_CH1_CMP_DATA_SPEC>,
    _reserved15: [u8; 0x38],
    #[doc = "0x80 - GPADC CH0 Data Register"]
    pub gp_ch0_data: crate::Reg<gp_ch0_data::GP_CH0_DATA_SPEC>,
    #[doc = "0x84 - GPADC CH1 Data Register"]
    pub gp_ch1_data: crate::Reg<gp_ch1_data::GP_CH1_DATA_SPEC>,
}
#[doc = "gp_sr_con register accessor: an alias for `Reg<GP_SR_CON_SPEC>`"]
pub type GP_SR_CON = crate::Reg<gp_sr_con::GP_SR_CON_SPEC>;
#[doc = "GPADC Sample Rate Configure Register"]
pub mod gp_sr_con;
#[doc = "gp_ctrl register accessor: an alias for `Reg<GP_CTRL_SPEC>`"]
pub type GP_CTRL = crate::Reg<gp_ctrl::GP_CTRL_SPEC>;
#[doc = "GPADC Control Register"]
pub mod gp_ctrl;
#[doc = "gp_cs_en register accessor: an alias for `Reg<GP_CS_EN_SPEC>`"]
pub type GP_CS_EN = crate::Reg<gp_cs_en::GP_CS_EN_SPEC>;
#[doc = "GPADC Compare and Select Enable Register"]
pub mod gp_cs_en;
#[doc = "gp_fifo_intc register accessor: an alias for `Reg<GP_FIFO_INTC_SPEC>`"]
pub type GP_FIFO_INTC = crate::Reg<gp_fifo_intc::GP_FIFO_INTC_SPEC>;
#[doc = "GPADC FIFO Interrupt Control Register"]
pub mod gp_fifo_intc;
#[doc = "gp_fifo_ints register accessor: an alias for `Reg<GP_FIFO_INTS_SPEC>`"]
pub type GP_FIFO_INTS = crate::Reg<gp_fifo_ints::GP_FIFO_INTS_SPEC>;
#[doc = "GPADC FIFO Interrupt Status Register"]
pub mod gp_fifo_ints;
#[doc = "gp_fifo_data register accessor: an alias for `Reg<GP_FIFO_DATA_SPEC>`"]
pub type GP_FIFO_DATA = crate::Reg<gp_fifo_data::GP_FIFO_DATA_SPEC>;
#[doc = "GPADC FIFO Data Register"]
pub mod gp_fifo_data;
#[doc = "gp_cdata register accessor: an alias for `Reg<GP_CDATA_SPEC>`"]
pub type GP_CDATA = crate::Reg<gp_cdata::GP_CDATA_SPEC>;
#[doc = "GPADC Calibration Data Register"]
pub mod gp_cdata;
#[doc = "gp_datal_intc register accessor: an alias for `Reg<GP_DATAL_INTC_SPEC>`"]
pub type GP_DATAL_INTC = crate::Reg<gp_datal_intc::GP_DATAL_INTC_SPEC>;
#[doc = "GPADC Data Low Interrupt Configure Register"]
pub mod gp_datal_intc;
#[doc = "gp_datah_intc register accessor: an alias for `Reg<GP_DATAH_INTC_SPEC>`"]
pub type GP_DATAH_INTC = crate::Reg<gp_datah_intc::GP_DATAH_INTC_SPEC>;
#[doc = "GPADC Data High Interrupt Configure Register"]
pub mod gp_datah_intc;
#[doc = "gp_data_intc register accessor: an alias for `Reg<GP_DATA_INTC_SPEC>`"]
pub type GP_DATA_INTC = crate::Reg<gp_data_intc::GP_DATA_INTC_SPEC>;
#[doc = "GPADC Data Interrupt Configure Register"]
pub mod gp_data_intc;
#[doc = "gp_datal_ints register accessor: an alias for `Reg<GP_DATAL_INTS_SPEC>`"]
pub type GP_DATAL_INTS = crate::Reg<gp_datal_ints::GP_DATAL_INTS_SPEC>;
#[doc = "GPADC Data Low Interrupt Status Register"]
pub mod gp_datal_ints;
#[doc = "gp_datah_ints register accessor: an alias for `Reg<GP_DATAH_INTS_SPEC>`"]
pub type GP_DATAH_INTS = crate::Reg<gp_datah_ints::GP_DATAH_INTS_SPEC>;
#[doc = "GPADC Data High Interrupt Status Register"]
pub mod gp_datah_ints;
#[doc = "gp_data_ints register accessor: an alias for `Reg<GP_DATA_INTS_SPEC>`"]
pub type GP_DATA_INTS = crate::Reg<gp_data_ints::GP_DATA_INTS_SPEC>;
#[doc = "GPADC Data Interrupt Status Register"]
pub mod gp_data_ints;
#[doc = "gp_ch0_cmp_data register accessor: an alias for `Reg<GP_CH0_CMP_DATA_SPEC>`"]
pub type GP_CH0_CMP_DATA = crate::Reg<gp_ch0_cmp_data::GP_CH0_CMP_DATA_SPEC>;
#[doc = "GPADC CH0 Compare Data Register"]
pub mod gp_ch0_cmp_data;
#[doc = "gp_ch1_cmp_data register accessor: an alias for `Reg<GP_CH1_CMP_DATA_SPEC>`"]
pub type GP_CH1_CMP_DATA = crate::Reg<gp_ch1_cmp_data::GP_CH1_CMP_DATA_SPEC>;
#[doc = "GPADC CH1 Compare Data Register"]
pub mod gp_ch1_cmp_data;
#[doc = "gp_ch0_data register accessor: an alias for `Reg<GP_CH0_DATA_SPEC>`"]
pub type GP_CH0_DATA = crate::Reg<gp_ch0_data::GP_CH0_DATA_SPEC>;
#[doc = "GPADC CH0 Data Register"]
pub mod gp_ch0_data;
#[doc = "gp_ch1_data register accessor: an alias for `Reg<GP_CH1_DATA_SPEC>`"]
pub type GP_CH1_DATA = crate::Reg<gp_ch1_data::GP_CH1_DATA_SPEC>;
#[doc = "GPADC CH1 Data Register"]
pub mod gp_ch1_data;
