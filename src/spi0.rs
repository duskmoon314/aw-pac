#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - SPI Global Control Register"]
    pub spi_gcr: crate::Reg<spi_gcr::SPI_GCR_SPEC>,
    #[doc = "0x08 - SPI Transfer Control Register"]
    pub spi_tcr: crate::Reg<spi_tcr::SPI_TCR_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x10 - SPI Interrupt Control Register"]
    pub spi_ier: crate::Reg<spi_ier::SPI_IER_SPEC>,
    #[doc = "0x14 - SPI Interrupt Status Register"]
    pub spi_isr: crate::Reg<spi_isr::SPI_ISR_SPEC>,
    #[doc = "0x18 - SPI FIFO Control Register"]
    pub spi_fcr: crate::Reg<spi_fcr::SPI_FCR_SPEC>,
    #[doc = "0x1c - SPI FIFO Status Register"]
    pub spi_fsr: crate::Reg<spi_fsr::SPI_FSR_SPEC>,
    #[doc = "0x20 - SPI Wait Clock Register"]
    pub spi_wcr: crate::Reg<spi_wcr::SPI_WCR_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x28 - SPI Sample Delay Control Register"]
    pub spi_samp_dl: crate::Reg<spi_samp_dl::SPI_SAMP_DL_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0x30 - SPI Master Burst Counter Register"]
    pub spi_mbc: crate::Reg<spi_mbc::SPI_MBC_SPEC>,
    #[doc = "0x34 - SPI Master Transmit Counter Register"]
    pub spi_mtc: crate::Reg<spi_mtc::SPI_MTC_SPEC>,
    #[doc = "0x38 - SPI Master Burst Control Register"]
    pub spi_bcc: crate::Reg<spi_bcc::SPI_BCC_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0x40 - SPI Bit-Aligned Transfer Configure Register"]
    pub spi_batc: crate::Reg<spi_batc::SPI_BATC_SPEC>,
    #[doc = "0x44 - SPI Bit-Aligned Clock Configuration Register"]
    pub spi_ba_ccr: crate::Reg<spi_ba_ccr::SPI_BA_CCR_SPEC>,
    #[doc = "0x48 - SPI TX Bit Register"]
    pub spi_tbr: crate::Reg<spi_tbr::SPI_TBR_SPEC>,
    #[doc = "0x4c - SPI RX Bit Register"]
    pub spi_rbr: crate::Reg<spi_rbr::SPI_RBR_SPEC>,
    _reserved15: [u8; 0x38],
    #[doc = "0x88 - SPI Normal DMA Mode Control Register"]
    pub spi_ndma_mode_ctl: crate::Reg<spi_ndma_mode_ctl::SPI_NDMA_MODE_CTL_SPEC>,
    _reserved16: [u8; 0x0174],
    #[doc = "0x200 - SPI TX Data Register"]
    pub spi_txd: crate::Reg<spi_txd::SPI_TXD_SPEC>,
    _reserved17: [u8; 0xfc],
    #[doc = "0x300 - SPI RX Data Register"]
    pub spi_rxd: crate::Reg<spi_rxd::SPI_RXD_SPEC>,
}
#[doc = "SPI_GCR register accessor: an alias for `Reg<SPI_GCR_SPEC>`"]
pub type SPI_GCR = crate::Reg<spi_gcr::SPI_GCR_SPEC>;
#[doc = "SPI Global Control Register"]
pub mod spi_gcr;
#[doc = "SPI_TCR register accessor: an alias for `Reg<SPI_TCR_SPEC>`"]
pub type SPI_TCR = crate::Reg<spi_tcr::SPI_TCR_SPEC>;
#[doc = "SPI Transfer Control Register"]
pub mod spi_tcr;
#[doc = "SPI_IER register accessor: an alias for `Reg<SPI_IER_SPEC>`"]
pub type SPI_IER = crate::Reg<spi_ier::SPI_IER_SPEC>;
#[doc = "SPI Interrupt Control Register"]
pub mod spi_ier;
#[doc = "SPI_ISR register accessor: an alias for `Reg<SPI_ISR_SPEC>`"]
pub type SPI_ISR = crate::Reg<spi_isr::SPI_ISR_SPEC>;
#[doc = "SPI Interrupt Status Register"]
pub mod spi_isr;
#[doc = "SPI_FCR register accessor: an alias for `Reg<SPI_FCR_SPEC>`"]
pub type SPI_FCR = crate::Reg<spi_fcr::SPI_FCR_SPEC>;
#[doc = "SPI FIFO Control Register"]
pub mod spi_fcr;
#[doc = "SPI_FSR register accessor: an alias for `Reg<SPI_FSR_SPEC>`"]
pub type SPI_FSR = crate::Reg<spi_fsr::SPI_FSR_SPEC>;
#[doc = "SPI FIFO Status Register"]
pub mod spi_fsr;
#[doc = "SPI_WCR register accessor: an alias for `Reg<SPI_WCR_SPEC>`"]
pub type SPI_WCR = crate::Reg<spi_wcr::SPI_WCR_SPEC>;
#[doc = "SPI Wait Clock Register"]
pub mod spi_wcr;
#[doc = "SPI_SAMP_DL register accessor: an alias for `Reg<SPI_SAMP_DL_SPEC>`"]
pub type SPI_SAMP_DL = crate::Reg<spi_samp_dl::SPI_SAMP_DL_SPEC>;
#[doc = "SPI Sample Delay Control Register"]
pub mod spi_samp_dl;
#[doc = "SPI_MBC register accessor: an alias for `Reg<SPI_MBC_SPEC>`"]
pub type SPI_MBC = crate::Reg<spi_mbc::SPI_MBC_SPEC>;
#[doc = "SPI Master Burst Counter Register"]
pub mod spi_mbc;
#[doc = "SPI_MTC register accessor: an alias for `Reg<SPI_MTC_SPEC>`"]
pub type SPI_MTC = crate::Reg<spi_mtc::SPI_MTC_SPEC>;
#[doc = "SPI Master Transmit Counter Register"]
pub mod spi_mtc;
#[doc = "SPI_BCC register accessor: an alias for `Reg<SPI_BCC_SPEC>`"]
pub type SPI_BCC = crate::Reg<spi_bcc::SPI_BCC_SPEC>;
#[doc = "SPI Master Burst Control Register"]
pub mod spi_bcc;
#[doc = "SPI_BATC register accessor: an alias for `Reg<SPI_BATC_SPEC>`"]
pub type SPI_BATC = crate::Reg<spi_batc::SPI_BATC_SPEC>;
#[doc = "SPI Bit-Aligned Transfer Configure Register"]
pub mod spi_batc;
#[doc = "SPI_BA_CCR register accessor: an alias for `Reg<SPI_BA_CCR_SPEC>`"]
pub type SPI_BA_CCR = crate::Reg<spi_ba_ccr::SPI_BA_CCR_SPEC>;
#[doc = "SPI Bit-Aligned Clock Configuration Register"]
pub mod spi_ba_ccr;
#[doc = "SPI_TBR register accessor: an alias for `Reg<SPI_TBR_SPEC>`"]
pub type SPI_TBR = crate::Reg<spi_tbr::SPI_TBR_SPEC>;
#[doc = "SPI TX Bit Register"]
pub mod spi_tbr;
#[doc = "SPI_RBR register accessor: an alias for `Reg<SPI_RBR_SPEC>`"]
pub type SPI_RBR = crate::Reg<spi_rbr::SPI_RBR_SPEC>;
#[doc = "SPI RX Bit Register"]
pub mod spi_rbr;
#[doc = "SPI_NDMA_MODE_CTL register accessor: an alias for `Reg<SPI_NDMA_MODE_CTL_SPEC>`"]
pub type SPI_NDMA_MODE_CTL = crate::Reg<spi_ndma_mode_ctl::SPI_NDMA_MODE_CTL_SPEC>;
#[doc = "SPI Normal DMA Mode Control Register"]
pub mod spi_ndma_mode_ctl;
#[doc = "SPI_TXD register accessor: an alias for `Reg<SPI_TXD_SPEC>`"]
pub type SPI_TXD = crate::Reg<spi_txd::SPI_TXD_SPEC>;
#[doc = "SPI TX Data Register"]
pub mod spi_txd;
#[doc = "SPI_RXD register accessor: an alias for `Reg<SPI_RXD_SPEC>`"]
pub type SPI_RXD = crate::Reg<spi_rxd::SPI_RXD_SPEC>;
#[doc = "SPI RX Data Register"]
pub mod spi_rxd;
