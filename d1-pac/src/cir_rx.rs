#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    cir_ctl: CIR_CTL,
    _reserved1: [u8; 0x0c],
    cir_rxpcfg: CIR_RXPCFG,
    _reserved2: [u8; 0x0c],
    cir_rxfifo: CIR_RXFIFO,
    _reserved3: [u8; 0x08],
    cir_rxint: CIR_RXINT,
    cir_rxsta: CIR_RXSTA,
    cir_rxcfg: CIR_RXCFG,
}
impl RegisterBlock {
    #[doc = "0x00 - CIR Control Register"]
    #[inline(always)]
    pub const fn cir_ctl(&self) -> &CIR_CTL {
        &self.cir_ctl
    }
    #[doc = "0x10 - CIR Receiver Pulse Configure Register"]
    #[inline(always)]
    pub const fn cir_rxpcfg(&self) -> &CIR_RXPCFG {
        &self.cir_rxpcfg
    }
    #[doc = "0x20 - CIR Receiver FIFO Register"]
    #[inline(always)]
    pub const fn cir_rxfifo(&self) -> &CIR_RXFIFO {
        &self.cir_rxfifo
    }
    #[doc = "0x2c - CIR Receiver Interrupt Control Register"]
    #[inline(always)]
    pub const fn cir_rxint(&self) -> &CIR_RXINT {
        &self.cir_rxint
    }
    #[doc = "0x30 - CIR Receiver Status Register"]
    #[inline(always)]
    pub const fn cir_rxsta(&self) -> &CIR_RXSTA {
        &self.cir_rxsta
    }
    #[doc = "0x34 - CIR Receiver Configure Register"]
    #[inline(always)]
    pub const fn cir_rxcfg(&self) -> &CIR_RXCFG {
        &self.cir_rxcfg
    }
}
#[doc = "cir_ctl (rw) register accessor: CIR Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cir_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cir_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cir_ctl`] module"]
pub type CIR_CTL = crate::Reg<cir_ctl::CIR_CTL_SPEC>;
#[doc = "CIR Control Register"]
pub mod cir_ctl;
#[doc = "cir_rxpcfg (rw) register accessor: CIR Receiver Pulse Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cir_rxpcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cir_rxpcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cir_rxpcfg`] module"]
pub type CIR_RXPCFG = crate::Reg<cir_rxpcfg::CIR_RXPCFG_SPEC>;
#[doc = "CIR Receiver Pulse Configure Register"]
pub mod cir_rxpcfg;
#[doc = "cir_rxfifo (rw) register accessor: CIR Receiver FIFO Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cir_rxfifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cir_rxfifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cir_rxfifo`] module"]
pub type CIR_RXFIFO = crate::Reg<cir_rxfifo::CIR_RXFIFO_SPEC>;
#[doc = "CIR Receiver FIFO Register"]
pub mod cir_rxfifo;
#[doc = "cir_rxint (rw) register accessor: CIR Receiver Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cir_rxint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cir_rxint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cir_rxint`] module"]
pub type CIR_RXINT = crate::Reg<cir_rxint::CIR_RXINT_SPEC>;
#[doc = "CIR Receiver Interrupt Control Register"]
pub mod cir_rxint;
#[doc = "cir_rxsta (rw) register accessor: CIR Receiver Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cir_rxsta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cir_rxsta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cir_rxsta`] module"]
pub type CIR_RXSTA = crate::Reg<cir_rxsta::CIR_RXSTA_SPEC>;
#[doc = "CIR Receiver Status Register"]
pub mod cir_rxsta;
#[doc = "cir_rxcfg (rw) register accessor: CIR Receiver Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cir_rxcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cir_rxcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cir_rxcfg`] module"]
pub type CIR_RXCFG = crate::Reg<cir_rxcfg::CIR_RXCFG_SPEC>;
#[doc = "CIR Receiver Configure Register"]
pub mod cir_rxcfg;
