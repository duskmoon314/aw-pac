#[doc = r"Register block"]
#[repr(C)]
pub struct CSIC_DMA {
    #[doc = "0x00 - CSIC DMA Enable Register"]
    pub csic_dma_en: CSIC_DMA_EN,
    #[doc = "0x04 - CSIC DMA Configuration Register"]
    pub csic_dma_cfg: CSIC_DMA_CFG,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - CSIC DMA Horizontal Size Register"]
    pub csic_dma_hsize: CSIC_DMA_HSIZE,
    #[doc = "0x14 - CSIC DMA Vertical Size Register"]
    pub csic_dma_vsize: CSIC_DMA_VSIZE,
    _reserved4: [u8; 0x08],
    #[doc = "0x20 - CSIC DMA FIFO 0 Output Buffer-A Address Register"]
    pub csic_dma_f0_bufa: CSIC_DMA_F0_BUFA,
    #[doc = "0x24 - CSIC DMA FIFO 0 Output Buffer-A Address Result Register"]
    pub csic_dma_f0_bufa_result: CSIC_DMA_F0_BUFA_RESULT,
    #[doc = "0x28 - CSIC DMA FIFO 1 Output Buffer-A Address Register"]
    pub csic_dma_f1_bufa: CSIC_DMA_F1_BUFA,
    #[doc = "0x2c - CSIC DMA FIFO 1 Output Buffer-A Address Result Register"]
    pub csic_dma_f1_bufa_result: CSIC_DMA_F1_BUFA_RESULT,
    #[doc = "0x30 - CSIC DMA FIFO 2 Output Buffer-A Address Register"]
    pub csic_dma_f2_bufa: CSIC_DMA_F2_BUFA,
    #[doc = "0x34 - CSIC DMA FIFO 2 Output Buffer-A Address Result Register"]
    pub csic_dma_f2_bufa_result: CSIC_DMA_F2_BUFA_RESULT,
    #[doc = "0x38 - CSIC DMA Buffer Length Register"]
    pub csic_dma_buf_len: CSIC_DMA_BUF_LEN,
    #[doc = "0x3c - CSIC DMA Flip Size Register"]
    pub csic_dma_flip_size: CSIC_DMA_FLIP_SIZE,
    #[doc = "0x40 - CSIC DMA Video Input Timeout Threshold0 Register"]
    pub csic_dma_vi_to_th0: CSIC_DMA_VI_TO_TH0,
    #[doc = "0x44 - CSIC DMA Video Input Timeout Threshold1 Register"]
    pub csic_dma_vi_to_th1: CSIC_DMA_VI_TO_TH1,
    #[doc = "0x48 - CSIC DMA Video Input Timeout Counter Value Register"]
    pub csic_dma_vi_to_cnt_val: CSIC_DMA_VI_TO_CNT_VAL,
    #[doc = "0x4c - CSIC DMA Capture Status Register"]
    pub csic_dma_cap_sta: CSIC_DMA_CAP_STA,
    #[doc = "0x50 - CSIC DMA Interrupt Enable Register"]
    pub csic_dma_int_en: CSIC_DMA_INT_EN,
    #[doc = "0x54 - CSIC DMA Interrupt Status Register"]
    pub csic_dma_int_sta: CSIC_DMA_INT_STA,
    #[doc = "0x58 - CSIC DMA LINE Counter Register"]
    pub csic_dma_line_cnt: CSIC_DMA_LINE_CNT,
    #[doc = "0x5c - CSIC DMA Frame Counter Register"]
    pub csic_dma_frm_cnt: CSIC_DMA_FRM_CNT,
    #[doc = "0x60 - CSIC DMA Frame Clock Counter Register"]
    pub csic_dma_frm_clk_cnt: CSIC_DMA_FRM_CLK_CNT,
    #[doc = "0x64 - CSIC DMA Accumulated And Internal Clock Counter Register"]
    pub csic_dma_acc_itnl_clk_cnt: CSIC_DMA_ACC_ITNL_CLK_CNT,
    #[doc = "0x68 - CSIC DMA FIFO Statistic Register"]
    pub csic_dma_fifo_stat: CSIC_DMA_FIFO_STAT,
    #[doc = "0x6c - CSIC DMA FIFO Threshold Register"]
    pub csic_dma_fifo_thrs: CSIC_DMA_FIFO_THRS,
    #[doc = "0x70 - CSIC DMA PCLK Statistic Register"]
    pub csic_dma_pclk_stat: CSIC_DMA_PCLK_STAT,
    _reserved25: [u8; 0x0c],
    #[doc = "0x80..0x8c - CSIC DMA BUF Address FIFO\\[i\\] Entry Register"]
    pub csic_dma_buf_addr_fifo_entry: [CSIC_DMA_BUF_ADDR_FIFO_ENTRY; 3],
    #[doc = "0x8c - CSIC DMA BUF Threshold Register"]
    pub csic_dma_buf_th: CSIC_DMA_BUF_TH,
    #[doc = "0x90 - CSIC DMA BUF Address FIFO Content Register"]
    pub csic_dma_buf_addr_fifo_con: CSIC_DMA_BUF_ADDR_FIFO_CON,
    #[doc = "0x94 - CSIC DMA Stored Frame Counter Register"]
    pub csic_dma_stored_frm_cnt: CSIC_DMA_STORED_FRM_CNT,
    _reserved29: [u8; 0x015c],
    #[doc = "0x1f4 - CSIC DMA Feature List Register"]
    pub csic_feature: CSIC_FEATURE,
}
#[doc = "csic_dma_en (rw) register accessor: an alias for `Reg<CSIC_DMA_EN_SPEC>`"]
pub type CSIC_DMA_EN = crate::Reg<csic_dma_en::CSIC_DMA_EN_SPEC>;
#[doc = "CSIC DMA Enable Register"]
pub mod csic_dma_en;
#[doc = "csic_dma_cfg (rw) register accessor: an alias for `Reg<CSIC_DMA_CFG_SPEC>`"]
pub type CSIC_DMA_CFG = crate::Reg<csic_dma_cfg::CSIC_DMA_CFG_SPEC>;
#[doc = "CSIC DMA Configuration Register"]
pub mod csic_dma_cfg;
#[doc = "csic_dma_hsize (rw) register accessor: an alias for `Reg<CSIC_DMA_HSIZE_SPEC>`"]
pub type CSIC_DMA_HSIZE = crate::Reg<csic_dma_hsize::CSIC_DMA_HSIZE_SPEC>;
#[doc = "CSIC DMA Horizontal Size Register"]
pub mod csic_dma_hsize;
#[doc = "csic_dma_vsize (rw) register accessor: an alias for `Reg<CSIC_DMA_VSIZE_SPEC>`"]
pub type CSIC_DMA_VSIZE = crate::Reg<csic_dma_vsize::CSIC_DMA_VSIZE_SPEC>;
#[doc = "CSIC DMA Vertical Size Register"]
pub mod csic_dma_vsize;
#[doc = "csic_dma_f0_bufa (rw) register accessor: an alias for `Reg<CSIC_DMA_F0_BUFA_SPEC>`"]
pub type CSIC_DMA_F0_BUFA = crate::Reg<csic_dma_f0_bufa::CSIC_DMA_F0_BUFA_SPEC>;
#[doc = "CSIC DMA FIFO 0 Output Buffer-A Address Register"]
pub mod csic_dma_f0_bufa;
#[doc = "csic_dma_f0_bufa_result (rw) register accessor: an alias for `Reg<CSIC_DMA_F0_BUFA_RESULT_SPEC>`"]
pub type CSIC_DMA_F0_BUFA_RESULT =
    crate::Reg<csic_dma_f0_bufa_result::CSIC_DMA_F0_BUFA_RESULT_SPEC>;
#[doc = "CSIC DMA FIFO 0 Output Buffer-A Address Result Register"]
pub mod csic_dma_f0_bufa_result;
#[doc = "csic_dma_f1_bufa (rw) register accessor: an alias for `Reg<CSIC_DMA_F1_BUFA_SPEC>`"]
pub type CSIC_DMA_F1_BUFA = crate::Reg<csic_dma_f1_bufa::CSIC_DMA_F1_BUFA_SPEC>;
#[doc = "CSIC DMA FIFO 1 Output Buffer-A Address Register"]
pub mod csic_dma_f1_bufa;
#[doc = "csic_dma_f1_bufa_result (rw) register accessor: an alias for `Reg<CSIC_DMA_F1_BUFA_RESULT_SPEC>`"]
pub type CSIC_DMA_F1_BUFA_RESULT =
    crate::Reg<csic_dma_f1_bufa_result::CSIC_DMA_F1_BUFA_RESULT_SPEC>;
#[doc = "CSIC DMA FIFO 1 Output Buffer-A Address Result Register"]
pub mod csic_dma_f1_bufa_result;
#[doc = "csic_dma_f2_bufa (rw) register accessor: an alias for `Reg<CSIC_DMA_F2_BUFA_SPEC>`"]
pub type CSIC_DMA_F2_BUFA = crate::Reg<csic_dma_f2_bufa::CSIC_DMA_F2_BUFA_SPEC>;
#[doc = "CSIC DMA FIFO 2 Output Buffer-A Address Register"]
pub mod csic_dma_f2_bufa;
#[doc = "csic_dma_f2_bufa_result (rw) register accessor: an alias for `Reg<CSIC_DMA_F2_BUFA_RESULT_SPEC>`"]
pub type CSIC_DMA_F2_BUFA_RESULT =
    crate::Reg<csic_dma_f2_bufa_result::CSIC_DMA_F2_BUFA_RESULT_SPEC>;
#[doc = "CSIC DMA FIFO 2 Output Buffer-A Address Result Register"]
pub mod csic_dma_f2_bufa_result;
#[doc = "csic_dma_buf_len (rw) register accessor: an alias for `Reg<CSIC_DMA_BUF_LEN_SPEC>`"]
pub type CSIC_DMA_BUF_LEN = crate::Reg<csic_dma_buf_len::CSIC_DMA_BUF_LEN_SPEC>;
#[doc = "CSIC DMA Buffer Length Register"]
pub mod csic_dma_buf_len;
#[doc = "csic_dma_flip_size (rw) register accessor: an alias for `Reg<CSIC_DMA_FLIP_SIZE_SPEC>`"]
pub type CSIC_DMA_FLIP_SIZE = crate::Reg<csic_dma_flip_size::CSIC_DMA_FLIP_SIZE_SPEC>;
#[doc = "CSIC DMA Flip Size Register"]
pub mod csic_dma_flip_size;
#[doc = "csic_dma_vi_to_th0 (rw) register accessor: an alias for `Reg<CSIC_DMA_VI_TO_TH0_SPEC>`"]
pub type CSIC_DMA_VI_TO_TH0 = crate::Reg<csic_dma_vi_to_th0::CSIC_DMA_VI_TO_TH0_SPEC>;
#[doc = "CSIC DMA Video Input Timeout Threshold0 Register"]
pub mod csic_dma_vi_to_th0;
#[doc = "csic_dma_vi_to_th1 (rw) register accessor: an alias for `Reg<CSIC_DMA_VI_TO_TH1_SPEC>`"]
pub type CSIC_DMA_VI_TO_TH1 = crate::Reg<csic_dma_vi_to_th1::CSIC_DMA_VI_TO_TH1_SPEC>;
#[doc = "CSIC DMA Video Input Timeout Threshold1 Register"]
pub mod csic_dma_vi_to_th1;
#[doc = "csic_dma_vi_to_cnt_val (rw) register accessor: an alias for `Reg<CSIC_DMA_VI_TO_CNT_VAL_SPEC>`"]
pub type CSIC_DMA_VI_TO_CNT_VAL = crate::Reg<csic_dma_vi_to_cnt_val::CSIC_DMA_VI_TO_CNT_VAL_SPEC>;
#[doc = "CSIC DMA Video Input Timeout Counter Value Register"]
pub mod csic_dma_vi_to_cnt_val;
#[doc = "csic_dma_cap_sta (rw) register accessor: an alias for `Reg<CSIC_DMA_CAP_STA_SPEC>`"]
pub type CSIC_DMA_CAP_STA = crate::Reg<csic_dma_cap_sta::CSIC_DMA_CAP_STA_SPEC>;
#[doc = "CSIC DMA Capture Status Register"]
pub mod csic_dma_cap_sta;
#[doc = "csic_dma_int_en (rw) register accessor: an alias for `Reg<CSIC_DMA_INT_EN_SPEC>`"]
pub type CSIC_DMA_INT_EN = crate::Reg<csic_dma_int_en::CSIC_DMA_INT_EN_SPEC>;
#[doc = "CSIC DMA Interrupt Enable Register"]
pub mod csic_dma_int_en;
#[doc = "csic_dma_int_sta (rw) register accessor: an alias for `Reg<CSIC_DMA_INT_STA_SPEC>`"]
pub type CSIC_DMA_INT_STA = crate::Reg<csic_dma_int_sta::CSIC_DMA_INT_STA_SPEC>;
#[doc = "CSIC DMA Interrupt Status Register"]
pub mod csic_dma_int_sta;
#[doc = "csic_dma_line_cnt (rw) register accessor: an alias for `Reg<CSIC_DMA_LINE_CNT_SPEC>`"]
pub type CSIC_DMA_LINE_CNT = crate::Reg<csic_dma_line_cnt::CSIC_DMA_LINE_CNT_SPEC>;
#[doc = "CSIC DMA LINE Counter Register"]
pub mod csic_dma_line_cnt;
#[doc = "csic_dma_frm_cnt (rw) register accessor: an alias for `Reg<CSIC_DMA_FRM_CNT_SPEC>`"]
pub type CSIC_DMA_FRM_CNT = crate::Reg<csic_dma_frm_cnt::CSIC_DMA_FRM_CNT_SPEC>;
#[doc = "CSIC DMA Frame Counter Register"]
pub mod csic_dma_frm_cnt;
#[doc = "csic_dma_frm_clk_cnt (rw) register accessor: an alias for `Reg<CSIC_DMA_FRM_CLK_CNT_SPEC>`"]
pub type CSIC_DMA_FRM_CLK_CNT = crate::Reg<csic_dma_frm_clk_cnt::CSIC_DMA_FRM_CLK_CNT_SPEC>;
#[doc = "CSIC DMA Frame Clock Counter Register"]
pub mod csic_dma_frm_clk_cnt;
#[doc = "csic_dma_acc_itnl_clk_cnt (rw) register accessor: an alias for `Reg<CSIC_DMA_ACC_ITNL_CLK_CNT_SPEC>`"]
pub type CSIC_DMA_ACC_ITNL_CLK_CNT =
    crate::Reg<csic_dma_acc_itnl_clk_cnt::CSIC_DMA_ACC_ITNL_CLK_CNT_SPEC>;
#[doc = "CSIC DMA Accumulated And Internal Clock Counter Register"]
pub mod csic_dma_acc_itnl_clk_cnt;
#[doc = "csic_dma_fifo_stat (rw) register accessor: an alias for `Reg<CSIC_DMA_FIFO_STAT_SPEC>`"]
pub type CSIC_DMA_FIFO_STAT = crate::Reg<csic_dma_fifo_stat::CSIC_DMA_FIFO_STAT_SPEC>;
#[doc = "CSIC DMA FIFO Statistic Register"]
pub mod csic_dma_fifo_stat;
#[doc = "csic_dma_fifo_thrs (rw) register accessor: an alias for `Reg<CSIC_DMA_FIFO_THRS_SPEC>`"]
pub type CSIC_DMA_FIFO_THRS = crate::Reg<csic_dma_fifo_thrs::CSIC_DMA_FIFO_THRS_SPEC>;
#[doc = "CSIC DMA FIFO Threshold Register"]
pub mod csic_dma_fifo_thrs;
#[doc = "csic_dma_pclk_stat (rw) register accessor: an alias for `Reg<CSIC_DMA_PCLK_STAT_SPEC>`"]
pub type CSIC_DMA_PCLK_STAT = crate::Reg<csic_dma_pclk_stat::CSIC_DMA_PCLK_STAT_SPEC>;
#[doc = "CSIC DMA PCLK Statistic Register"]
pub mod csic_dma_pclk_stat;
#[doc = "csic_dma_buf_addr_fifo_entry (rw) register accessor: an alias for `Reg<CSIC_DMA_BUF_ADDR_FIFO_ENTRY_SPEC>`"]
pub type CSIC_DMA_BUF_ADDR_FIFO_ENTRY =
    crate::Reg<csic_dma_buf_addr_fifo_entry::CSIC_DMA_BUF_ADDR_FIFO_ENTRY_SPEC>;
#[doc = "CSIC DMA BUF Address FIFO\\[i\\] Entry Register"]
pub mod csic_dma_buf_addr_fifo_entry;
#[doc = "csic_dma_buf_th (rw) register accessor: an alias for `Reg<CSIC_DMA_BUF_TH_SPEC>`"]
pub type CSIC_DMA_BUF_TH = crate::Reg<csic_dma_buf_th::CSIC_DMA_BUF_TH_SPEC>;
#[doc = "CSIC DMA BUF Threshold Register"]
pub mod csic_dma_buf_th;
#[doc = "csic_dma_buf_addr_fifo_con (rw) register accessor: an alias for `Reg<CSIC_DMA_BUF_ADDR_FIFO_CON_SPEC>`"]
pub type CSIC_DMA_BUF_ADDR_FIFO_CON =
    crate::Reg<csic_dma_buf_addr_fifo_con::CSIC_DMA_BUF_ADDR_FIFO_CON_SPEC>;
#[doc = "CSIC DMA BUF Address FIFO Content Register"]
pub mod csic_dma_buf_addr_fifo_con;
#[doc = "csic_dma_stored_frm_cnt (rw) register accessor: an alias for `Reg<CSIC_DMA_STORED_FRM_CNT_SPEC>`"]
pub type CSIC_DMA_STORED_FRM_CNT =
    crate::Reg<csic_dma_stored_frm_cnt::CSIC_DMA_STORED_FRM_CNT_SPEC>;
#[doc = "CSIC DMA Stored Frame Counter Register"]
pub mod csic_dma_stored_frm_cnt;
#[doc = "csic_feature (rw) register accessor: an alias for `Reg<CSIC_FEATURE_SPEC>`"]
pub type CSIC_FEATURE = crate::Reg<csic_feature::CSIC_FEATURE_SPEC>;
#[doc = "CSIC DMA Feature List Register"]
pub mod csic_feature;
