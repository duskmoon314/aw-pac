#[doc = r"Register block"]
#[repr(C)]
pub struct CSIC_PARSER0 {
    prs_en: PRS_EN,
    prs_ncsic_if_cfg: PRS_NCSIC_IF_CFG,
    _reserved2: [u8; 0x04],
    prs_cap: PRS_CAP,
    csic_prs_signal_sta: CSIC_PRS_SIGNAL_STA,
    csic_prs_ncsic_bt656_head_cfg: CSIC_PRS_NCSIC_BT656_HEAD_CFG,
    _reserved5: [u8; 0x0c],
    prs_ch_infmt: (),
    _reserved6: [u8; 0x04],
    prs_ch_output_hsize: (),
    _reserved7: [u8; 0x04],
    prs_ch_output_vsize: (),
    _reserved8: [u8; 0x04],
    prs_ch_input_para0: (),
    _reserved9: [u8; 0x04],
    prs_ch_input_para1: (),
    _reserved10: [u8; 0x04],
    prs_ch_input_para2: (),
    _reserved11: [u8; 0x04],
    prs_ch_input_para3: (),
    _reserved12: [u8; 0x04],
    prs_ch_int_en: (),
    _reserved13: [u8; 0x04],
    prs_ch_int_sta: (),
    _reserved14: [u8; 0x04],
    prs_ch0_line_time: (),
    _reserved15: [u8; 0x04b8],
    csic_prs_ncsic_rx_signal0_dly_adj: CSIC_PRS_NCSIC_RX_SIGNAL0_DLY_ADJ,
    _reserved16: [u8; 0x10],
    csic_prs_ncsic_rx_signal5_dly_adj: CSIC_PRS_NCSIC_RX_SIGNAL5_DLY_ADJ,
    csic_prs_ncsic_rx_signal6_dly_adj: CSIC_PRS_NCSIC_RX_SIGNAL6_DLY_ADJ,
}
impl CSIC_PARSER0 {
    #[doc = "0x00 - Parser Enable Register"]
    #[inline(always)]
    pub const fn prs_en(&self) -> &PRS_EN {
        &self.prs_en
    }
    #[doc = "0x04 - Parser NCSIC Interface Configuration Register"]
    #[inline(always)]
    pub const fn prs_ncsic_if_cfg(&self) -> &PRS_NCSIC_IF_CFG {
        &self.prs_ncsic_if_cfg
    }
    #[doc = "0x0c - Parser Capture Register"]
    #[inline(always)]
    pub const fn prs_cap(&self) -> &PRS_CAP {
        &self.prs_cap
    }
    #[doc = "0x10 - CSIC Parser Signal Status Register"]
    #[inline(always)]
    pub const fn csic_prs_signal_sta(&self) -> &CSIC_PRS_SIGNAL_STA {
        &self.csic_prs_signal_sta
    }
    #[doc = "0x14 - CSIC Parser NCSIC BT656 Header Configuration Register"]
    #[inline(always)]
    pub const fn csic_prs_ncsic_bt656_head_cfg(&self) -> &CSIC_PRS_NCSIC_BT656_HEAD_CFG {
        &self.csic_prs_ncsic_bt656_head_cfg
    }
    #[doc = "0x24..0x34 - Parser Channel\\[i\\] Input Format Register"]
    #[inline(always)]
    pub const fn prs_ch_infmt(&self, n: usize) -> &PRS_CH_INFMT {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(36)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "0x24 - Parser Channel\\[i\\] Input Format Register"]
    #[inline(always)]
    pub const fn prs_ch0_infmt(&self) -> &PRS_CH_INFMT {
        self.prs_ch_infmt(0)
    }
    #[doc = "0x124 - Parser Channel\\[i\\] Input Format Register"]
    #[inline(always)]
    pub const fn prs_ch1_infmt(&self) -> &PRS_CH_INFMT {
        self.prs_ch_infmt(1)
    }
    #[doc = "0x224 - Parser Channel\\[i\\] Input Format Register"]
    #[inline(always)]
    pub const fn prs_ch2_infmt(&self) -> &PRS_CH_INFMT {
        self.prs_ch_infmt(2)
    }
    #[doc = "0x324 - Parser Channel\\[i\\] Input Format Register"]
    #[inline(always)]
    pub const fn prs_ch3_infmt(&self) -> &PRS_CH_INFMT {
        self.prs_ch_infmt(3)
    }
    #[doc = "0x28..0x38 - Parser Channel\\[i\\] Output Horizontal Size Register"]
    #[inline(always)]
    pub const fn prs_ch_output_hsize(&self, n: usize) -> &PRS_CH_OUTPUT_HSIZE {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(40)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "0x28 - Parser Channel\\[i\\] Output Horizontal Size Register"]
    #[inline(always)]
    pub const fn prs_ch0_output_hsize(&self) -> &PRS_CH_OUTPUT_HSIZE {
        self.prs_ch_output_hsize(0)
    }
    #[doc = "0x128 - Parser Channel\\[i\\] Output Horizontal Size Register"]
    #[inline(always)]
    pub const fn prs_ch1_output_hsize(&self) -> &PRS_CH_OUTPUT_HSIZE {
        self.prs_ch_output_hsize(1)
    }
    #[doc = "0x228 - Parser Channel\\[i\\] Output Horizontal Size Register"]
    #[inline(always)]
    pub const fn prs_ch2_output_hsize(&self) -> &PRS_CH_OUTPUT_HSIZE {
        self.prs_ch_output_hsize(2)
    }
    #[doc = "0x328 - Parser Channel\\[i\\] Output Horizontal Size Register"]
    #[inline(always)]
    pub const fn prs_ch3_output_hsize(&self) -> &PRS_CH_OUTPUT_HSIZE {
        self.prs_ch_output_hsize(3)
    }
    #[doc = "0x2c..0x3c - Parser Channel\\[i\\] Output Vertical Size Register"]
    #[inline(always)]
    pub const fn prs_ch_output_vsize(&self, n: usize) -> &PRS_CH_OUTPUT_VSIZE {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(44)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "0x2c - Parser Channel\\[i\\] Output Vertical Size Register"]
    #[inline(always)]
    pub const fn prs_ch0_output_vsize(&self) -> &PRS_CH_OUTPUT_VSIZE {
        self.prs_ch_output_vsize(0)
    }
    #[doc = "0x12c - Parser Channel\\[i\\] Output Vertical Size Register"]
    #[inline(always)]
    pub const fn prs_ch1_output_vsize(&self) -> &PRS_CH_OUTPUT_VSIZE {
        self.prs_ch_output_vsize(1)
    }
    #[doc = "0x22c - Parser Channel\\[i\\] Output Vertical Size Register"]
    #[inline(always)]
    pub const fn prs_ch2_output_vsize(&self) -> &PRS_CH_OUTPUT_VSIZE {
        self.prs_ch_output_vsize(2)
    }
    #[doc = "0x32c - Parser Channel\\[i\\] Output Vertical Size Register"]
    #[inline(always)]
    pub const fn prs_ch3_output_vsize(&self) -> &PRS_CH_OUTPUT_VSIZE {
        self.prs_ch_output_vsize(3)
    }
    #[doc = "0x30..0x40 - Parser Channel\\[i\\] Input Parameter0 Register"]
    #[inline(always)]
    pub const fn prs_ch_input_para0(&self, n: usize) -> &PRS_CH_INPUT_PARA0 {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(48)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "0x30 - Parser Channel\\[i\\] Input Parameter0 Register"]
    #[inline(always)]
    pub const fn prs_ch0_input_para0(&self) -> &PRS_CH_INPUT_PARA0 {
        self.prs_ch_input_para0(0)
    }
    #[doc = "0x130 - Parser Channel\\[i\\] Input Parameter0 Register"]
    #[inline(always)]
    pub const fn prs_ch1_input_para0(&self) -> &PRS_CH_INPUT_PARA0 {
        self.prs_ch_input_para0(1)
    }
    #[doc = "0x230 - Parser Channel\\[i\\] Input Parameter0 Register"]
    #[inline(always)]
    pub const fn prs_ch2_input_para0(&self) -> &PRS_CH_INPUT_PARA0 {
        self.prs_ch_input_para0(2)
    }
    #[doc = "0x330 - Parser Channel\\[i\\] Input Parameter0 Register"]
    #[inline(always)]
    pub const fn prs_ch3_input_para0(&self) -> &PRS_CH_INPUT_PARA0 {
        self.prs_ch_input_para0(3)
    }
    #[doc = "0x34..0x44 - Parser Channel\\[i\\] Input Parameter1 Register"]
    #[inline(always)]
    pub const fn prs_ch_input_para1(&self, n: usize) -> &PRS_CH_INPUT_PARA1 {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(52)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "0x34 - Parser Channel\\[i\\] Input Parameter1 Register"]
    #[inline(always)]
    pub const fn prs_ch0_input_para1(&self) -> &PRS_CH_INPUT_PARA1 {
        self.prs_ch_input_para1(0)
    }
    #[doc = "0x134 - Parser Channel\\[i\\] Input Parameter1 Register"]
    #[inline(always)]
    pub const fn prs_ch1_input_para1(&self) -> &PRS_CH_INPUT_PARA1 {
        self.prs_ch_input_para1(1)
    }
    #[doc = "0x234 - Parser Channel\\[i\\] Input Parameter1 Register"]
    #[inline(always)]
    pub const fn prs_ch2_input_para1(&self) -> &PRS_CH_INPUT_PARA1 {
        self.prs_ch_input_para1(2)
    }
    #[doc = "0x334 - Parser Channel\\[i\\] Input Parameter1 Register"]
    #[inline(always)]
    pub const fn prs_ch3_input_para1(&self) -> &PRS_CH_INPUT_PARA1 {
        self.prs_ch_input_para1(3)
    }
    #[doc = "0x38..0x48 - Parser Channel\\[i\\] Input Parameter2 Register"]
    #[inline(always)]
    pub const fn prs_ch_input_para2(&self, n: usize) -> &PRS_CH_INPUT_PARA2 {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(56)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "0x38 - Parser Channel\\[i\\] Input Parameter2 Register"]
    #[inline(always)]
    pub const fn prs_ch0_input_para2(&self) -> &PRS_CH_INPUT_PARA2 {
        self.prs_ch_input_para2(0)
    }
    #[doc = "0x138 - Parser Channel\\[i\\] Input Parameter2 Register"]
    #[inline(always)]
    pub const fn prs_ch1_input_para2(&self) -> &PRS_CH_INPUT_PARA2 {
        self.prs_ch_input_para2(1)
    }
    #[doc = "0x238 - Parser Channel\\[i\\] Input Parameter2 Register"]
    #[inline(always)]
    pub const fn prs_ch2_input_para2(&self) -> &PRS_CH_INPUT_PARA2 {
        self.prs_ch_input_para2(2)
    }
    #[doc = "0x338 - Parser Channel\\[i\\] Input Parameter2 Register"]
    #[inline(always)]
    pub const fn prs_ch3_input_para2(&self) -> &PRS_CH_INPUT_PARA2 {
        self.prs_ch_input_para2(3)
    }
    #[doc = "0x3c..0x4c - Parser Channel\\[i\\] Input Parameter3 Register"]
    #[inline(always)]
    pub const fn prs_ch_input_para3(&self, n: usize) -> &PRS_CH_INPUT_PARA3 {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(60)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "0x3c - Parser Channel\\[i\\] Input Parameter3 Register"]
    #[inline(always)]
    pub const fn prs_ch0_input_para3(&self) -> &PRS_CH_INPUT_PARA3 {
        self.prs_ch_input_para3(0)
    }
    #[doc = "0x13c - Parser Channel\\[i\\] Input Parameter3 Register"]
    #[inline(always)]
    pub const fn prs_ch1_input_para3(&self) -> &PRS_CH_INPUT_PARA3 {
        self.prs_ch_input_para3(1)
    }
    #[doc = "0x23c - Parser Channel\\[i\\] Input Parameter3 Register"]
    #[inline(always)]
    pub const fn prs_ch2_input_para3(&self) -> &PRS_CH_INPUT_PARA3 {
        self.prs_ch_input_para3(2)
    }
    #[doc = "0x33c - Parser Channel\\[i\\] Input Parameter3 Register"]
    #[inline(always)]
    pub const fn prs_ch3_input_para3(&self) -> &PRS_CH_INPUT_PARA3 {
        self.prs_ch_input_para3(3)
    }
    #[doc = "0x40..0x50 - Parser Channel\\[i\\] Interrupt Enable Register"]
    #[inline(always)]
    pub const fn prs_ch_int_en(&self, n: usize) -> &PRS_CH_INT_EN {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(64)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "0x40 - Parser Channel\\[i\\] Interrupt Enable Register"]
    #[inline(always)]
    pub const fn prs_ch0_int_en(&self) -> &PRS_CH_INT_EN {
        self.prs_ch_int_en(0)
    }
    #[doc = "0x140 - Parser Channel\\[i\\] Interrupt Enable Register"]
    #[inline(always)]
    pub const fn prs_ch1_int_en(&self) -> &PRS_CH_INT_EN {
        self.prs_ch_int_en(1)
    }
    #[doc = "0x240 - Parser Channel\\[i\\] Interrupt Enable Register"]
    #[inline(always)]
    pub const fn prs_ch2_int_en(&self) -> &PRS_CH_INT_EN {
        self.prs_ch_int_en(2)
    }
    #[doc = "0x340 - Parser Channel\\[i\\] Interrupt Enable Register"]
    #[inline(always)]
    pub const fn prs_ch3_int_en(&self) -> &PRS_CH_INT_EN {
        self.prs_ch_int_en(3)
    }
    #[doc = "0x44..0x54 - Parser Channel\\[i\\] Interrupt Status Register"]
    #[inline(always)]
    pub const fn prs_ch_int_sta(&self, n: usize) -> &PRS_CH_INT_STA {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(68)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "0x44 - Parser Channel\\[i\\] Interrupt Status Register"]
    #[inline(always)]
    pub const fn prs_ch0_int_sta(&self) -> &PRS_CH_INT_STA {
        self.prs_ch_int_sta(0)
    }
    #[doc = "0x144 - Parser Channel\\[i\\] Interrupt Status Register"]
    #[inline(always)]
    pub const fn prs_ch1_int_sta(&self) -> &PRS_CH_INT_STA {
        self.prs_ch_int_sta(1)
    }
    #[doc = "0x244 - Parser Channel\\[i\\] Interrupt Status Register"]
    #[inline(always)]
    pub const fn prs_ch2_int_sta(&self) -> &PRS_CH_INT_STA {
        self.prs_ch_int_sta(2)
    }
    #[doc = "0x344 - Parser Channel\\[i\\] Interrupt Status Register"]
    #[inline(always)]
    pub const fn prs_ch3_int_sta(&self) -> &PRS_CH_INT_STA {
        self.prs_ch_int_sta(3)
    }
    #[doc = "0x48..0x58 - Parser Channel\\[i\\] Line Time Register"]
    #[inline(always)]
    pub const fn prs_ch0_line_time(&self, n: usize) -> &PRS_CH0_LINE_TIME {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(72)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "0x48 - Parser Channel\\[i\\] Line Time Register"]
    #[inline(always)]
    pub const fn prs_ch00_line_time(&self) -> &PRS_CH0_LINE_TIME {
        self.prs_ch0_line_time(0)
    }
    #[doc = "0x148 - Parser Channel\\[i\\] Line Time Register"]
    #[inline(always)]
    pub const fn prs_ch10_line_time(&self) -> &PRS_CH0_LINE_TIME {
        self.prs_ch0_line_time(1)
    }
    #[doc = "0x248 - Parser Channel\\[i\\] Line Time Register"]
    #[inline(always)]
    pub const fn prs_ch20_line_time(&self) -> &PRS_CH0_LINE_TIME {
        self.prs_ch0_line_time(2)
    }
    #[doc = "0x348 - Parser Channel\\[i\\] Line Time Register"]
    #[inline(always)]
    pub const fn prs_ch30_line_time(&self) -> &PRS_CH0_LINE_TIME {
        self.prs_ch0_line_time(3)
    }
    #[doc = "0x500 - CSIC Parser NCSIC RX Signal0 Delay Adjust Register"]
    #[inline(always)]
    pub const fn csic_prs_ncsic_rx_signal0_dly_adj(&self) -> &CSIC_PRS_NCSIC_RX_SIGNAL0_DLY_ADJ {
        &self.csic_prs_ncsic_rx_signal0_dly_adj
    }
    #[doc = "0x514 - CSIC Parser NCSIC RX Signal5 Delay Adjust Register"]
    #[inline(always)]
    pub const fn csic_prs_ncsic_rx_signal5_dly_adj(&self) -> &CSIC_PRS_NCSIC_RX_SIGNAL5_DLY_ADJ {
        &self.csic_prs_ncsic_rx_signal5_dly_adj
    }
    #[doc = "0x518 - CSIC Parser NCSIC RX Signal6 Delay Adjust Register"]
    #[inline(always)]
    pub const fn csic_prs_ncsic_rx_signal6_dly_adj(&self) -> &CSIC_PRS_NCSIC_RX_SIGNAL6_DLY_ADJ {
        &self.csic_prs_ncsic_rx_signal6_dly_adj
    }
}
#[doc = "prs_en (rw) register accessor: Parser Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prs_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prs_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs_en`] module"]
pub type PRS_EN = crate::Reg<prs_en::PRS_EN_SPEC>;
#[doc = "Parser Enable Register"]
pub mod prs_en;
#[doc = "prs_ncsic_if_cfg (rw) register accessor: Parser NCSIC Interface Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prs_ncsic_if_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prs_ncsic_if_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs_ncsic_if_cfg`] module"]
pub type PRS_NCSIC_IF_CFG = crate::Reg<prs_ncsic_if_cfg::PRS_NCSIC_IF_CFG_SPEC>;
#[doc = "Parser NCSIC Interface Configuration Register"]
pub mod prs_ncsic_if_cfg;
#[doc = "prs_cap (rw) register accessor: Parser Capture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prs_cap::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prs_cap::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs_cap`] module"]
pub type PRS_CAP = crate::Reg<prs_cap::PRS_CAP_SPEC>;
#[doc = "Parser Capture Register"]
pub mod prs_cap;
#[doc = "csic_prs_signal_sta (rw) register accessor: CSIC Parser Signal Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_prs_signal_sta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_prs_signal_sta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_prs_signal_sta`] module"]
pub type CSIC_PRS_SIGNAL_STA = crate::Reg<csic_prs_signal_sta::CSIC_PRS_SIGNAL_STA_SPEC>;
#[doc = "CSIC Parser Signal Status Register"]
pub mod csic_prs_signal_sta;
#[doc = "csic_prs_ncsic_bt656_head_cfg (rw) register accessor: CSIC Parser NCSIC BT656 Header Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_prs_ncsic_bt656_head_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_prs_ncsic_bt656_head_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_prs_ncsic_bt656_head_cfg`] module"]
pub type CSIC_PRS_NCSIC_BT656_HEAD_CFG =
    crate::Reg<csic_prs_ncsic_bt656_head_cfg::CSIC_PRS_NCSIC_BT656_HEAD_CFG_SPEC>;
#[doc = "CSIC Parser NCSIC BT656 Header Configuration Register"]
pub mod csic_prs_ncsic_bt656_head_cfg;
#[doc = "prs_ch_infmt (rw) register accessor: Parser Channel\\[i\\] Input Format Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prs_ch_infmt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prs_ch_infmt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs_ch_infmt`] module"]
pub type PRS_CH_INFMT = crate::Reg<prs_ch_infmt::PRS_CH_INFMT_SPEC>;
#[doc = "Parser Channel\\[i\\] Input Format Register"]
pub mod prs_ch_infmt;
#[doc = "prs_ch_output_hsize (rw) register accessor: Parser Channel\\[i\\] Output Horizontal Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prs_ch_output_hsize::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prs_ch_output_hsize::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs_ch_output_hsize`] module"]
pub type PRS_CH_OUTPUT_HSIZE = crate::Reg<prs_ch_output_hsize::PRS_CH_OUTPUT_HSIZE_SPEC>;
#[doc = "Parser Channel\\[i\\] Output Horizontal Size Register"]
pub mod prs_ch_output_hsize;
#[doc = "prs_ch_output_vsize (rw) register accessor: Parser Channel\\[i\\] Output Vertical Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prs_ch_output_vsize::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prs_ch_output_vsize::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs_ch_output_vsize`] module"]
pub type PRS_CH_OUTPUT_VSIZE = crate::Reg<prs_ch_output_vsize::PRS_CH_OUTPUT_VSIZE_SPEC>;
#[doc = "Parser Channel\\[i\\] Output Vertical Size Register"]
pub mod prs_ch_output_vsize;
#[doc = "prs_ch_input_para0 (r) register accessor: Parser Channel\\[i\\] Input Parameter0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prs_ch_input_para0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs_ch_input_para0`] module"]
pub type PRS_CH_INPUT_PARA0 = crate::Reg<prs_ch_input_para0::PRS_CH_INPUT_PARA0_SPEC>;
#[doc = "Parser Channel\\[i\\] Input Parameter0 Register"]
pub mod prs_ch_input_para0;
#[doc = "prs_ch_input_para1 (r) register accessor: Parser Channel\\[i\\] Input Parameter1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prs_ch_input_para1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs_ch_input_para1`] module"]
pub type PRS_CH_INPUT_PARA1 = crate::Reg<prs_ch_input_para1::PRS_CH_INPUT_PARA1_SPEC>;
#[doc = "Parser Channel\\[i\\] Input Parameter1 Register"]
pub mod prs_ch_input_para1;
#[doc = "prs_ch_input_para2 (r) register accessor: Parser Channel\\[i\\] Input Parameter2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prs_ch_input_para2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs_ch_input_para2`] module"]
pub type PRS_CH_INPUT_PARA2 = crate::Reg<prs_ch_input_para2::PRS_CH_INPUT_PARA2_SPEC>;
#[doc = "Parser Channel\\[i\\] Input Parameter2 Register"]
pub mod prs_ch_input_para2;
#[doc = "prs_ch_input_para3 (r) register accessor: Parser Channel\\[i\\] Input Parameter3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prs_ch_input_para3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs_ch_input_para3`] module"]
pub type PRS_CH_INPUT_PARA3 = crate::Reg<prs_ch_input_para3::PRS_CH_INPUT_PARA3_SPEC>;
#[doc = "Parser Channel\\[i\\] Input Parameter3 Register"]
pub mod prs_ch_input_para3;
#[doc = "prs_ch_int_en (rw) register accessor: Parser Channel\\[i\\] Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prs_ch_int_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prs_ch_int_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs_ch_int_en`] module"]
pub type PRS_CH_INT_EN = crate::Reg<prs_ch_int_en::PRS_CH_INT_EN_SPEC>;
#[doc = "Parser Channel\\[i\\] Interrupt Enable Register"]
pub mod prs_ch_int_en;
#[doc = "prs_ch_int_sta (rw) register accessor: Parser Channel\\[i\\] Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prs_ch_int_sta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prs_ch_int_sta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs_ch_int_sta`] module"]
pub type PRS_CH_INT_STA = crate::Reg<prs_ch_int_sta::PRS_CH_INT_STA_SPEC>;
#[doc = "Parser Channel\\[i\\] Interrupt Status Register"]
pub mod prs_ch_int_sta;
#[doc = "prs_ch0_line_time (r) register accessor: Parser Channel\\[i\\] Line Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prs_ch0_line_time::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs_ch0_line_time`] module"]
pub type PRS_CH0_LINE_TIME = crate::Reg<prs_ch0_line_time::PRS_CH0_LINE_TIME_SPEC>;
#[doc = "Parser Channel\\[i\\] Line Time Register"]
pub mod prs_ch0_line_time;
#[doc = "csic_prs_ncsic_rx_signal0_dly_adj (rw) register accessor: CSIC Parser NCSIC RX Signal0 Delay Adjust Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_prs_ncsic_rx_signal0_dly_adj::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_prs_ncsic_rx_signal0_dly_adj::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_prs_ncsic_rx_signal0_dly_adj`] module"]
pub type CSIC_PRS_NCSIC_RX_SIGNAL0_DLY_ADJ =
    crate::Reg<csic_prs_ncsic_rx_signal0_dly_adj::CSIC_PRS_NCSIC_RX_SIGNAL0_DLY_ADJ_SPEC>;
#[doc = "CSIC Parser NCSIC RX Signal0 Delay Adjust Register"]
pub mod csic_prs_ncsic_rx_signal0_dly_adj;
#[doc = "csic_prs_ncsic_rx_signal5_dly_adj (rw) register accessor: CSIC Parser NCSIC RX Signal5 Delay Adjust Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_prs_ncsic_rx_signal5_dly_adj::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_prs_ncsic_rx_signal5_dly_adj::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_prs_ncsic_rx_signal5_dly_adj`] module"]
pub type CSIC_PRS_NCSIC_RX_SIGNAL5_DLY_ADJ =
    crate::Reg<csic_prs_ncsic_rx_signal5_dly_adj::CSIC_PRS_NCSIC_RX_SIGNAL5_DLY_ADJ_SPEC>;
#[doc = "CSIC Parser NCSIC RX Signal5 Delay Adjust Register"]
pub mod csic_prs_ncsic_rx_signal5_dly_adj;
#[doc = "csic_prs_ncsic_rx_signal6_dly_adj (rw) register accessor: CSIC Parser NCSIC RX Signal6 Delay Adjust Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_prs_ncsic_rx_signal6_dly_adj::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_prs_ncsic_rx_signal6_dly_adj::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_prs_ncsic_rx_signal6_dly_adj`] module"]
pub type CSIC_PRS_NCSIC_RX_SIGNAL6_DLY_ADJ =
    crate::Reg<csic_prs_ncsic_rx_signal6_dly_adj::CSIC_PRS_NCSIC_RX_SIGNAL6_DLY_ADJ_SPEC>;
#[doc = "CSIC Parser NCSIC RX Signal6 Delay Adjust Register"]
pub mod csic_prs_ncsic_rx_signal6_dly_adj;
