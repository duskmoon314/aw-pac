#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DAC Digital Part Control Register"]
    pub ac_dac_dpc: AC_DAC_DPC,
    #[doc = "0x04 - DAC Volume Control Register"]
    pub dac_vol_ctrl: DAC_VOL_CTRL,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - DAC FIFO Control Register"]
    pub ac_dac_fifoc: AC_DAC_FIFOC,
    #[doc = "0x14 - DAC FIFO Status Register"]
    pub ac_dac_fifos: AC_DAC_FIFOS,
    _reserved4: [u8; 0x08],
    #[doc = "0x20 - DAC TX DATA Register"]
    pub ac_dac_txdata: AC_DAC_TXDATA,
    #[doc = "0x24 - DAC TX FIFO Counter Register"]
    pub ac_dac_cnt: AC_DAC_CNT,
    #[doc = "0x28 - DAC Debug Register"]
    pub ac_dac_dg: AC_DAC_DG,
    _reserved7: [u8; 0x04],
    #[doc = "0x30 - ADC FIFO Control Register"]
    pub ac_adc_fifoc: AC_ADC_FIFOC,
    #[doc = "0x34 - ADC Volume Control1 Register"]
    pub adc_vol_ctrl1: ADC_VOL_CTRL1,
    #[doc = "0x38 - ADC FIFO Status Register"]
    pub ac_adc_fifos: AC_ADC_FIFOS,
    _reserved10: [u8; 0x04],
    #[doc = "0x40 - ADC RX Data Register"]
    pub ac_adc_rxdata: AC_ADC_RXDATA,
    #[doc = "0x44 - ADC RX Counter Register"]
    pub ac_adc_cnt: AC_ADC_CNT,
    _reserved12: [u8; 0x04],
    #[doc = "0x4c - ADC Debug Register"]
    pub ac_adc_dg: AC_ADC_DG,
    #[doc = "0x50 - ADC Digtial Control Register"]
    pub adc_dig_ctrl: ADC_DIG_CTRL,
    #[doc = "0x54 - VRA1 Speedup Down Control Register"]
    pub vra1speedup_ctrl: VRA1SPEEDUP_CTRL,
    _reserved15: [u8; 0x98],
    #[doc = "0xf0 - DAC DAP Control Register"]
    pub ac_dac_dap_ctr: AC_DAC_DAP_CTR,
    _reserved16: [u8; 0x04],
    #[doc = "0xf8 - ADC DAP Control Register"]
    pub ac_adc_dap_ctr: AC_ADC_DAP_CTR,
    _reserved17: [u8; 0x04],
    #[doc = "0x100 - DAC DRC High HPF Coef Register"]
    pub ac_dac_drc_hhpfc: AC_DAC_DRC_HHPFC,
    #[doc = "0x104 - DAC DRC Low HPF Coef Register"]
    pub ac_dac_drc_lhpfc: AC_DAC_DRC_LHPFC,
    #[doc = "0x108 - DAC DRC Control Register"]
    pub ac_dac_drc_ctrl: AC_DAC_DRC_CTRL,
    #[doc = "0x10c - DAC DRC Left Peak Filter High Attack Time Coef Register"]
    pub ac_dac_drc_lpfhat: AC_DAC_DRC_LPFHAT,
    #[doc = "0x110 - DAC DRC Left Peak Filter Low Attack Time Coef Register"]
    pub ac_dac_drc_lpflat: AC_DAC_DRC_LPFLAT,
    #[doc = "0x114 - DAC DRC Right Peak Filter High Attack Time Coef Register"]
    pub ac_dac_drc_rpfhat: AC_DAC_DRC_RPFHAT,
    #[doc = "0x118 - DAC DRC Peak Filter Low Attack Time Coef Register"]
    pub ac_dac_drc_rpflat: AC_DAC_DRC_RPFLAT,
    #[doc = "0x11c - DAC DRC Left Peak Filter High Release Time Coef Register"]
    pub ac_dac_drc_lpfhrt: AC_DAC_DRC_LPFHRT,
    #[doc = "0x120 - DAC DRC Left Peak Filter Low Release Time Coef Register"]
    pub ac_dac_drc_lpflrt: AC_DAC_DRC_LPFLRT,
    #[doc = "0x124 - DAC DRC Right Peak filter High Release Time Coef Register"]
    pub ac_dac_drc_rpfhrt: AC_DAC_DRC_RPFHRT,
    #[doc = "0x128 - DAC DRC Right Peak filter Low Release Time Coef Register"]
    pub ac_dac_drc_rpflrt: AC_DAC_DRC_RPFLRT,
    #[doc = "0x12c - DAC DRC Left RMS Filter High Coef Register"]
    pub ac_dac_drc_lrmshat: AC_DAC_DRC_LRMSHAT,
    #[doc = "0x130 - DAC DRC Left RMS Filter Low Coef Register"]
    pub ac_dac_drc_lrmslat: AC_DAC_DRC_LRMSLAT,
    #[doc = "0x134 - DAC DRC Right RMS Filter High Coef Register"]
    pub ac_dac_drc_rrmshat: AC_DAC_DRC_RRMSHAT,
    #[doc = "0x138 - DAC DRC Right RMS Filter Low Coef Register"]
    pub ac_dac_drc_rrmslat: AC_DAC_DRC_RRMSLAT,
    #[doc = "0x13c - DAC DRC Compressor Threshold High Setting Register"]
    pub ac_dac_drc_hct: AC_DAC_DRC_HCT,
    #[doc = "0x140 - DAC DRC Compressor Slope High Setting Register"]
    pub ac_dac_drc_lct: AC_DAC_DRC_LCT,
    #[doc = "0x144 - DAC DRC Compressor Slope High Setting Register"]
    pub ac_dac_drc_hkc: AC_DAC_DRC_HKC,
    #[doc = "0x148 - DAC DRC Compressor Slope Low Setting Register"]
    pub ac_dac_drc_lkc: AC_DAC_DRC_LKC,
    #[doc = "0x14c - DAC DRC Compressor High Output at Compressor Threshold Register"]
    pub ac_dac_drc_hopc: AC_DAC_DRC_HOPC,
    #[doc = "0x150 - DAC DRC Compressor Low Output at Compressor Threshold Register"]
    pub ac_dac_drc_lopc: AC_DAC_DRC_LOPC,
    #[doc = "0x154 - DAC DRC Limiter Threshold High Setting Register"]
    pub ac_dac_drc_hlt: AC_DAC_DRC_HLT,
    #[doc = "0x158 - DAC DRC Limiter Threshold Low Setting Register"]
    pub ac_dac_drc_llt: AC_DAC_DRC_LLT,
    #[doc = "0x15c - DAC DRC Limiter Slope High Setting Register"]
    pub ac_dac_drc_hkl: AC_DAC_DRC_HKL,
    #[doc = "0x160 - DAC DRC Limiter Slope Low Setting Register"]
    pub ac_dac_drc_lkl: AC_DAC_DRC_LKL,
    #[doc = "0x164 - DAC DRC Limiter High Output at Limiter Threshold"]
    pub ac_dac_drc_hopl: AC_DAC_DRC_HOPL,
    #[doc = "0x168 - DAC DRC Limiter Low Output at Limiter Threshold"]
    pub ac_dac_drc_lopl: AC_DAC_DRC_LOPL,
    #[doc = "0x16c - DAC DRC Expander Threshold High Setting Register"]
    pub ac_dac_drc_het: AC_DAC_DRC_HET,
    #[doc = "0x170 - DAC DRC Expander Threshold Low Setting Register"]
    pub ac_dac_drc_let: AC_DAC_DRC_LET,
    #[doc = "0x174 - DAC DRC Expander Slope High Setting Register"]
    pub ac_dac_drc_hke: AC_DAC_DRC_HKE,
    #[doc = "0x178 - DAC DRC Expander Slope Low Setting Register"]
    pub ac_dac_drc_lke: AC_DAC_DRC_LKE,
    #[doc = "0x17c - DAC DRC Expander High Output at Expander Threshold"]
    pub ac_dac_drc_hope: AC_DAC_DRC_HOPE,
    #[doc = "0x180 - DAC DRC Expander Low Output at Expander Threshold"]
    pub ac_dac_drc_lope: AC_DAC_DRC_LOPE,
    #[doc = "0x184 - DAC DRC Linear Slope High Setting Register"]
    pub ac_dac_drc_hkn: AC_DAC_DRC_HKN,
    #[doc = "0x188 - DAC DRC Linear Slope Low Setting Register"]
    pub ac_dac_drc_lkn: AC_DAC_DRC_LKN,
    #[doc = "0x18c - DAC DRC Smooth filter Gain High Attack Time Coef Register"]
    pub ac_dac_drc_sfhat: AC_DAC_DRC_SFHAT,
    #[doc = "0x190 - DAC DRC Smooth filter Gain Low Attack Time Coef Register"]
    pub ac_dac_drc_sflat: AC_DAC_DRC_SFLAT,
    #[doc = "0x194 - DAC DRC Smooth filter Gain High Release Time Coef Register"]
    pub ac_dac_drc_sfhrt: AC_DAC_DRC_SFHRT,
    #[doc = "0x198 - DAC DRC Smooth filter Gain Low Release Time Coef Register"]
    pub ac_dac_drc_sflrt: AC_DAC_DRC_SFLRT,
    #[doc = "0x19c - DAC DRC MAX Gain High Setting Register"]
    pub ac_dac_drc_mxghs: AC_DAC_DRC_MXGHS,
    #[doc = "0x1a0 - DAC DRC MAX Gain Low Setting Register"]
    pub ac_dac_drc_mxgls: AC_DAC_DRC_MXGLS,
    #[doc = "0x1a4 - DAC DRC MIN Gain High Setting Register"]
    pub ac_dac_drc_mnghs: AC_DAC_DRC_MNGHS,
    #[doc = "0x1a8 - DAC DRC MIN Gain Low Setting Register"]
    pub ac_dac_drc_mngls: AC_DAC_DRC_MNGLS,
    #[doc = "0x1ac - DAC DRC Expander Smooth Time High Coef Register"]
    pub ac_dac_drc_epshc: AC_DAC_DRC_EPSHC,
    #[doc = "0x1b0 - DAC DRC Expander Smooth Time Low Coef Register"]
    pub ac_dac_drc_epslc: AC_DAC_DRC_EPSLC,
    _reserved62: [u8; 0x04],
    #[doc = "0x1b8 - DAC DRC HPF Gain High Coef Register"]
    pub ac_dac_drc_hpfhgain: AC_DAC_DRC_HPFHGAIN,
    #[doc = "0x1bc - DAC DRC HPF Gain Low Coef Register"]
    pub ac_dac_drc_hpflgain: AC_DAC_DRC_HPFLGAIN,
    _reserved64: [u8; 0x40],
    #[doc = "0x200 - ADC DRC High HPF Coef Register"]
    pub ac_adc_drc_hhpfc: AC_ADC_DRC_HHPFC,
    #[doc = "0x204 - ADC DRC Low HPF Coef Register"]
    pub ac_adc_drc_lhpfc: AC_ADC_DRC_LHPFC,
    #[doc = "0x208 - ADC DRC Control Register"]
    pub ac_adc_drc_ctrl: AC_ADC_DRC_CTRL,
    #[doc = "0x20c - ADC DRC Left Peak Filter High Attack Time Coef Register"]
    pub ac_adc_drc_lpfhat: AC_ADC_DRC_LPFHAT,
    #[doc = "0x210 - ADC DRC Left Peak Filter Low Attack Time Coef Register"]
    pub ac_adc_drc_lpflat: AC_ADC_DRC_LPFLAT,
    #[doc = "0x214 - ADC DRC Right Peak Filter High Attack Time Coef Register"]
    pub ac_adc_drc_rpfhat: AC_ADC_DRC_RPFHAT,
    #[doc = "0x218 - ADC DRC Right Peak Filter Low Attack Time Coef Register"]
    pub ac_adc_drc_rpflat: AC_ADC_DRC_RPFLAT,
    #[doc = "0x21c - ADC DRC Left Peak Filter High Release Time Coef Register"]
    pub ac_adc_drc_lpfhrt: AC_ADC_DRC_LPFHRT,
    #[doc = "0x220 - ADC DRC Left Peak Filter Low Release Time Coef Register"]
    pub ac_adc_drc_lpflrt: AC_ADC_DRC_LPFLRT,
    #[doc = "0x224 - ADC DRC Right Peak Filter High Release Time Coef Register"]
    pub ac_adc_drc_rpfhrt: AC_ADC_DRC_RPFHRT,
    #[doc = "0x228 - ADC DRC Right Peak Filter Low Release Time Coef Register"]
    pub ac_adc_drc_rpflrt: AC_ADC_DRC_RPFLRT,
    #[doc = "0x22c - ADC DRC Left RMS Filter High Coef Register"]
    pub ac_adc_drc_lrmshat: AC_ADC_DRC_LRMSHAT,
    #[doc = "0x230 - ADC DRC Left RMS Filter Low Coef Register"]
    pub ac_adc_drc_lrmslat: AC_ADC_DRC_LRMSLAT,
    #[doc = "0x234 - ADC DRC Right RMS Filter High Coef Register"]
    pub ac_adc_drc_rrmshat: AC_ADC_DRC_RRMSHAT,
    #[doc = "0x238 - ADC DRC Right RMS Filter Low Coef Register"]
    pub ac_adc_drc_rrmslat: AC_ADC_DRC_RRMSLAT,
    #[doc = "0x23c - ADC DRC Compressor Threshold High Setting Register"]
    pub ac_adc_drc_hct: AC_ADC_DRC_HCT,
    #[doc = "0x240 - ADC DRC Compressor Slope High Setting Register"]
    pub ac_adc_drc_lct: AC_ADC_DRC_LCT,
    #[doc = "0x244 - ADC DRC Compressor Slope High Setting Register"]
    pub ac_adc_drc_hkc: AC_ADC_DRC_HKC,
    #[doc = "0x248 - ADC DRC Compressor Slope Low Setting Register"]
    pub ac_adc_drc_lkc: AC_ADC_DRC_LKC,
    #[doc = "0x24c - ADC DRC Compressor High Output at Compressor Threshold Register"]
    pub ac_adc_drc_hopc: AC_ADC_DRC_HOPC,
    #[doc = "0x250 - ADC DRC Compressor Low Output at Compressor Threshold Register"]
    pub ac_adc_drc_lopc: AC_ADC_DRC_LOPC,
    #[doc = "0x254 - ADC DRC Limiter Threshold High Setting Register"]
    pub ac_adc_drc_hlt: AC_ADC_DRC_HLT,
    #[doc = "0x258 - ADC DRC Limiter Threshold Low Setting Register"]
    pub ac_adc_drc_llt: AC_ADC_DRC_LLT,
    #[doc = "0x25c - ADC DRC Limiter Slope High Setting Register"]
    pub ac_adc_drc_hkl: AC_ADC_DRC_HKL,
    #[doc = "0x260 - ADC DRC Limiter Slope Low Setting Register"]
    pub ac_adc_drc_lkl: AC_ADC_DRC_LKL,
    #[doc = "0x264 - ADC DRC Limiter High Output at Limiter Threshold"]
    pub ac_adc_drc_hopl: AC_ADC_DRC_HOPL,
    #[doc = "0x268 - ADC DRC Limiter Low Output at Limiter Threshold"]
    pub ac_adc_drc_lopl: AC_ADC_DRC_LOPL,
    #[doc = "0x26c - ADC DRC Expander Threshold High Setting Register"]
    pub ac_adc_drc_het: AC_ADC_DRC_HET,
    #[doc = "0x270 - ADC DRC Expander Threshold Low Setting Register"]
    pub ac_adc_drc_let: AC_ADC_DRC_LET,
    #[doc = "0x274 - ADC DRC Expander Slope High Setting Register"]
    pub ac_adc_drc_hke: AC_ADC_DRC_HKE,
    #[doc = "0x278 - ADC DRC Expander Slope Low Setting Register"]
    pub ac_adc_drc_lke: AC_ADC_DRC_LKE,
    #[doc = "0x27c - ADC DRC Expander High Output at Expander Threshold"]
    pub ac_adc_drc_hope: AC_ADC_DRC_HOPE,
    #[doc = "0x280 - ADC DRC Expander Low Output at Expander Threshold"]
    pub ac_adc_drc_lope: AC_ADC_DRC_LOPE,
    #[doc = "0x284 - ADC DRC Linear Slope High Setting Register"]
    pub ac_adc_drc_hkn: AC_ADC_DRC_HKN,
    #[doc = "0x288 - ADC DRC Linear Slope Low Setting Register"]
    pub ac_adc_drc_lkn: AC_ADC_DRC_LKN,
    #[doc = "0x28c - ADC DRC Smooth filter Gain High Attack Time Coef Register"]
    pub ac_adc_drc_sfhat: AC_ADC_DRC_SFHAT,
    #[doc = "0x290 - ADC DRC Smooth filter Gain Low Attack Time Coef Register"]
    pub ac_adc_drc_sflat: AC_ADC_DRC_SFLAT,
    #[doc = "0x294 - ADC DRC Smooth filter Gain High Release Time Coef Register"]
    pub ac_adc_drc_sfhrt: AC_ADC_DRC_SFHRT,
    #[doc = "0x298 - ADC DRC Smooth filter Gain Low Release Time Coef Register"]
    pub ac_adc_drc_sflrt: AC_ADC_DRC_SFLRT,
    #[doc = "0x29c - ADC DRC MAX Gain High Setting Register"]
    pub ac_adc_drc_mxghs: AC_ADC_DRC_MXGHS,
    #[doc = "0x2a0 - ADC DRC MAX Gain Low Setting Register"]
    pub ac_adc_drc_mxgls: AC_ADC_DRC_MXGLS,
    #[doc = "0x2a4 - ADC DRC MIN Gain High Setting Register"]
    pub ac_adc_drc_mnghs: AC_ADC_DRC_MNGHS,
    #[doc = "0x2a8 - ADC DRC MIN Gain Low Setting Register"]
    pub ac_adc_drc_mngls: AC_ADC_DRC_MNGLS,
    #[doc = "0x2ac - ADC DRC Expander Smooth Time High Coef Register"]
    pub ac_adc_drc_epshc: AC_ADC_DRC_EPSHC,
    #[doc = "0x2b0 - ADC DRC Expander Smooth Time Low Coef Register"]
    pub ac_adc_drc_epslc: AC_ADC_DRC_EPSLC,
    _reserved109: [u8; 0x04],
    #[doc = "0x2b8 - ADC DRC HPF Gain High Coef Register"]
    pub ac_adc_drc_hpfhgain: AC_ADC_DRC_HPFHGAIN,
    #[doc = "0x2bc - ADC DRC HPF Gain Low Coef Register"]
    pub ac_adc_drc_hpflgain: AC_ADC_DRC_HPFLGAIN,
    _reserved111: [u8; 0x40],
    #[doc = "0x300..0x30c - ADC\\[i\\] Analog Control Register"]
    pub adc: [ADC; 3],
    _reserved112: [u8; 0x04],
    #[doc = "0x310 - DAC Analog Control Register"]
    pub dac: DAC,
    _reserved113: [u8; 0x04],
    #[doc = "0x318 - MICBIAS Analog Control Register"]
    pub micbias: MICBIAS,
    #[doc = "0x31c - BIAS Analog Control Register"]
    pub ramp: RAMP,
    #[doc = "0x320 - BIAS Analog Control Register"]
    pub bias: BIAS,
    _reserved116: [u8; 0x04],
    #[doc = "0x328 - HMIC Control Register"]
    pub hmic_ctrl: HMIC_CTRL,
    #[doc = "0x32c - HMIC Status Register"]
    pub hmic_sts: HMIC_STS,
    _reserved118: [u8; 0x10],
    #[doc = "0x340 - Headphone2 Analog Control Register"]
    pub hp2: HP2,
    _reserved119: [u8; 0x04],
    #[doc = "0x348 - POWER Analog Control Register\n\nThe register is not controlled by the clock and reset of Audio Codec, only controlled by the clock and reset of system bus."]
    pub power: POWER,
}
#[doc = "ac_dac_dpc (rw) register accessor: an alias for `Reg<AC_DAC_DPC_SPEC>`"]
pub type AC_DAC_DPC = crate::Reg<ac_dac_dpc::AC_DAC_DPC_SPEC>;
#[doc = "DAC Digital Part Control Register"]
pub mod ac_dac_dpc;
#[doc = "dac_vol_ctrl (rw) register accessor: an alias for `Reg<DAC_VOL_CTRL_SPEC>`"]
pub type DAC_VOL_CTRL = crate::Reg<dac_vol_ctrl::DAC_VOL_CTRL_SPEC>;
#[doc = "DAC Volume Control Register"]
pub mod dac_vol_ctrl;
#[doc = "ac_dac_fifoc (rw) register accessor: an alias for `Reg<AC_DAC_FIFOC_SPEC>`"]
pub type AC_DAC_FIFOC = crate::Reg<ac_dac_fifoc::AC_DAC_FIFOC_SPEC>;
#[doc = "DAC FIFO Control Register"]
pub mod ac_dac_fifoc;
#[doc = "ac_dac_fifos (rw) register accessor: an alias for `Reg<AC_DAC_FIFOS_SPEC>`"]
pub type AC_DAC_FIFOS = crate::Reg<ac_dac_fifos::AC_DAC_FIFOS_SPEC>;
#[doc = "DAC FIFO Status Register"]
pub mod ac_dac_fifos;
#[doc = "ac_dac_txdata (w) register accessor: an alias for `Reg<AC_DAC_TXDATA_SPEC>`"]
pub type AC_DAC_TXDATA = crate::Reg<ac_dac_txdata::AC_DAC_TXDATA_SPEC>;
#[doc = "DAC TX DATA Register"]
pub mod ac_dac_txdata;
#[doc = "ac_dac_cnt (rw) register accessor: an alias for `Reg<AC_DAC_CNT_SPEC>`"]
pub type AC_DAC_CNT = crate::Reg<ac_dac_cnt::AC_DAC_CNT_SPEC>;
#[doc = "DAC TX FIFO Counter Register"]
pub mod ac_dac_cnt;
#[doc = "ac_dac_dg (rw) register accessor: an alias for `Reg<AC_DAC_DG_SPEC>`"]
pub type AC_DAC_DG = crate::Reg<ac_dac_dg::AC_DAC_DG_SPEC>;
#[doc = "DAC Debug Register"]
pub mod ac_dac_dg;
#[doc = "ac_adc_fifoc (rw) register accessor: an alias for `Reg<AC_ADC_FIFOC_SPEC>`"]
pub type AC_ADC_FIFOC = crate::Reg<ac_adc_fifoc::AC_ADC_FIFOC_SPEC>;
#[doc = "ADC FIFO Control Register"]
pub mod ac_adc_fifoc;
#[doc = "adc_vol_ctrl1 (rw) register accessor: an alias for `Reg<ADC_VOL_CTRL1_SPEC>`"]
pub type ADC_VOL_CTRL1 = crate::Reg<adc_vol_ctrl1::ADC_VOL_CTRL1_SPEC>;
#[doc = "ADC Volume Control1 Register"]
pub mod adc_vol_ctrl1;
#[doc = "ac_adc_fifos (rw) register accessor: an alias for `Reg<AC_ADC_FIFOS_SPEC>`"]
pub type AC_ADC_FIFOS = crate::Reg<ac_adc_fifos::AC_ADC_FIFOS_SPEC>;
#[doc = "ADC FIFO Status Register"]
pub mod ac_adc_fifos;
#[doc = "ac_adc_rxdata (rw) register accessor: an alias for `Reg<AC_ADC_RXDATA_SPEC>`"]
pub type AC_ADC_RXDATA = crate::Reg<ac_adc_rxdata::AC_ADC_RXDATA_SPEC>;
#[doc = "ADC RX Data Register"]
pub mod ac_adc_rxdata;
#[doc = "ac_adc_cnt (rw) register accessor: an alias for `Reg<AC_ADC_CNT_SPEC>`"]
pub type AC_ADC_CNT = crate::Reg<ac_adc_cnt::AC_ADC_CNT_SPEC>;
#[doc = "ADC RX Counter Register"]
pub mod ac_adc_cnt;
#[doc = "ac_adc_dg (rw) register accessor: an alias for `Reg<AC_ADC_DG_SPEC>`"]
pub type AC_ADC_DG = crate::Reg<ac_adc_dg::AC_ADC_DG_SPEC>;
#[doc = "ADC Debug Register"]
pub mod ac_adc_dg;
#[doc = "adc_dig_ctrl (rw) register accessor: an alias for `Reg<ADC_DIG_CTRL_SPEC>`"]
pub type ADC_DIG_CTRL = crate::Reg<adc_dig_ctrl::ADC_DIG_CTRL_SPEC>;
#[doc = "ADC Digtial Control Register"]
pub mod adc_dig_ctrl;
#[doc = "vra1speedup_ctrl (rw) register accessor: an alias for `Reg<VRA1SPEEDUP_CTRL_SPEC>`"]
pub type VRA1SPEEDUP_CTRL = crate::Reg<vra1speedup_ctrl::VRA1SPEEDUP_CTRL_SPEC>;
#[doc = "VRA1 Speedup Down Control Register"]
pub mod vra1speedup_ctrl;
#[doc = "ac_dac_dap_ctr (rw) register accessor: an alias for `Reg<AC_DAC_DAP_CTR_SPEC>`"]
pub type AC_DAC_DAP_CTR = crate::Reg<ac_dac_dap_ctr::AC_DAC_DAP_CTR_SPEC>;
#[doc = "DAC DAP Control Register"]
pub mod ac_dac_dap_ctr;
#[doc = "ac_adc_dap_ctr (rw) register accessor: an alias for `Reg<AC_ADC_DAP_CTR_SPEC>`"]
pub type AC_ADC_DAP_CTR = crate::Reg<ac_adc_dap_ctr::AC_ADC_DAP_CTR_SPEC>;
#[doc = "ADC DAP Control Register"]
pub mod ac_adc_dap_ctr;
#[doc = "ac_dac_drc_hhpfc (rw) register accessor: an alias for `Reg<AC_DAC_DRC_HHPFC_SPEC>`"]
pub type AC_DAC_DRC_HHPFC = crate::Reg<ac_dac_drc_hhpfc::AC_DAC_DRC_HHPFC_SPEC>;
#[doc = "DAC DRC High HPF Coef Register"]
pub mod ac_dac_drc_hhpfc;
#[doc = "ac_dac_drc_lhpfc (rw) register accessor: an alias for `Reg<AC_DAC_DRC_LHPFC_SPEC>`"]
pub type AC_DAC_DRC_LHPFC = crate::Reg<ac_dac_drc_lhpfc::AC_DAC_DRC_LHPFC_SPEC>;
#[doc = "DAC DRC Low HPF Coef Register"]
pub mod ac_dac_drc_lhpfc;
#[doc = "ac_dac_drc_ctrl (rw) register accessor: an alias for `Reg<AC_DAC_DRC_CTRL_SPEC>`"]
pub type AC_DAC_DRC_CTRL = crate::Reg<ac_dac_drc_ctrl::AC_DAC_DRC_CTRL_SPEC>;
#[doc = "DAC DRC Control Register"]
pub mod ac_dac_drc_ctrl;
#[doc = "ac_dac_drc_lpfhat (rw) register accessor: an alias for `Reg<AC_DAC_DRC_LPFHAT_SPEC>`"]
pub type AC_DAC_DRC_LPFHAT = crate::Reg<ac_dac_drc_lpfhat::AC_DAC_DRC_LPFHAT_SPEC>;
#[doc = "DAC DRC Left Peak Filter High Attack Time Coef Register"]
pub mod ac_dac_drc_lpfhat;
#[doc = "ac_dac_drc_lpflat (rw) register accessor: an alias for `Reg<AC_DAC_DRC_LPFLAT_SPEC>`"]
pub type AC_DAC_DRC_LPFLAT = crate::Reg<ac_dac_drc_lpflat::AC_DAC_DRC_LPFLAT_SPEC>;
#[doc = "DAC DRC Left Peak Filter Low Attack Time Coef Register"]
pub mod ac_dac_drc_lpflat;
#[doc = "ac_dac_drc_rpfhat (rw) register accessor: an alias for `Reg<AC_DAC_DRC_RPFHAT_SPEC>`"]
pub type AC_DAC_DRC_RPFHAT = crate::Reg<ac_dac_drc_rpfhat::AC_DAC_DRC_RPFHAT_SPEC>;
#[doc = "DAC DRC Right Peak Filter High Attack Time Coef Register"]
pub mod ac_dac_drc_rpfhat;
#[doc = "ac_dac_drc_rpflat (rw) register accessor: an alias for `Reg<AC_DAC_DRC_RPFLAT_SPEC>`"]
pub type AC_DAC_DRC_RPFLAT = crate::Reg<ac_dac_drc_rpflat::AC_DAC_DRC_RPFLAT_SPEC>;
#[doc = "DAC DRC Peak Filter Low Attack Time Coef Register"]
pub mod ac_dac_drc_rpflat;
#[doc = "ac_dac_drc_lpfhrt (rw) register accessor: an alias for `Reg<AC_DAC_DRC_LPFHRT_SPEC>`"]
pub type AC_DAC_DRC_LPFHRT = crate::Reg<ac_dac_drc_lpfhrt::AC_DAC_DRC_LPFHRT_SPEC>;
#[doc = "DAC DRC Left Peak Filter High Release Time Coef Register"]
pub mod ac_dac_drc_lpfhrt;
#[doc = "ac_dac_drc_lpflrt (rw) register accessor: an alias for `Reg<AC_DAC_DRC_LPFLRT_SPEC>`"]
pub type AC_DAC_DRC_LPFLRT = crate::Reg<ac_dac_drc_lpflrt::AC_DAC_DRC_LPFLRT_SPEC>;
#[doc = "DAC DRC Left Peak Filter Low Release Time Coef Register"]
pub mod ac_dac_drc_lpflrt;
#[doc = "ac_dac_drc_rpfhrt (rw) register accessor: an alias for `Reg<AC_DAC_DRC_RPFHRT_SPEC>`"]
pub type AC_DAC_DRC_RPFHRT = crate::Reg<ac_dac_drc_rpfhrt::AC_DAC_DRC_RPFHRT_SPEC>;
#[doc = "DAC DRC Right Peak filter High Release Time Coef Register"]
pub mod ac_dac_drc_rpfhrt;
#[doc = "ac_dac_drc_rpflrt (rw) register accessor: an alias for `Reg<AC_DAC_DRC_RPFLRT_SPEC>`"]
pub type AC_DAC_DRC_RPFLRT = crate::Reg<ac_dac_drc_rpflrt::AC_DAC_DRC_RPFLRT_SPEC>;
#[doc = "DAC DRC Right Peak filter Low Release Time Coef Register"]
pub mod ac_dac_drc_rpflrt;
#[doc = "ac_dac_drc_lrmshat (rw) register accessor: an alias for `Reg<AC_DAC_DRC_LRMSHAT_SPEC>`"]
pub type AC_DAC_DRC_LRMSHAT = crate::Reg<ac_dac_drc_lrmshat::AC_DAC_DRC_LRMSHAT_SPEC>;
#[doc = "DAC DRC Left RMS Filter High Coef Register"]
pub mod ac_dac_drc_lrmshat;
#[doc = "ac_dac_drc_lrmslat (rw) register accessor: an alias for `Reg<AC_DAC_DRC_LRMSLAT_SPEC>`"]
pub type AC_DAC_DRC_LRMSLAT = crate::Reg<ac_dac_drc_lrmslat::AC_DAC_DRC_LRMSLAT_SPEC>;
#[doc = "DAC DRC Left RMS Filter Low Coef Register"]
pub mod ac_dac_drc_lrmslat;
#[doc = "ac_dac_drc_rrmshat (rw) register accessor: an alias for `Reg<AC_DAC_DRC_RRMSHAT_SPEC>`"]
pub type AC_DAC_DRC_RRMSHAT = crate::Reg<ac_dac_drc_rrmshat::AC_DAC_DRC_RRMSHAT_SPEC>;
#[doc = "DAC DRC Right RMS Filter High Coef Register"]
pub mod ac_dac_drc_rrmshat;
#[doc = "ac_dac_drc_rrmslat (rw) register accessor: an alias for `Reg<AC_DAC_DRC_RRMSLAT_SPEC>`"]
pub type AC_DAC_DRC_RRMSLAT = crate::Reg<ac_dac_drc_rrmslat::AC_DAC_DRC_RRMSLAT_SPEC>;
#[doc = "DAC DRC Right RMS Filter Low Coef Register"]
pub mod ac_dac_drc_rrmslat;
#[doc = "ac_dac_drc_hct (rw) register accessor: an alias for `Reg<AC_DAC_DRC_HCT_SPEC>`"]
pub type AC_DAC_DRC_HCT = crate::Reg<ac_dac_drc_hct::AC_DAC_DRC_HCT_SPEC>;
#[doc = "DAC DRC Compressor Threshold High Setting Register"]
pub mod ac_dac_drc_hct;
#[doc = "ac_dac_drc_lct (rw) register accessor: an alias for `Reg<AC_DAC_DRC_LCT_SPEC>`"]
pub type AC_DAC_DRC_LCT = crate::Reg<ac_dac_drc_lct::AC_DAC_DRC_LCT_SPEC>;
#[doc = "DAC DRC Compressor Slope High Setting Register"]
pub mod ac_dac_drc_lct;
#[doc = "ac_dac_drc_hkc (rw) register accessor: an alias for `Reg<AC_DAC_DRC_HKC_SPEC>`"]
pub type AC_DAC_DRC_HKC = crate::Reg<ac_dac_drc_hkc::AC_DAC_DRC_HKC_SPEC>;
#[doc = "DAC DRC Compressor Slope High Setting Register"]
pub mod ac_dac_drc_hkc;
#[doc = "ac_dac_drc_lkc (rw) register accessor: an alias for `Reg<AC_DAC_DRC_LKC_SPEC>`"]
pub type AC_DAC_DRC_LKC = crate::Reg<ac_dac_drc_lkc::AC_DAC_DRC_LKC_SPEC>;
#[doc = "DAC DRC Compressor Slope Low Setting Register"]
pub mod ac_dac_drc_lkc;
#[doc = "ac_dac_drc_hopc (rw) register accessor: an alias for `Reg<AC_DAC_DRC_HOPC_SPEC>`"]
pub type AC_DAC_DRC_HOPC = crate::Reg<ac_dac_drc_hopc::AC_DAC_DRC_HOPC_SPEC>;
#[doc = "DAC DRC Compressor High Output at Compressor Threshold Register"]
pub mod ac_dac_drc_hopc;
#[doc = "ac_dac_drc_lopc (rw) register accessor: an alias for `Reg<AC_DAC_DRC_LOPC_SPEC>`"]
pub type AC_DAC_DRC_LOPC = crate::Reg<ac_dac_drc_lopc::AC_DAC_DRC_LOPC_SPEC>;
#[doc = "DAC DRC Compressor Low Output at Compressor Threshold Register"]
pub mod ac_dac_drc_lopc;
#[doc = "ac_dac_drc_hlt (rw) register accessor: an alias for `Reg<AC_DAC_DRC_HLT_SPEC>`"]
pub type AC_DAC_DRC_HLT = crate::Reg<ac_dac_drc_hlt::AC_DAC_DRC_HLT_SPEC>;
#[doc = "DAC DRC Limiter Threshold High Setting Register"]
pub mod ac_dac_drc_hlt;
#[doc = "ac_dac_drc_llt (rw) register accessor: an alias for `Reg<AC_DAC_DRC_LLT_SPEC>`"]
pub type AC_DAC_DRC_LLT = crate::Reg<ac_dac_drc_llt::AC_DAC_DRC_LLT_SPEC>;
#[doc = "DAC DRC Limiter Threshold Low Setting Register"]
pub mod ac_dac_drc_llt;
#[doc = "ac_dac_drc_hkl (rw) register accessor: an alias for `Reg<AC_DAC_DRC_HKL_SPEC>`"]
pub type AC_DAC_DRC_HKL = crate::Reg<ac_dac_drc_hkl::AC_DAC_DRC_HKL_SPEC>;
#[doc = "DAC DRC Limiter Slope High Setting Register"]
pub mod ac_dac_drc_hkl;
#[doc = "ac_dac_drc_lkl (rw) register accessor: an alias for `Reg<AC_DAC_DRC_LKL_SPEC>`"]
pub type AC_DAC_DRC_LKL = crate::Reg<ac_dac_drc_lkl::AC_DAC_DRC_LKL_SPEC>;
#[doc = "DAC DRC Limiter Slope Low Setting Register"]
pub mod ac_dac_drc_lkl;
#[doc = "ac_dac_drc_hopl (rw) register accessor: an alias for `Reg<AC_DAC_DRC_HOPL_SPEC>`"]
pub type AC_DAC_DRC_HOPL = crate::Reg<ac_dac_drc_hopl::AC_DAC_DRC_HOPL_SPEC>;
#[doc = "DAC DRC Limiter High Output at Limiter Threshold"]
pub mod ac_dac_drc_hopl;
#[doc = "ac_dac_drc_lopl (rw) register accessor: an alias for `Reg<AC_DAC_DRC_LOPL_SPEC>`"]
pub type AC_DAC_DRC_LOPL = crate::Reg<ac_dac_drc_lopl::AC_DAC_DRC_LOPL_SPEC>;
#[doc = "DAC DRC Limiter Low Output at Limiter Threshold"]
pub mod ac_dac_drc_lopl;
#[doc = "ac_dac_drc_het (rw) register accessor: an alias for `Reg<AC_DAC_DRC_HET_SPEC>`"]
pub type AC_DAC_DRC_HET = crate::Reg<ac_dac_drc_het::AC_DAC_DRC_HET_SPEC>;
#[doc = "DAC DRC Expander Threshold High Setting Register"]
pub mod ac_dac_drc_het;
#[doc = "ac_dac_drc_let (rw) register accessor: an alias for `Reg<AC_DAC_DRC_LET_SPEC>`"]
pub type AC_DAC_DRC_LET = crate::Reg<ac_dac_drc_let::AC_DAC_DRC_LET_SPEC>;
#[doc = "DAC DRC Expander Threshold Low Setting Register"]
pub mod ac_dac_drc_let;
#[doc = "ac_dac_drc_hke (rw) register accessor: an alias for `Reg<AC_DAC_DRC_HKE_SPEC>`"]
pub type AC_DAC_DRC_HKE = crate::Reg<ac_dac_drc_hke::AC_DAC_DRC_HKE_SPEC>;
#[doc = "DAC DRC Expander Slope High Setting Register"]
pub mod ac_dac_drc_hke;
#[doc = "ac_dac_drc_lke (rw) register accessor: an alias for `Reg<AC_DAC_DRC_LKE_SPEC>`"]
pub type AC_DAC_DRC_LKE = crate::Reg<ac_dac_drc_lke::AC_DAC_DRC_LKE_SPEC>;
#[doc = "DAC DRC Expander Slope Low Setting Register"]
pub mod ac_dac_drc_lke;
#[doc = "ac_dac_drc_hope (rw) register accessor: an alias for `Reg<AC_DAC_DRC_HOPE_SPEC>`"]
pub type AC_DAC_DRC_HOPE = crate::Reg<ac_dac_drc_hope::AC_DAC_DRC_HOPE_SPEC>;
#[doc = "DAC DRC Expander High Output at Expander Threshold"]
pub mod ac_dac_drc_hope;
#[doc = "ac_dac_drc_lope (rw) register accessor: an alias for `Reg<AC_DAC_DRC_LOPE_SPEC>`"]
pub type AC_DAC_DRC_LOPE = crate::Reg<ac_dac_drc_lope::AC_DAC_DRC_LOPE_SPEC>;
#[doc = "DAC DRC Expander Low Output at Expander Threshold"]
pub mod ac_dac_drc_lope;
#[doc = "ac_dac_drc_hkn (rw) register accessor: an alias for `Reg<AC_DAC_DRC_HKN_SPEC>`"]
pub type AC_DAC_DRC_HKN = crate::Reg<ac_dac_drc_hkn::AC_DAC_DRC_HKN_SPEC>;
#[doc = "DAC DRC Linear Slope High Setting Register"]
pub mod ac_dac_drc_hkn;
#[doc = "ac_dac_drc_lkn (rw) register accessor: an alias for `Reg<AC_DAC_DRC_LKN_SPEC>`"]
pub type AC_DAC_DRC_LKN = crate::Reg<ac_dac_drc_lkn::AC_DAC_DRC_LKN_SPEC>;
#[doc = "DAC DRC Linear Slope Low Setting Register"]
pub mod ac_dac_drc_lkn;
#[doc = "ac_dac_drc_sfhat (rw) register accessor: an alias for `Reg<AC_DAC_DRC_SFHAT_SPEC>`"]
pub type AC_DAC_DRC_SFHAT = crate::Reg<ac_dac_drc_sfhat::AC_DAC_DRC_SFHAT_SPEC>;
#[doc = "DAC DRC Smooth filter Gain High Attack Time Coef Register"]
pub mod ac_dac_drc_sfhat;
#[doc = "ac_dac_drc_sflat (rw) register accessor: an alias for `Reg<AC_DAC_DRC_SFLAT_SPEC>`"]
pub type AC_DAC_DRC_SFLAT = crate::Reg<ac_dac_drc_sflat::AC_DAC_DRC_SFLAT_SPEC>;
#[doc = "DAC DRC Smooth filter Gain Low Attack Time Coef Register"]
pub mod ac_dac_drc_sflat;
#[doc = "ac_dac_drc_sfhrt (rw) register accessor: an alias for `Reg<AC_DAC_DRC_SFHRT_SPEC>`"]
pub type AC_DAC_DRC_SFHRT = crate::Reg<ac_dac_drc_sfhrt::AC_DAC_DRC_SFHRT_SPEC>;
#[doc = "DAC DRC Smooth filter Gain High Release Time Coef Register"]
pub mod ac_dac_drc_sfhrt;
#[doc = "ac_dac_drc_sflrt (rw) register accessor: an alias for `Reg<AC_DAC_DRC_SFLRT_SPEC>`"]
pub type AC_DAC_DRC_SFLRT = crate::Reg<ac_dac_drc_sflrt::AC_DAC_DRC_SFLRT_SPEC>;
#[doc = "DAC DRC Smooth filter Gain Low Release Time Coef Register"]
pub mod ac_dac_drc_sflrt;
#[doc = "ac_dac_drc_mxghs (rw) register accessor: an alias for `Reg<AC_DAC_DRC_MXGHS_SPEC>`"]
pub type AC_DAC_DRC_MXGHS = crate::Reg<ac_dac_drc_mxghs::AC_DAC_DRC_MXGHS_SPEC>;
#[doc = "DAC DRC MAX Gain High Setting Register"]
pub mod ac_dac_drc_mxghs;
#[doc = "ac_dac_drc_mxgls (rw) register accessor: an alias for `Reg<AC_DAC_DRC_MXGLS_SPEC>`"]
pub type AC_DAC_DRC_MXGLS = crate::Reg<ac_dac_drc_mxgls::AC_DAC_DRC_MXGLS_SPEC>;
#[doc = "DAC DRC MAX Gain Low Setting Register"]
pub mod ac_dac_drc_mxgls;
#[doc = "ac_dac_drc_mnghs (rw) register accessor: an alias for `Reg<AC_DAC_DRC_MNGHS_SPEC>`"]
pub type AC_DAC_DRC_MNGHS = crate::Reg<ac_dac_drc_mnghs::AC_DAC_DRC_MNGHS_SPEC>;
#[doc = "DAC DRC MIN Gain High Setting Register"]
pub mod ac_dac_drc_mnghs;
#[doc = "ac_dac_drc_mngls (rw) register accessor: an alias for `Reg<AC_DAC_DRC_MNGLS_SPEC>`"]
pub type AC_DAC_DRC_MNGLS = crate::Reg<ac_dac_drc_mngls::AC_DAC_DRC_MNGLS_SPEC>;
#[doc = "DAC DRC MIN Gain Low Setting Register"]
pub mod ac_dac_drc_mngls;
#[doc = "ac_dac_drc_epshc (rw) register accessor: an alias for `Reg<AC_DAC_DRC_EPSHC_SPEC>`"]
pub type AC_DAC_DRC_EPSHC = crate::Reg<ac_dac_drc_epshc::AC_DAC_DRC_EPSHC_SPEC>;
#[doc = "DAC DRC Expander Smooth Time High Coef Register"]
pub mod ac_dac_drc_epshc;
#[doc = "ac_dac_drc_epslc (rw) register accessor: an alias for `Reg<AC_DAC_DRC_EPSLC_SPEC>`"]
pub type AC_DAC_DRC_EPSLC = crate::Reg<ac_dac_drc_epslc::AC_DAC_DRC_EPSLC_SPEC>;
#[doc = "DAC DRC Expander Smooth Time Low Coef Register"]
pub mod ac_dac_drc_epslc;
#[doc = "ac_dac_drc_hpfhgain (rw) register accessor: an alias for `Reg<AC_DAC_DRC_HPFHGAIN_SPEC>`"]
pub type AC_DAC_DRC_HPFHGAIN = crate::Reg<ac_dac_drc_hpfhgain::AC_DAC_DRC_HPFHGAIN_SPEC>;
#[doc = "DAC DRC HPF Gain High Coef Register"]
pub mod ac_dac_drc_hpfhgain;
#[doc = "ac_dac_drc_hpflgain (rw) register accessor: an alias for `Reg<AC_DAC_DRC_HPFLGAIN_SPEC>`"]
pub type AC_DAC_DRC_HPFLGAIN = crate::Reg<ac_dac_drc_hpflgain::AC_DAC_DRC_HPFLGAIN_SPEC>;
#[doc = "DAC DRC HPF Gain Low Coef Register"]
pub mod ac_dac_drc_hpflgain;
#[doc = "ac_adc_drc_hhpfc (rw) register accessor: an alias for `Reg<AC_ADC_DRC_HHPFC_SPEC>`"]
pub type AC_ADC_DRC_HHPFC = crate::Reg<ac_adc_drc_hhpfc::AC_ADC_DRC_HHPFC_SPEC>;
#[doc = "ADC DRC High HPF Coef Register"]
pub mod ac_adc_drc_hhpfc;
#[doc = "ac_adc_drc_lhpfc (rw) register accessor: an alias for `Reg<AC_ADC_DRC_LHPFC_SPEC>`"]
pub type AC_ADC_DRC_LHPFC = crate::Reg<ac_adc_drc_lhpfc::AC_ADC_DRC_LHPFC_SPEC>;
#[doc = "ADC DRC Low HPF Coef Register"]
pub mod ac_adc_drc_lhpfc;
#[doc = "ac_adc_drc_ctrl (rw) register accessor: an alias for `Reg<AC_ADC_DRC_CTRL_SPEC>`"]
pub type AC_ADC_DRC_CTRL = crate::Reg<ac_adc_drc_ctrl::AC_ADC_DRC_CTRL_SPEC>;
#[doc = "ADC DRC Control Register"]
pub mod ac_adc_drc_ctrl;
#[doc = "ac_adc_drc_lpfhat (rw) register accessor: an alias for `Reg<AC_ADC_DRC_LPFHAT_SPEC>`"]
pub type AC_ADC_DRC_LPFHAT = crate::Reg<ac_adc_drc_lpfhat::AC_ADC_DRC_LPFHAT_SPEC>;
#[doc = "ADC DRC Left Peak Filter High Attack Time Coef Register"]
pub mod ac_adc_drc_lpfhat;
#[doc = "ac_adc_drc_lpflat (rw) register accessor: an alias for `Reg<AC_ADC_DRC_LPFLAT_SPEC>`"]
pub type AC_ADC_DRC_LPFLAT = crate::Reg<ac_adc_drc_lpflat::AC_ADC_DRC_LPFLAT_SPEC>;
#[doc = "ADC DRC Left Peak Filter Low Attack Time Coef Register"]
pub mod ac_adc_drc_lpflat;
#[doc = "ac_adc_drc_rpfhat (rw) register accessor: an alias for `Reg<AC_ADC_DRC_RPFHAT_SPEC>`"]
pub type AC_ADC_DRC_RPFHAT = crate::Reg<ac_adc_drc_rpfhat::AC_ADC_DRC_RPFHAT_SPEC>;
#[doc = "ADC DRC Right Peak Filter High Attack Time Coef Register"]
pub mod ac_adc_drc_rpfhat;
#[doc = "ac_adc_drc_rpflat (rw) register accessor: an alias for `Reg<AC_ADC_DRC_RPFLAT_SPEC>`"]
pub type AC_ADC_DRC_RPFLAT = crate::Reg<ac_adc_drc_rpflat::AC_ADC_DRC_RPFLAT_SPEC>;
#[doc = "ADC DRC Right Peak Filter Low Attack Time Coef Register"]
pub mod ac_adc_drc_rpflat;
#[doc = "ac_adc_drc_lpfhrt (rw) register accessor: an alias for `Reg<AC_ADC_DRC_LPFHRT_SPEC>`"]
pub type AC_ADC_DRC_LPFHRT = crate::Reg<ac_adc_drc_lpfhrt::AC_ADC_DRC_LPFHRT_SPEC>;
#[doc = "ADC DRC Left Peak Filter High Release Time Coef Register"]
pub mod ac_adc_drc_lpfhrt;
#[doc = "ac_adc_drc_lpflrt (rw) register accessor: an alias for `Reg<AC_ADC_DRC_LPFLRT_SPEC>`"]
pub type AC_ADC_DRC_LPFLRT = crate::Reg<ac_adc_drc_lpflrt::AC_ADC_DRC_LPFLRT_SPEC>;
#[doc = "ADC DRC Left Peak Filter Low Release Time Coef Register"]
pub mod ac_adc_drc_lpflrt;
#[doc = "ac_adc_drc_rpfhrt (rw) register accessor: an alias for `Reg<AC_ADC_DRC_RPFHRT_SPEC>`"]
pub type AC_ADC_DRC_RPFHRT = crate::Reg<ac_adc_drc_rpfhrt::AC_ADC_DRC_RPFHRT_SPEC>;
#[doc = "ADC DRC Right Peak Filter High Release Time Coef Register"]
pub mod ac_adc_drc_rpfhrt;
#[doc = "ac_adc_drc_rpflrt (rw) register accessor: an alias for `Reg<AC_ADC_DRC_RPFLRT_SPEC>`"]
pub type AC_ADC_DRC_RPFLRT = crate::Reg<ac_adc_drc_rpflrt::AC_ADC_DRC_RPFLRT_SPEC>;
#[doc = "ADC DRC Right Peak Filter Low Release Time Coef Register"]
pub mod ac_adc_drc_rpflrt;
#[doc = "ac_adc_drc_lrmshat (rw) register accessor: an alias for `Reg<AC_ADC_DRC_LRMSHAT_SPEC>`"]
pub type AC_ADC_DRC_LRMSHAT = crate::Reg<ac_adc_drc_lrmshat::AC_ADC_DRC_LRMSHAT_SPEC>;
#[doc = "ADC DRC Left RMS Filter High Coef Register"]
pub mod ac_adc_drc_lrmshat;
#[doc = "ac_adc_drc_lrmslat (rw) register accessor: an alias for `Reg<AC_ADC_DRC_LRMSLAT_SPEC>`"]
pub type AC_ADC_DRC_LRMSLAT = crate::Reg<ac_adc_drc_lrmslat::AC_ADC_DRC_LRMSLAT_SPEC>;
#[doc = "ADC DRC Left RMS Filter Low Coef Register"]
pub mod ac_adc_drc_lrmslat;
#[doc = "ac_adc_drc_rrmshat (rw) register accessor: an alias for `Reg<AC_ADC_DRC_RRMSHAT_SPEC>`"]
pub type AC_ADC_DRC_RRMSHAT = crate::Reg<ac_adc_drc_rrmshat::AC_ADC_DRC_RRMSHAT_SPEC>;
#[doc = "ADC DRC Right RMS Filter High Coef Register"]
pub mod ac_adc_drc_rrmshat;
#[doc = "ac_adc_drc_rrmslat (rw) register accessor: an alias for `Reg<AC_ADC_DRC_RRMSLAT_SPEC>`"]
pub type AC_ADC_DRC_RRMSLAT = crate::Reg<ac_adc_drc_rrmslat::AC_ADC_DRC_RRMSLAT_SPEC>;
#[doc = "ADC DRC Right RMS Filter Low Coef Register"]
pub mod ac_adc_drc_rrmslat;
#[doc = "ac_adc_drc_hct (rw) register accessor: an alias for `Reg<AC_ADC_DRC_HCT_SPEC>`"]
pub type AC_ADC_DRC_HCT = crate::Reg<ac_adc_drc_hct::AC_ADC_DRC_HCT_SPEC>;
#[doc = "ADC DRC Compressor Threshold High Setting Register"]
pub mod ac_adc_drc_hct;
#[doc = "ac_adc_drc_lct (rw) register accessor: an alias for `Reg<AC_ADC_DRC_LCT_SPEC>`"]
pub type AC_ADC_DRC_LCT = crate::Reg<ac_adc_drc_lct::AC_ADC_DRC_LCT_SPEC>;
#[doc = "ADC DRC Compressor Slope High Setting Register"]
pub mod ac_adc_drc_lct;
#[doc = "ac_adc_drc_hkc (rw) register accessor: an alias for `Reg<AC_ADC_DRC_HKC_SPEC>`"]
pub type AC_ADC_DRC_HKC = crate::Reg<ac_adc_drc_hkc::AC_ADC_DRC_HKC_SPEC>;
#[doc = "ADC DRC Compressor Slope High Setting Register"]
pub mod ac_adc_drc_hkc;
#[doc = "ac_adc_drc_lkc (rw) register accessor: an alias for `Reg<AC_ADC_DRC_LKC_SPEC>`"]
pub type AC_ADC_DRC_LKC = crate::Reg<ac_adc_drc_lkc::AC_ADC_DRC_LKC_SPEC>;
#[doc = "ADC DRC Compressor Slope Low Setting Register"]
pub mod ac_adc_drc_lkc;
#[doc = "ac_adc_drc_hopc (rw) register accessor: an alias for `Reg<AC_ADC_DRC_HOPC_SPEC>`"]
pub type AC_ADC_DRC_HOPC = crate::Reg<ac_adc_drc_hopc::AC_ADC_DRC_HOPC_SPEC>;
#[doc = "ADC DRC Compressor High Output at Compressor Threshold Register"]
pub mod ac_adc_drc_hopc;
#[doc = "ac_adc_drc_lopc (rw) register accessor: an alias for `Reg<AC_ADC_DRC_LOPC_SPEC>`"]
pub type AC_ADC_DRC_LOPC = crate::Reg<ac_adc_drc_lopc::AC_ADC_DRC_LOPC_SPEC>;
#[doc = "ADC DRC Compressor Low Output at Compressor Threshold Register"]
pub mod ac_adc_drc_lopc;
#[doc = "ac_adc_drc_hlt (rw) register accessor: an alias for `Reg<AC_ADC_DRC_HLT_SPEC>`"]
pub type AC_ADC_DRC_HLT = crate::Reg<ac_adc_drc_hlt::AC_ADC_DRC_HLT_SPEC>;
#[doc = "ADC DRC Limiter Threshold High Setting Register"]
pub mod ac_adc_drc_hlt;
#[doc = "ac_adc_drc_llt (rw) register accessor: an alias for `Reg<AC_ADC_DRC_LLT_SPEC>`"]
pub type AC_ADC_DRC_LLT = crate::Reg<ac_adc_drc_llt::AC_ADC_DRC_LLT_SPEC>;
#[doc = "ADC DRC Limiter Threshold Low Setting Register"]
pub mod ac_adc_drc_llt;
#[doc = "ac_adc_drc_hkl (rw) register accessor: an alias for `Reg<AC_ADC_DRC_HKL_SPEC>`"]
pub type AC_ADC_DRC_HKL = crate::Reg<ac_adc_drc_hkl::AC_ADC_DRC_HKL_SPEC>;
#[doc = "ADC DRC Limiter Slope High Setting Register"]
pub mod ac_adc_drc_hkl;
#[doc = "ac_adc_drc_lkl (rw) register accessor: an alias for `Reg<AC_ADC_DRC_LKL_SPEC>`"]
pub type AC_ADC_DRC_LKL = crate::Reg<ac_adc_drc_lkl::AC_ADC_DRC_LKL_SPEC>;
#[doc = "ADC DRC Limiter Slope Low Setting Register"]
pub mod ac_adc_drc_lkl;
#[doc = "ac_adc_drc_hopl (rw) register accessor: an alias for `Reg<AC_ADC_DRC_HOPL_SPEC>`"]
pub type AC_ADC_DRC_HOPL = crate::Reg<ac_adc_drc_hopl::AC_ADC_DRC_HOPL_SPEC>;
#[doc = "ADC DRC Limiter High Output at Limiter Threshold"]
pub mod ac_adc_drc_hopl;
#[doc = "ac_adc_drc_lopl (rw) register accessor: an alias for `Reg<AC_ADC_DRC_LOPL_SPEC>`"]
pub type AC_ADC_DRC_LOPL = crate::Reg<ac_adc_drc_lopl::AC_ADC_DRC_LOPL_SPEC>;
#[doc = "ADC DRC Limiter Low Output at Limiter Threshold"]
pub mod ac_adc_drc_lopl;
#[doc = "ac_adc_drc_het (rw) register accessor: an alias for `Reg<AC_ADC_DRC_HET_SPEC>`"]
pub type AC_ADC_DRC_HET = crate::Reg<ac_adc_drc_het::AC_ADC_DRC_HET_SPEC>;
#[doc = "ADC DRC Expander Threshold High Setting Register"]
pub mod ac_adc_drc_het;
#[doc = "ac_adc_drc_let (rw) register accessor: an alias for `Reg<AC_ADC_DRC_LET_SPEC>`"]
pub type AC_ADC_DRC_LET = crate::Reg<ac_adc_drc_let::AC_ADC_DRC_LET_SPEC>;
#[doc = "ADC DRC Expander Threshold Low Setting Register"]
pub mod ac_adc_drc_let;
#[doc = "ac_adc_drc_hke (rw) register accessor: an alias for `Reg<AC_ADC_DRC_HKE_SPEC>`"]
pub type AC_ADC_DRC_HKE = crate::Reg<ac_adc_drc_hke::AC_ADC_DRC_HKE_SPEC>;
#[doc = "ADC DRC Expander Slope High Setting Register"]
pub mod ac_adc_drc_hke;
#[doc = "ac_adc_drc_lke (rw) register accessor: an alias for `Reg<AC_ADC_DRC_LKE_SPEC>`"]
pub type AC_ADC_DRC_LKE = crate::Reg<ac_adc_drc_lke::AC_ADC_DRC_LKE_SPEC>;
#[doc = "ADC DRC Expander Slope Low Setting Register"]
pub mod ac_adc_drc_lke;
#[doc = "ac_adc_drc_hope (rw) register accessor: an alias for `Reg<AC_ADC_DRC_HOPE_SPEC>`"]
pub type AC_ADC_DRC_HOPE = crate::Reg<ac_adc_drc_hope::AC_ADC_DRC_HOPE_SPEC>;
#[doc = "ADC DRC Expander High Output at Expander Threshold"]
pub mod ac_adc_drc_hope;
#[doc = "ac_adc_drc_lope (rw) register accessor: an alias for `Reg<AC_ADC_DRC_LOPE_SPEC>`"]
pub type AC_ADC_DRC_LOPE = crate::Reg<ac_adc_drc_lope::AC_ADC_DRC_LOPE_SPEC>;
#[doc = "ADC DRC Expander Low Output at Expander Threshold"]
pub mod ac_adc_drc_lope;
#[doc = "ac_adc_drc_hkn (rw) register accessor: an alias for `Reg<AC_ADC_DRC_HKN_SPEC>`"]
pub type AC_ADC_DRC_HKN = crate::Reg<ac_adc_drc_hkn::AC_ADC_DRC_HKN_SPEC>;
#[doc = "ADC DRC Linear Slope High Setting Register"]
pub mod ac_adc_drc_hkn;
#[doc = "ac_adc_drc_lkn (rw) register accessor: an alias for `Reg<AC_ADC_DRC_LKN_SPEC>`"]
pub type AC_ADC_DRC_LKN = crate::Reg<ac_adc_drc_lkn::AC_ADC_DRC_LKN_SPEC>;
#[doc = "ADC DRC Linear Slope Low Setting Register"]
pub mod ac_adc_drc_lkn;
#[doc = "ac_adc_drc_sfhat (rw) register accessor: an alias for `Reg<AC_ADC_DRC_SFHAT_SPEC>`"]
pub type AC_ADC_DRC_SFHAT = crate::Reg<ac_adc_drc_sfhat::AC_ADC_DRC_SFHAT_SPEC>;
#[doc = "ADC DRC Smooth filter Gain High Attack Time Coef Register"]
pub mod ac_adc_drc_sfhat;
#[doc = "ac_adc_drc_sflat (rw) register accessor: an alias for `Reg<AC_ADC_DRC_SFLAT_SPEC>`"]
pub type AC_ADC_DRC_SFLAT = crate::Reg<ac_adc_drc_sflat::AC_ADC_DRC_SFLAT_SPEC>;
#[doc = "ADC DRC Smooth filter Gain Low Attack Time Coef Register"]
pub mod ac_adc_drc_sflat;
#[doc = "ac_adc_drc_sfhrt (rw) register accessor: an alias for `Reg<AC_ADC_DRC_SFHRT_SPEC>`"]
pub type AC_ADC_DRC_SFHRT = crate::Reg<ac_adc_drc_sfhrt::AC_ADC_DRC_SFHRT_SPEC>;
#[doc = "ADC DRC Smooth filter Gain High Release Time Coef Register"]
pub mod ac_adc_drc_sfhrt;
#[doc = "ac_adc_drc_sflrt (rw) register accessor: an alias for `Reg<AC_ADC_DRC_SFLRT_SPEC>`"]
pub type AC_ADC_DRC_SFLRT = crate::Reg<ac_adc_drc_sflrt::AC_ADC_DRC_SFLRT_SPEC>;
#[doc = "ADC DRC Smooth filter Gain Low Release Time Coef Register"]
pub mod ac_adc_drc_sflrt;
#[doc = "ac_adc_drc_mxghs (rw) register accessor: an alias for `Reg<AC_ADC_DRC_MXGHS_SPEC>`"]
pub type AC_ADC_DRC_MXGHS = crate::Reg<ac_adc_drc_mxghs::AC_ADC_DRC_MXGHS_SPEC>;
#[doc = "ADC DRC MAX Gain High Setting Register"]
pub mod ac_adc_drc_mxghs;
#[doc = "ac_adc_drc_mxgls (rw) register accessor: an alias for `Reg<AC_ADC_DRC_MXGLS_SPEC>`"]
pub type AC_ADC_DRC_MXGLS = crate::Reg<ac_adc_drc_mxgls::AC_ADC_DRC_MXGLS_SPEC>;
#[doc = "ADC DRC MAX Gain Low Setting Register"]
pub mod ac_adc_drc_mxgls;
#[doc = "ac_adc_drc_mnghs (rw) register accessor: an alias for `Reg<AC_ADC_DRC_MNGHS_SPEC>`"]
pub type AC_ADC_DRC_MNGHS = crate::Reg<ac_adc_drc_mnghs::AC_ADC_DRC_MNGHS_SPEC>;
#[doc = "ADC DRC MIN Gain High Setting Register"]
pub mod ac_adc_drc_mnghs;
#[doc = "ac_adc_drc_mngls (rw) register accessor: an alias for `Reg<AC_ADC_DRC_MNGLS_SPEC>`"]
pub type AC_ADC_DRC_MNGLS = crate::Reg<ac_adc_drc_mngls::AC_ADC_DRC_MNGLS_SPEC>;
#[doc = "ADC DRC MIN Gain Low Setting Register"]
pub mod ac_adc_drc_mngls;
#[doc = "ac_adc_drc_epshc (rw) register accessor: an alias for `Reg<AC_ADC_DRC_EPSHC_SPEC>`"]
pub type AC_ADC_DRC_EPSHC = crate::Reg<ac_adc_drc_epshc::AC_ADC_DRC_EPSHC_SPEC>;
#[doc = "ADC DRC Expander Smooth Time High Coef Register"]
pub mod ac_adc_drc_epshc;
#[doc = "ac_adc_drc_epslc (rw) register accessor: an alias for `Reg<AC_ADC_DRC_EPSLC_SPEC>`"]
pub type AC_ADC_DRC_EPSLC = crate::Reg<ac_adc_drc_epslc::AC_ADC_DRC_EPSLC_SPEC>;
#[doc = "ADC DRC Expander Smooth Time Low Coef Register"]
pub mod ac_adc_drc_epslc;
#[doc = "ac_adc_drc_hpfhgain (rw) register accessor: an alias for `Reg<AC_ADC_DRC_HPFHGAIN_SPEC>`"]
pub type AC_ADC_DRC_HPFHGAIN = crate::Reg<ac_adc_drc_hpfhgain::AC_ADC_DRC_HPFHGAIN_SPEC>;
#[doc = "ADC DRC HPF Gain High Coef Register"]
pub mod ac_adc_drc_hpfhgain;
#[doc = "ac_adc_drc_hpflgain (rw) register accessor: an alias for `Reg<AC_ADC_DRC_HPFLGAIN_SPEC>`"]
pub type AC_ADC_DRC_HPFLGAIN = crate::Reg<ac_adc_drc_hpflgain::AC_ADC_DRC_HPFLGAIN_SPEC>;
#[doc = "ADC DRC HPF Gain Low Coef Register"]
pub mod ac_adc_drc_hpflgain;
#[doc = "adc (rw) register accessor: an alias for `Reg<ADC_SPEC>`"]
pub type ADC = crate::Reg<adc::ADC_SPEC>;
#[doc = "ADC\\[i\\] Analog Control Register"]
pub mod adc;
#[doc = "dac (rw) register accessor: an alias for `Reg<DAC_SPEC>`"]
pub type DAC = crate::Reg<dac::DAC_SPEC>;
#[doc = "DAC Analog Control Register"]
pub mod dac;
#[doc = "micbias (rw) register accessor: an alias for `Reg<MICBIAS_SPEC>`"]
pub type MICBIAS = crate::Reg<micbias::MICBIAS_SPEC>;
#[doc = "MICBIAS Analog Control Register"]
pub mod micbias;
#[doc = "ramp (rw) register accessor: an alias for `Reg<RAMP_SPEC>`"]
pub type RAMP = crate::Reg<ramp::RAMP_SPEC>;
#[doc = "BIAS Analog Control Register"]
pub mod ramp;
#[doc = "bias (rw) register accessor: an alias for `Reg<BIAS_SPEC>`"]
pub type BIAS = crate::Reg<bias::BIAS_SPEC>;
#[doc = "BIAS Analog Control Register"]
pub mod bias;
#[doc = "hmic_ctrl (rw) register accessor: an alias for `Reg<HMIC_CTRL_SPEC>`"]
pub type HMIC_CTRL = crate::Reg<hmic_ctrl::HMIC_CTRL_SPEC>;
#[doc = "HMIC Control Register"]
pub mod hmic_ctrl;
#[doc = "hmic_sts (rw) register accessor: an alias for `Reg<HMIC_STS_SPEC>`"]
pub type HMIC_STS = crate::Reg<hmic_sts::HMIC_STS_SPEC>;
#[doc = "HMIC Status Register"]
pub mod hmic_sts;
#[doc = "hp2 (rw) register accessor: an alias for `Reg<HP2_SPEC>`"]
pub type HP2 = crate::Reg<hp2::HP2_SPEC>;
#[doc = "Headphone2 Analog Control Register"]
pub mod hp2;
#[doc = "power (rw) register accessor: an alias for `Reg<POWER_SPEC>`"]
pub type POWER = crate::Reg<power::POWER_SPEC>;
#[doc = "POWER Analog Control Register\n\nThe register is not controlled by the clock and reset of Audio Codec, only controlled by the clock and reset of system bus."]
pub mod power;
