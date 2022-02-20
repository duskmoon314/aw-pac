#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2S/PCM Control Register"]
    pub i2s_pcm_ctl: crate::Reg<i2s_pcm_ctl::I2S_PCM_CTL_SPEC>,
    #[doc = "0x04 - I2S/PCM Format Register 0"]
    pub i2s_pcm_fmt0: crate::Reg<i2s_pcm_fmt0::I2S_PCM_FMT0_SPEC>,
    #[doc = "0x08 - I2S/PCM Format Register 1"]
    pub i2s_pcm_fmt1: crate::Reg<i2s_pcm_fmt1::I2S_PCM_FMT1_SPEC>,
    #[doc = "0x0c - I2S/PCM Interrupt Status Register"]
    pub i2s_pcm_ista: crate::Reg<i2s_pcm_ista::I2S_PCM_ISTA_SPEC>,
    #[doc = "0x10 - I2S/PCM RXFIFO Register"]
    pub i2s_pcm_rxfifo: crate::Reg<i2s_pcm_rxfifo::I2S_PCM_RXFIFO_SPEC>,
    #[doc = "0x14 - I2S/PCM FIFO Control Register"]
    pub i2s_pcm_fctl: crate::Reg<i2s_pcm_fctl::I2S_PCM_FCTL_SPEC>,
    #[doc = "0x18 - I2S/PCM FIFO Status Register"]
    pub i2s_pcm_fsta: crate::Reg<i2s_pcm_fsta::I2S_PCM_FSTA_SPEC>,
    #[doc = "0x1c - I2S/PCM DMA and Interrupt Control Register"]
    pub i2s_pcm_int: crate::Reg<i2s_pcm_int::I2S_PCM_INT_SPEC>,
    #[doc = "0x20 - I2S/PCM TXFIFO Register"]
    pub i2s_pcm_txfifo: crate::Reg<i2s_pcm_txfifo::I2S_PCM_TXFIFO_SPEC>,
    #[doc = "0x24 - I2S/PCM Clock Divide Register"]
    pub i2s_pcm_clkd: crate::Reg<i2s_pcm_clkd::I2S_PCM_CLKD_SPEC>,
    #[doc = "0x28 - I2S/PCM TX Sample Counter Register"]
    pub i2s_pcm_txcnt: crate::Reg<i2s_pcm_txcnt::I2S_PCM_TXCNT_SPEC>,
    #[doc = "0x2c - I2S/PCM RX Sample Counter Register"]
    pub i2s_pcm_rxcnt: crate::Reg<i2s_pcm_rxcnt::I2S_PCM_RXCNT_SPEC>,
    #[doc = "0x30 - I2S/PCM Channel Configuration Register"]
    pub i2s_pcm_chcfg: crate::Reg<i2s_pcm_chcfg::I2S_PCM_CHCFG_SPEC>,
    #[doc = "0x34 - I2S/PCM TX0 Channel Select Register"]
    pub i2s_pcm_tx0chsel: crate::Reg<i2s_pcm_tx0chsel::I2S_PCM_TX0CHSEL_SPEC>,
    #[doc = "0x38 - I2S/PCM TX1 Channel Select Register"]
    pub i2s_pcm_tx1chsel: crate::Reg<i2s_pcm_tx1chsel::I2S_PCM_TX1CHSEL_SPEC>,
    #[doc = "0x3c - I2S/PCM TX2 Channel Select Register"]
    pub i2s_pcm_tx2chsel: crate::Reg<i2s_pcm_tx2chsel::I2S_PCM_TX2CHSEL_SPEC>,
    #[doc = "0x40 - I2S/PCM TX3 Channel Select Register"]
    pub i2s_pcm_tx3chsel: crate::Reg<i2s_pcm_tx3chsel::I2S_PCM_TX3CHSEL_SPEC>,
    #[doc = "0x44 - I2S/PCM TX0 Channel Mapping Register0"]
    pub i2s_pcm_tx0chmap0: crate::Reg<i2s_pcm_tx0chmap0::I2S_PCM_TX0CHMAP0_SPEC>,
    #[doc = "0x48 - I2S/PCM TX0 Channel Mapping Register1"]
    pub i2s_pcm_tx0chmap1: crate::Reg<i2s_pcm_tx0chmap1::I2S_PCM_TX0CHMAP1_SPEC>,
    #[doc = "0x4c - I2S/PCM TX1 Channel Mapping Register0"]
    pub i2s_pcm_tx1chmap0: crate::Reg<i2s_pcm_tx1chmap0::I2S_PCM_TX1CHMAP0_SPEC>,
    #[doc = "0x50 - I2S/PCM TX1 Channel Mapping Register1"]
    pub i2s_pcm_tx1chmap1: crate::Reg<i2s_pcm_tx1chmap1::I2S_PCM_TX1CHMAP1_SPEC>,
    #[doc = "0x54 - I2S/PCM TX2 Channel Mapping Register0"]
    pub i2s_pcm_tx2chmap0: crate::Reg<i2s_pcm_tx2chmap0::I2S_PCM_TX2CHMAP0_SPEC>,
    #[doc = "0x58 - I2S/PCM TX2 Channel Mapping Register1"]
    pub i2s_pcm_tx2chmap1: crate::Reg<i2s_pcm_tx2chmap1::I2S_PCM_TX2CHMAP1_SPEC>,
    #[doc = "0x5c - I2S/PCM TX3 Channel Mapping Register0"]
    pub i2s_pcm_tx3chmap0: crate::Reg<i2s_pcm_tx3chmap0::I2S_PCM_TX3CHMAP0_SPEC>,
    #[doc = "0x60 - I2S/PCM TX3 Channel Mapping Register1"]
    pub i2s_pcm_tx3chmap1: crate::Reg<i2s_pcm_tx3chmap1::I2S_PCM_TX3CHMAP1_SPEC>,
    #[doc = "0x64 - I2S/PCM RX Channel Select Register"]
    pub i2s_pcm_rxchsel: crate::Reg<i2s_pcm_rxchsel::I2S_PCM_RXCHSEL_SPEC>,
    #[doc = "0x68 - I2S/PCM RX Channel Mapping Register0"]
    pub i2s_pcm_rxchmap0: crate::Reg<i2s_pcm_rxchmap0::I2S_PCM_RXCHMAP0_SPEC>,
    #[doc = "0x6c - I2S/PCM RX Channel Mapping Register1"]
    pub i2s_pcm_rxchmap1: crate::Reg<i2s_pcm_rxchmap1::I2S_PCM_RXCHMAP1_SPEC>,
    #[doc = "0x70 - I2S/PCM RX Channel Mapping Register2"]
    pub i2s_pcm_rxchmap2: crate::Reg<i2s_pcm_rxchmap2::I2S_PCM_RXCHMAP2_SPEC>,
    #[doc = "0x74 - I2S/PCM RX Channel Mapping Register3"]
    pub i2s_pcm_rxchmap3: crate::Reg<i2s_pcm_rxchmap3::I2S_PCM_RXCHMAP3_SPEC>,
    _reserved30: [u8; 0x08],
    #[doc = "0x80 - ASRC MCLK Configuration Register"]
    pub mclkcfg: crate::Reg<mclkcfg::MCLKCFG_SPEC>,
    #[doc = "0x84 - ASRC Out Sample Rate Configuration Register"]
    pub fsout_cfg: crate::Reg<fsout_cfg::FSOUTCFG_SPEC>,
    #[doc = "0x88 - ASRC Input Sample Pulse Extend Configuration Register"]
    pub fsin_extcfg: crate::Reg<fsin_extcfg::FSINEXTCFG_SPEC>,
    #[doc = "0x8c - ASRC Enable Register"]
    pub asrccfg: crate::Reg<asrccfg::ASRCCFG_SPEC>,
    #[doc = "0x90 - ASRC Manual Ratio Configuration Register"]
    pub asrcmancfg: crate::Reg<asrcmancfg::ASRCMANCFG_SPEC>,
    #[doc = "0x94 - ASRC Status Register"]
    pub asrcratiostat: crate::Reg<asrcratiostat::ASRCRATIOSTAT_SPEC>,
    #[doc = "0x98 - ASRC FIFO Level Status Register"]
    pub asrcfifostat: crate::Reg<asrcfifostat::ASRCFIFOSTAT_SPEC>,
    #[doc = "0x9c - ASRC MBIST Test Configuration Register"]
    pub asrcmbistcfg: crate::Reg<asrcmbistcfg::ASRCMBISTCFG_SPEC>,
    #[doc = "0xa0 - ASRC MBIST Test Status Register"]
    pub asrcmbiststat: crate::Reg<asrcmbiststat::ASRCMBISTSTAT_SPEC>,
}
#[doc = "I2S_PCM_CTL register accessor: an alias for `Reg<I2S_PCM_CTL_SPEC>`"]
pub type I2S_PCM_CTL = crate::Reg<i2s_pcm_ctl::I2S_PCM_CTL_SPEC>;
#[doc = "I2S/PCM Control Register"]
pub mod i2s_pcm_ctl;
#[doc = "I2S_PCM_FMT0 register accessor: an alias for `Reg<I2S_PCM_FMT0_SPEC>`"]
pub type I2S_PCM_FMT0 = crate::Reg<i2s_pcm_fmt0::I2S_PCM_FMT0_SPEC>;
#[doc = "I2S/PCM Format Register 0"]
pub mod i2s_pcm_fmt0;
#[doc = "I2S_PCM_FMT1 register accessor: an alias for `Reg<I2S_PCM_FMT1_SPEC>`"]
pub type I2S_PCM_FMT1 = crate::Reg<i2s_pcm_fmt1::I2S_PCM_FMT1_SPEC>;
#[doc = "I2S/PCM Format Register 1"]
pub mod i2s_pcm_fmt1;
#[doc = "I2S_PCM_ISTA register accessor: an alias for `Reg<I2S_PCM_ISTA_SPEC>`"]
pub type I2S_PCM_ISTA = crate::Reg<i2s_pcm_ista::I2S_PCM_ISTA_SPEC>;
#[doc = "I2S/PCM Interrupt Status Register"]
pub mod i2s_pcm_ista;
#[doc = "I2S_PCM_RXFIFO register accessor: an alias for `Reg<I2S_PCM_RXFIFO_SPEC>`"]
pub type I2S_PCM_RXFIFO = crate::Reg<i2s_pcm_rxfifo::I2S_PCM_RXFIFO_SPEC>;
#[doc = "I2S/PCM RXFIFO Register"]
pub mod i2s_pcm_rxfifo;
#[doc = "I2S_PCM_FCTL register accessor: an alias for `Reg<I2S_PCM_FCTL_SPEC>`"]
pub type I2S_PCM_FCTL = crate::Reg<i2s_pcm_fctl::I2S_PCM_FCTL_SPEC>;
#[doc = "I2S/PCM FIFO Control Register"]
pub mod i2s_pcm_fctl;
#[doc = "I2S_PCM_FSTA register accessor: an alias for `Reg<I2S_PCM_FSTA_SPEC>`"]
pub type I2S_PCM_FSTA = crate::Reg<i2s_pcm_fsta::I2S_PCM_FSTA_SPEC>;
#[doc = "I2S/PCM FIFO Status Register"]
pub mod i2s_pcm_fsta;
#[doc = "I2S_PCM_INT register accessor: an alias for `Reg<I2S_PCM_INT_SPEC>`"]
pub type I2S_PCM_INT = crate::Reg<i2s_pcm_int::I2S_PCM_INT_SPEC>;
#[doc = "I2S/PCM DMA and Interrupt Control Register"]
pub mod i2s_pcm_int;
#[doc = "I2S_PCM_TXFIFO register accessor: an alias for `Reg<I2S_PCM_TXFIFO_SPEC>`"]
pub type I2S_PCM_TXFIFO = crate::Reg<i2s_pcm_txfifo::I2S_PCM_TXFIFO_SPEC>;
#[doc = "I2S/PCM TXFIFO Register"]
pub mod i2s_pcm_txfifo;
#[doc = "I2S_PCM_CLKD register accessor: an alias for `Reg<I2S_PCM_CLKD_SPEC>`"]
pub type I2S_PCM_CLKD = crate::Reg<i2s_pcm_clkd::I2S_PCM_CLKD_SPEC>;
#[doc = "I2S/PCM Clock Divide Register"]
pub mod i2s_pcm_clkd;
#[doc = "I2S_PCM_TXCNT register accessor: an alias for `Reg<I2S_PCM_TXCNT_SPEC>`"]
pub type I2S_PCM_TXCNT = crate::Reg<i2s_pcm_txcnt::I2S_PCM_TXCNT_SPEC>;
#[doc = "I2S/PCM TX Sample Counter Register"]
pub mod i2s_pcm_txcnt;
#[doc = "I2S_PCM_RXCNT register accessor: an alias for `Reg<I2S_PCM_RXCNT_SPEC>`"]
pub type I2S_PCM_RXCNT = crate::Reg<i2s_pcm_rxcnt::I2S_PCM_RXCNT_SPEC>;
#[doc = "I2S/PCM RX Sample Counter Register"]
pub mod i2s_pcm_rxcnt;
#[doc = "I2S_PCM_CHCFG register accessor: an alias for `Reg<I2S_PCM_CHCFG_SPEC>`"]
pub type I2S_PCM_CHCFG = crate::Reg<i2s_pcm_chcfg::I2S_PCM_CHCFG_SPEC>;
#[doc = "I2S/PCM Channel Configuration Register"]
pub mod i2s_pcm_chcfg;
#[doc = "I2S_PCM_TX0CHSEL register accessor: an alias for `Reg<I2S_PCM_TX0CHSEL_SPEC>`"]
pub type I2S_PCM_TX0CHSEL = crate::Reg<i2s_pcm_tx0chsel::I2S_PCM_TX0CHSEL_SPEC>;
#[doc = "I2S/PCM TX0 Channel Select Register"]
pub mod i2s_pcm_tx0chsel;
#[doc = "I2S_PCM_TX1CHSEL register accessor: an alias for `Reg<I2S_PCM_TX1CHSEL_SPEC>`"]
pub type I2S_PCM_TX1CHSEL = crate::Reg<i2s_pcm_tx1chsel::I2S_PCM_TX1CHSEL_SPEC>;
#[doc = "I2S/PCM TX1 Channel Select Register"]
pub mod i2s_pcm_tx1chsel;
#[doc = "I2S_PCM_TX2CHSEL register accessor: an alias for `Reg<I2S_PCM_TX2CHSEL_SPEC>`"]
pub type I2S_PCM_TX2CHSEL = crate::Reg<i2s_pcm_tx2chsel::I2S_PCM_TX2CHSEL_SPEC>;
#[doc = "I2S/PCM TX2 Channel Select Register"]
pub mod i2s_pcm_tx2chsel;
#[doc = "I2S_PCM_TX3CHSEL register accessor: an alias for `Reg<I2S_PCM_TX3CHSEL_SPEC>`"]
pub type I2S_PCM_TX3CHSEL = crate::Reg<i2s_pcm_tx3chsel::I2S_PCM_TX3CHSEL_SPEC>;
#[doc = "I2S/PCM TX3 Channel Select Register"]
pub mod i2s_pcm_tx3chsel;
#[doc = "I2S_PCM_TX0CHMAP0 register accessor: an alias for `Reg<I2S_PCM_TX0CHMAP0_SPEC>`"]
pub type I2S_PCM_TX0CHMAP0 = crate::Reg<i2s_pcm_tx0chmap0::I2S_PCM_TX0CHMAP0_SPEC>;
#[doc = "I2S/PCM TX0 Channel Mapping Register0"]
pub mod i2s_pcm_tx0chmap0;
#[doc = "I2S_PCM_TX0CHMAP1 register accessor: an alias for `Reg<I2S_PCM_TX0CHMAP1_SPEC>`"]
pub type I2S_PCM_TX0CHMAP1 = crate::Reg<i2s_pcm_tx0chmap1::I2S_PCM_TX0CHMAP1_SPEC>;
#[doc = "I2S/PCM TX0 Channel Mapping Register1"]
pub mod i2s_pcm_tx0chmap1;
#[doc = "I2S_PCM_TX1CHMAP0 register accessor: an alias for `Reg<I2S_PCM_TX1CHMAP0_SPEC>`"]
pub type I2S_PCM_TX1CHMAP0 = crate::Reg<i2s_pcm_tx1chmap0::I2S_PCM_TX1CHMAP0_SPEC>;
#[doc = "I2S/PCM TX1 Channel Mapping Register0"]
pub mod i2s_pcm_tx1chmap0;
#[doc = "I2S_PCM_TX1CHMAP1 register accessor: an alias for `Reg<I2S_PCM_TX1CHMAP1_SPEC>`"]
pub type I2S_PCM_TX1CHMAP1 = crate::Reg<i2s_pcm_tx1chmap1::I2S_PCM_TX1CHMAP1_SPEC>;
#[doc = "I2S/PCM TX1 Channel Mapping Register1"]
pub mod i2s_pcm_tx1chmap1;
#[doc = "I2S_PCM_TX2CHMAP0 register accessor: an alias for `Reg<I2S_PCM_TX2CHMAP0_SPEC>`"]
pub type I2S_PCM_TX2CHMAP0 = crate::Reg<i2s_pcm_tx2chmap0::I2S_PCM_TX2CHMAP0_SPEC>;
#[doc = "I2S/PCM TX2 Channel Mapping Register0"]
pub mod i2s_pcm_tx2chmap0;
#[doc = "I2S_PCM_TX2CHMAP1 register accessor: an alias for `Reg<I2S_PCM_TX2CHMAP1_SPEC>`"]
pub type I2S_PCM_TX2CHMAP1 = crate::Reg<i2s_pcm_tx2chmap1::I2S_PCM_TX2CHMAP1_SPEC>;
#[doc = "I2S/PCM TX2 Channel Mapping Register1"]
pub mod i2s_pcm_tx2chmap1;
#[doc = "I2S_PCM_TX3CHMAP0 register accessor: an alias for `Reg<I2S_PCM_TX3CHMAP0_SPEC>`"]
pub type I2S_PCM_TX3CHMAP0 = crate::Reg<i2s_pcm_tx3chmap0::I2S_PCM_TX3CHMAP0_SPEC>;
#[doc = "I2S/PCM TX3 Channel Mapping Register0"]
pub mod i2s_pcm_tx3chmap0;
#[doc = "I2S_PCM_TX3CHMAP1 register accessor: an alias for `Reg<I2S_PCM_TX3CHMAP1_SPEC>`"]
pub type I2S_PCM_TX3CHMAP1 = crate::Reg<i2s_pcm_tx3chmap1::I2S_PCM_TX3CHMAP1_SPEC>;
#[doc = "I2S/PCM TX3 Channel Mapping Register1"]
pub mod i2s_pcm_tx3chmap1;
#[doc = "I2S_PCM_RXCHSEL register accessor: an alias for `Reg<I2S_PCM_RXCHSEL_SPEC>`"]
pub type I2S_PCM_RXCHSEL = crate::Reg<i2s_pcm_rxchsel::I2S_PCM_RXCHSEL_SPEC>;
#[doc = "I2S/PCM RX Channel Select Register"]
pub mod i2s_pcm_rxchsel;
#[doc = "I2S_PCM_RXCHMAP0 register accessor: an alias for `Reg<I2S_PCM_RXCHMAP0_SPEC>`"]
pub type I2S_PCM_RXCHMAP0 = crate::Reg<i2s_pcm_rxchmap0::I2S_PCM_RXCHMAP0_SPEC>;
#[doc = "I2S/PCM RX Channel Mapping Register0"]
pub mod i2s_pcm_rxchmap0;
#[doc = "I2S_PCM_RXCHMAP1 register accessor: an alias for `Reg<I2S_PCM_RXCHMAP1_SPEC>`"]
pub type I2S_PCM_RXCHMAP1 = crate::Reg<i2s_pcm_rxchmap1::I2S_PCM_RXCHMAP1_SPEC>;
#[doc = "I2S/PCM RX Channel Mapping Register1"]
pub mod i2s_pcm_rxchmap1;
#[doc = "I2S_PCM_RXCHMAP2 register accessor: an alias for `Reg<I2S_PCM_RXCHMAP2_SPEC>`"]
pub type I2S_PCM_RXCHMAP2 = crate::Reg<i2s_pcm_rxchmap2::I2S_PCM_RXCHMAP2_SPEC>;
#[doc = "I2S/PCM RX Channel Mapping Register2"]
pub mod i2s_pcm_rxchmap2;
#[doc = "I2S_PCM_RXCHMAP3 register accessor: an alias for `Reg<I2S_PCM_RXCHMAP3_SPEC>`"]
pub type I2S_PCM_RXCHMAP3 = crate::Reg<i2s_pcm_rxchmap3::I2S_PCM_RXCHMAP3_SPEC>;
#[doc = "I2S/PCM RX Channel Mapping Register3"]
pub mod i2s_pcm_rxchmap3;
#[doc = "MCLKCFG register accessor: an alias for `Reg<MCLKCFG_SPEC>`"]
pub type MCLKCFG = crate::Reg<mclkcfg::MCLKCFG_SPEC>;
#[doc = "ASRC MCLK Configuration Register"]
pub mod mclkcfg;
#[doc = "FsoutCFG register accessor: an alias for `Reg<FSOUTCFG_SPEC>`"]
pub type FSOUTCFG = crate::Reg<fsout_cfg::FSOUTCFG_SPEC>;
#[doc = "ASRC Out Sample Rate Configuration Register"]
pub mod fsout_cfg;
#[doc = "FsinEXTCFG register accessor: an alias for `Reg<FSINEXTCFG_SPEC>`"]
pub type FSINEXTCFG = crate::Reg<fsin_extcfg::FSINEXTCFG_SPEC>;
#[doc = "ASRC Input Sample Pulse Extend Configuration Register"]
pub mod fsin_extcfg;
#[doc = "ASRCCFG register accessor: an alias for `Reg<ASRCCFG_SPEC>`"]
pub type ASRCCFG = crate::Reg<asrccfg::ASRCCFG_SPEC>;
#[doc = "ASRC Enable Register"]
pub mod asrccfg;
#[doc = "ASRCMANCFG register accessor: an alias for `Reg<ASRCMANCFG_SPEC>`"]
pub type ASRCMANCFG = crate::Reg<asrcmancfg::ASRCMANCFG_SPEC>;
#[doc = "ASRC Manual Ratio Configuration Register"]
pub mod asrcmancfg;
#[doc = "ASRCRATIOSTAT register accessor: an alias for `Reg<ASRCRATIOSTAT_SPEC>`"]
pub type ASRCRATIOSTAT = crate::Reg<asrcratiostat::ASRCRATIOSTAT_SPEC>;
#[doc = "ASRC Status Register"]
pub mod asrcratiostat;
#[doc = "ASRCFIFOSTAT register accessor: an alias for `Reg<ASRCFIFOSTAT_SPEC>`"]
pub type ASRCFIFOSTAT = crate::Reg<asrcfifostat::ASRCFIFOSTAT_SPEC>;
#[doc = "ASRC FIFO Level Status Register"]
pub mod asrcfifostat;
#[doc = "ASRCMBISTCFG register accessor: an alias for `Reg<ASRCMBISTCFG_SPEC>`"]
pub type ASRCMBISTCFG = crate::Reg<asrcmbistcfg::ASRCMBISTCFG_SPEC>;
#[doc = "ASRC MBIST Test Configuration Register"]
pub mod asrcmbistcfg;
#[doc = "ASRCMBISTSTAT register accessor: an alias for `Reg<ASRCMBISTSTAT_SPEC>`"]
pub type ASRCMBISTSTAT = crate::Reg<asrcmbiststat::ASRCMBISTSTAT_SPEC>;
#[doc = "ASRC MBIST Test Status Register"]
pub mod asrcmbiststat;
