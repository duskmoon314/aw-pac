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
    #[doc = "0x24 - Parser Channel_0 Input Format Register"]
    pub prs_c0_infmt: PRS_C0_INFMT,
    #[doc = "0x28 - Parser Channel_0 Output Horizontal Size Register"]
    pub prs_c0_output_hsize: PRS_C0_OUTPUT_HSIZE,
    #[doc = "0x2c - Parser Channel_0 Output Vertical Size Register"]
    pub prs_c0_output_vsize: PRS_C0_OUTPUT_VSIZE,
    #[doc = "0x30 - Parser Channel_0 Input Parameter0 Register"]
    pub prs_c0_input_para0: PRS_C0_INPUT_PARA0,
    #[doc = "0x34 - Parser Channel_0 Input Parameter1 Register"]
    pub prs_c0_input_para1: PRS_C0_INPUT_PARA1,
    #[doc = "0x38 - Parser Channel_0 Input Parameter2 Register"]
    pub prs_c0_input_para2: PRS_C0_INPUT_PARA2,
    #[doc = "0x3c - Parser Channel_0 Input Parameter3 Register"]
    pub prs_c0_input_para3: PRS_C0_INPUT_PARA3,
    #[doc = "0x40 - Parser Channel_0 Interrupt Enable Register"]
    pub prs_c0_int_en: PRS_C0_INT_EN,
    #[doc = "0x44 - Parser Channel_0 Interrupt Status Register"]
    pub prs_c0_int_sta: PRS_C0_INT_STA,
    #[doc = "0x48 - Parser Channel_0 Line Time Register"]
    pub prs_ch0_line_time: PRS_CH0_LINE_TIME,
    _reserved15: [u8; 0xd8],
    #[doc = "0x124 - Parser Channel_1 Input Format Register"]
    pub prs_c1_infmt: PRS_C1_INFMT,
    #[doc = "0x128 - Parser Channel_1 Output Horizontal Size"]
    pub prs_c1_output_hsize: PRS_C1_OUTPUT_HSIZE,
    #[doc = "0x12c - Parser Channel_1 Output Vertical Size Register"]
    pub prs_c1_output_vsize: PRS_C1_OUTPUT_VSIZE,
    #[doc = "0x130 - Parser Channel_1 Input Parameter0 Register"]
    pub prs_c1_input_para0: PRS_C1_INPUT_PARA0,
    #[doc = "0x134 - Parser Channel_1 Input Parameter1 Register"]
    pub prs_c1_input_para1: PRS_C1_INPUT_PARA1,
    #[doc = "0x138 - Parser Channel_1 Input Parameter2 Register"]
    pub prs_c1_input_para2: PRS_C1_INPUT_PARA2,
    #[doc = "0x13c - Parser Channel_1 Input Parameter3 Register"]
    pub prs_c1_input_para3: PRS_C1_INPUT_PARA3,
    #[doc = "0x140 - Parser Channel_1 Interrupt Enable Register"]
    pub prs_c1_int_en: PRS_C1_INT_EN,
    #[doc = "0x144 - Parser Channel_1 Interrupt Status Register"]
    pub prs_c1_int_sta: PRS_C1_INT_STA,
    #[doc = "0x148 - Parser Channel_1 Line Time Register"]
    pub prs_ch1_line_time: PRS_CH1_LINE_TIME,
    _reserved25: [u8; 0xd8],
    #[doc = "0x224 - Parser Channel_2 Input Format Register"]
    pub prs_c2_infmt: PRS_C2_INFMT,
    #[doc = "0x228 - Parser Channel_2 Output Horizontal Size Register"]
    pub prs_c2_output_hsize: PRS_C2_OUTPUT_HSIZE,
    #[doc = "0x22c - Parser Channel_2 Output Vertical Size Register"]
    pub prs_c2_output_vsize: PRS_C2_OUTPUT_VSIZE,
    #[doc = "0x230 - Parser Channel_2 Input Parameter0 Register"]
    pub prs_c2_input_para0: PRS_C2_INPUT_PARA0,
    #[doc = "0x234 - Parser Channel_2 Input Parameter1 Register"]
    pub prs_c2_input_para1: PRS_C2_INPUT_PARA1,
    #[doc = "0x238 - Parser Channel_2 Input Parameter2 Register"]
    pub prs_c2_input_para2: PRS_C2_INPUT_PARA2,
    #[doc = "0x23c - Parser Channel_2 Input Parameter3 Register"]
    pub prs_c2_input_para3: PRS_C2_INPUT_PARA3,
    #[doc = "0x240 - Parser Channel_2 Interrupt Enable Register"]
    pub prs_c2_int_en: PRS_C2_INT_EN,
    #[doc = "0x244 - Parser Channel_2 Interrupt Status Register"]
    pub prs_c2_int_sta: PRS_C2_INT_STA,
    #[doc = "0x248 - Parser Channel_2 Line Time Register"]
    pub prs_ch2_line_time: PRS_CH2_LINE_TIME,
    _reserved35: [u8; 0xd8],
    #[doc = "0x324 - Parser Channel_3 Input Format Register"]
    pub prs_c3_infmt: PRS_C3_INFMT,
    #[doc = "0x328 - Parser Channel_3 Output Horizontal Size Register"]
    pub prs_c3_output_hsize: PRS_C3_OUTPUT_HSIZE,
    #[doc = "0x32c - Parser Channel_3 Output Vertical Size Register"]
    pub prs_c3_output_vsize: PRS_C3_OUTPUT_VSIZE,
    #[doc = "0x330 - Parser Channel_3 Input Parameter0 Register"]
    pub prs_c3_input_para0: PRS_C3_INPUT_PARA0,
    #[doc = "0x334 - Parser Channel_3 Input Parameter1 Register"]
    pub prs_c3_input_para1: PRS_C3_INPUT_PARA1,
    #[doc = "0x338 - Parser Channel_3 Input Parameter2 Register"]
    pub prs_c3_input_para2: PRS_C3_INPUT_PARA2,
    #[doc = "0x33c - Parser Channel_3 Input Parameter3 Register"]
    pub prs_c3_input_para3: PRS_C3_INPUT_PARA3,
    #[doc = "0x340 - Parser Channel_3 Interrupt Enable Register"]
    pub prs_c3_int_en: PRS_C3_INT_EN,
    #[doc = "0x344 - Parser Channel_3 Interrupt Status Register"]
    pub prs_c3_int_sta: PRS_C3_INT_STA,
    #[doc = "0x348 - Parser Channel_3 Line Time Register"]
    pub prs_ch3_line_time: PRS_CH3_LINE_TIME,
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
#[doc = "prs_c0_infmt (rw) register accessor: an alias for `Reg<PRS_C0_INFMT_SPEC>`"]
pub type PRS_C0_INFMT = crate::Reg<prs_c0_infmt::PRS_C0_INFMT_SPEC>;
#[doc = "Parser Channel_0 Input Format Register"]
pub mod prs_c0_infmt;
#[doc = "prs_c0_output_hsize (rw) register accessor: an alias for `Reg<PRS_C0_OUTPUT_HSIZE_SPEC>`"]
pub type PRS_C0_OUTPUT_HSIZE = crate::Reg<prs_c0_output_hsize::PRS_C0_OUTPUT_HSIZE_SPEC>;
#[doc = "Parser Channel_0 Output Horizontal Size Register"]
pub mod prs_c0_output_hsize;
#[doc = "prs_c0_output_vsize (rw) register accessor: an alias for `Reg<PRS_C0_OUTPUT_VSIZE_SPEC>`"]
pub type PRS_C0_OUTPUT_VSIZE = crate::Reg<prs_c0_output_vsize::PRS_C0_OUTPUT_VSIZE_SPEC>;
#[doc = "Parser Channel_0 Output Vertical Size Register"]
pub mod prs_c0_output_vsize;
#[doc = "prs_c0_input_para0 (rw) register accessor: an alias for `Reg<PRS_C0_INPUT_PARA0_SPEC>`"]
pub type PRS_C0_INPUT_PARA0 = crate::Reg<prs_c0_input_para0::PRS_C0_INPUT_PARA0_SPEC>;
#[doc = "Parser Channel_0 Input Parameter0 Register"]
pub mod prs_c0_input_para0;
#[doc = "prs_c0_input_para1 (rw) register accessor: an alias for `Reg<PRS_C0_INPUT_PARA1_SPEC>`"]
pub type PRS_C0_INPUT_PARA1 = crate::Reg<prs_c0_input_para1::PRS_C0_INPUT_PARA1_SPEC>;
#[doc = "Parser Channel_0 Input Parameter1 Register"]
pub mod prs_c0_input_para1;
#[doc = "prs_c0_input_para2 (rw) register accessor: an alias for `Reg<PRS_C0_INPUT_PARA2_SPEC>`"]
pub type PRS_C0_INPUT_PARA2 = crate::Reg<prs_c0_input_para2::PRS_C0_INPUT_PARA2_SPEC>;
#[doc = "Parser Channel_0 Input Parameter2 Register"]
pub mod prs_c0_input_para2;
#[doc = "prs_c0_input_para3 (rw) register accessor: an alias for `Reg<PRS_C0_INPUT_PARA3_SPEC>`"]
pub type PRS_C0_INPUT_PARA3 = crate::Reg<prs_c0_input_para3::PRS_C0_INPUT_PARA3_SPEC>;
#[doc = "Parser Channel_0 Input Parameter3 Register"]
pub mod prs_c0_input_para3;
#[doc = "prs_c0_int_en (rw) register accessor: an alias for `Reg<PRS_C0_INT_EN_SPEC>`"]
pub type PRS_C0_INT_EN = crate::Reg<prs_c0_int_en::PRS_C0_INT_EN_SPEC>;
#[doc = "Parser Channel_0 Interrupt Enable Register"]
pub mod prs_c0_int_en;
#[doc = "prs_c0_int_sta (rw) register accessor: an alias for `Reg<PRS_C0_INT_STA_SPEC>`"]
pub type PRS_C0_INT_STA = crate::Reg<prs_c0_int_sta::PRS_C0_INT_STA_SPEC>;
#[doc = "Parser Channel_0 Interrupt Status Register"]
pub mod prs_c0_int_sta;
#[doc = "prs_ch0_line_time (rw) register accessor: an alias for `Reg<PRS_CH0_LINE_TIME_SPEC>`"]
pub type PRS_CH0_LINE_TIME = crate::Reg<prs_ch0_line_time::PRS_CH0_LINE_TIME_SPEC>;
#[doc = "Parser Channel_0 Line Time Register"]
pub mod prs_ch0_line_time;
#[doc = "prs_c1_infmt (rw) register accessor: an alias for `Reg<PRS_C1_INFMT_SPEC>`"]
pub type PRS_C1_INFMT = crate::Reg<prs_c1_infmt::PRS_C1_INFMT_SPEC>;
#[doc = "Parser Channel_1 Input Format Register"]
pub mod prs_c1_infmt;
#[doc = "prs_c1_output_hsize (rw) register accessor: an alias for `Reg<PRS_C1_OUTPUT_HSIZE_SPEC>`"]
pub type PRS_C1_OUTPUT_HSIZE = crate::Reg<prs_c1_output_hsize::PRS_C1_OUTPUT_HSIZE_SPEC>;
#[doc = "Parser Channel_1 Output Horizontal Size"]
pub mod prs_c1_output_hsize;
#[doc = "prs_c1_output_vsize (rw) register accessor: an alias for `Reg<PRS_C1_OUTPUT_VSIZE_SPEC>`"]
pub type PRS_C1_OUTPUT_VSIZE = crate::Reg<prs_c1_output_vsize::PRS_C1_OUTPUT_VSIZE_SPEC>;
#[doc = "Parser Channel_1 Output Vertical Size Register"]
pub mod prs_c1_output_vsize;
#[doc = "prs_c1_input_para0 (rw) register accessor: an alias for `Reg<PRS_C1_INPUT_PARA0_SPEC>`"]
pub type PRS_C1_INPUT_PARA0 = crate::Reg<prs_c1_input_para0::PRS_C1_INPUT_PARA0_SPEC>;
#[doc = "Parser Channel_1 Input Parameter0 Register"]
pub mod prs_c1_input_para0;
#[doc = "prs_c1_input_para1 (rw) register accessor: an alias for `Reg<PRS_C1_INPUT_PARA1_SPEC>`"]
pub type PRS_C1_INPUT_PARA1 = crate::Reg<prs_c1_input_para1::PRS_C1_INPUT_PARA1_SPEC>;
#[doc = "Parser Channel_1 Input Parameter1 Register"]
pub mod prs_c1_input_para1;
#[doc = "prs_c1_input_para2 (rw) register accessor: an alias for `Reg<PRS_C1_INPUT_PARA2_SPEC>`"]
pub type PRS_C1_INPUT_PARA2 = crate::Reg<prs_c1_input_para2::PRS_C1_INPUT_PARA2_SPEC>;
#[doc = "Parser Channel_1 Input Parameter2 Register"]
pub mod prs_c1_input_para2;
#[doc = "prs_c1_input_para3 (rw) register accessor: an alias for `Reg<PRS_C1_INPUT_PARA3_SPEC>`"]
pub type PRS_C1_INPUT_PARA3 = crate::Reg<prs_c1_input_para3::PRS_C1_INPUT_PARA3_SPEC>;
#[doc = "Parser Channel_1 Input Parameter3 Register"]
pub mod prs_c1_input_para3;
#[doc = "prs_c1_int_en (rw) register accessor: an alias for `Reg<PRS_C1_INT_EN_SPEC>`"]
pub type PRS_C1_INT_EN = crate::Reg<prs_c1_int_en::PRS_C1_INT_EN_SPEC>;
#[doc = "Parser Channel_1 Interrupt Enable Register"]
pub mod prs_c1_int_en;
#[doc = "prs_c1_int_sta (rw) register accessor: an alias for `Reg<PRS_C1_INT_STA_SPEC>`"]
pub type PRS_C1_INT_STA = crate::Reg<prs_c1_int_sta::PRS_C1_INT_STA_SPEC>;
#[doc = "Parser Channel_1 Interrupt Status Register"]
pub mod prs_c1_int_sta;
#[doc = "prs_ch1_line_time (rw) register accessor: an alias for `Reg<PRS_CH1_LINE_TIME_SPEC>`"]
pub type PRS_CH1_LINE_TIME = crate::Reg<prs_ch1_line_time::PRS_CH1_LINE_TIME_SPEC>;
#[doc = "Parser Channel_1 Line Time Register"]
pub mod prs_ch1_line_time;
#[doc = "prs_c2_infmt (rw) register accessor: an alias for `Reg<PRS_C2_INFMT_SPEC>`"]
pub type PRS_C2_INFMT = crate::Reg<prs_c2_infmt::PRS_C2_INFMT_SPEC>;
#[doc = "Parser Channel_2 Input Format Register"]
pub mod prs_c2_infmt;
#[doc = "prs_c2_output_hsize (rw) register accessor: an alias for `Reg<PRS_C2_OUTPUT_HSIZE_SPEC>`"]
pub type PRS_C2_OUTPUT_HSIZE = crate::Reg<prs_c2_output_hsize::PRS_C2_OUTPUT_HSIZE_SPEC>;
#[doc = "Parser Channel_2 Output Horizontal Size Register"]
pub mod prs_c2_output_hsize;
#[doc = "prs_c2_output_vsize (rw) register accessor: an alias for `Reg<PRS_C2_OUTPUT_VSIZE_SPEC>`"]
pub type PRS_C2_OUTPUT_VSIZE = crate::Reg<prs_c2_output_vsize::PRS_C2_OUTPUT_VSIZE_SPEC>;
#[doc = "Parser Channel_2 Output Vertical Size Register"]
pub mod prs_c2_output_vsize;
#[doc = "prs_c2_input_para0 (rw) register accessor: an alias for `Reg<PRS_C2_INPUT_PARA0_SPEC>`"]
pub type PRS_C2_INPUT_PARA0 = crate::Reg<prs_c2_input_para0::PRS_C2_INPUT_PARA0_SPEC>;
#[doc = "Parser Channel_2 Input Parameter0 Register"]
pub mod prs_c2_input_para0;
#[doc = "prs_c2_input_para1 (rw) register accessor: an alias for `Reg<PRS_C2_INPUT_PARA1_SPEC>`"]
pub type PRS_C2_INPUT_PARA1 = crate::Reg<prs_c2_input_para1::PRS_C2_INPUT_PARA1_SPEC>;
#[doc = "Parser Channel_2 Input Parameter1 Register"]
pub mod prs_c2_input_para1;
#[doc = "prs_c2_input_para2 (rw) register accessor: an alias for `Reg<PRS_C2_INPUT_PARA2_SPEC>`"]
pub type PRS_C2_INPUT_PARA2 = crate::Reg<prs_c2_input_para2::PRS_C2_INPUT_PARA2_SPEC>;
#[doc = "Parser Channel_2 Input Parameter2 Register"]
pub mod prs_c2_input_para2;
#[doc = "prs_c2_input_para3 (rw) register accessor: an alias for `Reg<PRS_C2_INPUT_PARA3_SPEC>`"]
pub type PRS_C2_INPUT_PARA3 = crate::Reg<prs_c2_input_para3::PRS_C2_INPUT_PARA3_SPEC>;
#[doc = "Parser Channel_2 Input Parameter3 Register"]
pub mod prs_c2_input_para3;
#[doc = "prs_c2_int_en (rw) register accessor: an alias for `Reg<PRS_C2_INT_EN_SPEC>`"]
pub type PRS_C2_INT_EN = crate::Reg<prs_c2_int_en::PRS_C2_INT_EN_SPEC>;
#[doc = "Parser Channel_2 Interrupt Enable Register"]
pub mod prs_c2_int_en;
#[doc = "prs_c2_int_sta (rw) register accessor: an alias for `Reg<PRS_C2_INT_STA_SPEC>`"]
pub type PRS_C2_INT_STA = crate::Reg<prs_c2_int_sta::PRS_C2_INT_STA_SPEC>;
#[doc = "Parser Channel_2 Interrupt Status Register"]
pub mod prs_c2_int_sta;
#[doc = "prs_ch2_line_time (rw) register accessor: an alias for `Reg<PRS_CH2_LINE_TIME_SPEC>`"]
pub type PRS_CH2_LINE_TIME = crate::Reg<prs_ch2_line_time::PRS_CH2_LINE_TIME_SPEC>;
#[doc = "Parser Channel_2 Line Time Register"]
pub mod prs_ch2_line_time;
#[doc = "prs_c3_infmt (rw) register accessor: an alias for `Reg<PRS_C3_INFMT_SPEC>`"]
pub type PRS_C3_INFMT = crate::Reg<prs_c3_infmt::PRS_C3_INFMT_SPEC>;
#[doc = "Parser Channel_3 Input Format Register"]
pub mod prs_c3_infmt;
#[doc = "prs_c3_output_hsize (rw) register accessor: an alias for `Reg<PRS_C3_OUTPUT_HSIZE_SPEC>`"]
pub type PRS_C3_OUTPUT_HSIZE = crate::Reg<prs_c3_output_hsize::PRS_C3_OUTPUT_HSIZE_SPEC>;
#[doc = "Parser Channel_3 Output Horizontal Size Register"]
pub mod prs_c3_output_hsize;
#[doc = "prs_c3_output_vsize (rw) register accessor: an alias for `Reg<PRS_C3_OUTPUT_VSIZE_SPEC>`"]
pub type PRS_C3_OUTPUT_VSIZE = crate::Reg<prs_c3_output_vsize::PRS_C3_OUTPUT_VSIZE_SPEC>;
#[doc = "Parser Channel_3 Output Vertical Size Register"]
pub mod prs_c3_output_vsize;
#[doc = "prs_c3_input_para0 (rw) register accessor: an alias for `Reg<PRS_C3_INPUT_PARA0_SPEC>`"]
pub type PRS_C3_INPUT_PARA0 = crate::Reg<prs_c3_input_para0::PRS_C3_INPUT_PARA0_SPEC>;
#[doc = "Parser Channel_3 Input Parameter0 Register"]
pub mod prs_c3_input_para0;
#[doc = "prs_c3_input_para1 (rw) register accessor: an alias for `Reg<PRS_C3_INPUT_PARA1_SPEC>`"]
pub type PRS_C3_INPUT_PARA1 = crate::Reg<prs_c3_input_para1::PRS_C3_INPUT_PARA1_SPEC>;
#[doc = "Parser Channel_3 Input Parameter1 Register"]
pub mod prs_c3_input_para1;
#[doc = "prs_c3_input_para2 (rw) register accessor: an alias for `Reg<PRS_C3_INPUT_PARA2_SPEC>`"]
pub type PRS_C3_INPUT_PARA2 = crate::Reg<prs_c3_input_para2::PRS_C3_INPUT_PARA2_SPEC>;
#[doc = "Parser Channel_3 Input Parameter2 Register"]
pub mod prs_c3_input_para2;
#[doc = "prs_c3_input_para3 (rw) register accessor: an alias for `Reg<PRS_C3_INPUT_PARA3_SPEC>`"]
pub type PRS_C3_INPUT_PARA3 = crate::Reg<prs_c3_input_para3::PRS_C3_INPUT_PARA3_SPEC>;
#[doc = "Parser Channel_3 Input Parameter3 Register"]
pub mod prs_c3_input_para3;
#[doc = "prs_c3_int_en (rw) register accessor: an alias for `Reg<PRS_C3_INT_EN_SPEC>`"]
pub type PRS_C3_INT_EN = crate::Reg<prs_c3_int_en::PRS_C3_INT_EN_SPEC>;
#[doc = "Parser Channel_3 Interrupt Enable Register"]
pub mod prs_c3_int_en;
#[doc = "prs_c3_int_sta (rw) register accessor: an alias for `Reg<PRS_C3_INT_STA_SPEC>`"]
pub type PRS_C3_INT_STA = crate::Reg<prs_c3_int_sta::PRS_C3_INT_STA_SPEC>;
#[doc = "Parser Channel_3 Interrupt Status Register"]
pub mod prs_c3_int_sta;
#[doc = "prs_ch3_line_time (rw) register accessor: an alias for `Reg<PRS_CH3_LINE_TIME_SPEC>`"]
pub type PRS_CH3_LINE_TIME = crate::Reg<prs_ch3_line_time::PRS_CH3_LINE_TIME_SPEC>;
#[doc = "Parser Channel_3 Line Time Register"]
pub mod prs_ch3_line_time;
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
