#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    spi_gcr: SPI_GCR,
    spi_tcr: SPI_TCR,
    _reserved2: [u8; 0x04],
    spi_ier: SPI_IER,
    spi_isr: SPI_ISR,
    spi_fcr: SPI_FCR,
    spi_fsr: SPI_FSR,
    spi_wcr: SPI_WCR,
    _reserved7: [u8; 0x04],
    spi_samp_dl: SPI_SAMP_DL,
    _reserved8: [u8; 0x04],
    spi_mbc: SPI_MBC,
    spi_mtc: SPI_MTC,
    spi_bcc: SPI_BCC,
    _reserved11: [u8; 0x04],
    spi_batc: SPI_BATC,
    spi_ba_ccr: SPI_BA_CCR,
    spi_tbr: SPI_TBR,
    spi_rbr: SPI_RBR,
    _reserved15: [u8; 0x38],
    spi_ndma_mode_ctl: SPI_NDMA_MODE_CTL,
    _reserved16: [u8; 0x0174],
    _reserved_16_spi_: [u8; 0x04],
    _reserved17: [u8; 0xfc],
    _reserved_17_spi_: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x04 - SPI Global Control Register"]
    #[inline(always)]
    pub const fn spi_gcr(&self) -> &SPI_GCR {
        &self.spi_gcr
    }
    #[doc = "0x08 - SPI Transfer Control Register"]
    #[inline(always)]
    pub const fn spi_tcr(&self) -> &SPI_TCR {
        &self.spi_tcr
    }
    #[doc = "0x10 - SPI Interrupt Control Register"]
    #[inline(always)]
    pub const fn spi_ier(&self) -> &SPI_IER {
        &self.spi_ier
    }
    #[doc = "0x14 - SPI Interrupt Status Register"]
    #[inline(always)]
    pub const fn spi_isr(&self) -> &SPI_ISR {
        &self.spi_isr
    }
    #[doc = "0x18 - SPI FIFO Control Register"]
    #[inline(always)]
    pub const fn spi_fcr(&self) -> &SPI_FCR {
        &self.spi_fcr
    }
    #[doc = "0x1c - SPI FIFO Status Register"]
    #[inline(always)]
    pub const fn spi_fsr(&self) -> &SPI_FSR {
        &self.spi_fsr
    }
    #[doc = "0x20 - SPI Wait Clock Register"]
    #[inline(always)]
    pub const fn spi_wcr(&self) -> &SPI_WCR {
        &self.spi_wcr
    }
    #[doc = "0x28 - SPI Sample Delay Control Register"]
    #[inline(always)]
    pub const fn spi_samp_dl(&self) -> &SPI_SAMP_DL {
        &self.spi_samp_dl
    }
    #[doc = "0x30 - SPI Master Burst Counter Register"]
    #[inline(always)]
    pub const fn spi_mbc(&self) -> &SPI_MBC {
        &self.spi_mbc
    }
    #[doc = "0x34 - SPI Master Transmit Counter Register"]
    #[inline(always)]
    pub const fn spi_mtc(&self) -> &SPI_MTC {
        &self.spi_mtc
    }
    #[doc = "0x38 - SPI Master Burst Control Register"]
    #[inline(always)]
    pub const fn spi_bcc(&self) -> &SPI_BCC {
        &self.spi_bcc
    }
    #[doc = "0x40 - SPI Bit-Aligned Transfer Configure Register"]
    #[inline(always)]
    pub const fn spi_batc(&self) -> &SPI_BATC {
        &self.spi_batc
    }
    #[doc = "0x44 - SPI Bit-Aligned Clock Configuration Register"]
    #[inline(always)]
    pub const fn spi_ba_ccr(&self) -> &SPI_BA_CCR {
        &self.spi_ba_ccr
    }
    #[doc = "0x48 - SPI TX Bit Register\n\nVTB \\[31:0\\]: The Value of the Transmit Bits"]
    #[inline(always)]
    pub const fn spi_tbr(&self) -> &SPI_TBR {
        &self.spi_tbr
    }
    #[doc = "0x4c - SPI RX Bit Register\n\nVRB \\[31:0\\]: The Value of the Receive Bits"]
    #[inline(always)]
    pub const fn spi_rbr(&self) -> &SPI_RBR {
        &self.spi_rbr
    }
    #[doc = "0x88 - SPI Normal DMA Mode Control Register"]
    #[inline(always)]
    pub const fn spi_ndma_mode_ctl(&self) -> &SPI_NDMA_MODE_CTL {
        &self.spi_ndma_mode_ctl
    }
    #[doc = "0x200 - SPI TX Data Register\n\nTDATA \\[7:0\\]: Transmit Data in byte method"]
    #[inline(always)]
    pub const fn spi_txd_8(&self) -> &SPI_TXD_8 {
        unsafe { &*(self as *const Self).cast::<u8>().add(512).cast() }
    }
    #[doc = "0x200 - SPI TX Data Register\n\nTDATA \\[15:0\\]: Transmit Data in half-word method"]
    #[inline(always)]
    pub const fn spi_txd_16(&self) -> &SPI_TXD_16 {
        unsafe { &*(self as *const Self).cast::<u8>().add(512).cast() }
    }
    #[doc = "0x200 - SPI TX Data Register\n\nTDATA \\[31:0\\]: Transmit Data in word method"]
    #[inline(always)]
    pub const fn spi_txd(&self) -> &SPI_TXD {
        unsafe { &*(self as *const Self).cast::<u8>().add(512).cast() }
    }
    #[doc = "0x300 - SPI RX Data Register\n\nRDATA \\[7:0\\]: Receive Data and access in byte method"]
    #[inline(always)]
    pub const fn spi_rxd_8(&self) -> &SPI_RXD_8 {
        unsafe { &*(self as *const Self).cast::<u8>().add(768).cast() }
    }
    #[doc = "0x300 - SPI RX Data Register\n\nRDATA \\[15:0\\]: Receive Data and access in half-word method"]
    #[inline(always)]
    pub const fn spi_rxd_16(&self) -> &SPI_RXD_16 {
        unsafe { &*(self as *const Self).cast::<u8>().add(768).cast() }
    }
    #[doc = "0x300 - SPI RX Data Register\n\nRDATA \\[31:0\\]: Receive Data and access in word method"]
    #[inline(always)]
    pub const fn spi_rxd(&self) -> &SPI_RXD {
        unsafe { &*(self as *const Self).cast::<u8>().add(768).cast() }
    }
}
#[doc = "spi_gcr (rw) register accessor: SPI Global Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_gcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_gcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_gcr`] module"]
pub type SPI_GCR = crate::Reg<spi_gcr::SPI_GCR_SPEC>;
#[doc = "SPI Global Control Register"]
pub mod spi_gcr;
#[doc = "spi_tcr (rw) register accessor: SPI Transfer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_tcr`] module"]
pub type SPI_TCR = crate::Reg<spi_tcr::SPI_TCR_SPEC>;
#[doc = "SPI Transfer Control Register"]
pub mod spi_tcr;
#[doc = "spi_ier (rw) register accessor: SPI Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_ier`] module"]
pub type SPI_IER = crate::Reg<spi_ier::SPI_IER_SPEC>;
#[doc = "SPI Interrupt Control Register"]
pub mod spi_ier;
#[doc = "spi_isr (rw) register accessor: SPI Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_isr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_isr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_isr`] module"]
pub type SPI_ISR = crate::Reg<spi_isr::SPI_ISR_SPEC>;
#[doc = "SPI Interrupt Status Register"]
pub mod spi_isr;
#[doc = "spi_fcr (rw) register accessor: SPI FIFO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_fcr`] module"]
pub type SPI_FCR = crate::Reg<spi_fcr::SPI_FCR_SPEC>;
#[doc = "SPI FIFO Control Register"]
pub mod spi_fcr;
#[doc = "spi_fsr (r) register accessor: SPI FIFO Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_fsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_fsr`] module"]
pub type SPI_FSR = crate::Reg<spi_fsr::SPI_FSR_SPEC>;
#[doc = "SPI FIFO Status Register"]
pub mod spi_fsr;
#[doc = "spi_wcr (rw) register accessor: SPI Wait Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_wcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_wcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_wcr`] module"]
pub type SPI_WCR = crate::Reg<spi_wcr::SPI_WCR_SPEC>;
#[doc = "SPI Wait Clock Register"]
pub mod spi_wcr;
#[doc = "spi_samp_dl (rw) register accessor: SPI Sample Delay Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_samp_dl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_samp_dl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_samp_dl`] module"]
pub type SPI_SAMP_DL = crate::Reg<spi_samp_dl::SPI_SAMP_DL_SPEC>;
#[doc = "SPI Sample Delay Control Register"]
pub mod spi_samp_dl;
#[doc = "spi_mbc (rw) register accessor: SPI Master Burst Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mbc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mbc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mbc`] module"]
pub type SPI_MBC = crate::Reg<spi_mbc::SPI_MBC_SPEC>;
#[doc = "SPI Master Burst Counter Register"]
pub mod spi_mbc;
#[doc = "spi_mtc (rw) register accessor: SPI Master Transmit Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mtc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mtc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mtc`] module"]
pub type SPI_MTC = crate::Reg<spi_mtc::SPI_MTC_SPEC>;
#[doc = "SPI Master Transmit Counter Register"]
pub mod spi_mtc;
#[doc = "spi_bcc (rw) register accessor: SPI Master Burst Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_bcc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_bcc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_bcc`] module"]
pub type SPI_BCC = crate::Reg<spi_bcc::SPI_BCC_SPEC>;
#[doc = "SPI Master Burst Control Register"]
pub mod spi_bcc;
#[doc = "spi_batc (rw) register accessor: SPI Bit-Aligned Transfer Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_batc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_batc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_batc`] module"]
pub type SPI_BATC = crate::Reg<spi_batc::SPI_BATC_SPEC>;
#[doc = "SPI Bit-Aligned Transfer Configure Register"]
pub mod spi_batc;
#[doc = "spi_ba_ccr (rw) register accessor: SPI Bit-Aligned Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_ba_ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_ba_ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_ba_ccr`] module"]
pub type SPI_BA_CCR = crate::Reg<spi_ba_ccr::SPI_BA_CCR_SPEC>;
#[doc = "SPI Bit-Aligned Clock Configuration Register"]
pub mod spi_ba_ccr;
#[doc = "spi_tbr (rw) register accessor: SPI TX Bit Register\n\nVTB \\[31:0\\]: The Value of the Transmit Bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_tbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_tbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_tbr`] module"]
pub type SPI_TBR = crate::Reg<spi_tbr::SPI_TBR_SPEC>;
#[doc = "SPI TX Bit Register\n\nVTB \\[31:0\\]: The Value of the Transmit Bits"]
pub mod spi_tbr;
#[doc = "spi_rbr (rw) register accessor: SPI RX Bit Register\n\nVRB \\[31:0\\]: The Value of the Receive Bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_rbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_rbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_rbr`] module"]
pub type SPI_RBR = crate::Reg<spi_rbr::SPI_RBR_SPEC>;
#[doc = "SPI RX Bit Register\n\nVRB \\[31:0\\]: The Value of the Receive Bits"]
pub mod spi_rbr;
#[doc = "spi_ndma_mode_ctl (rw) register accessor: SPI Normal DMA Mode Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_ndma_mode_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_ndma_mode_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_ndma_mode_ctl`] module"]
pub type SPI_NDMA_MODE_CTL = crate::Reg<spi_ndma_mode_ctl::SPI_NDMA_MODE_CTL_SPEC>;
#[doc = "SPI Normal DMA Mode Control Register"]
pub mod spi_ndma_mode_ctl;
#[doc = "spi_txd (rw) register accessor: SPI TX Data Register\n\nTDATA \\[31:0\\]: Transmit Data in word method\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_txd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_txd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_txd`] module"]
pub type SPI_TXD = crate::Reg<spi_txd::SPI_TXD_SPEC>;
#[doc = "SPI TX Data Register\n\nTDATA \\[31:0\\]: Transmit Data in word method"]
pub mod spi_txd;
#[doc = "spi_txd_16 (rw) register accessor: SPI TX Data Register\n\nTDATA \\[15:0\\]: Transmit Data in half-word method\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_txd_16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_txd_16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_txd_16`] module"]
pub type SPI_TXD_16 = crate::Reg<spi_txd_16::SPI_TXD_16_SPEC>;
#[doc = "SPI TX Data Register\n\nTDATA \\[15:0\\]: Transmit Data in half-word method"]
pub mod spi_txd_16;
#[doc = "spi_txd_8 (rw) register accessor: SPI TX Data Register\n\nTDATA \\[7:0\\]: Transmit Data in byte method\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_txd_8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_txd_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_txd_8`] module"]
pub type SPI_TXD_8 = crate::Reg<spi_txd_8::SPI_TXD_8_SPEC>;
#[doc = "SPI TX Data Register\n\nTDATA \\[7:0\\]: Transmit Data in byte method"]
pub mod spi_txd_8;
#[doc = "spi_rxd (rw) register accessor: SPI RX Data Register\n\nRDATA \\[31:0\\]: Receive Data and access in word method\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_rxd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_rxd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_rxd`] module"]
pub type SPI_RXD = crate::Reg<spi_rxd::SPI_RXD_SPEC>;
#[doc = "SPI RX Data Register\n\nRDATA \\[31:0\\]: Receive Data and access in word method"]
pub mod spi_rxd;
#[doc = "spi_rxd_16 (rw) register accessor: SPI RX Data Register\n\nRDATA \\[15:0\\]: Receive Data and access in half-word method\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_rxd_16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_rxd_16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_rxd_16`] module"]
pub type SPI_RXD_16 = crate::Reg<spi_rxd_16::SPI_RXD_16_SPEC>;
#[doc = "SPI RX Data Register\n\nRDATA \\[15:0\\]: Receive Data and access in half-word method"]
pub mod spi_rxd_16;
#[doc = "spi_rxd_8 (rw) register accessor: SPI RX Data Register\n\nRDATA \\[7:0\\]: Receive Data and access in byte method\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_rxd_8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_rxd_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_rxd_8`] module"]
pub type SPI_RXD_8 = crate::Reg<spi_rxd_8::SPI_RXD_8_SPEC>;
#[doc = "SPI RX Data Register\n\nRDATA \\[7:0\\]: Receive Data and access in byte method"]
pub mod spi_rxd_8;
