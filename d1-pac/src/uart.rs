#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_dll: [u8; 0x04],
    _reserved_1_dlh: [u8; 0x04],
    _reserved_2_fcr: [u8; 0x04],
    lcr: LCR,
    mcr: MCR,
    lsr: LSR,
    msr: MSR,
    sch: SCH,
    _reserved8: [u8; 0x5c],
    usr: USR,
    tfl: TFL,
    rfl: RFL,
    hsk: HSK,
    dma_req_en: DMA_REQ_EN,
    _reserved13: [u8; 0x14],
    halt: HALT,
    _reserved14: [u8; 0x08],
    dbg_dll: DBG_DLL,
    dbg_dlh: DBG_DLH,
    _reserved16: [u8; 0x38],
    fcc: FCC,
    _reserved17: [u8; 0x0c],
    rxdma_ctrl: RXDMA_CTRL,
    rxdma_str: RXDMA_STR,
    rxdma_sta: RXDMA_STA,
    rxdma_lmt: RXDMA_LMT,
    rxdma_saddrl: RXDMA_SADDRL,
    rxdma_saddrh: RXDMA_SADDRH,
    rxdma_bl: RXDMA_BL,
    _reserved24: [u8; 0x04],
    rxdma_ie: RXDMA_IE,
    rxdma_is: RXDMA_IS,
    rxdma_waddrl: RXDMA_WADDRL,
    rxdma_waddrh: RXDMA_WADDRH,
    rxdma_raddrl: RXDMA_RADDRL,
    rxdma_raddrh: RXDMA_RADDRH,
    rxdma_dcnt: RXDMA_DCNT,
}
impl RegisterBlock {
    #[doc = "0x00 - UART Divisor Latch Low Register"]
    #[inline(always)]
    pub const fn dll(&self) -> &DLL {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00 - UART Transmit Holding Register"]
    #[inline(always)]
    pub const fn thr(&self) -> &THR {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00 - UART Receive Buffer Register"]
    #[inline(always)]
    pub const fn rbr(&self) -> &RBR {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x04 - UART Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - UART Divisor Latch High Register"]
    #[inline(always)]
    pub const fn dlh(&self) -> &DLH {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x08 - UART FIFO Control Register"]
    #[inline(always)]
    pub const fn fcr(&self) -> &FCR {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - UART Interrupt Identity Register"]
    #[inline(always)]
    pub const fn iir(&self) -> &IIR {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x0c - UART Line Control Register"]
    #[inline(always)]
    pub const fn lcr(&self) -> &LCR {
        &self.lcr
    }
    #[doc = "0x10 - UART Modem Control Register"]
    #[inline(always)]
    pub const fn mcr(&self) -> &MCR {
        &self.mcr
    }
    #[doc = "0x14 - UART Line Status Register"]
    #[inline(always)]
    pub const fn lsr(&self) -> &LSR {
        &self.lsr
    }
    #[doc = "0x18 - UART Modem Status Register"]
    #[inline(always)]
    pub const fn msr(&self) -> &MSR {
        &self.msr
    }
    #[doc = "0x1c - UART Scratch Register"]
    #[inline(always)]
    pub const fn sch(&self) -> &SCH {
        &self.sch
    }
    #[doc = "0x7c - UART Status Register"]
    #[inline(always)]
    pub const fn usr(&self) -> &USR {
        &self.usr
    }
    #[doc = "0x80 - UART Transmit FIFO Level Register"]
    #[inline(always)]
    pub const fn tfl(&self) -> &TFL {
        &self.tfl
    }
    #[doc = "0x84 - UART Receive FIFO Level Register"]
    #[inline(always)]
    pub const fn rfl(&self) -> &RFL {
        &self.rfl
    }
    #[doc = "0x88 - UART DMA Handshake Configuration Register"]
    #[inline(always)]
    pub const fn hsk(&self) -> &HSK {
        &self.hsk
    }
    #[doc = "0x8c - UART DMA Request Enable Register"]
    #[inline(always)]
    pub const fn dma_req_en(&self) -> &DMA_REQ_EN {
        &self.dma_req_en
    }
    #[doc = "0xa4 - UART Halt TX Register"]
    #[inline(always)]
    pub const fn halt(&self) -> &HALT {
        &self.halt
    }
    #[doc = "0xb0 - UART Debug DLL Register"]
    #[inline(always)]
    pub const fn dbg_dll(&self) -> &DBG_DLL {
        &self.dbg_dll
    }
    #[doc = "0xb4 - UART Debug DLH Register"]
    #[inline(always)]
    pub const fn dbg_dlh(&self) -> &DBG_DLH {
        &self.dbg_dlh
    }
    #[doc = "0xf0 - UART FIFO Clock Control Register"]
    #[inline(always)]
    pub const fn fcc(&self) -> &FCC {
        &self.fcc
    }
    #[doc = "0x100 - UART RXDMA Control Register"]
    #[inline(always)]
    pub const fn rxdma_ctrl(&self) -> &RXDMA_CTRL {
        &self.rxdma_ctrl
    }
    #[doc = "0x104 - UART RXDMA Start Register"]
    #[inline(always)]
    pub const fn rxdma_str(&self) -> &RXDMA_STR {
        &self.rxdma_str
    }
    #[doc = "0x108 - UART RXDMA Status Register"]
    #[inline(always)]
    pub const fn rxdma_sta(&self) -> &RXDMA_STA {
        &self.rxdma_sta
    }
    #[doc = "0x10c - UART RXDMA Limit Register"]
    #[inline(always)]
    pub const fn rxdma_lmt(&self) -> &RXDMA_LMT {
        &self.rxdma_lmt
    }
    #[doc = "0x110 - UART RXDMA Buffer Start Address Low Register"]
    #[inline(always)]
    pub const fn rxdma_saddrl(&self) -> &RXDMA_SADDRL {
        &self.rxdma_saddrl
    }
    #[doc = "0x114 - UART RXDMA Buffer Start Address High Register"]
    #[inline(always)]
    pub const fn rxdma_saddrh(&self) -> &RXDMA_SADDRH {
        &self.rxdma_saddrh
    }
    #[doc = "0x118 - UART RXDMA Buffer Length Register"]
    #[inline(always)]
    pub const fn rxdma_bl(&self) -> &RXDMA_BL {
        &self.rxdma_bl
    }
    #[doc = "0x120 - UART RXDMA Interrupt Enable Register"]
    #[inline(always)]
    pub const fn rxdma_ie(&self) -> &RXDMA_IE {
        &self.rxdma_ie
    }
    #[doc = "0x124 - UART RXDMA Interrupt Status Register"]
    #[inline(always)]
    pub const fn rxdma_is(&self) -> &RXDMA_IS {
        &self.rxdma_is
    }
    #[doc = "0x128 - UART RXDMA Write Address Low Register"]
    #[inline(always)]
    pub const fn rxdma_waddrl(&self) -> &RXDMA_WADDRL {
        &self.rxdma_waddrl
    }
    #[doc = "0x12c - UART RXDMA Write Address High Register"]
    #[inline(always)]
    pub const fn rxdma_waddrh(&self) -> &RXDMA_WADDRH {
        &self.rxdma_waddrh
    }
    #[doc = "0x130 - UART RXDMA Read Address Low Register"]
    #[inline(always)]
    pub const fn rxdma_raddrl(&self) -> &RXDMA_RADDRL {
        &self.rxdma_raddrl
    }
    #[doc = "0x134 - UART RXDMA Read Address High Register"]
    #[inline(always)]
    pub const fn rxdma_raddrh(&self) -> &RXDMA_RADDRH {
        &self.rxdma_raddrh
    }
    #[doc = "0x138 - UART RXDMA Data Count Register"]
    #[inline(always)]
    pub const fn rxdma_dcnt(&self) -> &RXDMA_DCNT {
        &self.rxdma_dcnt
    }
}
#[doc = "rbr (r) register accessor: UART Receive Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbr`] module"]
pub type RBR = crate::Reg<rbr::RBR_SPEC>;
#[doc = "UART Receive Buffer Register"]
pub mod rbr;
#[doc = "thr (w) register accessor: UART Transmit Holding Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`thr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thr`] module"]
pub type THR = crate::Reg<thr::THR_SPEC>;
#[doc = "UART Transmit Holding Register"]
pub mod thr;
#[doc = "dll (rw) register accessor: UART Divisor Latch Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dll::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dll::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dll`] module"]
pub type DLL = crate::Reg<dll::DLL_SPEC>;
#[doc = "UART Divisor Latch Low Register"]
pub mod dll;
#[doc = "dlh (rw) register accessor: UART Divisor Latch High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dlh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dlh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dlh`] module"]
pub type DLH = crate::Reg<dlh::DLH_SPEC>;
#[doc = "UART Divisor Latch High Register"]
pub mod dlh;
#[doc = "ier (rw) register accessor: UART Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "UART Interrupt Enable Register"]
pub mod ier;
#[doc = "iir (r) register accessor: UART Interrupt Identity Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iir::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iir`] module"]
pub type IIR = crate::Reg<iir::IIR_SPEC>;
#[doc = "UART Interrupt Identity Register"]
pub mod iir;
#[doc = "fcr (w) register accessor: UART FIFO Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr`] module"]
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
#[doc = "UART FIFO Control Register"]
pub mod fcr;
#[doc = "lcr (rw) register accessor: UART Line Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcr`] module"]
pub type LCR = crate::Reg<lcr::LCR_SPEC>;
#[doc = "UART Line Control Register"]
pub mod lcr;
#[doc = "mcr (rw) register accessor: UART Modem Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr`] module"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "UART Modem Control Register"]
pub mod mcr;
#[doc = "lsr (r) register accessor: UART Line Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsr`] module"]
pub type LSR = crate::Reg<lsr::LSR_SPEC>;
#[doc = "UART Line Status Register"]
pub mod lsr;
#[doc = "msr (r) register accessor: UART Modem Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msr`] module"]
pub type MSR = crate::Reg<msr::MSR_SPEC>;
#[doc = "UART Modem Status Register"]
pub mod msr;
#[doc = "sch (rw) register accessor: UART Scratch Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sch`] module"]
pub type SCH = crate::Reg<sch::SCH_SPEC>;
#[doc = "UART Scratch Register"]
pub mod sch;
#[doc = "usr (r) register accessor: UART Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usr`] module"]
pub type USR = crate::Reg<usr::USR_SPEC>;
#[doc = "UART Status Register"]
pub mod usr;
#[doc = "tfl (r) register accessor: UART Transmit FIFO Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tfl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tfl`] module"]
pub type TFL = crate::Reg<tfl::TFL_SPEC>;
#[doc = "UART Transmit FIFO Level Register"]
pub mod tfl;
#[doc = "rfl (r) register accessor: UART Receive FIFO Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfl`] module"]
pub type RFL = crate::Reg<rfl::RFL_SPEC>;
#[doc = "UART Receive FIFO Level Register"]
pub mod rfl;
#[doc = "hsk (rw) register accessor: UART DMA Handshake Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsk`] module"]
pub type HSK = crate::Reg<hsk::HSK_SPEC>;
#[doc = "UART DMA Handshake Configuration Register"]
pub mod hsk;
#[doc = "dma_req_en (rw) register accessor: UART DMA Request Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_req_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_req_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_req_en`] module"]
pub type DMA_REQ_EN = crate::Reg<dma_req_en::DMA_REQ_EN_SPEC>;
#[doc = "UART DMA Request Enable Register"]
pub mod dma_req_en;
#[doc = "halt (rw) register accessor: UART Halt TX Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`halt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`halt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@halt`] module"]
pub type HALT = crate::Reg<halt::HALT_SPEC>;
#[doc = "UART Halt TX Register"]
pub mod halt;
#[doc = "dbg_dll (r) register accessor: UART Debug DLL Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbg_dll::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_dll`] module"]
pub type DBG_DLL = crate::Reg<dbg_dll::DBG_DLL_SPEC>;
#[doc = "UART Debug DLL Register"]
pub mod dbg_dll;
#[doc = "dbg_dlh (r) register accessor: UART Debug DLH Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbg_dlh::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_dlh`] module"]
pub type DBG_DLH = crate::Reg<dbg_dlh::DBG_DLH_SPEC>;
#[doc = "UART Debug DLH Register"]
pub mod dbg_dlh;
#[doc = "fcc (rw) register accessor: UART FIFO Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcc`] module"]
pub type FCC = crate::Reg<fcc::FCC_SPEC>;
#[doc = "UART FIFO Clock Control Register"]
pub mod fcc;
#[doc = "rxdma_ctrl (rw) register accessor: UART RXDMA Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdma_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdma_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdma_ctrl`] module"]
pub type RXDMA_CTRL = crate::Reg<rxdma_ctrl::RXDMA_CTRL_SPEC>;
#[doc = "UART RXDMA Control Register"]
pub mod rxdma_ctrl;
#[doc = "rxdma_str (rw) register accessor: UART RXDMA Start Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdma_str::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdma_str::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdma_str`] module"]
pub type RXDMA_STR = crate::Reg<rxdma_str::RXDMA_STR_SPEC>;
#[doc = "UART RXDMA Start Register"]
pub mod rxdma_str;
#[doc = "rxdma_sta (rw) register accessor: UART RXDMA Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdma_sta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdma_sta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdma_sta`] module"]
pub type RXDMA_STA = crate::Reg<rxdma_sta::RXDMA_STA_SPEC>;
#[doc = "UART RXDMA Status Register"]
pub mod rxdma_sta;
#[doc = "rxdma_lmt (rw) register accessor: UART RXDMA Limit Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdma_lmt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdma_lmt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdma_lmt`] module"]
pub type RXDMA_LMT = crate::Reg<rxdma_lmt::RXDMA_LMT_SPEC>;
#[doc = "UART RXDMA Limit Register"]
pub mod rxdma_lmt;
#[doc = "rxdma_saddrl (rw) register accessor: UART RXDMA Buffer Start Address Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdma_saddrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdma_saddrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdma_saddrl`] module"]
pub type RXDMA_SADDRL = crate::Reg<rxdma_saddrl::RXDMA_SADDRL_SPEC>;
#[doc = "UART RXDMA Buffer Start Address Low Register"]
pub mod rxdma_saddrl;
#[doc = "rxdma_saddrh (rw) register accessor: UART RXDMA Buffer Start Address High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdma_saddrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdma_saddrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdma_saddrh`] module"]
pub type RXDMA_SADDRH = crate::Reg<rxdma_saddrh::RXDMA_SADDRH_SPEC>;
#[doc = "UART RXDMA Buffer Start Address High Register"]
pub mod rxdma_saddrh;
#[doc = "rxdma_bl (rw) register accessor: UART RXDMA Buffer Length Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdma_bl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdma_bl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdma_bl`] module"]
pub type RXDMA_BL = crate::Reg<rxdma_bl::RXDMA_BL_SPEC>;
#[doc = "UART RXDMA Buffer Length Register"]
pub mod rxdma_bl;
#[doc = "rxdma_ie (rw) register accessor: UART RXDMA Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdma_ie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdma_ie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdma_ie`] module"]
pub type RXDMA_IE = crate::Reg<rxdma_ie::RXDMA_IE_SPEC>;
#[doc = "UART RXDMA Interrupt Enable Register"]
pub mod rxdma_ie;
#[doc = "rxdma_is (rw) register accessor: UART RXDMA Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdma_is::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdma_is::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdma_is`] module"]
pub type RXDMA_IS = crate::Reg<rxdma_is::RXDMA_IS_SPEC>;
#[doc = "UART RXDMA Interrupt Status Register"]
pub mod rxdma_is;
#[doc = "rxdma_waddrl (r) register accessor: UART RXDMA Write Address Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdma_waddrl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdma_waddrl`] module"]
pub type RXDMA_WADDRL = crate::Reg<rxdma_waddrl::RXDMA_WADDRL_SPEC>;
#[doc = "UART RXDMA Write Address Low Register"]
pub mod rxdma_waddrl;
#[doc = "rxdma_waddrh (r) register accessor: UART RXDMA Write Address High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdma_waddrh::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdma_waddrh`] module"]
pub type RXDMA_WADDRH = crate::Reg<rxdma_waddrh::RXDMA_WADDRH_SPEC>;
#[doc = "UART RXDMA Write Address High Register"]
pub mod rxdma_waddrh;
#[doc = "rxdma_raddrl (rw) register accessor: UART RXDMA Read Address Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdma_raddrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdma_raddrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdma_raddrl`] module"]
pub type RXDMA_RADDRL = crate::Reg<rxdma_raddrl::RXDMA_RADDRL_SPEC>;
#[doc = "UART RXDMA Read Address Low Register"]
pub mod rxdma_raddrl;
#[doc = "rxdma_raddrh (rw) register accessor: UART RXDMA Read Address High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdma_raddrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdma_raddrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdma_raddrh`] module"]
pub type RXDMA_RADDRH = crate::Reg<rxdma_raddrh::RXDMA_RADDRH_SPEC>;
#[doc = "UART RXDMA Read Address High Register"]
pub mod rxdma_raddrh;
#[doc = "rxdma_dcnt (rw) register accessor: UART RXDMA Data Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdma_dcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdma_dcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdma_dcnt`] module"]
pub type RXDMA_DCNT = crate::Reg<rxdma_dcnt::RXDMA_DCNT_SPEC>;
#[doc = "UART RXDMA Data Count Register"]
pub mod rxdma_dcnt;
