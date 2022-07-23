#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_dll: [u8; 0x04],
    _reserved_1_dlh: [u8; 0x04],
    _reserved_2_fcr: [u8; 0x04],
    #[doc = "0x0c - UART Line Control Register"]
    pub lcr: crate::Reg<lcr::LCR_SPEC>,
    #[doc = "0x10 - UART Modem Control Register"]
    pub mcr: crate::Reg<mcr::MCR_SPEC>,
    #[doc = "0x14 - UART Line Status Register"]
    pub lsr: crate::Reg<lsr::LSR_SPEC>,
    #[doc = "0x18 - UART Modem Status Register"]
    pub msr: crate::Reg<msr::MSR_SPEC>,
    #[doc = "0x1c - UART Scratch Register"]
    pub sch: crate::Reg<sch::SCH_SPEC>,
    _reserved8: [u8; 0x5c],
    #[doc = "0x7c - UART Status Register"]
    pub usr: crate::Reg<usr::USR_SPEC>,
    #[doc = "0x80 - UART Transmit FIFO Level Register"]
    pub tfl: crate::Reg<tfl::TFL_SPEC>,
    #[doc = "0x84 - UART Receive FIFO Level Register"]
    pub rfl: crate::Reg<rfl::RFL_SPEC>,
    #[doc = "0x88 - UART DMA Handshake Configuration Register"]
    pub hsk: crate::Reg<hsk::HSK_SPEC>,
    #[doc = "0x8c - UART DMA Request Enable Register"]
    pub dma_req_en: crate::Reg<dma_req_en::DMA_REQ_EN_SPEC>,
    _reserved13: [u8; 0x14],
    #[doc = "0xa4 - UART Halt TX Register"]
    pub halt: crate::Reg<halt::HALT_SPEC>,
    _reserved14: [u8; 0x08],
    #[doc = "0xb0 - UART Debug DLL Register"]
    pub dbg_dll: crate::Reg<dbg_dll::DBG_DLL_SPEC>,
    #[doc = "0xb4 - UART Debug DLH Register"]
    pub dbg_dlh: crate::Reg<dbg_dlh::DBG_DLH_SPEC>,
    _reserved16: [u8; 0x38],
    #[doc = "0xf0 - UART FIFO Clock Control Register"]
    pub fcc: crate::Reg<fcc::FCC_SPEC>,
    _reserved17: [u8; 0x0c],
    #[doc = "0x100 - UART RXDMA Control Register"]
    pub rxdma_ctrl: crate::Reg<rxdma_ctrl::RXDMA_CTRL_SPEC>,
    #[doc = "0x104 - UART RXDMA Start Register"]
    pub rxdma_str: crate::Reg<rxdma_str::RXDMA_STR_SPEC>,
    #[doc = "0x108 - UART RXDMA Status Register"]
    pub rxdma_sta: crate::Reg<rxdma_sta::RXDMA_STA_SPEC>,
    #[doc = "0x10c - UART RXDMA Limit Register"]
    pub rxdma_lmt: crate::Reg<rxdma_lmt::RXDMA_LMT_SPEC>,
    #[doc = "0x110 - UART RXDMA Buffer Start Address Low Register"]
    pub rxdma_saddrl: crate::Reg<rxdma_saddrl::RXDMA_SADDRL_SPEC>,
    #[doc = "0x114 - UART RXDMA Buffer Start Address High Register"]
    pub rxdma_saddrh: crate::Reg<rxdma_saddrh::RXDMA_SADDRH_SPEC>,
    #[doc = "0x118 - UART RXDMA Buffer Length Register"]
    pub rxdma_bl: crate::Reg<rxdma_bl::RXDMA_BL_SPEC>,
    _reserved24: [u8; 0x04],
    #[doc = "0x120 - UART RXDMA Interrupt Enable Register"]
    pub rxdma_ie: crate::Reg<rxdma_ie::RXDMA_IE_SPEC>,
    #[doc = "0x124 - UART RXDMA Interrupt Status Register"]
    pub rxdma_is: crate::Reg<rxdma_is::RXDMA_IS_SPEC>,
    #[doc = "0x128 - UART RXDMA Write Address Low Register"]
    pub rxdma_waddrl: crate::Reg<rxdma_waddrl::RXDMA_WADDRL_SPEC>,
    #[doc = "0x12c - UART RXDMA Write Address High Register"]
    pub rxdma_waddrh: crate::Reg<rxdma_waddrh::RXDMA_WADDRH_SPEC>,
    #[doc = "0x130 - UART RXDMA Read Address Low Register"]
    pub rxdma_raddrl: crate::Reg<rxdma_raddrl::RXDMA_RADDRL_SPEC>,
    #[doc = "0x134 - UART RXDMA Read Address High Register"]
    pub rxdma_raddrh: crate::Reg<rxdma_raddrh::RXDMA_RADDRH_SPEC>,
    #[doc = "0x138 - UART RXDMA Data Count Register"]
    pub rxdma_dcnt: crate::Reg<rxdma_dcnt::RXDMA_DCNT_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x00 - UART Divisor Latch Low Register"]
    #[inline(always)]
    pub fn dll(&self) -> &crate::Reg<dll::DLL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize) as *const crate::Reg<dll::DLL_SPEC>)
        }
    }
    #[doc = "0x00 - UART Transmit Holding Register"]
    #[inline(always)]
    pub fn thr(&self) -> &crate::Reg<thr::THR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize) as *const crate::Reg<thr::THR_SPEC>)
        }
    }
    #[doc = "0x00 - UART Receive Buffer Register"]
    #[inline(always)]
    pub fn rbr(&self) -> &crate::Reg<rbr::RBR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize) as *const crate::Reg<rbr::RBR_SPEC>)
        }
    }
    #[doc = "0x04 - UART Interrupt Enable Register"]
    #[inline(always)]
    pub fn ier(&self) -> &crate::Reg<ier::IER_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize) as *const crate::Reg<ier::IER_SPEC>)
        }
    }
    #[doc = "0x04 - UART Divisor Latch High Register"]
    #[inline(always)]
    pub fn dlh(&self) -> &crate::Reg<dlh::DLH_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize) as *const crate::Reg<dlh::DLH_SPEC>)
        }
    }
    #[doc = "0x08 - UART FIFO Control Register"]
    #[inline(always)]
    pub fn fcr(&self) -> &crate::Reg<fcr::FCR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(8usize) as *const crate::Reg<fcr::FCR_SPEC>)
        }
    }
    #[doc = "0x08 - UART Interrupt Identity Register"]
    #[inline(always)]
    pub fn iir(&self) -> &crate::Reg<iir::IIR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(8usize) as *const crate::Reg<iir::IIR_SPEC>)
        }
    }
}
#[doc = "rbr register accessor: an alias for `Reg<RBR_SPEC>`"]
pub type RBR = crate::Reg<rbr::RBR_SPEC>;
#[doc = "UART Receive Buffer Register"]
pub mod rbr;
#[doc = "thr register accessor: an alias for `Reg<THR_SPEC>`"]
pub type THR = crate::Reg<thr::THR_SPEC>;
#[doc = "UART Transmit Holding Register"]
pub mod thr;
#[doc = "dll register accessor: an alias for `Reg<DLL_SPEC>`"]
pub type DLL = crate::Reg<dll::DLL_SPEC>;
#[doc = "UART Divisor Latch Low Register"]
pub mod dll;
#[doc = "dlh register accessor: an alias for `Reg<DLH_SPEC>`"]
pub type DLH = crate::Reg<dlh::DLH_SPEC>;
#[doc = "UART Divisor Latch High Register"]
pub mod dlh;
#[doc = "ier register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "UART Interrupt Enable Register"]
pub mod ier;
#[doc = "iir register accessor: an alias for `Reg<IIR_SPEC>`"]
pub type IIR = crate::Reg<iir::IIR_SPEC>;
#[doc = "UART Interrupt Identity Register"]
pub mod iir;
#[doc = "fcr register accessor: an alias for `Reg<FCR_SPEC>`"]
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
#[doc = "UART FIFO Control Register"]
pub mod fcr;
#[doc = "lcr register accessor: an alias for `Reg<LCR_SPEC>`"]
pub type LCR = crate::Reg<lcr::LCR_SPEC>;
#[doc = "UART Line Control Register"]
pub mod lcr;
#[doc = "mcr register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "UART Modem Control Register"]
pub mod mcr;
#[doc = "lsr register accessor: an alias for `Reg<LSR_SPEC>`"]
pub type LSR = crate::Reg<lsr::LSR_SPEC>;
#[doc = "UART Line Status Register"]
pub mod lsr;
#[doc = "msr register accessor: an alias for `Reg<MSR_SPEC>`"]
pub type MSR = crate::Reg<msr::MSR_SPEC>;
#[doc = "UART Modem Status Register"]
pub mod msr;
#[doc = "sch register accessor: an alias for `Reg<SCH_SPEC>`"]
pub type SCH = crate::Reg<sch::SCH_SPEC>;
#[doc = "UART Scratch Register"]
pub mod sch;
#[doc = "usr register accessor: an alias for `Reg<USR_SPEC>`"]
pub type USR = crate::Reg<usr::USR_SPEC>;
#[doc = "UART Status Register"]
pub mod usr;
#[doc = "tfl register accessor: an alias for `Reg<TFL_SPEC>`"]
pub type TFL = crate::Reg<tfl::TFL_SPEC>;
#[doc = "UART Transmit FIFO Level Register"]
pub mod tfl;
#[doc = "rfl register accessor: an alias for `Reg<RFL_SPEC>`"]
pub type RFL = crate::Reg<rfl::RFL_SPEC>;
#[doc = "UART Receive FIFO Level Register"]
pub mod rfl;
#[doc = "hsk register accessor: an alias for `Reg<HSK_SPEC>`"]
pub type HSK = crate::Reg<hsk::HSK_SPEC>;
#[doc = "UART DMA Handshake Configuration Register"]
pub mod hsk;
#[doc = "dma_req_en register accessor: an alias for `Reg<DMA_REQ_EN_SPEC>`"]
pub type DMA_REQ_EN = crate::Reg<dma_req_en::DMA_REQ_EN_SPEC>;
#[doc = "UART DMA Request Enable Register"]
pub mod dma_req_en;
#[doc = "halt register accessor: an alias for `Reg<HALT_SPEC>`"]
pub type HALT = crate::Reg<halt::HALT_SPEC>;
#[doc = "UART Halt TX Register"]
pub mod halt;
#[doc = "dbg_dll register accessor: an alias for `Reg<DBG_DLL_SPEC>`"]
pub type DBG_DLL = crate::Reg<dbg_dll::DBG_DLL_SPEC>;
#[doc = "UART Debug DLL Register"]
pub mod dbg_dll;
#[doc = "dbg_dlh register accessor: an alias for `Reg<DBG_DLH_SPEC>`"]
pub type DBG_DLH = crate::Reg<dbg_dlh::DBG_DLH_SPEC>;
#[doc = "UART Debug DLH Register"]
pub mod dbg_dlh;
#[doc = "fcc register accessor: an alias for `Reg<FCC_SPEC>`"]
pub type FCC = crate::Reg<fcc::FCC_SPEC>;
#[doc = "UART FIFO Clock Control Register"]
pub mod fcc;
#[doc = "rxdma_ctrl register accessor: an alias for `Reg<RXDMA_CTRL_SPEC>`"]
pub type RXDMA_CTRL = crate::Reg<rxdma_ctrl::RXDMA_CTRL_SPEC>;
#[doc = "UART RXDMA Control Register"]
pub mod rxdma_ctrl;
#[doc = "rxdma_str register accessor: an alias for `Reg<RXDMA_STR_SPEC>`"]
pub type RXDMA_STR = crate::Reg<rxdma_str::RXDMA_STR_SPEC>;
#[doc = "UART RXDMA Start Register"]
pub mod rxdma_str;
#[doc = "rxdma_sta register accessor: an alias for `Reg<RXDMA_STA_SPEC>`"]
pub type RXDMA_STA = crate::Reg<rxdma_sta::RXDMA_STA_SPEC>;
#[doc = "UART RXDMA Status Register"]
pub mod rxdma_sta;
#[doc = "rxdma_lmt register accessor: an alias for `Reg<RXDMA_LMT_SPEC>`"]
pub type RXDMA_LMT = crate::Reg<rxdma_lmt::RXDMA_LMT_SPEC>;
#[doc = "UART RXDMA Limit Register"]
pub mod rxdma_lmt;
#[doc = "rxdma_saddrl register accessor: an alias for `Reg<RXDMA_SADDRL_SPEC>`"]
pub type RXDMA_SADDRL = crate::Reg<rxdma_saddrl::RXDMA_SADDRL_SPEC>;
#[doc = "UART RXDMA Buffer Start Address Low Register"]
pub mod rxdma_saddrl;
#[doc = "rxdma_saddrh register accessor: an alias for `Reg<RXDMA_SADDRH_SPEC>`"]
pub type RXDMA_SADDRH = crate::Reg<rxdma_saddrh::RXDMA_SADDRH_SPEC>;
#[doc = "UART RXDMA Buffer Start Address High Register"]
pub mod rxdma_saddrh;
#[doc = "rxdma_bl register accessor: an alias for `Reg<RXDMA_BL_SPEC>`"]
pub type RXDMA_BL = crate::Reg<rxdma_bl::RXDMA_BL_SPEC>;
#[doc = "UART RXDMA Buffer Length Register"]
pub mod rxdma_bl;
#[doc = "rxdma_ie register accessor: an alias for `Reg<RXDMA_IE_SPEC>`"]
pub type RXDMA_IE = crate::Reg<rxdma_ie::RXDMA_IE_SPEC>;
#[doc = "UART RXDMA Interrupt Enable Register"]
pub mod rxdma_ie;
#[doc = "rxdma_is register accessor: an alias for `Reg<RXDMA_IS_SPEC>`"]
pub type RXDMA_IS = crate::Reg<rxdma_is::RXDMA_IS_SPEC>;
#[doc = "UART RXDMA Interrupt Status Register"]
pub mod rxdma_is;
#[doc = "rxdma_waddrl register accessor: an alias for `Reg<RXDMA_WADDRL_SPEC>`"]
pub type RXDMA_WADDRL = crate::Reg<rxdma_waddrl::RXDMA_WADDRL_SPEC>;
#[doc = "UART RXDMA Write Address Low Register"]
pub mod rxdma_waddrl;
#[doc = "rxdma_waddrh register accessor: an alias for `Reg<RXDMA_WADDRH_SPEC>`"]
pub type RXDMA_WADDRH = crate::Reg<rxdma_waddrh::RXDMA_WADDRH_SPEC>;
#[doc = "UART RXDMA Write Address High Register"]
pub mod rxdma_waddrh;
#[doc = "rxdma_raddrl register accessor: an alias for `Reg<RXDMA_RADDRL_SPEC>`"]
pub type RXDMA_RADDRL = crate::Reg<rxdma_raddrl::RXDMA_RADDRL_SPEC>;
#[doc = "UART RXDMA Read Address Low Register"]
pub mod rxdma_raddrl;
#[doc = "rxdma_raddrh register accessor: an alias for `Reg<RXDMA_RADDRH_SPEC>`"]
pub type RXDMA_RADDRH = crate::Reg<rxdma_raddrh::RXDMA_RADDRH_SPEC>;
#[doc = "UART RXDMA Read Address High Register"]
pub mod rxdma_raddrh;
#[doc = "rxdma_dcnt register accessor: an alias for `Reg<RXDMA_DCNT_SPEC>`"]
pub type RXDMA_DCNT = crate::Reg<rxdma_dcnt::RXDMA_DCNT_SPEC>;
#[doc = "UART RXDMA Data Count Register"]
pub mod rxdma_dcnt;
