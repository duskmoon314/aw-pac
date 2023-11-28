#[doc = r"Register block"]
#[repr(C)]
pub struct CSIC_DMA {
    csic_dma_en: CSIC_DMA_EN,
    csic_dma_cfg: CSIC_DMA_CFG,
    _reserved2: [u8; 0x08],
    csic_dma_hsize: CSIC_DMA_HSIZE,
    csic_dma_vsize: CSIC_DMA_VSIZE,
    _reserved4: [u8; 0x08],
    csic_dma_f0_bufa: CSIC_DMA_F0_BUFA,
    csic_dma_f0_bufa_result: CSIC_DMA_F0_BUFA_RESULT,
    csic_dma_f1_bufa: CSIC_DMA_F1_BUFA,
    csic_dma_f1_bufa_result: CSIC_DMA_F1_BUFA_RESULT,
    csic_dma_f2_bufa: CSIC_DMA_F2_BUFA,
    csic_dma_f2_bufa_result: CSIC_DMA_F2_BUFA_RESULT,
    csic_dma_buf_len: CSIC_DMA_BUF_LEN,
    csic_dma_flip_size: CSIC_DMA_FLIP_SIZE,
    csic_dma_vi_to_th0: CSIC_DMA_VI_TO_TH0,
    csic_dma_vi_to_th1: CSIC_DMA_VI_TO_TH1,
    csic_dma_vi_to_cnt_val: CSIC_DMA_VI_TO_CNT_VAL,
    csic_dma_cap_sta: CSIC_DMA_CAP_STA,
    csic_dma_int_en: CSIC_DMA_INT_EN,
    csic_dma_int_sta: CSIC_DMA_INT_STA,
    csic_dma_line_cnt: CSIC_DMA_LINE_CNT,
    csic_dma_frm_cnt: CSIC_DMA_FRM_CNT,
    csic_dma_frm_clk_cnt: CSIC_DMA_FRM_CLK_CNT,
    csic_dma_acc_itnl_clk_cnt: CSIC_DMA_ACC_ITNL_CLK_CNT,
    csic_dma_fifo_stat: CSIC_DMA_FIFO_STAT,
    csic_dma_fifo_thrs: CSIC_DMA_FIFO_THRS,
    csic_dma_pclk_stat: CSIC_DMA_PCLK_STAT,
    _reserved25: [u8; 0x0c],
    csic_dma_buf_addr_fifo_entry: [CSIC_DMA_BUF_ADDR_FIFO_ENTRY; 3],
    csic_dma_buf_th: CSIC_DMA_BUF_TH,
    csic_dma_buf_addr_fifo_con: CSIC_DMA_BUF_ADDR_FIFO_CON,
    csic_dma_stored_frm_cnt: CSIC_DMA_STORED_FRM_CNT,
    _reserved29: [u8; 0x015c],
    csic_feature: CSIC_FEATURE,
}
impl CSIC_DMA {
    #[doc = "0x00 - CSIC DMA Enable Register"]
    #[inline(always)]
    pub const fn csic_dma_en(&self) -> &CSIC_DMA_EN {
        &self.csic_dma_en
    }
    #[doc = "0x04 - CSIC DMA Configuration Register"]
    #[inline(always)]
    pub const fn csic_dma_cfg(&self) -> &CSIC_DMA_CFG {
        &self.csic_dma_cfg
    }
    #[doc = "0x10 - CSIC DMA Horizontal Size Register"]
    #[inline(always)]
    pub const fn csic_dma_hsize(&self) -> &CSIC_DMA_HSIZE {
        &self.csic_dma_hsize
    }
    #[doc = "0x14 - CSIC DMA Vertical Size Register"]
    #[inline(always)]
    pub const fn csic_dma_vsize(&self) -> &CSIC_DMA_VSIZE {
        &self.csic_dma_vsize
    }
    #[doc = "0x20 - CSIC DMA FIFO 0 Output Buffer-A Address Register"]
    #[inline(always)]
    pub const fn csic_dma_f0_bufa(&self) -> &CSIC_DMA_F0_BUFA {
        &self.csic_dma_f0_bufa
    }
    #[doc = "0x24 - CSIC DMA FIFO 0 Output Buffer-A Address Result Register"]
    #[inline(always)]
    pub const fn csic_dma_f0_bufa_result(&self) -> &CSIC_DMA_F0_BUFA_RESULT {
        &self.csic_dma_f0_bufa_result
    }
    #[doc = "0x28 - CSIC DMA FIFO 1 Output Buffer-A Address Register"]
    #[inline(always)]
    pub const fn csic_dma_f1_bufa(&self) -> &CSIC_DMA_F1_BUFA {
        &self.csic_dma_f1_bufa
    }
    #[doc = "0x2c - CSIC DMA FIFO 1 Output Buffer-A Address Result Register"]
    #[inline(always)]
    pub const fn csic_dma_f1_bufa_result(&self) -> &CSIC_DMA_F1_BUFA_RESULT {
        &self.csic_dma_f1_bufa_result
    }
    #[doc = "0x30 - CSIC DMA FIFO 2 Output Buffer-A Address Register"]
    #[inline(always)]
    pub const fn csic_dma_f2_bufa(&self) -> &CSIC_DMA_F2_BUFA {
        &self.csic_dma_f2_bufa
    }
    #[doc = "0x34 - CSIC DMA FIFO 2 Output Buffer-A Address Result Register"]
    #[inline(always)]
    pub const fn csic_dma_f2_bufa_result(&self) -> &CSIC_DMA_F2_BUFA_RESULT {
        &self.csic_dma_f2_bufa_result
    }
    #[doc = "0x38 - CSIC DMA Buffer Length Register"]
    #[inline(always)]
    pub const fn csic_dma_buf_len(&self) -> &CSIC_DMA_BUF_LEN {
        &self.csic_dma_buf_len
    }
    #[doc = "0x3c - CSIC DMA Flip Size Register"]
    #[inline(always)]
    pub const fn csic_dma_flip_size(&self) -> &CSIC_DMA_FLIP_SIZE {
        &self.csic_dma_flip_size
    }
    #[doc = "0x40 - CSIC DMA Video Input Timeout Threshold0 Register"]
    #[inline(always)]
    pub const fn csic_dma_vi_to_th0(&self) -> &CSIC_DMA_VI_TO_TH0 {
        &self.csic_dma_vi_to_th0
    }
    #[doc = "0x44 - CSIC DMA Video Input Timeout Threshold1 Register"]
    #[inline(always)]
    pub const fn csic_dma_vi_to_th1(&self) -> &CSIC_DMA_VI_TO_TH1 {
        &self.csic_dma_vi_to_th1
    }
    #[doc = "0x48 - CSIC DMA Video Input Timeout Counter Value Register"]
    #[inline(always)]
    pub const fn csic_dma_vi_to_cnt_val(&self) -> &CSIC_DMA_VI_TO_CNT_VAL {
        &self.csic_dma_vi_to_cnt_val
    }
    #[doc = "0x4c - CSIC DMA Capture Status Register"]
    #[inline(always)]
    pub const fn csic_dma_cap_sta(&self) -> &CSIC_DMA_CAP_STA {
        &self.csic_dma_cap_sta
    }
    #[doc = "0x50 - CSIC DMA Interrupt Enable Register"]
    #[inline(always)]
    pub const fn csic_dma_int_en(&self) -> &CSIC_DMA_INT_EN {
        &self.csic_dma_int_en
    }
    #[doc = "0x54 - CSIC DMA Interrupt Status Register"]
    #[inline(always)]
    pub const fn csic_dma_int_sta(&self) -> &CSIC_DMA_INT_STA {
        &self.csic_dma_int_sta
    }
    #[doc = "0x58 - CSIC DMA LINE Counter Register"]
    #[inline(always)]
    pub const fn csic_dma_line_cnt(&self) -> &CSIC_DMA_LINE_CNT {
        &self.csic_dma_line_cnt
    }
    #[doc = "0x5c - CSIC DMA Frame Counter Register"]
    #[inline(always)]
    pub const fn csic_dma_frm_cnt(&self) -> &CSIC_DMA_FRM_CNT {
        &self.csic_dma_frm_cnt
    }
    #[doc = "0x60 - CSIC DMA Frame Clock Counter Register"]
    #[inline(always)]
    pub const fn csic_dma_frm_clk_cnt(&self) -> &CSIC_DMA_FRM_CLK_CNT {
        &self.csic_dma_frm_clk_cnt
    }
    #[doc = "0x64 - CSIC DMA Accumulated And Internal Clock Counter Register"]
    #[inline(always)]
    pub const fn csic_dma_acc_itnl_clk_cnt(&self) -> &CSIC_DMA_ACC_ITNL_CLK_CNT {
        &self.csic_dma_acc_itnl_clk_cnt
    }
    #[doc = "0x68 - CSIC DMA FIFO Statistic Register"]
    #[inline(always)]
    pub const fn csic_dma_fifo_stat(&self) -> &CSIC_DMA_FIFO_STAT {
        &self.csic_dma_fifo_stat
    }
    #[doc = "0x6c - CSIC DMA FIFO Threshold Register"]
    #[inline(always)]
    pub const fn csic_dma_fifo_thrs(&self) -> &CSIC_DMA_FIFO_THRS {
        &self.csic_dma_fifo_thrs
    }
    #[doc = "0x70 - CSIC DMA PCLK Statistic Register"]
    #[inline(always)]
    pub const fn csic_dma_pclk_stat(&self) -> &CSIC_DMA_PCLK_STAT {
        &self.csic_dma_pclk_stat
    }
    #[doc = "0x80..0x8c - CSIC DMA BUF Address FIFO\\[i\\] Entry Register"]
    #[inline(always)]
    pub const fn csic_dma_buf_addr_fifo_entry(&self, n: usize) -> &CSIC_DMA_BUF_ADDR_FIFO_ENTRY {
        &self.csic_dma_buf_addr_fifo_entry[n]
    }
    #[doc = "0x80 - CSIC DMA BUF Address FIFO\\[i\\] Entry Register"]
    #[inline(always)]
    pub const fn csic_dma_buf_addr_fifo0_entry(&self) -> &CSIC_DMA_BUF_ADDR_FIFO_ENTRY {
        self.csic_dma_buf_addr_fifo_entry(0)
    }
    #[doc = "0x84 - CSIC DMA BUF Address FIFO\\[i\\] Entry Register"]
    #[inline(always)]
    pub const fn csic_dma_buf_addr_fifo1_entry(&self) -> &CSIC_DMA_BUF_ADDR_FIFO_ENTRY {
        self.csic_dma_buf_addr_fifo_entry(1)
    }
    #[doc = "0x88 - CSIC DMA BUF Address FIFO\\[i\\] Entry Register"]
    #[inline(always)]
    pub const fn csic_dma_buf_addr_fifo2_entry(&self) -> &CSIC_DMA_BUF_ADDR_FIFO_ENTRY {
        self.csic_dma_buf_addr_fifo_entry(2)
    }
    #[doc = "0x8c - CSIC DMA BUF Threshold Register"]
    #[inline(always)]
    pub const fn csic_dma_buf_th(&self) -> &CSIC_DMA_BUF_TH {
        &self.csic_dma_buf_th
    }
    #[doc = "0x90 - CSIC DMA BUF Address FIFO Content Register"]
    #[inline(always)]
    pub const fn csic_dma_buf_addr_fifo_con(&self) -> &CSIC_DMA_BUF_ADDR_FIFO_CON {
        &self.csic_dma_buf_addr_fifo_con
    }
    #[doc = "0x94 - CSIC DMA Stored Frame Counter Register"]
    #[inline(always)]
    pub const fn csic_dma_stored_frm_cnt(&self) -> &CSIC_DMA_STORED_FRM_CNT {
        &self.csic_dma_stored_frm_cnt
    }
    #[doc = "0x1f4 - CSIC DMA Feature List Register"]
    #[inline(always)]
    pub const fn csic_feature(&self) -> &CSIC_FEATURE {
        &self.csic_feature
    }
}
#[doc = "csic_dma_en (rw) register accessor: CSIC DMA Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_dma_en`] module"]
pub type CSIC_DMA_EN = crate::Reg<csic_dma_en::CSIC_DMA_EN_SPEC>;
#[doc = "CSIC DMA Enable Register"]
pub mod csic_dma_en;
#[doc = "csic_dma_cfg (rw) register accessor: CSIC DMA Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_dma_cfg`] module"]
pub type CSIC_DMA_CFG = crate::Reg<csic_dma_cfg::CSIC_DMA_CFG_SPEC>;
#[doc = "CSIC DMA Configuration Register"]
pub mod csic_dma_cfg;
#[doc = "csic_dma_hsize (rw) register accessor: CSIC DMA Horizontal Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_hsize::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_hsize::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_dma_hsize`] module"]
pub type CSIC_DMA_HSIZE = crate::Reg<csic_dma_hsize::CSIC_DMA_HSIZE_SPEC>;
#[doc = "CSIC DMA Horizontal Size Register"]
pub mod csic_dma_hsize;
#[doc = "csic_dma_vsize (rw) register accessor: CSIC DMA Vertical Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_vsize::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_vsize::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_dma_vsize`] module"]
pub type CSIC_DMA_VSIZE = crate::Reg<csic_dma_vsize::CSIC_DMA_VSIZE_SPEC>;
#[doc = "CSIC DMA Vertical Size Register"]
pub mod csic_dma_vsize;
#[doc = "csic_dma_f0_bufa (rw) register accessor: CSIC DMA FIFO 0 Output Buffer-A Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_f0_bufa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_f0_bufa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_dma_f0_bufa`] module"]
pub type CSIC_DMA_F0_BUFA = crate::Reg<csic_dma_f0_bufa::CSIC_DMA_F0_BUFA_SPEC>;
#[doc = "CSIC DMA FIFO 0 Output Buffer-A Address Register"]
pub mod csic_dma_f0_bufa;
#[doc = "csic_dma_f0_bufa_result (rw) register accessor: CSIC DMA FIFO 0 Output Buffer-A Address Result Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_f0_bufa_result::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_f0_bufa_result::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_dma_f0_bufa_result`] module"]
pub type CSIC_DMA_F0_BUFA_RESULT =
    crate::Reg<csic_dma_f0_bufa_result::CSIC_DMA_F0_BUFA_RESULT_SPEC>;
#[doc = "CSIC DMA FIFO 0 Output Buffer-A Address Result Register"]
pub mod csic_dma_f0_bufa_result;
#[doc = "csic_dma_f1_bufa (rw) register accessor: CSIC DMA FIFO 1 Output Buffer-A Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_f1_bufa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_f1_bufa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_dma_f1_bufa`] module"]
pub type CSIC_DMA_F1_BUFA = crate::Reg<csic_dma_f1_bufa::CSIC_DMA_F1_BUFA_SPEC>;
#[doc = "CSIC DMA FIFO 1 Output Buffer-A Address Register"]
pub mod csic_dma_f1_bufa;
#[doc = "csic_dma_f1_bufa_result (rw) register accessor: CSIC DMA FIFO 1 Output Buffer-A Address Result Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_f1_bufa_result::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_f1_bufa_result::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_dma_f1_bufa_result`] module"]
pub type CSIC_DMA_F1_BUFA_RESULT =
    crate::Reg<csic_dma_f1_bufa_result::CSIC_DMA_F1_BUFA_RESULT_SPEC>;
#[doc = "CSIC DMA FIFO 1 Output Buffer-A Address Result Register"]
pub mod csic_dma_f1_bufa_result;
#[doc = "csic_dma_f2_bufa (rw) register accessor: CSIC DMA FIFO 2 Output Buffer-A Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_f2_bufa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_f2_bufa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_dma_f2_bufa`] module"]
pub type CSIC_DMA_F2_BUFA = crate::Reg<csic_dma_f2_bufa::CSIC_DMA_F2_BUFA_SPEC>;
#[doc = "CSIC DMA FIFO 2 Output Buffer-A Address Register"]
pub mod csic_dma_f2_bufa;
#[doc = "csic_dma_f2_bufa_result (rw) register accessor: CSIC DMA FIFO 2 Output Buffer-A Address Result Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_f2_bufa_result::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_f2_bufa_result::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_dma_f2_bufa_result`] module"]
pub type CSIC_DMA_F2_BUFA_RESULT =
    crate::Reg<csic_dma_f2_bufa_result::CSIC_DMA_F2_BUFA_RESULT_SPEC>;
#[doc = "CSIC DMA FIFO 2 Output Buffer-A Address Result Register"]
pub mod csic_dma_f2_bufa_result;
#[doc = "csic_dma_buf_len (rw) register accessor: CSIC DMA Buffer Length Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_buf_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_buf_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_dma_buf_len`] module"]
pub type CSIC_DMA_BUF_LEN = crate::Reg<csic_dma_buf_len::CSIC_DMA_BUF_LEN_SPEC>;
#[doc = "CSIC DMA Buffer Length Register"]
pub mod csic_dma_buf_len;
#[doc = "csic_dma_flip_size (rw) register accessor: CSIC DMA Flip Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_flip_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_flip_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_dma_flip_size`] module"]
pub type CSIC_DMA_FLIP_SIZE = crate::Reg<csic_dma_flip_size::CSIC_DMA_FLIP_SIZE_SPEC>;
#[doc = "CSIC DMA Flip Size Register"]
pub mod csic_dma_flip_size;
#[doc = "csic_dma_vi_to_th0 (rw) register accessor: CSIC DMA Video Input Timeout Threshold0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_vi_to_th0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_vi_to_th0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_dma_vi_to_th0`] module"]
pub type CSIC_DMA_VI_TO_TH0 = crate::Reg<csic_dma_vi_to_th0::CSIC_DMA_VI_TO_TH0_SPEC>;
#[doc = "CSIC DMA Video Input Timeout Threshold0 Register"]
pub mod csic_dma_vi_to_th0;
#[doc = "csic_dma_vi_to_th1 (rw) register accessor: CSIC DMA Video Input Timeout Threshold1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_vi_to_th1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_vi_to_th1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_dma_vi_to_th1`] module"]
pub type CSIC_DMA_VI_TO_TH1 = crate::Reg<csic_dma_vi_to_th1::CSIC_DMA_VI_TO_TH1_SPEC>;
#[doc = "CSIC DMA Video Input Timeout Threshold1 Register"]
pub mod csic_dma_vi_to_th1;
#[doc = "csic_dma_vi_to_cnt_val (rw) register accessor: CSIC DMA Video Input Timeout Counter Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_vi_to_cnt_val::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_vi_to_cnt_val::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_dma_vi_to_cnt_val`] module"]
pub type CSIC_DMA_VI_TO_CNT_VAL = crate::Reg<csic_dma_vi_to_cnt_val::CSIC_DMA_VI_TO_CNT_VAL_SPEC>;
#[doc = "CSIC DMA Video Input Timeout Counter Value Register"]
pub mod csic_dma_vi_to_cnt_val;
#[doc = "csic_dma_cap_sta (rw) register accessor: CSIC DMA Capture Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_cap_sta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_cap_sta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_dma_cap_sta`] module"]
pub type CSIC_DMA_CAP_STA = crate::Reg<csic_dma_cap_sta::CSIC_DMA_CAP_STA_SPEC>;
#[doc = "CSIC DMA Capture Status Register"]
pub mod csic_dma_cap_sta;
#[doc = "csic_dma_int_en (rw) register accessor: CSIC DMA Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_int_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_int_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_dma_int_en`] module"]
pub type CSIC_DMA_INT_EN = crate::Reg<csic_dma_int_en::CSIC_DMA_INT_EN_SPEC>;
#[doc = "CSIC DMA Interrupt Enable Register"]
pub mod csic_dma_int_en;
#[doc = "csic_dma_int_sta (rw) register accessor: CSIC DMA Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_int_sta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_int_sta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_dma_int_sta`] module"]
pub type CSIC_DMA_INT_STA = crate::Reg<csic_dma_int_sta::CSIC_DMA_INT_STA_SPEC>;
#[doc = "CSIC DMA Interrupt Status Register"]
pub mod csic_dma_int_sta;
#[doc = "csic_dma_line_cnt (rw) register accessor: CSIC DMA LINE Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_line_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_line_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_dma_line_cnt`] module"]
pub type CSIC_DMA_LINE_CNT = crate::Reg<csic_dma_line_cnt::CSIC_DMA_LINE_CNT_SPEC>;
#[doc = "CSIC DMA LINE Counter Register"]
pub mod csic_dma_line_cnt;
#[doc = "csic_dma_frm_cnt (rw) register accessor: CSIC DMA Frame Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_frm_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_frm_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_dma_frm_cnt`] module"]
pub type CSIC_DMA_FRM_CNT = crate::Reg<csic_dma_frm_cnt::CSIC_DMA_FRM_CNT_SPEC>;
#[doc = "CSIC DMA Frame Counter Register"]
pub mod csic_dma_frm_cnt;
#[doc = "csic_dma_frm_clk_cnt (rw) register accessor: CSIC DMA Frame Clock Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_frm_clk_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_frm_clk_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_dma_frm_clk_cnt`] module"]
pub type CSIC_DMA_FRM_CLK_CNT = crate::Reg<csic_dma_frm_clk_cnt::CSIC_DMA_FRM_CLK_CNT_SPEC>;
#[doc = "CSIC DMA Frame Clock Counter Register"]
pub mod csic_dma_frm_clk_cnt;
#[doc = "csic_dma_acc_itnl_clk_cnt (rw) register accessor: CSIC DMA Accumulated And Internal Clock Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_acc_itnl_clk_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_acc_itnl_clk_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_dma_acc_itnl_clk_cnt`] module"]
pub type CSIC_DMA_ACC_ITNL_CLK_CNT =
    crate::Reg<csic_dma_acc_itnl_clk_cnt::CSIC_DMA_ACC_ITNL_CLK_CNT_SPEC>;
#[doc = "CSIC DMA Accumulated And Internal Clock Counter Register"]
pub mod csic_dma_acc_itnl_clk_cnt;
#[doc = "csic_dma_fifo_stat (rw) register accessor: CSIC DMA FIFO Statistic Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_fifo_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_fifo_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_dma_fifo_stat`] module"]
pub type CSIC_DMA_FIFO_STAT = crate::Reg<csic_dma_fifo_stat::CSIC_DMA_FIFO_STAT_SPEC>;
#[doc = "CSIC DMA FIFO Statistic Register"]
pub mod csic_dma_fifo_stat;
#[doc = "csic_dma_fifo_thrs (rw) register accessor: CSIC DMA FIFO Threshold Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_fifo_thrs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_fifo_thrs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_dma_fifo_thrs`] module"]
pub type CSIC_DMA_FIFO_THRS = crate::Reg<csic_dma_fifo_thrs::CSIC_DMA_FIFO_THRS_SPEC>;
#[doc = "CSIC DMA FIFO Threshold Register"]
pub mod csic_dma_fifo_thrs;
#[doc = "csic_dma_pclk_stat (rw) register accessor: CSIC DMA PCLK Statistic Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_pclk_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_pclk_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_dma_pclk_stat`] module"]
pub type CSIC_DMA_PCLK_STAT = crate::Reg<csic_dma_pclk_stat::CSIC_DMA_PCLK_STAT_SPEC>;
#[doc = "CSIC DMA PCLK Statistic Register"]
pub mod csic_dma_pclk_stat;
#[doc = "csic_dma_buf_addr_fifo_entry (rw) register accessor: CSIC DMA BUF Address FIFO\\[i\\] Entry Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_buf_addr_fifo_entry::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_buf_addr_fifo_entry::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_dma_buf_addr_fifo_entry`] module"]
pub type CSIC_DMA_BUF_ADDR_FIFO_ENTRY =
    crate::Reg<csic_dma_buf_addr_fifo_entry::CSIC_DMA_BUF_ADDR_FIFO_ENTRY_SPEC>;
#[doc = "CSIC DMA BUF Address FIFO\\[i\\] Entry Register"]
pub mod csic_dma_buf_addr_fifo_entry;
#[doc = "csic_dma_buf_th (rw) register accessor: CSIC DMA BUF Threshold Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_buf_th::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_buf_th::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_dma_buf_th`] module"]
pub type CSIC_DMA_BUF_TH = crate::Reg<csic_dma_buf_th::CSIC_DMA_BUF_TH_SPEC>;
#[doc = "CSIC DMA BUF Threshold Register"]
pub mod csic_dma_buf_th;
#[doc = "csic_dma_buf_addr_fifo_con (rw) register accessor: CSIC DMA BUF Address FIFO Content Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_buf_addr_fifo_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_buf_addr_fifo_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_dma_buf_addr_fifo_con`] module"]
pub type CSIC_DMA_BUF_ADDR_FIFO_CON =
    crate::Reg<csic_dma_buf_addr_fifo_con::CSIC_DMA_BUF_ADDR_FIFO_CON_SPEC>;
#[doc = "CSIC DMA BUF Address FIFO Content Register"]
pub mod csic_dma_buf_addr_fifo_con;
#[doc = "csic_dma_stored_frm_cnt (rw) register accessor: CSIC DMA Stored Frame Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_stored_frm_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_stored_frm_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_dma_stored_frm_cnt`] module"]
pub type CSIC_DMA_STORED_FRM_CNT =
    crate::Reg<csic_dma_stored_frm_cnt::CSIC_DMA_STORED_FRM_CNT_SPEC>;
#[doc = "CSIC DMA Stored Frame Counter Register"]
pub mod csic_dma_stored_frm_cnt;
#[doc = "csic_feature (rw) register accessor: CSIC DMA Feature List Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_feature::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_feature::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_feature`] module"]
pub type CSIC_FEATURE = crate::Reg<csic_feature::CSIC_FEATURE_SPEC>;
#[doc = "CSIC DMA Feature List Register"]
pub mod csic_feature;
