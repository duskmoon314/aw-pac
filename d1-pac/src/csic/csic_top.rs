#[doc = r"Register block"]
#[repr(C)]
pub struct CSIC_TOP {
    csic_top_en: CSIC_TOP_EN,
    csic_ptn_gen_en: CSIC_PTN_GEN_EN,
    csic_ptn_ctrl: CSIC_PTN_CTRL,
    _reserved3: [u8; 0x14],
    csic_ptn_len: CSIC_PTN_LEN,
    csic_ptn_addr: CSIC_PTN_ADDR,
    csic_ptn_isp_size: CSIC_PTN_ISP_SIZE,
    _reserved6: [u8; 0x74],
    csic_dma_input_sel: [CSIC_DMA_INPUT_SEL; 2],
    _reserved7: [u8; 0x34],
    csic_bist_cs: CSIC_BIST_CS,
    csic_bist_control: CSIC_BIST_CONTROL,
    csic_bist_start_addr: CSIC_BIST_START_ADDR,
    csic_bist_end_addr: CSIC_BIST_END_ADDR,
    csic_bist_data_mask: CSIC_BIST_DATA_MASK,
    csic_mbus_req_max: CSIC_MBUS_REQ_MAX,
    _reserved13: [u8; 0x0c],
    csic_mulf_mod: CSIC_MULF_MOD,
    csic_mulf_int: CSIC_MULF_INT,
}
impl CSIC_TOP {
    #[doc = "0x00 - CSIC TOP Enable Register"]
    #[inline(always)]
    pub const fn csic_top_en(&self) -> &CSIC_TOP_EN {
        &self.csic_top_en
    }
    #[doc = "0x04 - CSIC Pattern Generation Enable Register"]
    #[inline(always)]
    pub const fn csic_ptn_gen_en(&self) -> &CSIC_PTN_GEN_EN {
        &self.csic_ptn_gen_en
    }
    #[doc = "0x08 - CSIC Pattern Control Register"]
    #[inline(always)]
    pub const fn csic_ptn_ctrl(&self) -> &CSIC_PTN_CTRL {
        &self.csic_ptn_ctrl
    }
    #[doc = "0x20 - CSIC Pattern Generation Length Register"]
    #[inline(always)]
    pub const fn csic_ptn_len(&self) -> &CSIC_PTN_LEN {
        &self.csic_ptn_len
    }
    #[doc = "0x24 - CSIC Pattern Generation Address Register"]
    #[inline(always)]
    pub const fn csic_ptn_addr(&self) -> &CSIC_PTN_ADDR {
        &self.csic_ptn_addr
    }
    #[doc = "0x28 - CSIC Pattern ISP Size Register"]
    #[inline(always)]
    pub const fn csic_ptn_isp_size(&self) -> &CSIC_PTN_ISP_SIZE {
        &self.csic_ptn_isp_size
    }
    #[doc = "0xa0..0xa8 - CSIC DMA\\[i\\] Input Select Register"]
    #[inline(always)]
    pub const fn csic_dma_input_sel(&self, n: usize) -> &CSIC_DMA_INPUT_SEL {
        &self.csic_dma_input_sel[n]
    }
    #[doc = "0xa0 - CSIC DMA\\[i\\] Input Select Register"]
    #[inline(always)]
    pub const fn csic_dma0_input_sel(&self) -> &CSIC_DMA_INPUT_SEL {
        self.csic_dma_input_sel(0)
    }
    #[doc = "0xa4 - CSIC DMA\\[i\\] Input Select Register"]
    #[inline(always)]
    pub const fn csic_dma1_input_sel(&self) -> &CSIC_DMA_INPUT_SEL {
        self.csic_dma_input_sel(1)
    }
    #[doc = "0xdc - CSIC BIST CS Register"]
    #[inline(always)]
    pub const fn csic_bist_cs(&self) -> &CSIC_BIST_CS {
        &self.csic_bist_cs
    }
    #[doc = "0xe0 - CSIC BIST Control Register"]
    #[inline(always)]
    pub const fn csic_bist_control(&self) -> &CSIC_BIST_CONTROL {
        &self.csic_bist_control
    }
    #[doc = "0xe4 - CSIC BIST Start Address Register"]
    #[inline(always)]
    pub const fn csic_bist_start_addr(&self) -> &CSIC_BIST_START_ADDR {
        &self.csic_bist_start_addr
    }
    #[doc = "0xe8 - CSIC BIST End Address Register"]
    #[inline(always)]
    pub const fn csic_bist_end_addr(&self) -> &CSIC_BIST_END_ADDR {
        &self.csic_bist_end_addr
    }
    #[doc = "0xec - CSIC BIST Data Mask Register"]
    #[inline(always)]
    pub const fn csic_bist_data_mask(&self) -> &CSIC_BIST_DATA_MASK {
        &self.csic_bist_data_mask
    }
    #[doc = "0xf0 - CSIC MBUS REQ MAX Register"]
    #[inline(always)]
    pub const fn csic_mbus_req_max(&self) -> &CSIC_MBUS_REQ_MAX {
        &self.csic_mbus_req_max
    }
    #[doc = "0x100 - CSIC Multi-Frame Mode Register"]
    #[inline(always)]
    pub const fn csic_mulf_mod(&self) -> &CSIC_MULF_MOD {
        &self.csic_mulf_mod
    }
    #[doc = "0x104 - CSIC Multi-Frame Interrupt Register"]
    #[inline(always)]
    pub const fn csic_mulf_int(&self) -> &CSIC_MULF_INT {
        &self.csic_mulf_int
    }
}
#[doc = "csic_top_en (rw) register accessor: CSIC TOP Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_top_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_top_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_top_en`] module"]
pub type CSIC_TOP_EN = crate::Reg<csic_top_en::CSIC_TOP_EN_SPEC>;
#[doc = "CSIC TOP Enable Register"]
pub mod csic_top_en;
#[doc = "csic_ptn_gen_en (rw) register accessor: CSIC Pattern Generation Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_ptn_gen_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_ptn_gen_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_ptn_gen_en`] module"]
pub type CSIC_PTN_GEN_EN = crate::Reg<csic_ptn_gen_en::CSIC_PTN_GEN_EN_SPEC>;
#[doc = "CSIC Pattern Generation Enable Register"]
pub mod csic_ptn_gen_en;
#[doc = "csic_ptn_ctrl (rw) register accessor: CSIC Pattern Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_ptn_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_ptn_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_ptn_ctrl`] module"]
pub type CSIC_PTN_CTRL = crate::Reg<csic_ptn_ctrl::CSIC_PTN_CTRL_SPEC>;
#[doc = "CSIC Pattern Control Register"]
pub mod csic_ptn_ctrl;
#[doc = "csic_ptn_len (rw) register accessor: CSIC Pattern Generation Length Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_ptn_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_ptn_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_ptn_len`] module"]
pub type CSIC_PTN_LEN = crate::Reg<csic_ptn_len::CSIC_PTN_LEN_SPEC>;
#[doc = "CSIC Pattern Generation Length Register"]
pub mod csic_ptn_len;
#[doc = "csic_ptn_addr (rw) register accessor: CSIC Pattern Generation Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_ptn_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_ptn_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_ptn_addr`] module"]
pub type CSIC_PTN_ADDR = crate::Reg<csic_ptn_addr::CSIC_PTN_ADDR_SPEC>;
#[doc = "CSIC Pattern Generation Address Register"]
pub mod csic_ptn_addr;
#[doc = "csic_ptn_isp_size (rw) register accessor: CSIC Pattern ISP Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_ptn_isp_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_ptn_isp_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_ptn_isp_size`] module"]
pub type CSIC_PTN_ISP_SIZE = crate::Reg<csic_ptn_isp_size::CSIC_PTN_ISP_SIZE_SPEC>;
#[doc = "CSIC Pattern ISP Size Register"]
pub mod csic_ptn_isp_size;
#[doc = "csic_dma_input_sel (rw) register accessor: CSIC DMA\\[i\\] Input Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_input_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_input_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_dma_input_sel`] module"]
pub type CSIC_DMA_INPUT_SEL = crate::Reg<csic_dma_input_sel::CSIC_DMA_INPUT_SEL_SPEC>;
#[doc = "CSIC DMA\\[i\\] Input Select Register"]
pub mod csic_dma_input_sel;
#[doc = "csic_bist_cs (rw) register accessor: CSIC BIST CS Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_bist_cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_bist_cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_bist_cs`] module"]
pub type CSIC_BIST_CS = crate::Reg<csic_bist_cs::CSIC_BIST_CS_SPEC>;
#[doc = "CSIC BIST CS Register"]
pub mod csic_bist_cs;
#[doc = "csic_bist_control (rw) register accessor: CSIC BIST Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_bist_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_bist_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_bist_control`] module"]
pub type CSIC_BIST_CONTROL = crate::Reg<csic_bist_control::CSIC_BIST_CONTROL_SPEC>;
#[doc = "CSIC BIST Control Register"]
pub mod csic_bist_control;
#[doc = "csic_bist_start_addr (rw) register accessor: CSIC BIST Start Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_bist_start_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_bist_start_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_bist_start_addr`] module"]
pub type CSIC_BIST_START_ADDR = crate::Reg<csic_bist_start_addr::CSIC_BIST_START_ADDR_SPEC>;
#[doc = "CSIC BIST Start Address Register"]
pub mod csic_bist_start_addr;
#[doc = "csic_bist_end_addr (rw) register accessor: CSIC BIST End Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_bist_end_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_bist_end_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_bist_end_addr`] module"]
pub type CSIC_BIST_END_ADDR = crate::Reg<csic_bist_end_addr::CSIC_BIST_END_ADDR_SPEC>;
#[doc = "CSIC BIST End Address Register"]
pub mod csic_bist_end_addr;
#[doc = "csic_bist_data_mask (rw) register accessor: CSIC BIST Data Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_bist_data_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_bist_data_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_bist_data_mask`] module"]
pub type CSIC_BIST_DATA_MASK = crate::Reg<csic_bist_data_mask::CSIC_BIST_DATA_MASK_SPEC>;
#[doc = "CSIC BIST Data Mask Register"]
pub mod csic_bist_data_mask;
#[doc = "csic_mbus_req_max (rw) register accessor: CSIC MBUS REQ MAX Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_mbus_req_max::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_mbus_req_max::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_mbus_req_max`] module"]
pub type CSIC_MBUS_REQ_MAX = crate::Reg<csic_mbus_req_max::CSIC_MBUS_REQ_MAX_SPEC>;
#[doc = "CSIC MBUS REQ MAX Register"]
pub mod csic_mbus_req_max;
#[doc = "csic_mulf_mod (rw) register accessor: CSIC Multi-Frame Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_mulf_mod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_mulf_mod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_mulf_mod`] module"]
pub type CSIC_MULF_MOD = crate::Reg<csic_mulf_mod::CSIC_MULF_MOD_SPEC>;
#[doc = "CSIC Multi-Frame Mode Register"]
pub mod csic_mulf_mod;
#[doc = "csic_mulf_int (rw) register accessor: CSIC Multi-Frame Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_mulf_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_mulf_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csic_mulf_int`] module"]
pub type CSIC_MULF_INT = crate::Reg<csic_mulf_int::CSIC_MULF_INT_SPEC>;
#[doc = "CSIC Multi-Frame Interrupt Register"]
pub mod csic_mulf_int;
