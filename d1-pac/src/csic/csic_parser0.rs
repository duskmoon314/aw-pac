#[doc = r"Register block"]
#[repr(C)]
pub struct CSIC_PARSER0 {
    #[doc = "0x00 - Parser Enable Register"]
    pub prs_en: PRS_EN,
    #[doc = "0x04 - Parser NCSIC Interface Configuration Register"]
    pub prs_ncsic_if_cfg: PRS_NCSIC_IF_CFG,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - Parser Capture Register"]
    pub prs_cap: PRS_CAP,
    #[doc = "0x10 - CSIC Parser Signal Status Register"]
    pub csic_prs_signal_sta: CSIC_PRS_SIGNAL_STA,
    #[doc = "0x14 - CSIC Parser NCSIC BT656 Header Configuration Register"]
    pub csic_prs_ncsic_bt656_head_cfg: CSIC_PRS_NCSIC_BT656_HEAD_CFG,
    _reserved5: [u8; 0x0c],
    #[doc = "0x24 - Parser Channel\\[i\\] Input Format Register"]
    pub prs_ch0_infmt: PRS_CH_INFMT,
    #[doc = "0x28 - Parser Channel\\[i\\] Output Horizontal Size Register"]
    pub prs_ch0_output_hsize: PRS_CH_OUTPUT_HSIZE,
    #[doc = "0x2c - Parser Channel\\[i\\] Output Vertical Size Register"]
    pub prs_ch0_output_vsize: PRS_CH_OUTPUT_VSIZE,
    #[doc = "0x30 - Parser Channel\\[i\\] Input Parameter0 Register"]
    pub prs_ch0_input_para0: PRS_CH_INPUT_PARA0,
    #[doc = "0x34 - Parser Channel\\[i\\] Input Parameter1 Register"]
    pub prs_ch0_input_para1: PRS_CH_INPUT_PARA1,
    #[doc = "0x38 - Parser Channel\\[i\\] Input Parameter2 Register"]
    pub prs_ch0_input_para2: PRS_CH_INPUT_PARA2,
    #[doc = "0x3c - Parser Channel\\[i\\] Input Parameter3 Register"]
    pub prs_ch0_input_para3: PRS_CH_INPUT_PARA3,
    #[doc = "0x40 - Parser Channel\\[i\\] Interrupt Enable Register"]
    pub prs_ch0_int_en: PRS_CH_INT_EN,
    #[doc = "0x44 - Parser Channel\\[i\\] Interrupt Status Register"]
    pub prs_ch0_int_sta: PRS_CH_INT_STA,
    #[doc = "0x48 - Parser Channel\\[i\\] Line Time Register"]
    pub prs_ch00_line_time: PRS_CH0_LINE_TIME,
    _reserved15: [u8; 0xd8],
    #[doc = "0x124 - Parser Channel\\[i\\] Input Format Register"]
    pub prs_ch1_infmt: PRS_CH_INFMT,
    #[doc = "0x128 - Parser Channel\\[i\\] Output Horizontal Size Register"]
    pub prs_ch1_output_hsize: PRS_CH_OUTPUT_HSIZE,
    #[doc = "0x12c - Parser Channel\\[i\\] Output Vertical Size Register"]
    pub prs_ch1_output_vsize: PRS_CH_OUTPUT_VSIZE,
    #[doc = "0x130 - Parser Channel\\[i\\] Input Parameter0 Register"]
    pub prs_ch1_input_para0: PRS_CH_INPUT_PARA0,
    #[doc = "0x134 - Parser Channel\\[i\\] Input Parameter1 Register"]
    pub prs_ch1_input_para1: PRS_CH_INPUT_PARA1,
    #[doc = "0x138 - Parser Channel\\[i\\] Input Parameter2 Register"]
    pub prs_ch1_input_para2: PRS_CH_INPUT_PARA2,
    #[doc = "0x13c - Parser Channel\\[i\\] Input Parameter3 Register"]
    pub prs_ch1_input_para3: PRS_CH_INPUT_PARA3,
    #[doc = "0x140 - Parser Channel\\[i\\] Interrupt Enable Register"]
    pub prs_ch1_int_en: PRS_CH_INT_EN,
    #[doc = "0x144 - Parser Channel\\[i\\] Interrupt Status Register"]
    pub prs_ch1_int_sta: PRS_CH_INT_STA,
    #[doc = "0x148 - Parser Channel\\[i\\] Line Time Register"]
    pub prs_ch10_line_time: PRS_CH0_LINE_TIME,
    _reserved25: [u8; 0xd8],
    #[doc = "0x224 - Parser Channel\\[i\\] Input Format Register"]
    pub prs_ch2_infmt: PRS_CH_INFMT,
    #[doc = "0x228 - Parser Channel\\[i\\] Output Horizontal Size Register"]
    pub prs_ch2_output_hsize: PRS_CH_OUTPUT_HSIZE,
    #[doc = "0x22c - Parser Channel\\[i\\] Output Vertical Size Register"]
    pub prs_ch2_output_vsize: PRS_CH_OUTPUT_VSIZE,
    #[doc = "0x230 - Parser Channel\\[i\\] Input Parameter0 Register"]
    pub prs_ch2_input_para0: PRS_CH_INPUT_PARA0,
    #[doc = "0x234 - Parser Channel\\[i\\] Input Parameter1 Register"]
    pub prs_ch2_input_para1: PRS_CH_INPUT_PARA1,
    #[doc = "0x238 - Parser Channel\\[i\\] Input Parameter2 Register"]
    pub prs_ch2_input_para2: PRS_CH_INPUT_PARA2,
    #[doc = "0x23c - Parser Channel\\[i\\] Input Parameter3 Register"]
    pub prs_ch2_input_para3: PRS_CH_INPUT_PARA3,
    #[doc = "0x240 - Parser Channel\\[i\\] Interrupt Enable Register"]
    pub prs_ch2_int_en: PRS_CH_INT_EN,
    #[doc = "0x244 - Parser Channel\\[i\\] Interrupt Status Register"]
    pub prs_ch2_int_sta: PRS_CH_INT_STA,
    #[doc = "0x248 - Parser Channel\\[i\\] Line Time Register"]
    pub prs_ch20_line_time: PRS_CH0_LINE_TIME,
    _reserved35: [u8; 0xd8],
    #[doc = "0x324 - Parser Channel\\[i\\] Input Format Register"]
    pub prs_ch3_infmt: PRS_CH_INFMT,
    #[doc = "0x328 - Parser Channel\\[i\\] Output Horizontal Size Register"]
    pub prs_ch3_output_hsize: PRS_CH_OUTPUT_HSIZE,
    #[doc = "0x32c - Parser Channel\\[i\\] Output Vertical Size Register"]
    pub prs_ch3_output_vsize: PRS_CH_OUTPUT_VSIZE,
    #[doc = "0x330 - Parser Channel\\[i\\] Input Parameter0 Register"]
    pub prs_ch3_input_para0: PRS_CH_INPUT_PARA0,
    #[doc = "0x334 - Parser Channel\\[i\\] Input Parameter1 Register"]
    pub prs_ch3_input_para1: PRS_CH_INPUT_PARA1,
    #[doc = "0x338 - Parser Channel\\[i\\] Input Parameter2 Register"]
    pub prs_ch3_input_para2: PRS_CH_INPUT_PARA2,
    #[doc = "0x33c - Parser Channel\\[i\\] Input Parameter3 Register"]
    pub prs_ch3_input_para3: PRS_CH_INPUT_PARA3,
    #[doc = "0x340 - Parser Channel\\[i\\] Interrupt Enable Register"]
    pub prs_ch3_int_en: PRS_CH_INT_EN,
    #[doc = "0x344 - Parser Channel\\[i\\] Interrupt Status Register"]
    pub prs_ch3_int_sta: PRS_CH_INT_STA,
    #[doc = "0x348 - Parser Channel\\[i\\] Line Time Register"]
    pub prs_ch30_line_time: PRS_CH0_LINE_TIME,
    _reserved45: [u8; 0x01b4],
    #[doc = "0x500 - CSIC Parser NCSIC RX Signal0 Delay Adjust Register"]
    pub csic_prs_ncsic_rx_signal0_dly_adj: CSIC_PRS_NCSIC_RX_SIGNAL0_DLY_ADJ,
    _reserved46: [u8; 0x10],
    #[doc = "0x514 - CSIC Parser NCSIC RX Signal5 Delay Adjust Register"]
    pub csic_prs_ncsic_rx_signal5_dly_adj: CSIC_PRS_NCSIC_RX_SIGNAL5_DLY_ADJ,
    #[doc = "0x518 - CSIC Parser NCSIC RX Signal6 Delay Adjust Register"]
    pub csic_prs_ncsic_rx_signal6_dly_adj: CSIC_PRS_NCSIC_RX_SIGNAL6_DLY_ADJ,
}
#[doc = "prs_en (rw) register accessor: an alias for `Reg<PRS_EN_SPEC>`"]
pub type PRS_EN = crate::Reg<prs_en::PRS_EN_SPEC>;
#[doc = "Parser Enable Register"]
pub mod prs_en;
#[doc = "prs_ncsic_if_cfg (rw) register accessor: an alias for `Reg<PRS_NCSIC_IF_CFG_SPEC>`"]
pub type PRS_NCSIC_IF_CFG = crate::Reg<prs_ncsic_if_cfg::PRS_NCSIC_IF_CFG_SPEC>;
#[doc = "Parser NCSIC Interface Configuration Register"]
pub mod prs_ncsic_if_cfg;
#[doc = "prs_cap (rw) register accessor: an alias for `Reg<PRS_CAP_SPEC>`"]
pub type PRS_CAP = crate::Reg<prs_cap::PRS_CAP_SPEC>;
#[doc = "Parser Capture Register"]
pub mod prs_cap;
#[doc = "csic_prs_signal_sta (rw) register accessor: an alias for `Reg<CSIC_PRS_SIGNAL_STA_SPEC>`"]
pub type CSIC_PRS_SIGNAL_STA = crate::Reg<csic_prs_signal_sta::CSIC_PRS_SIGNAL_STA_SPEC>;
#[doc = "CSIC Parser Signal Status Register"]
pub mod csic_prs_signal_sta;
#[doc = "csic_prs_ncsic_bt656_head_cfg (rw) register accessor: an alias for `Reg<CSIC_PRS_NCSIC_BT656_HEAD_CFG_SPEC>`"]
pub type CSIC_PRS_NCSIC_BT656_HEAD_CFG =
    crate::Reg<csic_prs_ncsic_bt656_head_cfg::CSIC_PRS_NCSIC_BT656_HEAD_CFG_SPEC>;
#[doc = "CSIC Parser NCSIC BT656 Header Configuration Register"]
pub mod csic_prs_ncsic_bt656_head_cfg;
#[doc = "prs_ch_infmt (rw) register accessor: an alias for `Reg<PRS_CH_INFMT_SPEC>`"]
pub type PRS_CH_INFMT = crate::Reg<prs_ch_infmt::PRS_CH_INFMT_SPEC>;
#[doc = "Parser Channel\\[i\\] Input Format Register"]
pub mod prs_ch_infmt;
#[doc = "prs_ch_output_hsize (rw) register accessor: an alias for `Reg<PRS_CH_OUTPUT_HSIZE_SPEC>`"]
pub type PRS_CH_OUTPUT_HSIZE = crate::Reg<prs_ch_output_hsize::PRS_CH_OUTPUT_HSIZE_SPEC>;
#[doc = "Parser Channel\\[i\\] Output Horizontal Size Register"]
pub mod prs_ch_output_hsize;
#[doc = "prs_ch_output_vsize (rw) register accessor: an alias for `Reg<PRS_CH_OUTPUT_VSIZE_SPEC>`"]
pub type PRS_CH_OUTPUT_VSIZE = crate::Reg<prs_ch_output_vsize::PRS_CH_OUTPUT_VSIZE_SPEC>;
#[doc = "Parser Channel\\[i\\] Output Vertical Size Register"]
pub mod prs_ch_output_vsize;
#[doc = "prs_ch_input_para0 (r) register accessor: an alias for `Reg<PRS_CH_INPUT_PARA0_SPEC>`"]
pub type PRS_CH_INPUT_PARA0 = crate::Reg<prs_ch_input_para0::PRS_CH_INPUT_PARA0_SPEC>;
#[doc = "Parser Channel\\[i\\] Input Parameter0 Register"]
pub mod prs_ch_input_para0;
#[doc = "prs_ch_input_para1 (r) register accessor: an alias for `Reg<PRS_CH_INPUT_PARA1_SPEC>`"]
pub type PRS_CH_INPUT_PARA1 = crate::Reg<prs_ch_input_para1::PRS_CH_INPUT_PARA1_SPEC>;
#[doc = "Parser Channel\\[i\\] Input Parameter1 Register"]
pub mod prs_ch_input_para1;
#[doc = "prs_ch_input_para2 (r) register accessor: an alias for `Reg<PRS_CH_INPUT_PARA2_SPEC>`"]
pub type PRS_CH_INPUT_PARA2 = crate::Reg<prs_ch_input_para2::PRS_CH_INPUT_PARA2_SPEC>;
#[doc = "Parser Channel\\[i\\] Input Parameter2 Register"]
pub mod prs_ch_input_para2;
#[doc = "prs_ch_input_para3 (r) register accessor: an alias for `Reg<PRS_CH_INPUT_PARA3_SPEC>`"]
pub type PRS_CH_INPUT_PARA3 = crate::Reg<prs_ch_input_para3::PRS_CH_INPUT_PARA3_SPEC>;
#[doc = "Parser Channel\\[i\\] Input Parameter3 Register"]
pub mod prs_ch_input_para3;
#[doc = "prs_ch_int_en (rw) register accessor: an alias for `Reg<PRS_CH_INT_EN_SPEC>`"]
pub type PRS_CH_INT_EN = crate::Reg<prs_ch_int_en::PRS_CH_INT_EN_SPEC>;
#[doc = "Parser Channel\\[i\\] Interrupt Enable Register"]
pub mod prs_ch_int_en;
#[doc = "prs_ch_int_sta (rw) register accessor: an alias for `Reg<PRS_CH_INT_STA_SPEC>`"]
pub type PRS_CH_INT_STA = crate::Reg<prs_ch_int_sta::PRS_CH_INT_STA_SPEC>;
#[doc = "Parser Channel\\[i\\] Interrupt Status Register"]
pub mod prs_ch_int_sta;
#[doc = "prs_ch0_line_time (r) register accessor: an alias for `Reg<PRS_CH0_LINE_TIME_SPEC>`"]
pub type PRS_CH0_LINE_TIME = crate::Reg<prs_ch0_line_time::PRS_CH0_LINE_TIME_SPEC>;
#[doc = "Parser Channel\\[i\\] Line Time Register"]
pub mod prs_ch0_line_time;
#[doc = "csic_prs_ncsic_rx_signal0_dly_adj (rw) register accessor: an alias for `Reg<CSIC_PRS_NCSIC_RX_SIGNAL0_DLY_ADJ_SPEC>`"]
pub type CSIC_PRS_NCSIC_RX_SIGNAL0_DLY_ADJ =
    crate::Reg<csic_prs_ncsic_rx_signal0_dly_adj::CSIC_PRS_NCSIC_RX_SIGNAL0_DLY_ADJ_SPEC>;
#[doc = "CSIC Parser NCSIC RX Signal0 Delay Adjust Register"]
pub mod csic_prs_ncsic_rx_signal0_dly_adj;
#[doc = "csic_prs_ncsic_rx_signal5_dly_adj (rw) register accessor: an alias for `Reg<CSIC_PRS_NCSIC_RX_SIGNAL5_DLY_ADJ_SPEC>`"]
pub type CSIC_PRS_NCSIC_RX_SIGNAL5_DLY_ADJ =
    crate::Reg<csic_prs_ncsic_rx_signal5_dly_adj::CSIC_PRS_NCSIC_RX_SIGNAL5_DLY_ADJ_SPEC>;
#[doc = "CSIC Parser NCSIC RX Signal5 Delay Adjust Register"]
pub mod csic_prs_ncsic_rx_signal5_dly_adj;
#[doc = "csic_prs_ncsic_rx_signal6_dly_adj (rw) register accessor: an alias for `Reg<CSIC_PRS_NCSIC_RX_SIGNAL6_DLY_ADJ_SPEC>`"]
pub type CSIC_PRS_NCSIC_RX_SIGNAL6_DLY_ADJ =
    crate::Reg<csic_prs_ncsic_rx_signal6_dly_adj::CSIC_PRS_NCSIC_RX_SIGNAL6_DLY_ADJ_SPEC>;
#[doc = "CSIC Parser NCSIC RX Signal6 Delay Adjust Register"]
pub mod csic_prs_ncsic_rx_signal6_dly_adj;
