#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    cir_tglr: CIR_TGLR,
    cir_tmcr: CIR_TMCR,
    cir_tcr: CIR_TCR,
    cir_idc_h: CIR_IDC_H,
    cir_idc_l: CIR_IDC_L,
    cir_ticr_h: CIR_TICR_H,
    cir_ticr_l: CIR_TICR_L,
    _reserved7: [u8; 0x04],
    cir_tel: CIR_TEL,
    cir_txint: CIR_TXINT,
    cir_tac: CIR_TAC,
    cir_txsta: CIR_TXSTA,
    cir_txt: CIR_TXT,
    cir_dma_ctl: CIR_DMA_CTL,
    _reserved13: [u8; 0x48],
    cir_txfifo: CIR_TXFIFO,
}
impl RegisterBlock {
    #[doc = "0x00 - CIR Transmit Global Register"]
    #[inline(always)]
    pub const fn cir_tglr(&self) -> &CIR_TGLR {
        &self.cir_tglr
    }
    #[doc = "0x04 - CIR Transmit Modulation Control Register"]
    #[inline(always)]
    pub const fn cir_tmcr(&self) -> &CIR_TMCR {
        &self.cir_tmcr
    }
    #[doc = "0x08 - CIR Transmit Control Register"]
    #[inline(always)]
    pub const fn cir_tcr(&self) -> &CIR_TCR {
        &self.cir_tcr
    }
    #[doc = "0x0c - CIR Transmit Idle Duration Threshold High Bit Register"]
    #[inline(always)]
    pub const fn cir_idc_h(&self) -> &CIR_IDC_H {
        &self.cir_idc_h
    }
    #[doc = "0x10 - CIR Transmit Idle Duration Threshold Low Bit Register"]
    #[inline(always)]
    pub const fn cir_idc_l(&self) -> &CIR_IDC_L {
        &self.cir_idc_l
    }
    #[doc = "0x14 - CIR Transmit Idle Counter High Bit Register"]
    #[inline(always)]
    pub const fn cir_ticr_h(&self) -> &CIR_TICR_H {
        &self.cir_ticr_h
    }
    #[doc = "0x18 - CIR Transmit Idle Counter Low Bit Register"]
    #[inline(always)]
    pub const fn cir_ticr_l(&self) -> &CIR_TICR_L {
        &self.cir_ticr_l
    }
    #[doc = "0x20 - CIR TX FIFO Empty Level Register"]
    #[inline(always)]
    pub const fn cir_tel(&self) -> &CIR_TEL {
        &self.cir_tel
    }
    #[doc = "0x24 - CIR Transmit Interrupt Control Register"]
    #[inline(always)]
    pub const fn cir_txint(&self) -> &CIR_TXINT {
        &self.cir_txint
    }
    #[doc = "0x28 - CIR Transmit FIFO Available Counter Register"]
    #[inline(always)]
    pub const fn cir_tac(&self) -> &CIR_TAC {
        &self.cir_tac
    }
    #[doc = "0x2c - CIR Transmit Status Register"]
    #[inline(always)]
    pub const fn cir_txsta(&self) -> &CIR_TXSTA {
        &self.cir_txsta
    }
    #[doc = "0x30 - CIR Transmit Threshold Register"]
    #[inline(always)]
    pub const fn cir_txt(&self) -> &CIR_TXT {
        &self.cir_txt
    }
    #[doc = "0x34 - CIR DMA Control Register"]
    #[inline(always)]
    pub const fn cir_dma_ctl(&self) -> &CIR_DMA_CTL {
        &self.cir_dma_ctl
    }
    #[doc = "0x80 - CIR Transmit FIFO Data Register"]
    #[inline(always)]
    pub const fn cir_txfifo(&self) -> &CIR_TXFIFO {
        &self.cir_txfifo
    }
}
#[doc = "cir_tglr (rw) register accessor: CIR Transmit Global Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cir_tglr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cir_tglr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cir_tglr`] module"]
pub type CIR_TGLR = crate::Reg<cir_tglr::CIR_TGLR_SPEC>;
#[doc = "CIR Transmit Global Register"]
pub mod cir_tglr;
#[doc = "cir_tmcr (rw) register accessor: CIR Transmit Modulation Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cir_tmcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cir_tmcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cir_tmcr`] module"]
pub type CIR_TMCR = crate::Reg<cir_tmcr::CIR_TMCR_SPEC>;
#[doc = "CIR Transmit Modulation Control Register"]
pub mod cir_tmcr;
#[doc = "cir_tcr (rw) register accessor: CIR Transmit Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cir_tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cir_tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cir_tcr`] module"]
pub type CIR_TCR = crate::Reg<cir_tcr::CIR_TCR_SPEC>;
#[doc = "CIR Transmit Control Register"]
pub mod cir_tcr;
#[doc = "cir_idc_h (rw) register accessor: CIR Transmit Idle Duration Threshold High Bit Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cir_idc_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cir_idc_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cir_idc_h`] module"]
pub type CIR_IDC_H = crate::Reg<cir_idc_h::CIR_IDC_H_SPEC>;
#[doc = "CIR Transmit Idle Duration Threshold High Bit Register"]
pub mod cir_idc_h;
#[doc = "cir_idc_l (rw) register accessor: CIR Transmit Idle Duration Threshold Low Bit Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cir_idc_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cir_idc_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cir_idc_l`] module"]
pub type CIR_IDC_L = crate::Reg<cir_idc_l::CIR_IDC_L_SPEC>;
#[doc = "CIR Transmit Idle Duration Threshold Low Bit Register"]
pub mod cir_idc_l;
#[doc = "cir_ticr_h (rw) register accessor: CIR Transmit Idle Counter High Bit Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cir_ticr_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cir_ticr_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cir_ticr_h`] module"]
pub type CIR_TICR_H = crate::Reg<cir_ticr_h::CIR_TICR_H_SPEC>;
#[doc = "CIR Transmit Idle Counter High Bit Register"]
pub mod cir_ticr_h;
#[doc = "cir_ticr_l (rw) register accessor: CIR Transmit Idle Counter Low Bit Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cir_ticr_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cir_ticr_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cir_ticr_l`] module"]
pub type CIR_TICR_L = crate::Reg<cir_ticr_l::CIR_TICR_L_SPEC>;
#[doc = "CIR Transmit Idle Counter Low Bit Register"]
pub mod cir_ticr_l;
#[doc = "cir_tel (rw) register accessor: CIR TX FIFO Empty Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cir_tel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cir_tel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cir_tel`] module"]
pub type CIR_TEL = crate::Reg<cir_tel::CIR_TEL_SPEC>;
#[doc = "CIR TX FIFO Empty Level Register"]
pub mod cir_tel;
#[doc = "cir_txint (rw) register accessor: CIR Transmit Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cir_txint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cir_txint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cir_txint`] module"]
pub type CIR_TXINT = crate::Reg<cir_txint::CIR_TXINT_SPEC>;
#[doc = "CIR Transmit Interrupt Control Register"]
pub mod cir_txint;
#[doc = "cir_tac (rw) register accessor: CIR Transmit FIFO Available Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cir_tac::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cir_tac::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cir_tac`] module"]
pub type CIR_TAC = crate::Reg<cir_tac::CIR_TAC_SPEC>;
#[doc = "CIR Transmit FIFO Available Counter Register"]
pub mod cir_tac;
#[doc = "cir_txsta (rw) register accessor: CIR Transmit Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cir_txsta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cir_txsta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cir_txsta`] module"]
pub type CIR_TXSTA = crate::Reg<cir_txsta::CIR_TXSTA_SPEC>;
#[doc = "CIR Transmit Status Register"]
pub mod cir_txsta;
#[doc = "cir_txt (rw) register accessor: CIR Transmit Threshold Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cir_txt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cir_txt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cir_txt`] module"]
pub type CIR_TXT = crate::Reg<cir_txt::CIR_TXT_SPEC>;
#[doc = "CIR Transmit Threshold Register"]
pub mod cir_txt;
#[doc = "cir_dma_ctl (rw) register accessor: CIR DMA Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cir_dma_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cir_dma_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cir_dma_ctl`] module"]
pub type CIR_DMA_CTL = crate::Reg<cir_dma_ctl::CIR_DMA_CTL_SPEC>;
#[doc = "CIR DMA Control Register"]
pub mod cir_dma_ctl;
#[doc = "cir_txfifo (rw) register accessor: CIR Transmit FIFO Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cir_txfifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cir_txfifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cir_txfifo`] module"]
pub type CIR_TXFIFO = crate::Reg<cir_txfifo::CIR_TXFIFO_SPEC>;
#[doc = "CIR Transmit FIFO Data Register"]
pub mod cir_txfifo;
