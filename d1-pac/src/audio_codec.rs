#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ac_dac_dpc: AC_DAC_DPC,
    dac_vol_ctrl: DAC_VOL_CTRL,
    _reserved2: [u8; 0x08],
    ac_dac_fifoc: AC_DAC_FIFOC,
    ac_dac_fifos: AC_DAC_FIFOS,
    _reserved4: [u8; 0x08],
    ac_dac_txdata: AC_DAC_TXDATA,
    ac_dac_cnt: AC_DAC_CNT,
    ac_dac_dg: AC_DAC_DG,
    _reserved7: [u8; 0x04],
    ac_adc_fifoc: AC_ADC_FIFOC,
    adc_vol_ctrl1: ADC_VOL_CTRL1,
    ac_adc_fifos: AC_ADC_FIFOS,
    _reserved10: [u8; 0x04],
    ac_adc_rxdata: AC_ADC_RXDATA,
    ac_adc_cnt: AC_ADC_CNT,
    _reserved12: [u8; 0x04],
    ac_adc_dg: AC_ADC_DG,
    adc_dig_ctrl: ADC_DIG_CTRL,
    vra1speedup_ctrl: VRA1SPEEDUP_CTRL,
    _reserved15: [u8; 0x98],
    ac_dac_dap_ctr: AC_DAC_DAP_CTR,
    _reserved16: [u8; 0x04],
    ac_adc_dap_ctr: AC_ADC_DAP_CTR,
    _reserved17: [u8; 0x04],
    ac_dac_drc_hhpfc: AC_DAC_DRC_HHPFC,
    ac_dac_drc_lhpfc: AC_DAC_DRC_LHPFC,
    ac_dac_drc_ctrl: AC_DAC_DRC_CTRL,
    ac_dac_drc_lpfhat: AC_DAC_DRC_LPFHAT,
    ac_dac_drc_lpflat: AC_DAC_DRC_LPFLAT,
    ac_dac_drc_rpfhat: AC_DAC_DRC_RPFHAT,
    ac_dac_drc_rpflat: AC_DAC_DRC_RPFLAT,
    ac_dac_drc_lpfhrt: AC_DAC_DRC_LPFHRT,
    ac_dac_drc_lpflrt: AC_DAC_DRC_LPFLRT,
    ac_dac_drc_rpfhrt: AC_DAC_DRC_RPFHRT,
    ac_dac_drc_rpflrt: AC_DAC_DRC_RPFLRT,
    ac_dac_drc_lrmshat: AC_DAC_DRC_LRMSHAT,
    ac_dac_drc_lrmslat: AC_DAC_DRC_LRMSLAT,
    ac_dac_drc_rrmshat: AC_DAC_DRC_RRMSHAT,
    ac_dac_drc_rrmslat: AC_DAC_DRC_RRMSLAT,
    ac_dac_drc_hct: AC_DAC_DRC_HCT,
    ac_dac_drc_lct: AC_DAC_DRC_LCT,
    ac_dac_drc_hkc: AC_DAC_DRC_HKC,
    ac_dac_drc_lkc: AC_DAC_DRC_LKC,
    ac_dac_drc_hopc: AC_DAC_DRC_HOPC,
    ac_dac_drc_lopc: AC_DAC_DRC_LOPC,
    ac_dac_drc_hlt: AC_DAC_DRC_HLT,
    ac_dac_drc_llt: AC_DAC_DRC_LLT,
    ac_dac_drc_hkl: AC_DAC_DRC_HKL,
    ac_dac_drc_lkl: AC_DAC_DRC_LKL,
    ac_dac_drc_hopl: AC_DAC_DRC_HOPL,
    ac_dac_drc_lopl: AC_DAC_DRC_LOPL,
    ac_dac_drc_het: AC_DAC_DRC_HET,
    ac_dac_drc_let: AC_DAC_DRC_LET,
    ac_dac_drc_hke: AC_DAC_DRC_HKE,
    ac_dac_drc_lke: AC_DAC_DRC_LKE,
    ac_dac_drc_hope: AC_DAC_DRC_HOPE,
    ac_dac_drc_lope: AC_DAC_DRC_LOPE,
    ac_dac_drc_hkn: AC_DAC_DRC_HKN,
    ac_dac_drc_lkn: AC_DAC_DRC_LKN,
    ac_dac_drc_sfhat: AC_DAC_DRC_SFHAT,
    ac_dac_drc_sflat: AC_DAC_DRC_SFLAT,
    ac_dac_drc_sfhrt: AC_DAC_DRC_SFHRT,
    ac_dac_drc_sflrt: AC_DAC_DRC_SFLRT,
    ac_dac_drc_mxghs: AC_DAC_DRC_MXGHS,
    ac_dac_drc_mxgls: AC_DAC_DRC_MXGLS,
    ac_dac_drc_mnghs: AC_DAC_DRC_MNGHS,
    ac_dac_drc_mngls: AC_DAC_DRC_MNGLS,
    ac_dac_drc_epshc: AC_DAC_DRC_EPSHC,
    ac_dac_drc_epslc: AC_DAC_DRC_EPSLC,
    _reserved62: [u8; 0x04],
    ac_dac_drc_hpfhgain: AC_DAC_DRC_HPFHGAIN,
    ac_dac_drc_hpflgain: AC_DAC_DRC_HPFLGAIN,
    _reserved64: [u8; 0x40],
    ac_adc_drc_hhpfc: AC_ADC_DRC_HHPFC,
    ac_adc_drc_lhpfc: AC_ADC_DRC_LHPFC,
    ac_adc_drc_ctrl: AC_ADC_DRC_CTRL,
    ac_adc_drc_lpfhat: AC_ADC_DRC_LPFHAT,
    ac_adc_drc_lpflat: AC_ADC_DRC_LPFLAT,
    ac_adc_drc_rpfhat: AC_ADC_DRC_RPFHAT,
    ac_adc_drc_rpflat: AC_ADC_DRC_RPFLAT,
    ac_adc_drc_lpfhrt: AC_ADC_DRC_LPFHRT,
    ac_adc_drc_lpflrt: AC_ADC_DRC_LPFLRT,
    ac_adc_drc_rpfhrt: AC_ADC_DRC_RPFHRT,
    ac_adc_drc_rpflrt: AC_ADC_DRC_RPFLRT,
    ac_adc_drc_lrmshat: AC_ADC_DRC_LRMSHAT,
    ac_adc_drc_lrmslat: AC_ADC_DRC_LRMSLAT,
    ac_adc_drc_rrmshat: AC_ADC_DRC_RRMSHAT,
    ac_adc_drc_rrmslat: AC_ADC_DRC_RRMSLAT,
    ac_adc_drc_hct: AC_ADC_DRC_HCT,
    ac_adc_drc_lct: AC_ADC_DRC_LCT,
    ac_adc_drc_hkc: AC_ADC_DRC_HKC,
    ac_adc_drc_lkc: AC_ADC_DRC_LKC,
    ac_adc_drc_hopc: AC_ADC_DRC_HOPC,
    ac_adc_drc_lopc: AC_ADC_DRC_LOPC,
    ac_adc_drc_hlt: AC_ADC_DRC_HLT,
    ac_adc_drc_llt: AC_ADC_DRC_LLT,
    ac_adc_drc_hkl: AC_ADC_DRC_HKL,
    ac_adc_drc_lkl: AC_ADC_DRC_LKL,
    ac_adc_drc_hopl: AC_ADC_DRC_HOPL,
    ac_adc_drc_lopl: AC_ADC_DRC_LOPL,
    ac_adc_drc_het: AC_ADC_DRC_HET,
    ac_adc_drc_let: AC_ADC_DRC_LET,
    ac_adc_drc_hke: AC_ADC_DRC_HKE,
    ac_adc_drc_lke: AC_ADC_DRC_LKE,
    ac_adc_drc_hope: AC_ADC_DRC_HOPE,
    ac_adc_drc_lope: AC_ADC_DRC_LOPE,
    ac_adc_drc_hkn: AC_ADC_DRC_HKN,
    ac_adc_drc_lkn: AC_ADC_DRC_LKN,
    ac_adc_drc_sfhat: AC_ADC_DRC_SFHAT,
    ac_adc_drc_sflat: AC_ADC_DRC_SFLAT,
    ac_adc_drc_sfhrt: AC_ADC_DRC_SFHRT,
    ac_adc_drc_sflrt: AC_ADC_DRC_SFLRT,
    ac_adc_drc_mxghs: AC_ADC_DRC_MXGHS,
    ac_adc_drc_mxgls: AC_ADC_DRC_MXGLS,
    ac_adc_drc_mnghs: AC_ADC_DRC_MNGHS,
    ac_adc_drc_mngls: AC_ADC_DRC_MNGLS,
    ac_adc_drc_epshc: AC_ADC_DRC_EPSHC,
    ac_adc_drc_epslc: AC_ADC_DRC_EPSLC,
    _reserved109: [u8; 0x04],
    ac_adc_drc_hpfhgain: AC_ADC_DRC_HPFHGAIN,
    ac_adc_drc_hpflgain: AC_ADC_DRC_HPFLGAIN,
    _reserved111: [u8; 0x40],
    adc: [ADC; 3],
    _reserved112: [u8; 0x04],
    dac: DAC,
    _reserved113: [u8; 0x04],
    micbias: MICBIAS,
    ramp: RAMP,
    bias: BIAS,
    _reserved116: [u8; 0x04],
    hmic_ctrl: HMIC_CTRL,
    hmic_sts: HMIC_STS,
    _reserved118: [u8; 0x10],
    hp2: HP2,
    _reserved119: [u8; 0x04],
    power: POWER,
}
impl RegisterBlock {
    #[doc = "0x00 - DAC Digital Part Control Register"]
    #[inline(always)]
    pub const fn ac_dac_dpc(&self) -> &AC_DAC_DPC {
        &self.ac_dac_dpc
    }
    #[doc = "0x04 - DAC Volume Control Register"]
    #[inline(always)]
    pub const fn dac_vol_ctrl(&self) -> &DAC_VOL_CTRL {
        &self.dac_vol_ctrl
    }
    #[doc = "0x10 - DAC FIFO Control Register"]
    #[inline(always)]
    pub const fn ac_dac_fifoc(&self) -> &AC_DAC_FIFOC {
        &self.ac_dac_fifoc
    }
    #[doc = "0x14 - DAC FIFO Status Register"]
    #[inline(always)]
    pub const fn ac_dac_fifos(&self) -> &AC_DAC_FIFOS {
        &self.ac_dac_fifos
    }
    #[doc = "0x20 - DAC TX DATA Register"]
    #[inline(always)]
    pub const fn ac_dac_txdata(&self) -> &AC_DAC_TXDATA {
        &self.ac_dac_txdata
    }
    #[doc = "0x24 - DAC TX FIFO Counter Register"]
    #[inline(always)]
    pub const fn ac_dac_cnt(&self) -> &AC_DAC_CNT {
        &self.ac_dac_cnt
    }
    #[doc = "0x28 - DAC Debug Register"]
    #[inline(always)]
    pub const fn ac_dac_dg(&self) -> &AC_DAC_DG {
        &self.ac_dac_dg
    }
    #[doc = "0x30 - ADC FIFO Control Register"]
    #[inline(always)]
    pub const fn ac_adc_fifoc(&self) -> &AC_ADC_FIFOC {
        &self.ac_adc_fifoc
    }
    #[doc = "0x34 - ADC Volume Control1 Register"]
    #[inline(always)]
    pub const fn adc_vol_ctrl1(&self) -> &ADC_VOL_CTRL1 {
        &self.adc_vol_ctrl1
    }
    #[doc = "0x38 - ADC FIFO Status Register"]
    #[inline(always)]
    pub const fn ac_adc_fifos(&self) -> &AC_ADC_FIFOS {
        &self.ac_adc_fifos
    }
    #[doc = "0x40 - ADC RX Data Register"]
    #[inline(always)]
    pub const fn ac_adc_rxdata(&self) -> &AC_ADC_RXDATA {
        &self.ac_adc_rxdata
    }
    #[doc = "0x44 - ADC RX Counter Register"]
    #[inline(always)]
    pub const fn ac_adc_cnt(&self) -> &AC_ADC_CNT {
        &self.ac_adc_cnt
    }
    #[doc = "0x4c - ADC Debug Register"]
    #[inline(always)]
    pub const fn ac_adc_dg(&self) -> &AC_ADC_DG {
        &self.ac_adc_dg
    }
    #[doc = "0x50 - ADC Digtial Control Register"]
    #[inline(always)]
    pub const fn adc_dig_ctrl(&self) -> &ADC_DIG_CTRL {
        &self.adc_dig_ctrl
    }
    #[doc = "0x54 - VRA1 Speedup Down Control Register"]
    #[inline(always)]
    pub const fn vra1speedup_ctrl(&self) -> &VRA1SPEEDUP_CTRL {
        &self.vra1speedup_ctrl
    }
    #[doc = "0xf0 - DAC DAP Control Register"]
    #[inline(always)]
    pub const fn ac_dac_dap_ctr(&self) -> &AC_DAC_DAP_CTR {
        &self.ac_dac_dap_ctr
    }
    #[doc = "0xf8 - ADC DAP Control Register"]
    #[inline(always)]
    pub const fn ac_adc_dap_ctr(&self) -> &AC_ADC_DAP_CTR {
        &self.ac_adc_dap_ctr
    }
    #[doc = "0x100 - DAC DRC High HPF Coef Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_hhpfc(&self) -> &AC_DAC_DRC_HHPFC {
        &self.ac_dac_drc_hhpfc
    }
    #[doc = "0x104 - DAC DRC Low HPF Coef Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_lhpfc(&self) -> &AC_DAC_DRC_LHPFC {
        &self.ac_dac_drc_lhpfc
    }
    #[doc = "0x108 - DAC DRC Control Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_ctrl(&self) -> &AC_DAC_DRC_CTRL {
        &self.ac_dac_drc_ctrl
    }
    #[doc = "0x10c - DAC DRC Left Peak Filter High Attack Time Coef Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_lpfhat(&self) -> &AC_DAC_DRC_LPFHAT {
        &self.ac_dac_drc_lpfhat
    }
    #[doc = "0x110 - DAC DRC Left Peak Filter Low Attack Time Coef Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_lpflat(&self) -> &AC_DAC_DRC_LPFLAT {
        &self.ac_dac_drc_lpflat
    }
    #[doc = "0x114 - DAC DRC Right Peak Filter High Attack Time Coef Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_rpfhat(&self) -> &AC_DAC_DRC_RPFHAT {
        &self.ac_dac_drc_rpfhat
    }
    #[doc = "0x118 - DAC DRC Peak Filter Low Attack Time Coef Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_rpflat(&self) -> &AC_DAC_DRC_RPFLAT {
        &self.ac_dac_drc_rpflat
    }
    #[doc = "0x11c - DAC DRC Left Peak Filter High Release Time Coef Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_lpfhrt(&self) -> &AC_DAC_DRC_LPFHRT {
        &self.ac_dac_drc_lpfhrt
    }
    #[doc = "0x120 - DAC DRC Left Peak Filter Low Release Time Coef Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_lpflrt(&self) -> &AC_DAC_DRC_LPFLRT {
        &self.ac_dac_drc_lpflrt
    }
    #[doc = "0x124 - DAC DRC Right Peak filter High Release Time Coef Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_rpfhrt(&self) -> &AC_DAC_DRC_RPFHRT {
        &self.ac_dac_drc_rpfhrt
    }
    #[doc = "0x128 - DAC DRC Right Peak filter Low Release Time Coef Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_rpflrt(&self) -> &AC_DAC_DRC_RPFLRT {
        &self.ac_dac_drc_rpflrt
    }
    #[doc = "0x12c - DAC DRC Left RMS Filter High Coef Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_lrmshat(&self) -> &AC_DAC_DRC_LRMSHAT {
        &self.ac_dac_drc_lrmshat
    }
    #[doc = "0x130 - DAC DRC Left RMS Filter Low Coef Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_lrmslat(&self) -> &AC_DAC_DRC_LRMSLAT {
        &self.ac_dac_drc_lrmslat
    }
    #[doc = "0x134 - DAC DRC Right RMS Filter High Coef Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_rrmshat(&self) -> &AC_DAC_DRC_RRMSHAT {
        &self.ac_dac_drc_rrmshat
    }
    #[doc = "0x138 - DAC DRC Right RMS Filter Low Coef Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_rrmslat(&self) -> &AC_DAC_DRC_RRMSLAT {
        &self.ac_dac_drc_rrmslat
    }
    #[doc = "0x13c - DAC DRC Compressor Threshold High Setting Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_hct(&self) -> &AC_DAC_DRC_HCT {
        &self.ac_dac_drc_hct
    }
    #[doc = "0x140 - DAC DRC Compressor Slope High Setting Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_lct(&self) -> &AC_DAC_DRC_LCT {
        &self.ac_dac_drc_lct
    }
    #[doc = "0x144 - DAC DRC Compressor Slope High Setting Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_hkc(&self) -> &AC_DAC_DRC_HKC {
        &self.ac_dac_drc_hkc
    }
    #[doc = "0x148 - DAC DRC Compressor Slope Low Setting Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_lkc(&self) -> &AC_DAC_DRC_LKC {
        &self.ac_dac_drc_lkc
    }
    #[doc = "0x14c - DAC DRC Compressor High Output at Compressor Threshold Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_hopc(&self) -> &AC_DAC_DRC_HOPC {
        &self.ac_dac_drc_hopc
    }
    #[doc = "0x150 - DAC DRC Compressor Low Output at Compressor Threshold Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_lopc(&self) -> &AC_DAC_DRC_LOPC {
        &self.ac_dac_drc_lopc
    }
    #[doc = "0x154 - DAC DRC Limiter Threshold High Setting Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_hlt(&self) -> &AC_DAC_DRC_HLT {
        &self.ac_dac_drc_hlt
    }
    #[doc = "0x158 - DAC DRC Limiter Threshold Low Setting Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_llt(&self) -> &AC_DAC_DRC_LLT {
        &self.ac_dac_drc_llt
    }
    #[doc = "0x15c - DAC DRC Limiter Slope High Setting Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_hkl(&self) -> &AC_DAC_DRC_HKL {
        &self.ac_dac_drc_hkl
    }
    #[doc = "0x160 - DAC DRC Limiter Slope Low Setting Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_lkl(&self) -> &AC_DAC_DRC_LKL {
        &self.ac_dac_drc_lkl
    }
    #[doc = "0x164 - DAC DRC Limiter High Output at Limiter Threshold"]
    #[inline(always)]
    pub const fn ac_dac_drc_hopl(&self) -> &AC_DAC_DRC_HOPL {
        &self.ac_dac_drc_hopl
    }
    #[doc = "0x168 - DAC DRC Limiter Low Output at Limiter Threshold"]
    #[inline(always)]
    pub const fn ac_dac_drc_lopl(&self) -> &AC_DAC_DRC_LOPL {
        &self.ac_dac_drc_lopl
    }
    #[doc = "0x16c - DAC DRC Expander Threshold High Setting Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_het(&self) -> &AC_DAC_DRC_HET {
        &self.ac_dac_drc_het
    }
    #[doc = "0x170 - DAC DRC Expander Threshold Low Setting Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_let(&self) -> &AC_DAC_DRC_LET {
        &self.ac_dac_drc_let
    }
    #[doc = "0x174 - DAC DRC Expander Slope High Setting Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_hke(&self) -> &AC_DAC_DRC_HKE {
        &self.ac_dac_drc_hke
    }
    #[doc = "0x178 - DAC DRC Expander Slope Low Setting Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_lke(&self) -> &AC_DAC_DRC_LKE {
        &self.ac_dac_drc_lke
    }
    #[doc = "0x17c - DAC DRC Expander High Output at Expander Threshold"]
    #[inline(always)]
    pub const fn ac_dac_drc_hope(&self) -> &AC_DAC_DRC_HOPE {
        &self.ac_dac_drc_hope
    }
    #[doc = "0x180 - DAC DRC Expander Low Output at Expander Threshold"]
    #[inline(always)]
    pub const fn ac_dac_drc_lope(&self) -> &AC_DAC_DRC_LOPE {
        &self.ac_dac_drc_lope
    }
    #[doc = "0x184 - DAC DRC Linear Slope High Setting Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_hkn(&self) -> &AC_DAC_DRC_HKN {
        &self.ac_dac_drc_hkn
    }
    #[doc = "0x188 - DAC DRC Linear Slope Low Setting Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_lkn(&self) -> &AC_DAC_DRC_LKN {
        &self.ac_dac_drc_lkn
    }
    #[doc = "0x18c - DAC DRC Smooth filter Gain High Attack Time Coef Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_sfhat(&self) -> &AC_DAC_DRC_SFHAT {
        &self.ac_dac_drc_sfhat
    }
    #[doc = "0x190 - DAC DRC Smooth filter Gain Low Attack Time Coef Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_sflat(&self) -> &AC_DAC_DRC_SFLAT {
        &self.ac_dac_drc_sflat
    }
    #[doc = "0x194 - DAC DRC Smooth filter Gain High Release Time Coef Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_sfhrt(&self) -> &AC_DAC_DRC_SFHRT {
        &self.ac_dac_drc_sfhrt
    }
    #[doc = "0x198 - DAC DRC Smooth filter Gain Low Release Time Coef Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_sflrt(&self) -> &AC_DAC_DRC_SFLRT {
        &self.ac_dac_drc_sflrt
    }
    #[doc = "0x19c - DAC DRC MAX Gain High Setting Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_mxghs(&self) -> &AC_DAC_DRC_MXGHS {
        &self.ac_dac_drc_mxghs
    }
    #[doc = "0x1a0 - DAC DRC MAX Gain Low Setting Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_mxgls(&self) -> &AC_DAC_DRC_MXGLS {
        &self.ac_dac_drc_mxgls
    }
    #[doc = "0x1a4 - DAC DRC MIN Gain High Setting Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_mnghs(&self) -> &AC_DAC_DRC_MNGHS {
        &self.ac_dac_drc_mnghs
    }
    #[doc = "0x1a8 - DAC DRC MIN Gain Low Setting Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_mngls(&self) -> &AC_DAC_DRC_MNGLS {
        &self.ac_dac_drc_mngls
    }
    #[doc = "0x1ac - DAC DRC Expander Smooth Time High Coef Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_epshc(&self) -> &AC_DAC_DRC_EPSHC {
        &self.ac_dac_drc_epshc
    }
    #[doc = "0x1b0 - DAC DRC Expander Smooth Time Low Coef Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_epslc(&self) -> &AC_DAC_DRC_EPSLC {
        &self.ac_dac_drc_epslc
    }
    #[doc = "0x1b8 - DAC DRC HPF Gain High Coef Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_hpfhgain(&self) -> &AC_DAC_DRC_HPFHGAIN {
        &self.ac_dac_drc_hpfhgain
    }
    #[doc = "0x1bc - DAC DRC HPF Gain Low Coef Register"]
    #[inline(always)]
    pub const fn ac_dac_drc_hpflgain(&self) -> &AC_DAC_DRC_HPFLGAIN {
        &self.ac_dac_drc_hpflgain
    }
    #[doc = "0x200 - ADC DRC High HPF Coef Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_hhpfc(&self) -> &AC_ADC_DRC_HHPFC {
        &self.ac_adc_drc_hhpfc
    }
    #[doc = "0x204 - ADC DRC Low HPF Coef Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_lhpfc(&self) -> &AC_ADC_DRC_LHPFC {
        &self.ac_adc_drc_lhpfc
    }
    #[doc = "0x208 - ADC DRC Control Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_ctrl(&self) -> &AC_ADC_DRC_CTRL {
        &self.ac_adc_drc_ctrl
    }
    #[doc = "0x20c - ADC DRC Left Peak Filter High Attack Time Coef Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_lpfhat(&self) -> &AC_ADC_DRC_LPFHAT {
        &self.ac_adc_drc_lpfhat
    }
    #[doc = "0x210 - ADC DRC Left Peak Filter Low Attack Time Coef Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_lpflat(&self) -> &AC_ADC_DRC_LPFLAT {
        &self.ac_adc_drc_lpflat
    }
    #[doc = "0x214 - ADC DRC Right Peak Filter High Attack Time Coef Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_rpfhat(&self) -> &AC_ADC_DRC_RPFHAT {
        &self.ac_adc_drc_rpfhat
    }
    #[doc = "0x218 - ADC DRC Right Peak Filter Low Attack Time Coef Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_rpflat(&self) -> &AC_ADC_DRC_RPFLAT {
        &self.ac_adc_drc_rpflat
    }
    #[doc = "0x21c - ADC DRC Left Peak Filter High Release Time Coef Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_lpfhrt(&self) -> &AC_ADC_DRC_LPFHRT {
        &self.ac_adc_drc_lpfhrt
    }
    #[doc = "0x220 - ADC DRC Left Peak Filter Low Release Time Coef Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_lpflrt(&self) -> &AC_ADC_DRC_LPFLRT {
        &self.ac_adc_drc_lpflrt
    }
    #[doc = "0x224 - ADC DRC Right Peak Filter High Release Time Coef Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_rpfhrt(&self) -> &AC_ADC_DRC_RPFHRT {
        &self.ac_adc_drc_rpfhrt
    }
    #[doc = "0x228 - ADC DRC Right Peak Filter Low Release Time Coef Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_rpflrt(&self) -> &AC_ADC_DRC_RPFLRT {
        &self.ac_adc_drc_rpflrt
    }
    #[doc = "0x22c - ADC DRC Left RMS Filter High Coef Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_lrmshat(&self) -> &AC_ADC_DRC_LRMSHAT {
        &self.ac_adc_drc_lrmshat
    }
    #[doc = "0x230 - ADC DRC Left RMS Filter Low Coef Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_lrmslat(&self) -> &AC_ADC_DRC_LRMSLAT {
        &self.ac_adc_drc_lrmslat
    }
    #[doc = "0x234 - ADC DRC Right RMS Filter High Coef Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_rrmshat(&self) -> &AC_ADC_DRC_RRMSHAT {
        &self.ac_adc_drc_rrmshat
    }
    #[doc = "0x238 - ADC DRC Right RMS Filter Low Coef Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_rrmslat(&self) -> &AC_ADC_DRC_RRMSLAT {
        &self.ac_adc_drc_rrmslat
    }
    #[doc = "0x23c - ADC DRC Compressor Threshold High Setting Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_hct(&self) -> &AC_ADC_DRC_HCT {
        &self.ac_adc_drc_hct
    }
    #[doc = "0x240 - ADC DRC Compressor Slope High Setting Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_lct(&self) -> &AC_ADC_DRC_LCT {
        &self.ac_adc_drc_lct
    }
    #[doc = "0x244 - ADC DRC Compressor Slope High Setting Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_hkc(&self) -> &AC_ADC_DRC_HKC {
        &self.ac_adc_drc_hkc
    }
    #[doc = "0x248 - ADC DRC Compressor Slope Low Setting Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_lkc(&self) -> &AC_ADC_DRC_LKC {
        &self.ac_adc_drc_lkc
    }
    #[doc = "0x24c - ADC DRC Compressor High Output at Compressor Threshold Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_hopc(&self) -> &AC_ADC_DRC_HOPC {
        &self.ac_adc_drc_hopc
    }
    #[doc = "0x250 - ADC DRC Compressor Low Output at Compressor Threshold Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_lopc(&self) -> &AC_ADC_DRC_LOPC {
        &self.ac_adc_drc_lopc
    }
    #[doc = "0x254 - ADC DRC Limiter Threshold High Setting Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_hlt(&self) -> &AC_ADC_DRC_HLT {
        &self.ac_adc_drc_hlt
    }
    #[doc = "0x258 - ADC DRC Limiter Threshold Low Setting Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_llt(&self) -> &AC_ADC_DRC_LLT {
        &self.ac_adc_drc_llt
    }
    #[doc = "0x25c - ADC DRC Limiter Slope High Setting Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_hkl(&self) -> &AC_ADC_DRC_HKL {
        &self.ac_adc_drc_hkl
    }
    #[doc = "0x260 - ADC DRC Limiter Slope Low Setting Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_lkl(&self) -> &AC_ADC_DRC_LKL {
        &self.ac_adc_drc_lkl
    }
    #[doc = "0x264 - ADC DRC Limiter High Output at Limiter Threshold"]
    #[inline(always)]
    pub const fn ac_adc_drc_hopl(&self) -> &AC_ADC_DRC_HOPL {
        &self.ac_adc_drc_hopl
    }
    #[doc = "0x268 - ADC DRC Limiter Low Output at Limiter Threshold"]
    #[inline(always)]
    pub const fn ac_adc_drc_lopl(&self) -> &AC_ADC_DRC_LOPL {
        &self.ac_adc_drc_lopl
    }
    #[doc = "0x26c - ADC DRC Expander Threshold High Setting Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_het(&self) -> &AC_ADC_DRC_HET {
        &self.ac_adc_drc_het
    }
    #[doc = "0x270 - ADC DRC Expander Threshold Low Setting Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_let(&self) -> &AC_ADC_DRC_LET {
        &self.ac_adc_drc_let
    }
    #[doc = "0x274 - ADC DRC Expander Slope High Setting Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_hke(&self) -> &AC_ADC_DRC_HKE {
        &self.ac_adc_drc_hke
    }
    #[doc = "0x278 - ADC DRC Expander Slope Low Setting Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_lke(&self) -> &AC_ADC_DRC_LKE {
        &self.ac_adc_drc_lke
    }
    #[doc = "0x27c - ADC DRC Expander High Output at Expander Threshold"]
    #[inline(always)]
    pub const fn ac_adc_drc_hope(&self) -> &AC_ADC_DRC_HOPE {
        &self.ac_adc_drc_hope
    }
    #[doc = "0x280 - ADC DRC Expander Low Output at Expander Threshold"]
    #[inline(always)]
    pub const fn ac_adc_drc_lope(&self) -> &AC_ADC_DRC_LOPE {
        &self.ac_adc_drc_lope
    }
    #[doc = "0x284 - ADC DRC Linear Slope High Setting Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_hkn(&self) -> &AC_ADC_DRC_HKN {
        &self.ac_adc_drc_hkn
    }
    #[doc = "0x288 - ADC DRC Linear Slope Low Setting Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_lkn(&self) -> &AC_ADC_DRC_LKN {
        &self.ac_adc_drc_lkn
    }
    #[doc = "0x28c - ADC DRC Smooth filter Gain High Attack Time Coef Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_sfhat(&self) -> &AC_ADC_DRC_SFHAT {
        &self.ac_adc_drc_sfhat
    }
    #[doc = "0x290 - ADC DRC Smooth filter Gain Low Attack Time Coef Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_sflat(&self) -> &AC_ADC_DRC_SFLAT {
        &self.ac_adc_drc_sflat
    }
    #[doc = "0x294 - ADC DRC Smooth filter Gain High Release Time Coef Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_sfhrt(&self) -> &AC_ADC_DRC_SFHRT {
        &self.ac_adc_drc_sfhrt
    }
    #[doc = "0x298 - ADC DRC Smooth filter Gain Low Release Time Coef Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_sflrt(&self) -> &AC_ADC_DRC_SFLRT {
        &self.ac_adc_drc_sflrt
    }
    #[doc = "0x29c - ADC DRC MAX Gain High Setting Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_mxghs(&self) -> &AC_ADC_DRC_MXGHS {
        &self.ac_adc_drc_mxghs
    }
    #[doc = "0x2a0 - ADC DRC MAX Gain Low Setting Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_mxgls(&self) -> &AC_ADC_DRC_MXGLS {
        &self.ac_adc_drc_mxgls
    }
    #[doc = "0x2a4 - ADC DRC MIN Gain High Setting Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_mnghs(&self) -> &AC_ADC_DRC_MNGHS {
        &self.ac_adc_drc_mnghs
    }
    #[doc = "0x2a8 - ADC DRC MIN Gain Low Setting Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_mngls(&self) -> &AC_ADC_DRC_MNGLS {
        &self.ac_adc_drc_mngls
    }
    #[doc = "0x2ac - ADC DRC Expander Smooth Time High Coef Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_epshc(&self) -> &AC_ADC_DRC_EPSHC {
        &self.ac_adc_drc_epshc
    }
    #[doc = "0x2b0 - ADC DRC Expander Smooth Time Low Coef Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_epslc(&self) -> &AC_ADC_DRC_EPSLC {
        &self.ac_adc_drc_epslc
    }
    #[doc = "0x2b8 - ADC DRC HPF Gain High Coef Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_hpfhgain(&self) -> &AC_ADC_DRC_HPFHGAIN {
        &self.ac_adc_drc_hpfhgain
    }
    #[doc = "0x2bc - ADC DRC HPF Gain Low Coef Register"]
    #[inline(always)]
    pub const fn ac_adc_drc_hpflgain(&self) -> &AC_ADC_DRC_HPFLGAIN {
        &self.ac_adc_drc_hpflgain
    }
    #[doc = "0x300..0x30c - ADC\\[i\\] Analog Control Register"]
    #[inline(always)]
    pub const fn adc(&self, n: usize) -> &ADC {
        &self.adc[n]
    }
    #[doc = "0x310 - DAC Analog Control Register"]
    #[inline(always)]
    pub const fn dac(&self) -> &DAC {
        &self.dac
    }
    #[doc = "0x318 - MICBIAS Analog Control Register"]
    #[inline(always)]
    pub const fn micbias(&self) -> &MICBIAS {
        &self.micbias
    }
    #[doc = "0x31c - BIAS Analog Control Register"]
    #[inline(always)]
    pub const fn ramp(&self) -> &RAMP {
        &self.ramp
    }
    #[doc = "0x320 - BIAS Analog Control Register"]
    #[inline(always)]
    pub const fn bias(&self) -> &BIAS {
        &self.bias
    }
    #[doc = "0x328 - HMIC Control Register"]
    #[inline(always)]
    pub const fn hmic_ctrl(&self) -> &HMIC_CTRL {
        &self.hmic_ctrl
    }
    #[doc = "0x32c - HMIC Status Register"]
    #[inline(always)]
    pub const fn hmic_sts(&self) -> &HMIC_STS {
        &self.hmic_sts
    }
    #[doc = "0x340 - Headphone2 Analog Control Register"]
    #[inline(always)]
    pub const fn hp2(&self) -> &HP2 {
        &self.hp2
    }
    #[doc = "0x348 - POWER Analog Control Register\n\nThe register is not controlled by the clock and reset of Audio Codec, only controlled by the clock and reset of system bus."]
    #[inline(always)]
    pub const fn power(&self) -> &POWER {
        &self.power
    }
}
#[doc = "ac_dac_dpc (rw) register accessor: DAC Digital Part Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_dpc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_dpc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_dpc`] module"]
pub type AC_DAC_DPC = crate::Reg<ac_dac_dpc::AC_DAC_DPC_SPEC>;
#[doc = "DAC Digital Part Control Register"]
pub mod ac_dac_dpc;
#[doc = "dac_vol_ctrl (rw) register accessor: DAC Volume Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_vol_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_vol_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_vol_ctrl`] module"]
pub type DAC_VOL_CTRL = crate::Reg<dac_vol_ctrl::DAC_VOL_CTRL_SPEC>;
#[doc = "DAC Volume Control Register"]
pub mod dac_vol_ctrl;
#[doc = "ac_dac_fifoc (rw) register accessor: DAC FIFO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_fifoc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_fifoc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_fifoc`] module"]
pub type AC_DAC_FIFOC = crate::Reg<ac_dac_fifoc::AC_DAC_FIFOC_SPEC>;
#[doc = "DAC FIFO Control Register"]
pub mod ac_dac_fifoc;
#[doc = "ac_dac_fifos (rw) register accessor: DAC FIFO Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_fifos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_fifos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_fifos`] module"]
pub type AC_DAC_FIFOS = crate::Reg<ac_dac_fifos::AC_DAC_FIFOS_SPEC>;
#[doc = "DAC FIFO Status Register"]
pub mod ac_dac_fifos;
#[doc = "ac_dac_txdata (w) register accessor: DAC TX DATA Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_txdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_txdata`] module"]
pub type AC_DAC_TXDATA = crate::Reg<ac_dac_txdata::AC_DAC_TXDATA_SPEC>;
#[doc = "DAC TX DATA Register"]
pub mod ac_dac_txdata;
#[doc = "ac_dac_cnt (rw) register accessor: DAC TX FIFO Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_cnt`] module"]
pub type AC_DAC_CNT = crate::Reg<ac_dac_cnt::AC_DAC_CNT_SPEC>;
#[doc = "DAC TX FIFO Counter Register"]
pub mod ac_dac_cnt;
#[doc = "ac_dac_dg (rw) register accessor: DAC Debug Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_dg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_dg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_dg`] module"]
pub type AC_DAC_DG = crate::Reg<ac_dac_dg::AC_DAC_DG_SPEC>;
#[doc = "DAC Debug Register"]
pub mod ac_dac_dg;
#[doc = "ac_adc_fifoc (rw) register accessor: ADC FIFO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_fifoc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_fifoc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_fifoc`] module"]
pub type AC_ADC_FIFOC = crate::Reg<ac_adc_fifoc::AC_ADC_FIFOC_SPEC>;
#[doc = "ADC FIFO Control Register"]
pub mod ac_adc_fifoc;
#[doc = "adc_vol_ctrl1 (rw) register accessor: ADC Volume Control1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_vol_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_vol_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_vol_ctrl1`] module"]
pub type ADC_VOL_CTRL1 = crate::Reg<adc_vol_ctrl1::ADC_VOL_CTRL1_SPEC>;
#[doc = "ADC Volume Control1 Register"]
pub mod adc_vol_ctrl1;
#[doc = "ac_adc_fifos (rw) register accessor: ADC FIFO Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_fifos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_fifos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_fifos`] module"]
pub type AC_ADC_FIFOS = crate::Reg<ac_adc_fifos::AC_ADC_FIFOS_SPEC>;
#[doc = "ADC FIFO Status Register"]
pub mod ac_adc_fifos;
#[doc = "ac_adc_rxdata (rw) register accessor: ADC RX Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_rxdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_rxdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_rxdata`] module"]
pub type AC_ADC_RXDATA = crate::Reg<ac_adc_rxdata::AC_ADC_RXDATA_SPEC>;
#[doc = "ADC RX Data Register"]
pub mod ac_adc_rxdata;
#[doc = "ac_adc_cnt (rw) register accessor: ADC RX Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_cnt`] module"]
pub type AC_ADC_CNT = crate::Reg<ac_adc_cnt::AC_ADC_CNT_SPEC>;
#[doc = "ADC RX Counter Register"]
pub mod ac_adc_cnt;
#[doc = "ac_adc_dg (rw) register accessor: ADC Debug Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_dg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_dg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_dg`] module"]
pub type AC_ADC_DG = crate::Reg<ac_adc_dg::AC_ADC_DG_SPEC>;
#[doc = "ADC Debug Register"]
pub mod ac_adc_dg;
#[doc = "adc_dig_ctrl (rw) register accessor: ADC Digtial Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_dig_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_dig_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_dig_ctrl`] module"]
pub type ADC_DIG_CTRL = crate::Reg<adc_dig_ctrl::ADC_DIG_CTRL_SPEC>;
#[doc = "ADC Digtial Control Register"]
pub mod adc_dig_ctrl;
#[doc = "vra1speedup_ctrl (rw) register accessor: VRA1 Speedup Down Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vra1speedup_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vra1speedup_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vra1speedup_ctrl`] module"]
pub type VRA1SPEEDUP_CTRL = crate::Reg<vra1speedup_ctrl::VRA1SPEEDUP_CTRL_SPEC>;
#[doc = "VRA1 Speedup Down Control Register"]
pub mod vra1speedup_ctrl;
#[doc = "ac_dac_dap_ctr (rw) register accessor: DAC DAP Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_dap_ctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_dap_ctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_dap_ctr`] module"]
pub type AC_DAC_DAP_CTR = crate::Reg<ac_dac_dap_ctr::AC_DAC_DAP_CTR_SPEC>;
#[doc = "DAC DAP Control Register"]
pub mod ac_dac_dap_ctr;
#[doc = "ac_adc_dap_ctr (rw) register accessor: ADC DAP Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_dap_ctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_dap_ctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_dap_ctr`] module"]
pub type AC_ADC_DAP_CTR = crate::Reg<ac_adc_dap_ctr::AC_ADC_DAP_CTR_SPEC>;
#[doc = "ADC DAP Control Register"]
pub mod ac_adc_dap_ctr;
#[doc = "ac_dac_drc_hhpfc (rw) register accessor: DAC DRC High HPF Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_hhpfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_hhpfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_hhpfc`] module"]
pub type AC_DAC_DRC_HHPFC = crate::Reg<ac_dac_drc_hhpfc::AC_DAC_DRC_HHPFC_SPEC>;
#[doc = "DAC DRC High HPF Coef Register"]
pub mod ac_dac_drc_hhpfc;
#[doc = "ac_dac_drc_lhpfc (rw) register accessor: DAC DRC Low HPF Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_lhpfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_lhpfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_lhpfc`] module"]
pub type AC_DAC_DRC_LHPFC = crate::Reg<ac_dac_drc_lhpfc::AC_DAC_DRC_LHPFC_SPEC>;
#[doc = "DAC DRC Low HPF Coef Register"]
pub mod ac_dac_drc_lhpfc;
#[doc = "ac_dac_drc_ctrl (rw) register accessor: DAC DRC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_ctrl`] module"]
pub type AC_DAC_DRC_CTRL = crate::Reg<ac_dac_drc_ctrl::AC_DAC_DRC_CTRL_SPEC>;
#[doc = "DAC DRC Control Register"]
pub mod ac_dac_drc_ctrl;
#[doc = "ac_dac_drc_lpfhat (rw) register accessor: DAC DRC Left Peak Filter High Attack Time Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_lpfhat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_lpfhat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_lpfhat`] module"]
pub type AC_DAC_DRC_LPFHAT = crate::Reg<ac_dac_drc_lpfhat::AC_DAC_DRC_LPFHAT_SPEC>;
#[doc = "DAC DRC Left Peak Filter High Attack Time Coef Register"]
pub mod ac_dac_drc_lpfhat;
#[doc = "ac_dac_drc_lpflat (rw) register accessor: DAC DRC Left Peak Filter Low Attack Time Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_lpflat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_lpflat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_lpflat`] module"]
pub type AC_DAC_DRC_LPFLAT = crate::Reg<ac_dac_drc_lpflat::AC_DAC_DRC_LPFLAT_SPEC>;
#[doc = "DAC DRC Left Peak Filter Low Attack Time Coef Register"]
pub mod ac_dac_drc_lpflat;
#[doc = "ac_dac_drc_rpfhat (rw) register accessor: DAC DRC Right Peak Filter High Attack Time Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_rpfhat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_rpfhat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_rpfhat`] module"]
pub type AC_DAC_DRC_RPFHAT = crate::Reg<ac_dac_drc_rpfhat::AC_DAC_DRC_RPFHAT_SPEC>;
#[doc = "DAC DRC Right Peak Filter High Attack Time Coef Register"]
pub mod ac_dac_drc_rpfhat;
#[doc = "ac_dac_drc_rpflat (rw) register accessor: DAC DRC Peak Filter Low Attack Time Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_rpflat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_rpflat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_rpflat`] module"]
pub type AC_DAC_DRC_RPFLAT = crate::Reg<ac_dac_drc_rpflat::AC_DAC_DRC_RPFLAT_SPEC>;
#[doc = "DAC DRC Peak Filter Low Attack Time Coef Register"]
pub mod ac_dac_drc_rpflat;
#[doc = "ac_dac_drc_lpfhrt (rw) register accessor: DAC DRC Left Peak Filter High Release Time Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_lpfhrt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_lpfhrt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_lpfhrt`] module"]
pub type AC_DAC_DRC_LPFHRT = crate::Reg<ac_dac_drc_lpfhrt::AC_DAC_DRC_LPFHRT_SPEC>;
#[doc = "DAC DRC Left Peak Filter High Release Time Coef Register"]
pub mod ac_dac_drc_lpfhrt;
#[doc = "ac_dac_drc_lpflrt (rw) register accessor: DAC DRC Left Peak Filter Low Release Time Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_lpflrt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_lpflrt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_lpflrt`] module"]
pub type AC_DAC_DRC_LPFLRT = crate::Reg<ac_dac_drc_lpflrt::AC_DAC_DRC_LPFLRT_SPEC>;
#[doc = "DAC DRC Left Peak Filter Low Release Time Coef Register"]
pub mod ac_dac_drc_lpflrt;
#[doc = "ac_dac_drc_rpfhrt (rw) register accessor: DAC DRC Right Peak filter High Release Time Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_rpfhrt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_rpfhrt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_rpfhrt`] module"]
pub type AC_DAC_DRC_RPFHRT = crate::Reg<ac_dac_drc_rpfhrt::AC_DAC_DRC_RPFHRT_SPEC>;
#[doc = "DAC DRC Right Peak filter High Release Time Coef Register"]
pub mod ac_dac_drc_rpfhrt;
#[doc = "ac_dac_drc_rpflrt (rw) register accessor: DAC DRC Right Peak filter Low Release Time Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_rpflrt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_rpflrt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_rpflrt`] module"]
pub type AC_DAC_DRC_RPFLRT = crate::Reg<ac_dac_drc_rpflrt::AC_DAC_DRC_RPFLRT_SPEC>;
#[doc = "DAC DRC Right Peak filter Low Release Time Coef Register"]
pub mod ac_dac_drc_rpflrt;
#[doc = "ac_dac_drc_lrmshat (rw) register accessor: DAC DRC Left RMS Filter High Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_lrmshat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_lrmshat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_lrmshat`] module"]
pub type AC_DAC_DRC_LRMSHAT = crate::Reg<ac_dac_drc_lrmshat::AC_DAC_DRC_LRMSHAT_SPEC>;
#[doc = "DAC DRC Left RMS Filter High Coef Register"]
pub mod ac_dac_drc_lrmshat;
#[doc = "ac_dac_drc_lrmslat (rw) register accessor: DAC DRC Left RMS Filter Low Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_lrmslat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_lrmslat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_lrmslat`] module"]
pub type AC_DAC_DRC_LRMSLAT = crate::Reg<ac_dac_drc_lrmslat::AC_DAC_DRC_LRMSLAT_SPEC>;
#[doc = "DAC DRC Left RMS Filter Low Coef Register"]
pub mod ac_dac_drc_lrmslat;
#[doc = "ac_dac_drc_rrmshat (rw) register accessor: DAC DRC Right RMS Filter High Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_rrmshat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_rrmshat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_rrmshat`] module"]
pub type AC_DAC_DRC_RRMSHAT = crate::Reg<ac_dac_drc_rrmshat::AC_DAC_DRC_RRMSHAT_SPEC>;
#[doc = "DAC DRC Right RMS Filter High Coef Register"]
pub mod ac_dac_drc_rrmshat;
#[doc = "ac_dac_drc_rrmslat (rw) register accessor: DAC DRC Right RMS Filter Low Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_rrmslat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_rrmslat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_rrmslat`] module"]
pub type AC_DAC_DRC_RRMSLAT = crate::Reg<ac_dac_drc_rrmslat::AC_DAC_DRC_RRMSLAT_SPEC>;
#[doc = "DAC DRC Right RMS Filter Low Coef Register"]
pub mod ac_dac_drc_rrmslat;
#[doc = "ac_dac_drc_hct (rw) register accessor: DAC DRC Compressor Threshold High Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_hct::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_hct::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_hct`] module"]
pub type AC_DAC_DRC_HCT = crate::Reg<ac_dac_drc_hct::AC_DAC_DRC_HCT_SPEC>;
#[doc = "DAC DRC Compressor Threshold High Setting Register"]
pub mod ac_dac_drc_hct;
#[doc = "ac_dac_drc_lct (rw) register accessor: DAC DRC Compressor Slope High Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_lct::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_lct::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_lct`] module"]
pub type AC_DAC_DRC_LCT = crate::Reg<ac_dac_drc_lct::AC_DAC_DRC_LCT_SPEC>;
#[doc = "DAC DRC Compressor Slope High Setting Register"]
pub mod ac_dac_drc_lct;
#[doc = "ac_dac_drc_hkc (rw) register accessor: DAC DRC Compressor Slope High Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_hkc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_hkc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_hkc`] module"]
pub type AC_DAC_DRC_HKC = crate::Reg<ac_dac_drc_hkc::AC_DAC_DRC_HKC_SPEC>;
#[doc = "DAC DRC Compressor Slope High Setting Register"]
pub mod ac_dac_drc_hkc;
#[doc = "ac_dac_drc_lkc (rw) register accessor: DAC DRC Compressor Slope Low Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_lkc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_lkc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_lkc`] module"]
pub type AC_DAC_DRC_LKC = crate::Reg<ac_dac_drc_lkc::AC_DAC_DRC_LKC_SPEC>;
#[doc = "DAC DRC Compressor Slope Low Setting Register"]
pub mod ac_dac_drc_lkc;
#[doc = "ac_dac_drc_hopc (rw) register accessor: DAC DRC Compressor High Output at Compressor Threshold Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_hopc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_hopc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_hopc`] module"]
pub type AC_DAC_DRC_HOPC = crate::Reg<ac_dac_drc_hopc::AC_DAC_DRC_HOPC_SPEC>;
#[doc = "DAC DRC Compressor High Output at Compressor Threshold Register"]
pub mod ac_dac_drc_hopc;
#[doc = "ac_dac_drc_lopc (rw) register accessor: DAC DRC Compressor Low Output at Compressor Threshold Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_lopc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_lopc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_lopc`] module"]
pub type AC_DAC_DRC_LOPC = crate::Reg<ac_dac_drc_lopc::AC_DAC_DRC_LOPC_SPEC>;
#[doc = "DAC DRC Compressor Low Output at Compressor Threshold Register"]
pub mod ac_dac_drc_lopc;
#[doc = "ac_dac_drc_hlt (rw) register accessor: DAC DRC Limiter Threshold High Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_hlt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_hlt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_hlt`] module"]
pub type AC_DAC_DRC_HLT = crate::Reg<ac_dac_drc_hlt::AC_DAC_DRC_HLT_SPEC>;
#[doc = "DAC DRC Limiter Threshold High Setting Register"]
pub mod ac_dac_drc_hlt;
#[doc = "ac_dac_drc_llt (rw) register accessor: DAC DRC Limiter Threshold Low Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_llt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_llt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_llt`] module"]
pub type AC_DAC_DRC_LLT = crate::Reg<ac_dac_drc_llt::AC_DAC_DRC_LLT_SPEC>;
#[doc = "DAC DRC Limiter Threshold Low Setting Register"]
pub mod ac_dac_drc_llt;
#[doc = "ac_dac_drc_hkl (rw) register accessor: DAC DRC Limiter Slope High Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_hkl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_hkl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_hkl`] module"]
pub type AC_DAC_DRC_HKL = crate::Reg<ac_dac_drc_hkl::AC_DAC_DRC_HKL_SPEC>;
#[doc = "DAC DRC Limiter Slope High Setting Register"]
pub mod ac_dac_drc_hkl;
#[doc = "ac_dac_drc_lkl (rw) register accessor: DAC DRC Limiter Slope Low Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_lkl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_lkl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_lkl`] module"]
pub type AC_DAC_DRC_LKL = crate::Reg<ac_dac_drc_lkl::AC_DAC_DRC_LKL_SPEC>;
#[doc = "DAC DRC Limiter Slope Low Setting Register"]
pub mod ac_dac_drc_lkl;
#[doc = "ac_dac_drc_hopl (rw) register accessor: DAC DRC Limiter High Output at Limiter Threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_hopl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_hopl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_hopl`] module"]
pub type AC_DAC_DRC_HOPL = crate::Reg<ac_dac_drc_hopl::AC_DAC_DRC_HOPL_SPEC>;
#[doc = "DAC DRC Limiter High Output at Limiter Threshold"]
pub mod ac_dac_drc_hopl;
#[doc = "ac_dac_drc_lopl (rw) register accessor: DAC DRC Limiter Low Output at Limiter Threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_lopl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_lopl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_lopl`] module"]
pub type AC_DAC_DRC_LOPL = crate::Reg<ac_dac_drc_lopl::AC_DAC_DRC_LOPL_SPEC>;
#[doc = "DAC DRC Limiter Low Output at Limiter Threshold"]
pub mod ac_dac_drc_lopl;
#[doc = "ac_dac_drc_het (rw) register accessor: DAC DRC Expander Threshold High Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_het::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_het::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_het`] module"]
pub type AC_DAC_DRC_HET = crate::Reg<ac_dac_drc_het::AC_DAC_DRC_HET_SPEC>;
#[doc = "DAC DRC Expander Threshold High Setting Register"]
pub mod ac_dac_drc_het;
#[doc = "ac_dac_drc_let (rw) register accessor: DAC DRC Expander Threshold Low Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_let::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_let::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_let`] module"]
pub type AC_DAC_DRC_LET = crate::Reg<ac_dac_drc_let::AC_DAC_DRC_LET_SPEC>;
#[doc = "DAC DRC Expander Threshold Low Setting Register"]
pub mod ac_dac_drc_let;
#[doc = "ac_dac_drc_hke (rw) register accessor: DAC DRC Expander Slope High Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_hke::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_hke::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_hke`] module"]
pub type AC_DAC_DRC_HKE = crate::Reg<ac_dac_drc_hke::AC_DAC_DRC_HKE_SPEC>;
#[doc = "DAC DRC Expander Slope High Setting Register"]
pub mod ac_dac_drc_hke;
#[doc = "ac_dac_drc_lke (rw) register accessor: DAC DRC Expander Slope Low Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_lke::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_lke::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_lke`] module"]
pub type AC_DAC_DRC_LKE = crate::Reg<ac_dac_drc_lke::AC_DAC_DRC_LKE_SPEC>;
#[doc = "DAC DRC Expander Slope Low Setting Register"]
pub mod ac_dac_drc_lke;
#[doc = "ac_dac_drc_hope (rw) register accessor: DAC DRC Expander High Output at Expander Threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_hope::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_hope::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_hope`] module"]
pub type AC_DAC_DRC_HOPE = crate::Reg<ac_dac_drc_hope::AC_DAC_DRC_HOPE_SPEC>;
#[doc = "DAC DRC Expander High Output at Expander Threshold"]
pub mod ac_dac_drc_hope;
#[doc = "ac_dac_drc_lope (rw) register accessor: DAC DRC Expander Low Output at Expander Threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_lope::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_lope::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_lope`] module"]
pub type AC_DAC_DRC_LOPE = crate::Reg<ac_dac_drc_lope::AC_DAC_DRC_LOPE_SPEC>;
#[doc = "DAC DRC Expander Low Output at Expander Threshold"]
pub mod ac_dac_drc_lope;
#[doc = "ac_dac_drc_hkn (rw) register accessor: DAC DRC Linear Slope High Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_hkn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_hkn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_hkn`] module"]
pub type AC_DAC_DRC_HKN = crate::Reg<ac_dac_drc_hkn::AC_DAC_DRC_HKN_SPEC>;
#[doc = "DAC DRC Linear Slope High Setting Register"]
pub mod ac_dac_drc_hkn;
#[doc = "ac_dac_drc_lkn (rw) register accessor: DAC DRC Linear Slope Low Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_lkn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_lkn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_lkn`] module"]
pub type AC_DAC_DRC_LKN = crate::Reg<ac_dac_drc_lkn::AC_DAC_DRC_LKN_SPEC>;
#[doc = "DAC DRC Linear Slope Low Setting Register"]
pub mod ac_dac_drc_lkn;
#[doc = "ac_dac_drc_sfhat (rw) register accessor: DAC DRC Smooth filter Gain High Attack Time Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_sfhat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_sfhat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_sfhat`] module"]
pub type AC_DAC_DRC_SFHAT = crate::Reg<ac_dac_drc_sfhat::AC_DAC_DRC_SFHAT_SPEC>;
#[doc = "DAC DRC Smooth filter Gain High Attack Time Coef Register"]
pub mod ac_dac_drc_sfhat;
#[doc = "ac_dac_drc_sflat (rw) register accessor: DAC DRC Smooth filter Gain Low Attack Time Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_sflat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_sflat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_sflat`] module"]
pub type AC_DAC_DRC_SFLAT = crate::Reg<ac_dac_drc_sflat::AC_DAC_DRC_SFLAT_SPEC>;
#[doc = "DAC DRC Smooth filter Gain Low Attack Time Coef Register"]
pub mod ac_dac_drc_sflat;
#[doc = "ac_dac_drc_sfhrt (rw) register accessor: DAC DRC Smooth filter Gain High Release Time Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_sfhrt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_sfhrt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_sfhrt`] module"]
pub type AC_DAC_DRC_SFHRT = crate::Reg<ac_dac_drc_sfhrt::AC_DAC_DRC_SFHRT_SPEC>;
#[doc = "DAC DRC Smooth filter Gain High Release Time Coef Register"]
pub mod ac_dac_drc_sfhrt;
#[doc = "ac_dac_drc_sflrt (rw) register accessor: DAC DRC Smooth filter Gain Low Release Time Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_sflrt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_sflrt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_sflrt`] module"]
pub type AC_DAC_DRC_SFLRT = crate::Reg<ac_dac_drc_sflrt::AC_DAC_DRC_SFLRT_SPEC>;
#[doc = "DAC DRC Smooth filter Gain Low Release Time Coef Register"]
pub mod ac_dac_drc_sflrt;
#[doc = "ac_dac_drc_mxghs (rw) register accessor: DAC DRC MAX Gain High Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_mxghs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_mxghs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_mxghs`] module"]
pub type AC_DAC_DRC_MXGHS = crate::Reg<ac_dac_drc_mxghs::AC_DAC_DRC_MXGHS_SPEC>;
#[doc = "DAC DRC MAX Gain High Setting Register"]
pub mod ac_dac_drc_mxghs;
#[doc = "ac_dac_drc_mxgls (rw) register accessor: DAC DRC MAX Gain Low Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_mxgls::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_mxgls::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_mxgls`] module"]
pub type AC_DAC_DRC_MXGLS = crate::Reg<ac_dac_drc_mxgls::AC_DAC_DRC_MXGLS_SPEC>;
#[doc = "DAC DRC MAX Gain Low Setting Register"]
pub mod ac_dac_drc_mxgls;
#[doc = "ac_dac_drc_mnghs (rw) register accessor: DAC DRC MIN Gain High Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_mnghs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_mnghs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_mnghs`] module"]
pub type AC_DAC_DRC_MNGHS = crate::Reg<ac_dac_drc_mnghs::AC_DAC_DRC_MNGHS_SPEC>;
#[doc = "DAC DRC MIN Gain High Setting Register"]
pub mod ac_dac_drc_mnghs;
#[doc = "ac_dac_drc_mngls (rw) register accessor: DAC DRC MIN Gain Low Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_mngls::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_mngls::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_mngls`] module"]
pub type AC_DAC_DRC_MNGLS = crate::Reg<ac_dac_drc_mngls::AC_DAC_DRC_MNGLS_SPEC>;
#[doc = "DAC DRC MIN Gain Low Setting Register"]
pub mod ac_dac_drc_mngls;
#[doc = "ac_dac_drc_epshc (rw) register accessor: DAC DRC Expander Smooth Time High Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_epshc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_epshc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_epshc`] module"]
pub type AC_DAC_DRC_EPSHC = crate::Reg<ac_dac_drc_epshc::AC_DAC_DRC_EPSHC_SPEC>;
#[doc = "DAC DRC Expander Smooth Time High Coef Register"]
pub mod ac_dac_drc_epshc;
#[doc = "ac_dac_drc_epslc (rw) register accessor: DAC DRC Expander Smooth Time Low Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_epslc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_epslc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_epslc`] module"]
pub type AC_DAC_DRC_EPSLC = crate::Reg<ac_dac_drc_epslc::AC_DAC_DRC_EPSLC_SPEC>;
#[doc = "DAC DRC Expander Smooth Time Low Coef Register"]
pub mod ac_dac_drc_epslc;
#[doc = "ac_dac_drc_hpfhgain (rw) register accessor: DAC DRC HPF Gain High Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_hpfhgain::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_hpfhgain::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_hpfhgain`] module"]
pub type AC_DAC_DRC_HPFHGAIN = crate::Reg<ac_dac_drc_hpfhgain::AC_DAC_DRC_HPFHGAIN_SPEC>;
#[doc = "DAC DRC HPF Gain High Coef Register"]
pub mod ac_dac_drc_hpfhgain;
#[doc = "ac_dac_drc_hpflgain (rw) register accessor: DAC DRC HPF Gain Low Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_hpflgain::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_hpflgain::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_dac_drc_hpflgain`] module"]
pub type AC_DAC_DRC_HPFLGAIN = crate::Reg<ac_dac_drc_hpflgain::AC_DAC_DRC_HPFLGAIN_SPEC>;
#[doc = "DAC DRC HPF Gain Low Coef Register"]
pub mod ac_dac_drc_hpflgain;
#[doc = "ac_adc_drc_hhpfc (rw) register accessor: ADC DRC High HPF Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_hhpfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_hhpfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_hhpfc`] module"]
pub type AC_ADC_DRC_HHPFC = crate::Reg<ac_adc_drc_hhpfc::AC_ADC_DRC_HHPFC_SPEC>;
#[doc = "ADC DRC High HPF Coef Register"]
pub mod ac_adc_drc_hhpfc;
#[doc = "ac_adc_drc_lhpfc (rw) register accessor: ADC DRC Low HPF Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_lhpfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_lhpfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_lhpfc`] module"]
pub type AC_ADC_DRC_LHPFC = crate::Reg<ac_adc_drc_lhpfc::AC_ADC_DRC_LHPFC_SPEC>;
#[doc = "ADC DRC Low HPF Coef Register"]
pub mod ac_adc_drc_lhpfc;
#[doc = "ac_adc_drc_ctrl (rw) register accessor: ADC DRC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_ctrl`] module"]
pub type AC_ADC_DRC_CTRL = crate::Reg<ac_adc_drc_ctrl::AC_ADC_DRC_CTRL_SPEC>;
#[doc = "ADC DRC Control Register"]
pub mod ac_adc_drc_ctrl;
#[doc = "ac_adc_drc_lpfhat (rw) register accessor: ADC DRC Left Peak Filter High Attack Time Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_lpfhat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_lpfhat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_lpfhat`] module"]
pub type AC_ADC_DRC_LPFHAT = crate::Reg<ac_adc_drc_lpfhat::AC_ADC_DRC_LPFHAT_SPEC>;
#[doc = "ADC DRC Left Peak Filter High Attack Time Coef Register"]
pub mod ac_adc_drc_lpfhat;
#[doc = "ac_adc_drc_lpflat (rw) register accessor: ADC DRC Left Peak Filter Low Attack Time Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_lpflat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_lpflat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_lpflat`] module"]
pub type AC_ADC_DRC_LPFLAT = crate::Reg<ac_adc_drc_lpflat::AC_ADC_DRC_LPFLAT_SPEC>;
#[doc = "ADC DRC Left Peak Filter Low Attack Time Coef Register"]
pub mod ac_adc_drc_lpflat;
#[doc = "ac_adc_drc_rpfhat (rw) register accessor: ADC DRC Right Peak Filter High Attack Time Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_rpfhat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_rpfhat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_rpfhat`] module"]
pub type AC_ADC_DRC_RPFHAT = crate::Reg<ac_adc_drc_rpfhat::AC_ADC_DRC_RPFHAT_SPEC>;
#[doc = "ADC DRC Right Peak Filter High Attack Time Coef Register"]
pub mod ac_adc_drc_rpfhat;
#[doc = "ac_adc_drc_rpflat (rw) register accessor: ADC DRC Right Peak Filter Low Attack Time Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_rpflat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_rpflat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_rpflat`] module"]
pub type AC_ADC_DRC_RPFLAT = crate::Reg<ac_adc_drc_rpflat::AC_ADC_DRC_RPFLAT_SPEC>;
#[doc = "ADC DRC Right Peak Filter Low Attack Time Coef Register"]
pub mod ac_adc_drc_rpflat;
#[doc = "ac_adc_drc_lpfhrt (rw) register accessor: ADC DRC Left Peak Filter High Release Time Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_lpfhrt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_lpfhrt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_lpfhrt`] module"]
pub type AC_ADC_DRC_LPFHRT = crate::Reg<ac_adc_drc_lpfhrt::AC_ADC_DRC_LPFHRT_SPEC>;
#[doc = "ADC DRC Left Peak Filter High Release Time Coef Register"]
pub mod ac_adc_drc_lpfhrt;
#[doc = "ac_adc_drc_lpflrt (rw) register accessor: ADC DRC Left Peak Filter Low Release Time Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_lpflrt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_lpflrt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_lpflrt`] module"]
pub type AC_ADC_DRC_LPFLRT = crate::Reg<ac_adc_drc_lpflrt::AC_ADC_DRC_LPFLRT_SPEC>;
#[doc = "ADC DRC Left Peak Filter Low Release Time Coef Register"]
pub mod ac_adc_drc_lpflrt;
#[doc = "ac_adc_drc_rpfhrt (rw) register accessor: ADC DRC Right Peak Filter High Release Time Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_rpfhrt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_rpfhrt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_rpfhrt`] module"]
pub type AC_ADC_DRC_RPFHRT = crate::Reg<ac_adc_drc_rpfhrt::AC_ADC_DRC_RPFHRT_SPEC>;
#[doc = "ADC DRC Right Peak Filter High Release Time Coef Register"]
pub mod ac_adc_drc_rpfhrt;
#[doc = "ac_adc_drc_rpflrt (rw) register accessor: ADC DRC Right Peak Filter Low Release Time Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_rpflrt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_rpflrt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_rpflrt`] module"]
pub type AC_ADC_DRC_RPFLRT = crate::Reg<ac_adc_drc_rpflrt::AC_ADC_DRC_RPFLRT_SPEC>;
#[doc = "ADC DRC Right Peak Filter Low Release Time Coef Register"]
pub mod ac_adc_drc_rpflrt;
#[doc = "ac_adc_drc_lrmshat (rw) register accessor: ADC DRC Left RMS Filter High Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_lrmshat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_lrmshat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_lrmshat`] module"]
pub type AC_ADC_DRC_LRMSHAT = crate::Reg<ac_adc_drc_lrmshat::AC_ADC_DRC_LRMSHAT_SPEC>;
#[doc = "ADC DRC Left RMS Filter High Coef Register"]
pub mod ac_adc_drc_lrmshat;
#[doc = "ac_adc_drc_lrmslat (rw) register accessor: ADC DRC Left RMS Filter Low Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_lrmslat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_lrmslat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_lrmslat`] module"]
pub type AC_ADC_DRC_LRMSLAT = crate::Reg<ac_adc_drc_lrmslat::AC_ADC_DRC_LRMSLAT_SPEC>;
#[doc = "ADC DRC Left RMS Filter Low Coef Register"]
pub mod ac_adc_drc_lrmslat;
#[doc = "ac_adc_drc_rrmshat (rw) register accessor: ADC DRC Right RMS Filter High Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_rrmshat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_rrmshat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_rrmshat`] module"]
pub type AC_ADC_DRC_RRMSHAT = crate::Reg<ac_adc_drc_rrmshat::AC_ADC_DRC_RRMSHAT_SPEC>;
#[doc = "ADC DRC Right RMS Filter High Coef Register"]
pub mod ac_adc_drc_rrmshat;
#[doc = "ac_adc_drc_rrmslat (rw) register accessor: ADC DRC Right RMS Filter Low Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_rrmslat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_rrmslat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_rrmslat`] module"]
pub type AC_ADC_DRC_RRMSLAT = crate::Reg<ac_adc_drc_rrmslat::AC_ADC_DRC_RRMSLAT_SPEC>;
#[doc = "ADC DRC Right RMS Filter Low Coef Register"]
pub mod ac_adc_drc_rrmslat;
#[doc = "ac_adc_drc_hct (rw) register accessor: ADC DRC Compressor Threshold High Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_hct::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_hct::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_hct`] module"]
pub type AC_ADC_DRC_HCT = crate::Reg<ac_adc_drc_hct::AC_ADC_DRC_HCT_SPEC>;
#[doc = "ADC DRC Compressor Threshold High Setting Register"]
pub mod ac_adc_drc_hct;
#[doc = "ac_adc_drc_lct (rw) register accessor: ADC DRC Compressor Slope High Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_lct::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_lct::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_lct`] module"]
pub type AC_ADC_DRC_LCT = crate::Reg<ac_adc_drc_lct::AC_ADC_DRC_LCT_SPEC>;
#[doc = "ADC DRC Compressor Slope High Setting Register"]
pub mod ac_adc_drc_lct;
#[doc = "ac_adc_drc_hkc (rw) register accessor: ADC DRC Compressor Slope High Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_hkc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_hkc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_hkc`] module"]
pub type AC_ADC_DRC_HKC = crate::Reg<ac_adc_drc_hkc::AC_ADC_DRC_HKC_SPEC>;
#[doc = "ADC DRC Compressor Slope High Setting Register"]
pub mod ac_adc_drc_hkc;
#[doc = "ac_adc_drc_lkc (rw) register accessor: ADC DRC Compressor Slope Low Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_lkc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_lkc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_lkc`] module"]
pub type AC_ADC_DRC_LKC = crate::Reg<ac_adc_drc_lkc::AC_ADC_DRC_LKC_SPEC>;
#[doc = "ADC DRC Compressor Slope Low Setting Register"]
pub mod ac_adc_drc_lkc;
#[doc = "ac_adc_drc_hopc (rw) register accessor: ADC DRC Compressor High Output at Compressor Threshold Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_hopc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_hopc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_hopc`] module"]
pub type AC_ADC_DRC_HOPC = crate::Reg<ac_adc_drc_hopc::AC_ADC_DRC_HOPC_SPEC>;
#[doc = "ADC DRC Compressor High Output at Compressor Threshold Register"]
pub mod ac_adc_drc_hopc;
#[doc = "ac_adc_drc_lopc (rw) register accessor: ADC DRC Compressor Low Output at Compressor Threshold Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_lopc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_lopc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_lopc`] module"]
pub type AC_ADC_DRC_LOPC = crate::Reg<ac_adc_drc_lopc::AC_ADC_DRC_LOPC_SPEC>;
#[doc = "ADC DRC Compressor Low Output at Compressor Threshold Register"]
pub mod ac_adc_drc_lopc;
#[doc = "ac_adc_drc_hlt (rw) register accessor: ADC DRC Limiter Threshold High Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_hlt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_hlt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_hlt`] module"]
pub type AC_ADC_DRC_HLT = crate::Reg<ac_adc_drc_hlt::AC_ADC_DRC_HLT_SPEC>;
#[doc = "ADC DRC Limiter Threshold High Setting Register"]
pub mod ac_adc_drc_hlt;
#[doc = "ac_adc_drc_llt (rw) register accessor: ADC DRC Limiter Threshold Low Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_llt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_llt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_llt`] module"]
pub type AC_ADC_DRC_LLT = crate::Reg<ac_adc_drc_llt::AC_ADC_DRC_LLT_SPEC>;
#[doc = "ADC DRC Limiter Threshold Low Setting Register"]
pub mod ac_adc_drc_llt;
#[doc = "ac_adc_drc_hkl (rw) register accessor: ADC DRC Limiter Slope High Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_hkl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_hkl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_hkl`] module"]
pub type AC_ADC_DRC_HKL = crate::Reg<ac_adc_drc_hkl::AC_ADC_DRC_HKL_SPEC>;
#[doc = "ADC DRC Limiter Slope High Setting Register"]
pub mod ac_adc_drc_hkl;
#[doc = "ac_adc_drc_lkl (rw) register accessor: ADC DRC Limiter Slope Low Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_lkl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_lkl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_lkl`] module"]
pub type AC_ADC_DRC_LKL = crate::Reg<ac_adc_drc_lkl::AC_ADC_DRC_LKL_SPEC>;
#[doc = "ADC DRC Limiter Slope Low Setting Register"]
pub mod ac_adc_drc_lkl;
#[doc = "ac_adc_drc_hopl (rw) register accessor: ADC DRC Limiter High Output at Limiter Threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_hopl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_hopl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_hopl`] module"]
pub type AC_ADC_DRC_HOPL = crate::Reg<ac_adc_drc_hopl::AC_ADC_DRC_HOPL_SPEC>;
#[doc = "ADC DRC Limiter High Output at Limiter Threshold"]
pub mod ac_adc_drc_hopl;
#[doc = "ac_adc_drc_lopl (rw) register accessor: ADC DRC Limiter Low Output at Limiter Threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_lopl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_lopl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_lopl`] module"]
pub type AC_ADC_DRC_LOPL = crate::Reg<ac_adc_drc_lopl::AC_ADC_DRC_LOPL_SPEC>;
#[doc = "ADC DRC Limiter Low Output at Limiter Threshold"]
pub mod ac_adc_drc_lopl;
#[doc = "ac_adc_drc_het (rw) register accessor: ADC DRC Expander Threshold High Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_het::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_het::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_het`] module"]
pub type AC_ADC_DRC_HET = crate::Reg<ac_adc_drc_het::AC_ADC_DRC_HET_SPEC>;
#[doc = "ADC DRC Expander Threshold High Setting Register"]
pub mod ac_adc_drc_het;
#[doc = "ac_adc_drc_let (rw) register accessor: ADC DRC Expander Threshold Low Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_let::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_let::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_let`] module"]
pub type AC_ADC_DRC_LET = crate::Reg<ac_adc_drc_let::AC_ADC_DRC_LET_SPEC>;
#[doc = "ADC DRC Expander Threshold Low Setting Register"]
pub mod ac_adc_drc_let;
#[doc = "ac_adc_drc_hke (rw) register accessor: ADC DRC Expander Slope High Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_hke::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_hke::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_hke`] module"]
pub type AC_ADC_DRC_HKE = crate::Reg<ac_adc_drc_hke::AC_ADC_DRC_HKE_SPEC>;
#[doc = "ADC DRC Expander Slope High Setting Register"]
pub mod ac_adc_drc_hke;
#[doc = "ac_adc_drc_lke (rw) register accessor: ADC DRC Expander Slope Low Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_lke::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_lke::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_lke`] module"]
pub type AC_ADC_DRC_LKE = crate::Reg<ac_adc_drc_lke::AC_ADC_DRC_LKE_SPEC>;
#[doc = "ADC DRC Expander Slope Low Setting Register"]
pub mod ac_adc_drc_lke;
#[doc = "ac_adc_drc_hope (rw) register accessor: ADC DRC Expander High Output at Expander Threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_hope::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_hope::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_hope`] module"]
pub type AC_ADC_DRC_HOPE = crate::Reg<ac_adc_drc_hope::AC_ADC_DRC_HOPE_SPEC>;
#[doc = "ADC DRC Expander High Output at Expander Threshold"]
pub mod ac_adc_drc_hope;
#[doc = "ac_adc_drc_lope (rw) register accessor: ADC DRC Expander Low Output at Expander Threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_lope::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_lope::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_lope`] module"]
pub type AC_ADC_DRC_LOPE = crate::Reg<ac_adc_drc_lope::AC_ADC_DRC_LOPE_SPEC>;
#[doc = "ADC DRC Expander Low Output at Expander Threshold"]
pub mod ac_adc_drc_lope;
#[doc = "ac_adc_drc_hkn (rw) register accessor: ADC DRC Linear Slope High Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_hkn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_hkn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_hkn`] module"]
pub type AC_ADC_DRC_HKN = crate::Reg<ac_adc_drc_hkn::AC_ADC_DRC_HKN_SPEC>;
#[doc = "ADC DRC Linear Slope High Setting Register"]
pub mod ac_adc_drc_hkn;
#[doc = "ac_adc_drc_lkn (rw) register accessor: ADC DRC Linear Slope Low Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_lkn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_lkn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_lkn`] module"]
pub type AC_ADC_DRC_LKN = crate::Reg<ac_adc_drc_lkn::AC_ADC_DRC_LKN_SPEC>;
#[doc = "ADC DRC Linear Slope Low Setting Register"]
pub mod ac_adc_drc_lkn;
#[doc = "ac_adc_drc_sfhat (rw) register accessor: ADC DRC Smooth filter Gain High Attack Time Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_sfhat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_sfhat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_sfhat`] module"]
pub type AC_ADC_DRC_SFHAT = crate::Reg<ac_adc_drc_sfhat::AC_ADC_DRC_SFHAT_SPEC>;
#[doc = "ADC DRC Smooth filter Gain High Attack Time Coef Register"]
pub mod ac_adc_drc_sfhat;
#[doc = "ac_adc_drc_sflat (rw) register accessor: ADC DRC Smooth filter Gain Low Attack Time Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_sflat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_sflat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_sflat`] module"]
pub type AC_ADC_DRC_SFLAT = crate::Reg<ac_adc_drc_sflat::AC_ADC_DRC_SFLAT_SPEC>;
#[doc = "ADC DRC Smooth filter Gain Low Attack Time Coef Register"]
pub mod ac_adc_drc_sflat;
#[doc = "ac_adc_drc_sfhrt (rw) register accessor: ADC DRC Smooth filter Gain High Release Time Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_sfhrt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_sfhrt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_sfhrt`] module"]
pub type AC_ADC_DRC_SFHRT = crate::Reg<ac_adc_drc_sfhrt::AC_ADC_DRC_SFHRT_SPEC>;
#[doc = "ADC DRC Smooth filter Gain High Release Time Coef Register"]
pub mod ac_adc_drc_sfhrt;
#[doc = "ac_adc_drc_sflrt (rw) register accessor: ADC DRC Smooth filter Gain Low Release Time Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_sflrt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_sflrt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_sflrt`] module"]
pub type AC_ADC_DRC_SFLRT = crate::Reg<ac_adc_drc_sflrt::AC_ADC_DRC_SFLRT_SPEC>;
#[doc = "ADC DRC Smooth filter Gain Low Release Time Coef Register"]
pub mod ac_adc_drc_sflrt;
#[doc = "ac_adc_drc_mxghs (rw) register accessor: ADC DRC MAX Gain High Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_mxghs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_mxghs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_mxghs`] module"]
pub type AC_ADC_DRC_MXGHS = crate::Reg<ac_adc_drc_mxghs::AC_ADC_DRC_MXGHS_SPEC>;
#[doc = "ADC DRC MAX Gain High Setting Register"]
pub mod ac_adc_drc_mxghs;
#[doc = "ac_adc_drc_mxgls (rw) register accessor: ADC DRC MAX Gain Low Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_mxgls::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_mxgls::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_mxgls`] module"]
pub type AC_ADC_DRC_MXGLS = crate::Reg<ac_adc_drc_mxgls::AC_ADC_DRC_MXGLS_SPEC>;
#[doc = "ADC DRC MAX Gain Low Setting Register"]
pub mod ac_adc_drc_mxgls;
#[doc = "ac_adc_drc_mnghs (rw) register accessor: ADC DRC MIN Gain High Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_mnghs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_mnghs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_mnghs`] module"]
pub type AC_ADC_DRC_MNGHS = crate::Reg<ac_adc_drc_mnghs::AC_ADC_DRC_MNGHS_SPEC>;
#[doc = "ADC DRC MIN Gain High Setting Register"]
pub mod ac_adc_drc_mnghs;
#[doc = "ac_adc_drc_mngls (rw) register accessor: ADC DRC MIN Gain Low Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_mngls::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_mngls::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_mngls`] module"]
pub type AC_ADC_DRC_MNGLS = crate::Reg<ac_adc_drc_mngls::AC_ADC_DRC_MNGLS_SPEC>;
#[doc = "ADC DRC MIN Gain Low Setting Register"]
pub mod ac_adc_drc_mngls;
#[doc = "ac_adc_drc_epshc (rw) register accessor: ADC DRC Expander Smooth Time High Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_epshc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_epshc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_epshc`] module"]
pub type AC_ADC_DRC_EPSHC = crate::Reg<ac_adc_drc_epshc::AC_ADC_DRC_EPSHC_SPEC>;
#[doc = "ADC DRC Expander Smooth Time High Coef Register"]
pub mod ac_adc_drc_epshc;
#[doc = "ac_adc_drc_epslc (rw) register accessor: ADC DRC Expander Smooth Time Low Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_epslc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_epslc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_epslc`] module"]
pub type AC_ADC_DRC_EPSLC = crate::Reg<ac_adc_drc_epslc::AC_ADC_DRC_EPSLC_SPEC>;
#[doc = "ADC DRC Expander Smooth Time Low Coef Register"]
pub mod ac_adc_drc_epslc;
#[doc = "ac_adc_drc_hpfhgain (rw) register accessor: ADC DRC HPF Gain High Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_hpfhgain::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_hpfhgain::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_hpfhgain`] module"]
pub type AC_ADC_DRC_HPFHGAIN = crate::Reg<ac_adc_drc_hpfhgain::AC_ADC_DRC_HPFHGAIN_SPEC>;
#[doc = "ADC DRC HPF Gain High Coef Register"]
pub mod ac_adc_drc_hpfhgain;
#[doc = "ac_adc_drc_hpflgain (rw) register accessor: ADC DRC HPF Gain Low Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_hpflgain::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_hpflgain::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_adc_drc_hpflgain`] module"]
pub type AC_ADC_DRC_HPFLGAIN = crate::Reg<ac_adc_drc_hpflgain::AC_ADC_DRC_HPFLGAIN_SPEC>;
#[doc = "ADC DRC HPF Gain Low Coef Register"]
pub mod ac_adc_drc_hpflgain;
#[doc = "adc (rw) register accessor: ADC\\[i\\] Analog Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc`] module"]
pub type ADC = crate::Reg<adc::ADC_SPEC>;
#[doc = "ADC\\[i\\] Analog Control Register"]
pub mod adc;
#[doc = "dac (rw) register accessor: DAC Analog Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac`] module"]
pub type DAC = crate::Reg<dac::DAC_SPEC>;
#[doc = "DAC Analog Control Register"]
pub mod dac;
#[doc = "micbias (rw) register accessor: MICBIAS Analog Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`micbias::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`micbias::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@micbias`] module"]
pub type MICBIAS = crate::Reg<micbias::MICBIAS_SPEC>;
#[doc = "MICBIAS Analog Control Register"]
pub mod micbias;
#[doc = "ramp (rw) register accessor: BIAS Analog Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ramp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ramp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ramp`] module"]
pub type RAMP = crate::Reg<ramp::RAMP_SPEC>;
#[doc = "BIAS Analog Control Register"]
pub mod ramp;
#[doc = "bias (rw) register accessor: BIAS Analog Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bias::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bias::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bias`] module"]
pub type BIAS = crate::Reg<bias::BIAS_SPEC>;
#[doc = "BIAS Analog Control Register"]
pub mod bias;
#[doc = "hmic_ctrl (rw) register accessor: HMIC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hmic_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hmic_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hmic_ctrl`] module"]
pub type HMIC_CTRL = crate::Reg<hmic_ctrl::HMIC_CTRL_SPEC>;
#[doc = "HMIC Control Register"]
pub mod hmic_ctrl;
#[doc = "hmic_sts (rw) register accessor: HMIC Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hmic_sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hmic_sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hmic_sts`] module"]
pub type HMIC_STS = crate::Reg<hmic_sts::HMIC_STS_SPEC>;
#[doc = "HMIC Status Register"]
pub mod hmic_sts;
#[doc = "hp2 (rw) register accessor: Headphone2 Analog Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp2`] module"]
pub type HP2 = crate::Reg<hp2::HP2_SPEC>;
#[doc = "Headphone2 Analog Control Register"]
pub mod hp2;
#[doc = "power (rw) register accessor: POWER Analog Control Register\n\nThe register is not controlled by the clock and reset of Audio Codec, only controlled by the clock and reset of system bus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power`] module"]
pub type POWER = crate::Reg<power::POWER_SPEC>;
#[doc = "POWER Analog Control Register\n\nThe register is not controlled by the clock and reset of Audio Codec, only controlled by the clock and reset of system bus."]
pub mod power;
