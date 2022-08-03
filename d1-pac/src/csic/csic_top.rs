#[doc = r"Register block"]
#[repr(C)]
pub struct CSIC_TOP {
    #[doc = "0x00 - CSIC TOP Enable Register"]
    pub csic_top_en: CSIC_TOP_EN,
    #[doc = "0x04 - CSIC Pattern Generation Enable Register"]
    pub csic_ptn_gen_en: CSIC_PTN_GEN_EN,
    #[doc = "0x08 - CSIC Pattern Control Register"]
    pub csic_ptn_ctrl: CSIC_PTN_CTRL,
    _reserved3: [u8; 0x14],
    #[doc = "0x20 - CSIC Pattern Generation Length Register"]
    pub csic_ptn_len: CSIC_PTN_LEN,
    #[doc = "0x24 - CSIC Pattern Generation Address Register"]
    pub csic_ptn_addr: CSIC_PTN_ADDR,
    #[doc = "0x28 - CSIC Pattern ISP Size Register"]
    pub csic_ptn_isp_size: CSIC_PTN_ISP_SIZE,
    _reserved6: [u8; 0x74],
    #[doc = "0xa0 - CSIC DMA0 Input Select Register"]
    pub csic_dma0_input_sel: CSIC_DMA0_INPUT_SEL,
    #[doc = "0xa4 - CSIC DMA1 Input Select Register"]
    pub csic_dma1_input_sel: CSIC_DMA1_INPUT_SEL,
    _reserved8: [u8; 0x34],
    #[doc = "0xdc - CSIC BIST CS Register"]
    pub csic_bist_cs: CSIC_BIST_CS,
    #[doc = "0xe0 - CSIC BIST Control Register"]
    pub csic_bist_control: CSIC_BIST_CONTROL,
    #[doc = "0xe4 - CSIC BIST Start Register"]
    pub csic_bist_start: CSIC_BIST_START,
    #[doc = "0xe8 - CSIC BIST End Register"]
    pub csic_bist_end: CSIC_BIST_END,
    #[doc = "0xec - CSIC BIST Data Mask Register"]
    pub csic_bist_data_mask: CSIC_BIST_DATA_MASK,
    #[doc = "0xf0 - CSIC MBUS REQ MAX Register"]
    pub csic_mbus_req_max: CSIC_MBUS_REQ_MAX,
    _reserved14: [u8; 0x0c],
    #[doc = "0x100 - CSIC Multi-Frame Mode Register"]
    pub csic_mulf_mod: CSIC_MULF_MOD,
    #[doc = "0x104 - CSIC Multi-Frame Interrupt Register"]
    pub csic_mulf_int: CSIC_MULF_INT,
}
#[doc = "csic_top_en (rw) register accessor: an alias for `Reg<CSIC_TOP_EN_SPEC>`"]
pub type CSIC_TOP_EN = crate::Reg<csic_top_en::CSIC_TOP_EN_SPEC>;
#[doc = "CSIC TOP Enable Register"]
pub mod csic_top_en;
#[doc = "csic_ptn_gen_en (rw) register accessor: an alias for `Reg<CSIC_PTN_GEN_EN_SPEC>`"]
pub type CSIC_PTN_GEN_EN = crate::Reg<csic_ptn_gen_en::CSIC_PTN_GEN_EN_SPEC>;
#[doc = "CSIC Pattern Generation Enable Register"]
pub mod csic_ptn_gen_en;
#[doc = "csic_ptn_ctrl (rw) register accessor: an alias for `Reg<CSIC_PTN_CTRL_SPEC>`"]
pub type CSIC_PTN_CTRL = crate::Reg<csic_ptn_ctrl::CSIC_PTN_CTRL_SPEC>;
#[doc = "CSIC Pattern Control Register"]
pub mod csic_ptn_ctrl;
#[doc = "csic_ptn_len (rw) register accessor: an alias for `Reg<CSIC_PTN_LEN_SPEC>`"]
pub type CSIC_PTN_LEN = crate::Reg<csic_ptn_len::CSIC_PTN_LEN_SPEC>;
#[doc = "CSIC Pattern Generation Length Register"]
pub mod csic_ptn_len;
#[doc = "csic_ptn_addr (rw) register accessor: an alias for `Reg<CSIC_PTN_ADDR_SPEC>`"]
pub type CSIC_PTN_ADDR = crate::Reg<csic_ptn_addr::CSIC_PTN_ADDR_SPEC>;
#[doc = "CSIC Pattern Generation Address Register"]
pub mod csic_ptn_addr;
#[doc = "csic_ptn_isp_size (rw) register accessor: an alias for `Reg<CSIC_PTN_ISP_SIZE_SPEC>`"]
pub type CSIC_PTN_ISP_SIZE = crate::Reg<csic_ptn_isp_size::CSIC_PTN_ISP_SIZE_SPEC>;
#[doc = "CSIC Pattern ISP Size Register"]
pub mod csic_ptn_isp_size;
#[doc = "csic_dma0_input_sel (rw) register accessor: an alias for `Reg<CSIC_DMA0_INPUT_SEL_SPEC>`"]
pub type CSIC_DMA0_INPUT_SEL = crate::Reg<csic_dma0_input_sel::CSIC_DMA0_INPUT_SEL_SPEC>;
#[doc = "CSIC DMA0 Input Select Register"]
pub mod csic_dma0_input_sel;
#[doc = "csic_dma1_input_sel (rw) register accessor: an alias for `Reg<CSIC_DMA1_INPUT_SEL_SPEC>`"]
pub type CSIC_DMA1_INPUT_SEL = crate::Reg<csic_dma1_input_sel::CSIC_DMA1_INPUT_SEL_SPEC>;
#[doc = "CSIC DMA1 Input Select Register"]
pub mod csic_dma1_input_sel;
#[doc = "csic_bist_cs (rw) register accessor: an alias for `Reg<CSIC_BIST_CS_SPEC>`"]
pub type CSIC_BIST_CS = crate::Reg<csic_bist_cs::CSIC_BIST_CS_SPEC>;
#[doc = "CSIC BIST CS Register"]
pub mod csic_bist_cs;
#[doc = "csic_bist_control (rw) register accessor: an alias for `Reg<CSIC_BIST_CONTROL_SPEC>`"]
pub type CSIC_BIST_CONTROL = crate::Reg<csic_bist_control::CSIC_BIST_CONTROL_SPEC>;
#[doc = "CSIC BIST Control Register"]
pub mod csic_bist_control;
#[doc = "csic_bist_start (rw) register accessor: an alias for `Reg<CSIC_BIST_START_SPEC>`"]
pub type CSIC_BIST_START = crate::Reg<csic_bist_start::CSIC_BIST_START_SPEC>;
#[doc = "CSIC BIST Start Register"]
pub mod csic_bist_start;
#[doc = "csic_bist_end (rw) register accessor: an alias for `Reg<CSIC_BIST_END_SPEC>`"]
pub type CSIC_BIST_END = crate::Reg<csic_bist_end::CSIC_BIST_END_SPEC>;
#[doc = "CSIC BIST End Register"]
pub mod csic_bist_end;
#[doc = "csic_bist_data_mask (rw) register accessor: an alias for `Reg<CSIC_BIST_DATA_MASK_SPEC>`"]
pub type CSIC_BIST_DATA_MASK = crate::Reg<csic_bist_data_mask::CSIC_BIST_DATA_MASK_SPEC>;
#[doc = "CSIC BIST Data Mask Register"]
pub mod csic_bist_data_mask;
#[doc = "csic_mbus_req_max (rw) register accessor: an alias for `Reg<CSIC_MBUS_REQ_MAX_SPEC>`"]
pub type CSIC_MBUS_REQ_MAX = crate::Reg<csic_mbus_req_max::CSIC_MBUS_REQ_MAX_SPEC>;
#[doc = "CSIC MBUS REQ MAX Register"]
pub mod csic_mbus_req_max;
#[doc = "csic_mulf_mod (rw) register accessor: an alias for `Reg<CSIC_MULF_MOD_SPEC>`"]
pub type CSIC_MULF_MOD = crate::Reg<csic_mulf_mod::CSIC_MULF_MOD_SPEC>;
#[doc = "CSIC Multi-Frame Mode Register"]
pub mod csic_mulf_mod;
#[doc = "csic_mulf_int (rw) register accessor: an alias for `Reg<CSIC_MULF_INT_SPEC>`"]
pub type CSIC_MULF_INT = crate::Reg<csic_mulf_int::CSIC_MULF_INT_SPEC>;
#[doc = "CSIC Multi-Frame Interrupt Register"]
pub mod csic_mulf_int;
