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
    #[doc = "0x48 - SPI TX Bit Register\n\nVTB \\[31:0\\]: The Value of the Transmit Bits"]
    pub spi_tbr: crate::Reg<spi_tbr::SPI_TBR_SPEC>,
    #[doc = "0x4c - SPI RX Bit Register\n\nVRB \\[31:0\\]: The Value of the Receive Bits"]
    pub spi_rbr: crate::Reg<spi_rbr::SPI_RBR_SPEC>,
    _reserved15: [u8; 0x38],
    #[doc = "0x88 - SPI Normal DMA Mode Control Register"]
    pub spi_ndma_mode_ctl: crate::Reg<spi_ndma_mode_ctl::SPI_NDMA_MODE_CTL_SPEC>,
    _reserved16: [u8; 0x0174],
    _reserved_16_spi_: [u8; 0x04],
    _reserved17: [u8; 0xfc],
    _reserved_17_spi_: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x200 - SPI TX Data Register\n\nTDATA \\[7:0\\]: Transmit Data in byte method"]
    #[inline(always)]
    pub fn spi_txd_8(&self) -> &crate::Reg<spi_txd_8::SPI_TXD_8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(512usize)
                as *const crate::Reg<spi_txd_8::SPI_TXD_8_SPEC>)
        }
    }
    #[doc = "0x200 - SPI TX Data Register\n\nTDATA \\[15:0\\]: Transmit Data in half-word method"]
    #[inline(always)]
    pub fn spi_txd_16(&self) -> &crate::Reg<spi_txd_16::SPI_TXD_16_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(512usize)
                as *const crate::Reg<spi_txd_16::SPI_TXD_16_SPEC>)
        }
    }
    #[doc = "0x200 - SPI TX Data Register\n\nTDATA \\[31:0\\]: Transmit Data in word method"]
    #[inline(always)]
    pub fn spi_txd(&self) -> &crate::Reg<spi_txd::SPI_TXD_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(512usize)
                as *const crate::Reg<spi_txd::SPI_TXD_SPEC>)
        }
    }
    #[doc = "0x300 - SPI RX Data Register\n\nRDATA \\[7:0\\]: Receive Data and access in byte method"]
    #[inline(always)]
    pub fn spi_rxd_8(&self) -> &crate::Reg<spi_rxd_8::SPI_RXD_8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(768usize)
                as *const crate::Reg<spi_rxd_8::SPI_RXD_8_SPEC>)
        }
    }
    #[doc = "0x300 - SPI RX Data Register\n\nRDATA \\[15:0\\]: Receive Data and access in half-word method"]
    #[inline(always)]
    pub fn spi_rxd_16(&self) -> &crate::Reg<spi_rxd_16::SPI_RXD_16_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(768usize)
                as *const crate::Reg<spi_rxd_16::SPI_RXD_16_SPEC>)
        }
    }
    #[doc = "0x300 - SPI RX Data Register\n\nRDATA \\[31:0\\]: Receive Data and access in word method"]
    #[inline(always)]
    pub fn spi_rxd(&self) -> &crate::Reg<spi_rxd::SPI_RXD_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(768usize)
                as *const crate::Reg<spi_rxd::SPI_RXD_SPEC>)
        }
    }
}
#[doc = "spi_gcr register accessor: an alias for `Reg<SPI_GCR_SPEC>`"]
pub type SPI_GCR = crate::Reg<spi_gcr::SPI_GCR_SPEC>;
#[doc = "SPI Global Control Register"]
pub mod spi_gcr;
#[doc = "spi_tcr register accessor: an alias for `Reg<SPI_TCR_SPEC>`"]
pub type SPI_TCR = crate::Reg<spi_tcr::SPI_TCR_SPEC>;
#[doc = "SPI Transfer Control Register"]
pub mod spi_tcr;
#[doc = "spi_ier register accessor: an alias for `Reg<SPI_IER_SPEC>`"]
pub type SPI_IER = crate::Reg<spi_ier::SPI_IER_SPEC>;
#[doc = "SPI Interrupt Control Register"]
pub mod spi_ier;
#[doc = "spi_isr register accessor: an alias for `Reg<SPI_ISR_SPEC>`"]
pub type SPI_ISR = crate::Reg<spi_isr::SPI_ISR_SPEC>;
#[doc = "SPI Interrupt Status Register"]
pub mod spi_isr;
#[doc = "spi_fcr register accessor: an alias for `Reg<SPI_FCR_SPEC>`"]
pub type SPI_FCR = crate::Reg<spi_fcr::SPI_FCR_SPEC>;
#[doc = "SPI FIFO Control Register"]
pub mod spi_fcr;
#[doc = "spi_fsr register accessor: an alias for `Reg<SPI_FSR_SPEC>`"]
pub type SPI_FSR = crate::Reg<spi_fsr::SPI_FSR_SPEC>;
#[doc = "SPI FIFO Status Register"]
pub mod spi_fsr;
#[doc = "spi_wcr register accessor: an alias for `Reg<SPI_WCR_SPEC>`"]
pub type SPI_WCR = crate::Reg<spi_wcr::SPI_WCR_SPEC>;
#[doc = "SPI Wait Clock Register"]
pub mod spi_wcr;
#[doc = "spi_samp_dl register accessor: an alias for `Reg<SPI_SAMP_DL_SPEC>`"]
pub type SPI_SAMP_DL = crate::Reg<spi_samp_dl::SPI_SAMP_DL_SPEC>;
#[doc = "SPI Sample Delay Control Register"]
pub mod spi_samp_dl;
#[doc = "spi_mbc register accessor: an alias for `Reg<SPI_MBC_SPEC>`"]
pub type SPI_MBC = crate::Reg<spi_mbc::SPI_MBC_SPEC>;
#[doc = "SPI Master Burst Counter Register"]
pub mod spi_mbc;
#[doc = "spi_mtc register accessor: an alias for `Reg<SPI_MTC_SPEC>`"]
pub type SPI_MTC = crate::Reg<spi_mtc::SPI_MTC_SPEC>;
#[doc = "SPI Master Transmit Counter Register"]
pub mod spi_mtc;
#[doc = "spi_bcc register accessor: an alias for `Reg<SPI_BCC_SPEC>`"]
pub type SPI_BCC = crate::Reg<spi_bcc::SPI_BCC_SPEC>;
#[doc = "SPI Master Burst Control Register"]
pub mod spi_bcc;
#[doc = "spi_batc register accessor: an alias for `Reg<SPI_BATC_SPEC>`"]
pub type SPI_BATC = crate::Reg<spi_batc::SPI_BATC_SPEC>;
#[doc = "SPI Bit-Aligned Transfer Configure Register"]
pub mod spi_batc;
#[doc = "spi_ba_ccr register accessor: an alias for `Reg<SPI_BA_CCR_SPEC>`"]
pub type SPI_BA_CCR = crate::Reg<spi_ba_ccr::SPI_BA_CCR_SPEC>;
#[doc = "SPI Bit-Aligned Clock Configuration Register"]
pub mod spi_ba_ccr;
#[doc = "spi_tbr register accessor: an alias for `Reg<SPI_TBR_SPEC>`"]
pub type SPI_TBR = crate::Reg<spi_tbr::SPI_TBR_SPEC>;
#[doc = "SPI TX Bit Register\n\nVTB \\[31:0\\]: The Value of the Transmit Bits"]
pub mod spi_tbr;
#[doc = "spi_rbr register accessor: an alias for `Reg<SPI_RBR_SPEC>`"]
pub type SPI_RBR = crate::Reg<spi_rbr::SPI_RBR_SPEC>;
#[doc = "SPI RX Bit Register\n\nVRB \\[31:0\\]: The Value of the Receive Bits"]
pub mod spi_rbr;
#[doc = "spi_ndma_mode_ctl register accessor: an alias for `Reg<SPI_NDMA_MODE_CTL_SPEC>`"]
pub type SPI_NDMA_MODE_CTL = crate::Reg<spi_ndma_mode_ctl::SPI_NDMA_MODE_CTL_SPEC>;
#[doc = "SPI Normal DMA Mode Control Register"]
pub mod spi_ndma_mode_ctl;
#[doc = "spi_txd register accessor: an alias for `Reg<SPI_TXD_SPEC>`"]
pub type SPI_TXD = crate::Reg<spi_txd::SPI_TXD_SPEC>;
#[doc = "SPI TX Data Register\n\nTDATA \\[31:0\\]: Transmit Data in word method"]
pub mod spi_txd;
#[doc = "spi_txd_16 register accessor: an alias for `Reg<SPI_TXD_16_SPEC>`"]
pub type SPI_TXD_16 = crate::Reg<spi_txd_16::SPI_TXD_16_SPEC>;
#[doc = "SPI TX Data Register\n\nTDATA \\[15:0\\]: Transmit Data in half-word method"]
pub mod spi_txd_16;
#[doc = "spi_txd_8 register accessor: an alias for `Reg<SPI_TXD_8_SPEC>`"]
pub type SPI_TXD_8 = crate::Reg<spi_txd_8::SPI_TXD_8_SPEC>;
#[doc = "SPI TX Data Register\n\nTDATA \\[7:0\\]: Transmit Data in byte method"]
pub mod spi_txd_8;
#[doc = "spi_rxd register accessor: an alias for `Reg<SPI_RXD_SPEC>`"]
pub type SPI_RXD = crate::Reg<spi_rxd::SPI_RXD_SPEC>;
#[doc = "SPI RX Data Register\n\nRDATA \\[31:0\\]: Receive Data and access in word method"]
pub mod spi_rxd;
#[doc = "spi_rxd_16 register accessor: an alias for `Reg<SPI_RXD_16_SPEC>`"]
pub type SPI_RXD_16 = crate::Reg<spi_rxd_16::SPI_RXD_16_SPEC>;
#[doc = "SPI RX Data Register\n\nRDATA \\[15:0\\]: Receive Data and access in half-word method"]
pub mod spi_rxd_16;
#[doc = "spi_rxd_8 register accessor: an alias for `Reg<SPI_RXD_8_SPEC>`"]
pub type SPI_RXD_8 = crate::Reg<spi_rxd_8::SPI_RXD_8_SPEC>;
#[doc = "SPI RX Data Register\n\nRDATA \\[7:0\\]: Receive Data and access in byte method"]
pub mod spi_rxd_8;
