#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    i2s_pcm_ctl: I2S_PCM_CTL,
    i2s_pcm_fmt0: I2S_PCM_FMT0,
    i2s_pcm_fmt1: I2S_PCM_FMT1,
    i2s_pcm_ista: I2S_PCM_ISTA,
    i2s_pcm_rxfifo: I2S_PCM_RXFIFO,
    i2s_pcm_fctl: I2S_PCM_FCTL,
    i2s_pcm_fsta: I2S_PCM_FSTA,
    i2s_pcm_int: I2S_PCM_INT,
    i2s_pcm_txfifo: I2S_PCM_TXFIFO,
    i2s_pcm_clkd: I2S_PCM_CLKD,
    i2s_pcm_txcnt: I2S_PCM_TXCNT,
    i2s_pcm_rxcnt: I2S_PCM_RXCNT,
    i2s_pcm_chcfg: I2S_PCM_CHCFG,
    i2s_pcm_tx0chsel: I2S_PCM_TX0CHSEL,
    i2s_pcm_tx1chsel: I2S_PCM_TX1CHSEL,
    i2s_pcm_tx2chsel: I2S_PCM_TX2CHSEL,
    i2s_pcm_tx3chsel: I2S_PCM_TX3CHSEL,
    i2s_pcm_tx0chmap0: I2S_PCM_TX0CHMAP0,
    i2s_pcm_tx0chmap1: I2S_PCM_TX0CHMAP1,
    i2s_pcm_tx1chmap0: I2S_PCM_TX1CHMAP0,
    i2s_pcm_tx1chmap1: I2S_PCM_TX1CHMAP1,
    i2s_pcm_tx2chmap0: I2S_PCM_TX2CHMAP0,
    i2s_pcm_tx2chmap1: I2S_PCM_TX2CHMAP1,
    i2s_pcm_tx3chmap0: I2S_PCM_TX3CHMAP0,
    i2s_pcm_tx3chmap1: I2S_PCM_TX3CHMAP1,
    i2s_pcm_rxchsel: I2S_PCM_RXCHSEL,
    i2s_pcm_rxchmap0: I2S_PCM_RXCHMAP0,
    i2s_pcm_rxchmap1: I2S_PCM_RXCHMAP1,
    i2s_pcm_rxchmap2: I2S_PCM_RXCHMAP2,
    i2s_pcm_rxchmap3: I2S_PCM_RXCHMAP3,
    _reserved30: [u8; 0x08],
    mclkcfg: MCLKCFG,
    fsout_cfg: FSOUT_CFG,
    fsin_extcfg: FSIN_EXTCFG,
    asrcen: ASRCEN,
    asrcmancfg: ASRCMANCFG,
    asrcratiostat: ASRCRATIOSTAT,
    asrcfifostat: ASRCFIFOSTAT,
    asrcmbistcfg: ASRCMBISTCFG,
    asrcmbiststat: ASRCMBISTSTAT,
}
impl RegisterBlock {
    #[doc = "0x00 - I2S/PCM Control Register"]
    #[inline(always)]
    pub const fn i2s_pcm_ctl(&self) -> &I2S_PCM_CTL {
        &self.i2s_pcm_ctl
    }
    #[doc = "0x04 - I2S/PCM Format Register 0"]
    #[inline(always)]
    pub const fn i2s_pcm_fmt0(&self) -> &I2S_PCM_FMT0 {
        &self.i2s_pcm_fmt0
    }
    #[doc = "0x08 - I2S/PCM Format Register 1"]
    #[inline(always)]
    pub const fn i2s_pcm_fmt1(&self) -> &I2S_PCM_FMT1 {
        &self.i2s_pcm_fmt1
    }
    #[doc = "0x0c - I2S/PCM Interrupt Status Register"]
    #[inline(always)]
    pub const fn i2s_pcm_ista(&self) -> &I2S_PCM_ISTA {
        &self.i2s_pcm_ista
    }
    #[doc = "0x10 - I2S/PCM RXFIFO Register"]
    #[inline(always)]
    pub const fn i2s_pcm_rxfifo(&self) -> &I2S_PCM_RXFIFO {
        &self.i2s_pcm_rxfifo
    }
    #[doc = "0x14 - I2S/PCM FIFO Control Register"]
    #[inline(always)]
    pub const fn i2s_pcm_fctl(&self) -> &I2S_PCM_FCTL {
        &self.i2s_pcm_fctl
    }
    #[doc = "0x18 - I2S/PCM FIFO Status Register"]
    #[inline(always)]
    pub const fn i2s_pcm_fsta(&self) -> &I2S_PCM_FSTA {
        &self.i2s_pcm_fsta
    }
    #[doc = "0x1c - I2S/PCM DMA and Interrupt Control Register"]
    #[inline(always)]
    pub const fn i2s_pcm_int(&self) -> &I2S_PCM_INT {
        &self.i2s_pcm_int
    }
    #[doc = "0x20 - I2S/PCM TXFIFO Register"]
    #[inline(always)]
    pub const fn i2s_pcm_txfifo(&self) -> &I2S_PCM_TXFIFO {
        &self.i2s_pcm_txfifo
    }
    #[doc = "0x24 - I2S/PCM Clock Divide Register"]
    #[inline(always)]
    pub const fn i2s_pcm_clkd(&self) -> &I2S_PCM_CLKD {
        &self.i2s_pcm_clkd
    }
    #[doc = "0x28 - I2S/PCM TX Sample Counter Register"]
    #[inline(always)]
    pub const fn i2s_pcm_txcnt(&self) -> &I2S_PCM_TXCNT {
        &self.i2s_pcm_txcnt
    }
    #[doc = "0x2c - I2S/PCM RX Sample Counter Register"]
    #[inline(always)]
    pub const fn i2s_pcm_rxcnt(&self) -> &I2S_PCM_RXCNT {
        &self.i2s_pcm_rxcnt
    }
    #[doc = "0x30 - I2S/PCM Channel Configuration Register"]
    #[inline(always)]
    pub const fn i2s_pcm_chcfg(&self) -> &I2S_PCM_CHCFG {
        &self.i2s_pcm_chcfg
    }
    #[doc = "0x34 - I2S/PCM TX0 Channel Select Register"]
    #[inline(always)]
    pub const fn i2s_pcm_tx0chsel(&self) -> &I2S_PCM_TX0CHSEL {
        &self.i2s_pcm_tx0chsel
    }
    #[doc = "0x38 - I2S/PCM TX1 Channel Select Register"]
    #[inline(always)]
    pub const fn i2s_pcm_tx1chsel(&self) -> &I2S_PCM_TX1CHSEL {
        &self.i2s_pcm_tx1chsel
    }
    #[doc = "0x3c - I2S/PCM TX2 Channel Select Register"]
    #[inline(always)]
    pub const fn i2s_pcm_tx2chsel(&self) -> &I2S_PCM_TX2CHSEL {
        &self.i2s_pcm_tx2chsel
    }
    #[doc = "0x40 - I2S/PCM TX3 Channel Select Register"]
    #[inline(always)]
    pub const fn i2s_pcm_tx3chsel(&self) -> &I2S_PCM_TX3CHSEL {
        &self.i2s_pcm_tx3chsel
    }
    #[doc = "0x44 - I2S/PCM TX0 Channel Mapping Register0"]
    #[inline(always)]
    pub const fn i2s_pcm_tx0chmap0(&self) -> &I2S_PCM_TX0CHMAP0 {
        &self.i2s_pcm_tx0chmap0
    }
    #[doc = "0x48 - I2S/PCM TX0 Channel Mapping Register1"]
    #[inline(always)]
    pub const fn i2s_pcm_tx0chmap1(&self) -> &I2S_PCM_TX0CHMAP1 {
        &self.i2s_pcm_tx0chmap1
    }
    #[doc = "0x4c - I2S/PCM TX1 Channel Mapping Register0"]
    #[inline(always)]
    pub const fn i2s_pcm_tx1chmap0(&self) -> &I2S_PCM_TX1CHMAP0 {
        &self.i2s_pcm_tx1chmap0
    }
    #[doc = "0x50 - I2S/PCM TX1 Channel Mapping Register1"]
    #[inline(always)]
    pub const fn i2s_pcm_tx1chmap1(&self) -> &I2S_PCM_TX1CHMAP1 {
        &self.i2s_pcm_tx1chmap1
    }
    #[doc = "0x54 - I2S/PCM TX2 Channel Mapping Register0"]
    #[inline(always)]
    pub const fn i2s_pcm_tx2chmap0(&self) -> &I2S_PCM_TX2CHMAP0 {
        &self.i2s_pcm_tx2chmap0
    }
    #[doc = "0x58 - I2S/PCM TX2 Channel Mapping Register1"]
    #[inline(always)]
    pub const fn i2s_pcm_tx2chmap1(&self) -> &I2S_PCM_TX2CHMAP1 {
        &self.i2s_pcm_tx2chmap1
    }
    #[doc = "0x5c - I2S/PCM TX3 Channel Mapping Register0"]
    #[inline(always)]
    pub const fn i2s_pcm_tx3chmap0(&self) -> &I2S_PCM_TX3CHMAP0 {
        &self.i2s_pcm_tx3chmap0
    }
    #[doc = "0x60 - I2S/PCM TX3 Channel Mapping Register1"]
    #[inline(always)]
    pub const fn i2s_pcm_tx3chmap1(&self) -> &I2S_PCM_TX3CHMAP1 {
        &self.i2s_pcm_tx3chmap1
    }
    #[doc = "0x64 - I2S/PCM RX Channel Select Register"]
    #[inline(always)]
    pub const fn i2s_pcm_rxchsel(&self) -> &I2S_PCM_RXCHSEL {
        &self.i2s_pcm_rxchsel
    }
    #[doc = "0x68 - I2S/PCM RX Channel Mapping Register0"]
    #[inline(always)]
    pub const fn i2s_pcm_rxchmap0(&self) -> &I2S_PCM_RXCHMAP0 {
        &self.i2s_pcm_rxchmap0
    }
    #[doc = "0x6c - I2S/PCM RX Channel Mapping Register1"]
    #[inline(always)]
    pub const fn i2s_pcm_rxchmap1(&self) -> &I2S_PCM_RXCHMAP1 {
        &self.i2s_pcm_rxchmap1
    }
    #[doc = "0x70 - I2S/PCM RX Channel Mapping Register2"]
    #[inline(always)]
    pub const fn i2s_pcm_rxchmap2(&self) -> &I2S_PCM_RXCHMAP2 {
        &self.i2s_pcm_rxchmap2
    }
    #[doc = "0x74 - I2S/PCM RX Channel Mapping Register3"]
    #[inline(always)]
    pub const fn i2s_pcm_rxchmap3(&self) -> &I2S_PCM_RXCHMAP3 {
        &self.i2s_pcm_rxchmap3
    }
    #[doc = "0x80 - ASRC MCLK Configuration Register"]
    #[inline(always)]
    pub const fn mclkcfg(&self) -> &MCLKCFG {
        &self.mclkcfg
    }
    #[doc = "0x84 - ASRC Out Sample Rate Configuration Register"]
    #[inline(always)]
    pub const fn fsout_cfg(&self) -> &FSOUT_CFG {
        &self.fsout_cfg
    }
    #[doc = "0x88 - ASRC Input Sample Pulse Extend Configuration Register"]
    #[inline(always)]
    pub const fn fsin_extcfg(&self) -> &FSIN_EXTCFG {
        &self.fsin_extcfg
    }
    #[doc = "0x8c - ASRC Enable Register"]
    #[inline(always)]
    pub const fn asrcen(&self) -> &ASRCEN {
        &self.asrcen
    }
    #[doc = "0x90 - ASRC Manual Ratio Configuration Register"]
    #[inline(always)]
    pub const fn asrcmancfg(&self) -> &ASRCMANCFG {
        &self.asrcmancfg
    }
    #[doc = "0x94 - ASRC Status Register"]
    #[inline(always)]
    pub const fn asrcratiostat(&self) -> &ASRCRATIOSTAT {
        &self.asrcratiostat
    }
    #[doc = "0x98 - ASRC FIFO Level Status Register"]
    #[inline(always)]
    pub const fn asrcfifostat(&self) -> &ASRCFIFOSTAT {
        &self.asrcfifostat
    }
    #[doc = "0x9c - ASRC MBIST Test Configuration Register"]
    #[inline(always)]
    pub const fn asrcmbistcfg(&self) -> &ASRCMBISTCFG {
        &self.asrcmbistcfg
    }
    #[doc = "0xa0 - ASRC MBIST Test Status Register"]
    #[inline(always)]
    pub const fn asrcmbiststat(&self) -> &ASRCMBISTSTAT {
        &self.asrcmbiststat
    }
}
#[doc = "i2s_pcm_ctl (rw) register accessor: I2S/PCM Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_pcm_ctl`] module"]
pub type I2S_PCM_CTL = crate::Reg<i2s_pcm_ctl::I2S_PCM_CTL_SPEC>;
#[doc = "I2S/PCM Control Register"]
pub mod i2s_pcm_ctl;
#[doc = "i2s_pcm_fmt0 (rw) register accessor: I2S/PCM Format Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_fmt0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_fmt0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_pcm_fmt0`] module"]
pub type I2S_PCM_FMT0 = crate::Reg<i2s_pcm_fmt0::I2S_PCM_FMT0_SPEC>;
#[doc = "I2S/PCM Format Register 0"]
pub mod i2s_pcm_fmt0;
#[doc = "i2s_pcm_fmt1 (rw) register accessor: I2S/PCM Format Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_fmt1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_fmt1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_pcm_fmt1`] module"]
pub type I2S_PCM_FMT1 = crate::Reg<i2s_pcm_fmt1::I2S_PCM_FMT1_SPEC>;
#[doc = "I2S/PCM Format Register 1"]
pub mod i2s_pcm_fmt1;
#[doc = "i2s_pcm_ista (rw) register accessor: I2S/PCM Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_ista::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_ista::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_pcm_ista`] module"]
pub type I2S_PCM_ISTA = crate::Reg<i2s_pcm_ista::I2S_PCM_ISTA_SPEC>;
#[doc = "I2S/PCM Interrupt Status Register"]
pub mod i2s_pcm_ista;
#[doc = "i2s_pcm_rxfifo (rw) register accessor: I2S/PCM RXFIFO Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_rxfifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_rxfifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_pcm_rxfifo`] module"]
pub type I2S_PCM_RXFIFO = crate::Reg<i2s_pcm_rxfifo::I2S_PCM_RXFIFO_SPEC>;
#[doc = "I2S/PCM RXFIFO Register"]
pub mod i2s_pcm_rxfifo;
#[doc = "i2s_pcm_fctl (rw) register accessor: I2S/PCM FIFO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_fctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_fctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_pcm_fctl`] module"]
pub type I2S_PCM_FCTL = crate::Reg<i2s_pcm_fctl::I2S_PCM_FCTL_SPEC>;
#[doc = "I2S/PCM FIFO Control Register"]
pub mod i2s_pcm_fctl;
#[doc = "i2s_pcm_fsta (rw) register accessor: I2S/PCM FIFO Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_fsta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_fsta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_pcm_fsta`] module"]
pub type I2S_PCM_FSTA = crate::Reg<i2s_pcm_fsta::I2S_PCM_FSTA_SPEC>;
#[doc = "I2S/PCM FIFO Status Register"]
pub mod i2s_pcm_fsta;
#[doc = "i2s_pcm_int (rw) register accessor: I2S/PCM DMA and Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_pcm_int`] module"]
pub type I2S_PCM_INT = crate::Reg<i2s_pcm_int::I2S_PCM_INT_SPEC>;
#[doc = "I2S/PCM DMA and Interrupt Control Register"]
pub mod i2s_pcm_int;
#[doc = "i2s_pcm_txfifo (rw) register accessor: I2S/PCM TXFIFO Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_txfifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_txfifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_pcm_txfifo`] module"]
pub type I2S_PCM_TXFIFO = crate::Reg<i2s_pcm_txfifo::I2S_PCM_TXFIFO_SPEC>;
#[doc = "I2S/PCM TXFIFO Register"]
pub mod i2s_pcm_txfifo;
#[doc = "i2s_pcm_clkd (rw) register accessor: I2S/PCM Clock Divide Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_clkd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_clkd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_pcm_clkd`] module"]
pub type I2S_PCM_CLKD = crate::Reg<i2s_pcm_clkd::I2S_PCM_CLKD_SPEC>;
#[doc = "I2S/PCM Clock Divide Register"]
pub mod i2s_pcm_clkd;
#[doc = "i2s_pcm_txcnt (rw) register accessor: I2S/PCM TX Sample Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_txcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_txcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_pcm_txcnt`] module"]
pub type I2S_PCM_TXCNT = crate::Reg<i2s_pcm_txcnt::I2S_PCM_TXCNT_SPEC>;
#[doc = "I2S/PCM TX Sample Counter Register"]
pub mod i2s_pcm_txcnt;
#[doc = "i2s_pcm_rxcnt (rw) register accessor: I2S/PCM RX Sample Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_rxcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_rxcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_pcm_rxcnt`] module"]
pub type I2S_PCM_RXCNT = crate::Reg<i2s_pcm_rxcnt::I2S_PCM_RXCNT_SPEC>;
#[doc = "I2S/PCM RX Sample Counter Register"]
pub mod i2s_pcm_rxcnt;
#[doc = "i2s_pcm_chcfg (rw) register accessor: I2S/PCM Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_chcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_chcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_pcm_chcfg`] module"]
pub type I2S_PCM_CHCFG = crate::Reg<i2s_pcm_chcfg::I2S_PCM_CHCFG_SPEC>;
#[doc = "I2S/PCM Channel Configuration Register"]
pub mod i2s_pcm_chcfg;
#[doc = "i2s_pcm_tx0chsel (rw) register accessor: I2S/PCM TX0 Channel Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_tx0chsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_tx0chsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_pcm_tx0chsel`] module"]
pub type I2S_PCM_TX0CHSEL = crate::Reg<i2s_pcm_tx0chsel::I2S_PCM_TX0CHSEL_SPEC>;
#[doc = "I2S/PCM TX0 Channel Select Register"]
pub mod i2s_pcm_tx0chsel;
#[doc = "i2s_pcm_tx1chsel (rw) register accessor: I2S/PCM TX1 Channel Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_tx1chsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_tx1chsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_pcm_tx1chsel`] module"]
pub type I2S_PCM_TX1CHSEL = crate::Reg<i2s_pcm_tx1chsel::I2S_PCM_TX1CHSEL_SPEC>;
#[doc = "I2S/PCM TX1 Channel Select Register"]
pub mod i2s_pcm_tx1chsel;
#[doc = "i2s_pcm_tx2chsel (rw) register accessor: I2S/PCM TX2 Channel Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_tx2chsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_tx2chsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_pcm_tx2chsel`] module"]
pub type I2S_PCM_TX2CHSEL = crate::Reg<i2s_pcm_tx2chsel::I2S_PCM_TX2CHSEL_SPEC>;
#[doc = "I2S/PCM TX2 Channel Select Register"]
pub mod i2s_pcm_tx2chsel;
#[doc = "i2s_pcm_tx3chsel (rw) register accessor: I2S/PCM TX3 Channel Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_tx3chsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_tx3chsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_pcm_tx3chsel`] module"]
pub type I2S_PCM_TX3CHSEL = crate::Reg<i2s_pcm_tx3chsel::I2S_PCM_TX3CHSEL_SPEC>;
#[doc = "I2S/PCM TX3 Channel Select Register"]
pub mod i2s_pcm_tx3chsel;
#[doc = "i2s_pcm_tx0chmap0 (rw) register accessor: I2S/PCM TX0 Channel Mapping Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_tx0chmap0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_tx0chmap0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_pcm_tx0chmap0`] module"]
pub type I2S_PCM_TX0CHMAP0 = crate::Reg<i2s_pcm_tx0chmap0::I2S_PCM_TX0CHMAP0_SPEC>;
#[doc = "I2S/PCM TX0 Channel Mapping Register0"]
pub mod i2s_pcm_tx0chmap0;
#[doc = "i2s_pcm_tx0chmap1 (rw) register accessor: I2S/PCM TX0 Channel Mapping Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_tx0chmap1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_tx0chmap1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_pcm_tx0chmap1`] module"]
pub type I2S_PCM_TX0CHMAP1 = crate::Reg<i2s_pcm_tx0chmap1::I2S_PCM_TX0CHMAP1_SPEC>;
#[doc = "I2S/PCM TX0 Channel Mapping Register1"]
pub mod i2s_pcm_tx0chmap1;
#[doc = "i2s_pcm_tx1chmap0 (rw) register accessor: I2S/PCM TX1 Channel Mapping Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_tx1chmap0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_tx1chmap0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_pcm_tx1chmap0`] module"]
pub type I2S_PCM_TX1CHMAP0 = crate::Reg<i2s_pcm_tx1chmap0::I2S_PCM_TX1CHMAP0_SPEC>;
#[doc = "I2S/PCM TX1 Channel Mapping Register0"]
pub mod i2s_pcm_tx1chmap0;
#[doc = "i2s_pcm_tx1chmap1 (rw) register accessor: I2S/PCM TX1 Channel Mapping Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_tx1chmap1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_tx1chmap1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_pcm_tx1chmap1`] module"]
pub type I2S_PCM_TX1CHMAP1 = crate::Reg<i2s_pcm_tx1chmap1::I2S_PCM_TX1CHMAP1_SPEC>;
#[doc = "I2S/PCM TX1 Channel Mapping Register1"]
pub mod i2s_pcm_tx1chmap1;
#[doc = "i2s_pcm_tx2chmap0 (rw) register accessor: I2S/PCM TX2 Channel Mapping Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_tx2chmap0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_tx2chmap0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_pcm_tx2chmap0`] module"]
pub type I2S_PCM_TX2CHMAP0 = crate::Reg<i2s_pcm_tx2chmap0::I2S_PCM_TX2CHMAP0_SPEC>;
#[doc = "I2S/PCM TX2 Channel Mapping Register0"]
pub mod i2s_pcm_tx2chmap0;
#[doc = "i2s_pcm_tx2chmap1 (rw) register accessor: I2S/PCM TX2 Channel Mapping Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_tx2chmap1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_tx2chmap1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_pcm_tx2chmap1`] module"]
pub type I2S_PCM_TX2CHMAP1 = crate::Reg<i2s_pcm_tx2chmap1::I2S_PCM_TX2CHMAP1_SPEC>;
#[doc = "I2S/PCM TX2 Channel Mapping Register1"]
pub mod i2s_pcm_tx2chmap1;
#[doc = "i2s_pcm_tx3chmap0 (rw) register accessor: I2S/PCM TX3 Channel Mapping Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_tx3chmap0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_tx3chmap0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_pcm_tx3chmap0`] module"]
pub type I2S_PCM_TX3CHMAP0 = crate::Reg<i2s_pcm_tx3chmap0::I2S_PCM_TX3CHMAP0_SPEC>;
#[doc = "I2S/PCM TX3 Channel Mapping Register0"]
pub mod i2s_pcm_tx3chmap0;
#[doc = "i2s_pcm_tx3chmap1 (rw) register accessor: I2S/PCM TX3 Channel Mapping Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_tx3chmap1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_tx3chmap1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_pcm_tx3chmap1`] module"]
pub type I2S_PCM_TX3CHMAP1 = crate::Reg<i2s_pcm_tx3chmap1::I2S_PCM_TX3CHMAP1_SPEC>;
#[doc = "I2S/PCM TX3 Channel Mapping Register1"]
pub mod i2s_pcm_tx3chmap1;
#[doc = "i2s_pcm_rxchsel (rw) register accessor: I2S/PCM RX Channel Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_rxchsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_rxchsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_pcm_rxchsel`] module"]
pub type I2S_PCM_RXCHSEL = crate::Reg<i2s_pcm_rxchsel::I2S_PCM_RXCHSEL_SPEC>;
#[doc = "I2S/PCM RX Channel Select Register"]
pub mod i2s_pcm_rxchsel;
#[doc = "i2s_pcm_rxchmap0 (rw) register accessor: I2S/PCM RX Channel Mapping Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_rxchmap0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_rxchmap0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_pcm_rxchmap0`] module"]
pub type I2S_PCM_RXCHMAP0 = crate::Reg<i2s_pcm_rxchmap0::I2S_PCM_RXCHMAP0_SPEC>;
#[doc = "I2S/PCM RX Channel Mapping Register0"]
pub mod i2s_pcm_rxchmap0;
#[doc = "i2s_pcm_rxchmap1 (rw) register accessor: I2S/PCM RX Channel Mapping Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_rxchmap1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_rxchmap1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_pcm_rxchmap1`] module"]
pub type I2S_PCM_RXCHMAP1 = crate::Reg<i2s_pcm_rxchmap1::I2S_PCM_RXCHMAP1_SPEC>;
#[doc = "I2S/PCM RX Channel Mapping Register1"]
pub mod i2s_pcm_rxchmap1;
#[doc = "i2s_pcm_rxchmap2 (rw) register accessor: I2S/PCM RX Channel Mapping Register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_rxchmap2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_rxchmap2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_pcm_rxchmap2`] module"]
pub type I2S_PCM_RXCHMAP2 = crate::Reg<i2s_pcm_rxchmap2::I2S_PCM_RXCHMAP2_SPEC>;
#[doc = "I2S/PCM RX Channel Mapping Register2"]
pub mod i2s_pcm_rxchmap2;
#[doc = "i2s_pcm_rxchmap3 (rw) register accessor: I2S/PCM RX Channel Mapping Register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_rxchmap3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_rxchmap3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_pcm_rxchmap3`] module"]
pub type I2S_PCM_RXCHMAP3 = crate::Reg<i2s_pcm_rxchmap3::I2S_PCM_RXCHMAP3_SPEC>;
#[doc = "I2S/PCM RX Channel Mapping Register3"]
pub mod i2s_pcm_rxchmap3;
#[doc = "mclkcfg (rw) register accessor: ASRC MCLK Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mclkcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mclkcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mclkcfg`] module"]
pub type MCLKCFG = crate::Reg<mclkcfg::MCLKCFG_SPEC>;
#[doc = "ASRC MCLK Configuration Register"]
pub mod mclkcfg;
#[doc = "fsout_cfg (rw) register accessor: ASRC Out Sample Rate Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsout_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsout_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsout_cfg`] module"]
pub type FSOUT_CFG = crate::Reg<fsout_cfg::FSOUT_CFG_SPEC>;
#[doc = "ASRC Out Sample Rate Configuration Register"]
pub mod fsout_cfg;
#[doc = "fsin_extcfg (rw) register accessor: ASRC Input Sample Pulse Extend Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsin_extcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsin_extcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsin_extcfg`] module"]
pub type FSIN_EXTCFG = crate::Reg<fsin_extcfg::FSIN_EXTCFG_SPEC>;
#[doc = "ASRC Input Sample Pulse Extend Configuration Register"]
pub mod fsin_extcfg;
#[doc = "asrcen (rw) register accessor: ASRC Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asrcen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asrcen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asrcen`] module"]
pub type ASRCEN = crate::Reg<asrcen::ASRCEN_SPEC>;
#[doc = "ASRC Enable Register"]
pub mod asrcen;
#[doc = "asrcmancfg (rw) register accessor: ASRC Manual Ratio Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asrcmancfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asrcmancfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asrcmancfg`] module"]
pub type ASRCMANCFG = crate::Reg<asrcmancfg::ASRCMANCFG_SPEC>;
#[doc = "ASRC Manual Ratio Configuration Register"]
pub mod asrcmancfg;
#[doc = "asrcratiostat (rw) register accessor: ASRC Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asrcratiostat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asrcratiostat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asrcratiostat`] module"]
pub type ASRCRATIOSTAT = crate::Reg<asrcratiostat::ASRCRATIOSTAT_SPEC>;
#[doc = "ASRC Status Register"]
pub mod asrcratiostat;
#[doc = "asrcfifostat (rw) register accessor: ASRC FIFO Level Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asrcfifostat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asrcfifostat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asrcfifostat`] module"]
pub type ASRCFIFOSTAT = crate::Reg<asrcfifostat::ASRCFIFOSTAT_SPEC>;
#[doc = "ASRC FIFO Level Status Register"]
pub mod asrcfifostat;
#[doc = "asrcmbistcfg (rw) register accessor: ASRC MBIST Test Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asrcmbistcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asrcmbistcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asrcmbistcfg`] module"]
pub type ASRCMBISTCFG = crate::Reg<asrcmbistcfg::ASRCMBISTCFG_SPEC>;
#[doc = "ASRC MBIST Test Configuration Register"]
pub mod asrcmbistcfg;
#[doc = "asrcmbiststat (rw) register accessor: ASRC MBIST Test Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asrcmbiststat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asrcmbiststat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asrcmbiststat`] module"]
pub type ASRCMBISTSTAT = crate::Reg<asrcmbiststat::ASRCMBISTSTAT_SPEC>;
#[doc = "ASRC MBIST Test Status Register"]
pub mod asrcmbiststat;
