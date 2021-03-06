#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DAC Digital Part Control Register"]
    pub ac_dac_dpc: crate::Reg<ac_dac_dpc::AC_DAC_DPC_SPEC>,
    #[doc = "0x04 - DAC Volume Control Register"]
    pub dac_vol_ctrl: crate::Reg<dac_vol_ctrl::DAC_VOL_CTRL_SPEC>,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - DAC FIFO Control Register"]
    pub ac_dac_fifoc: crate::Reg<ac_dac_fifoc::AC_DAC_FIFOC_SPEC>,
    #[doc = "0x14 - DAC FIFO Status Register"]
    pub ac_dac_fifos: crate::Reg<ac_dac_fifos::AC_DAC_FIFOS_SPEC>,
    _reserved4: [u8; 0x08],
    #[doc = "0x20 - DAC TX DATA Register"]
    pub ac_dac_txdata: crate::Reg<ac_dac_txdata::AC_DAC_TXDATA_SPEC>,
    #[doc = "0x24 - DAC TX FIFO Counter Register"]
    pub ac_dac_cnt: crate::Reg<ac_dac_cnt::AC_DAC_CNT_SPEC>,
    #[doc = "0x28 - DAC Debug Register"]
    pub ac_dac_dg: crate::Reg<ac_dac_dg::AC_DAC_DG_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x30 - ADC FIFO Control Register"]
    pub ac_adc_fifoc: crate::Reg<ac_adc_fifoc::AC_ADC_FIFOC_SPEC>,
    #[doc = "0x34 - ADC Volume Control1 Register"]
    pub adc_vol_ctrl1: crate::Reg<adc_vol_ctrl1::ADC_VOL_CTRL1_SPEC>,
    #[doc = "0x38 - ADC FIFO Status Register"]
    pub ac_adc_fifos: crate::Reg<ac_adc_fifos::AC_ADC_FIFOS_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x40 - ADC RX Data Register"]
    pub ac_adc_rxdata: crate::Reg<ac_adc_rxdata::AC_ADC_RXDATA_SPEC>,
    #[doc = "0x44 - ADC RX Counter Register"]
    pub ac_adc_cnt: crate::Reg<ac_adc_cnt::AC_ADC_CNT_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x4c - ADC Debug Register"]
    pub ac_adc_dg: crate::Reg<ac_adc_dg::AC_ADC_DG_SPEC>,
    #[doc = "0x50 - ADC Digtial Control Register"]
    pub adc_dig_ctrl: crate::Reg<adc_dig_ctrl::ADC_DIG_CTRL_SPEC>,
    #[doc = "0x54 - VRA1 Speedup Down Control Register"]
    pub vra1speedup_down_ctrl: crate::Reg<vra1speedup_down_ctrl::VRA1SPEEDUP_DOWN_CTRL_SPEC>,
    _reserved15: [u8; 0x98],
    #[doc = "0xf0 - DAC DAP Control Register"]
    pub ac_dac_dap_ctrl: crate::Reg<ac_dac_dap_ctrl::AC_DAC_DAP_CTRL_SPEC>,
    _reserved16: [u8; 0x04],
    #[doc = "0xf8 - ADC DAP Control Register"]
    pub ac_adc_dap_ctr: crate::Reg<ac_adc_dap_ctr::AC_ADC_DAP_CTR_SPEC>,
    _reserved17: [u8; 0x04],
    #[doc = "0x100 - DAC DRC High HPF Coef Register"]
    pub ac_dac_drc_hhpfc: crate::Reg<ac_dac_drc_hhpfc::AC_DAC_DRC_HHPFC_SPEC>,
    #[doc = "0x104 - DAC DRC Low HPF Coef Register"]
    pub ac_dac_drc_lhpfc: crate::Reg<ac_dac_drc_lhpfc::AC_DAC_DRC_LHPFC_SPEC>,
    #[doc = "0x108 - DAC DRC Control Register"]
    pub ac_dac_drc_ctrl: crate::Reg<ac_dac_drc_ctrl::AC_DAC_DRC_CTRL_SPEC>,
    #[doc = "0x10c - DAC DRC Left Peak Filter High Attack Time Coef Register"]
    pub ac_dac_drc_lpfhat: crate::Reg<ac_dac_drc_lpfhat::AC_DAC_DRC_LPFHAT_SPEC>,
    #[doc = "0x110 - DAC DRC Left Peak Filter Low Attack Time Coef Register"]
    pub ac_dac_drc_lpflat: crate::Reg<ac_dac_drc_lpflat::AC_DAC_DRC_LPFLAT_SPEC>,
    #[doc = "0x114 - DAC DRC Right Peak Filter High Attack Time Coef Register"]
    pub ac_dac_drc_rpfhat: crate::Reg<ac_dac_drc_rpfhat::AC_DAC_DRC_RPFHAT_SPEC>,
    #[doc = "0x118 - DAC DRC Peak Filter Low Attack Time Coef Register"]
    pub ac_dac_drc_rpflat: crate::Reg<ac_dac_drc_rpflat::AC_DAC_DRC_RPFLAT_SPEC>,
    #[doc = "0x11c - DAC DRC Left Peak Filter High Release Time Coef Register"]
    pub ac_dac_drc_lpfhrt: crate::Reg<ac_dac_drc_lpfhrt::AC_DAC_DRC_LPFHRT_SPEC>,
    #[doc = "0x120 - DAC DRC Left Peak Filter Low Release Time Coef Register"]
    pub ac_dac_drc_lpflrt: crate::Reg<ac_dac_drc_lpflrt::AC_DAC_DRC_LPFLRT_SPEC>,
    #[doc = "0x124 - DAC DRC Right Peak filter High Release Time Coef Register"]
    pub ac_dac_drc_rpfhrt: crate::Reg<ac_dac_drc_rpfhrt::AC_DAC_DRC_RPFHRT_SPEC>,
    #[doc = "0x128 - DAC DRC Right Peak filter Low Release Time Coef Register"]
    pub ac_dac_drc_rpflrt: crate::Reg<ac_dac_drc_rpflrt::AC_DAC_DRC_RPFLRT_SPEC>,
    #[doc = "0x12c - DAC DRC Left RMS Filter High Coef Register"]
    pub ac_dac_drc_lrmshat: crate::Reg<ac_dac_drc_lrmshat::AC_DAC_DRC_LRMSHAT_SPEC>,
    #[doc = "0x130 - DAC DRC Left RMS Filter Low Coef Register"]
    pub ac_dac_drc_lrmslat: crate::Reg<ac_dac_drc_lrmslat::AC_DAC_DRC_LRMSLAT_SPEC>,
    #[doc = "0x134 - DAC DRC Right RMS Filter High Coef Register"]
    pub ac_dac_drc_rrmshat: crate::Reg<ac_dac_drc_rrmshat::AC_DAC_DRC_RRMSHAT_SPEC>,
    #[doc = "0x138 - DAC DRC Right RMS Filter Low Coef Register"]
    pub ac_dac_drc_rrmslat: crate::Reg<ac_dac_drc_rrmslat::AC_DAC_DRC_RRMSLAT_SPEC>,
    #[doc = "0x13c - DAC DRC Compressor Threshold High Setting Register"]
    pub ac_dac_drc_hct: crate::Reg<ac_dac_drc_hct::AC_DAC_DRC_HCT_SPEC>,
    #[doc = "0x140 - DAC DRC Compressor Slope High Setting Register"]
    pub ac_dac_drc_lct: crate::Reg<ac_dac_drc_lct::AC_DAC_DRC_LCT_SPEC>,
    #[doc = "0x144 - DAC DRC Compressor Slope High Setting Register"]
    pub ac_dac_drc_hkc: crate::Reg<ac_dac_drc_hkc::AC_DAC_DRC_HKC_SPEC>,
    #[doc = "0x148 - DAC DRC Compressor Slope Low Setting Register"]
    pub ac_dac_drc_lkc: crate::Reg<ac_dac_drc_lkc::AC_DAC_DRC_LKC_SPEC>,
    #[doc = "0x14c - DAC DRC Compressor High Output at Compressor Threshold Register"]
    pub ac_dac_drc_hopc: crate::Reg<ac_dac_drc_hopc::AC_DAC_DRC_HOPC_SPEC>,
    #[doc = "0x150 - DAC DRC Compressor Low Output at Compressor Threshold Register"]
    pub ac_dac_drc_lopc: crate::Reg<ac_dac_drc_lopc::AC_DAC_DRC_LOPC_SPEC>,
    #[doc = "0x154 - DAC DRC Limiter Threshold High Setting Register"]
    pub ac_dac_drc_hlt: crate::Reg<ac_dac_drc_hlt::AC_DAC_DRC_HLT_SPEC>,
    #[doc = "0x158 - DAC DRC Limiter Threshold Low Setting Register"]
    pub ac_dac_drc_llt: crate::Reg<ac_dac_drc_llt::AC_DAC_DRC_LLT_SPEC>,
    #[doc = "0x15c - DAC DRC Limiter Slope High Setting Register"]
    pub ac_dac_drc_h_kl: crate::Reg<ac_dac_drc_h_kl::AC_DAC_DRC_H_KL_SPEC>,
    #[doc = "0x160 - DAC DRC Limiter Slope Low Setting Register"]
    pub ac_dac_drc_l_kl: crate::Reg<ac_dac_drc_l_kl::AC_DAC_DRC_L_KL_SPEC>,
    #[doc = "0x164 - DAC DRC Limiter High Output at Limiter Threshold"]
    pub ac_dac_drc_hopl: crate::Reg<ac_dac_drc_hopl::AC_DAC_DRC_HOPL_SPEC>,
    #[doc = "0x168 - DAC DRC Limiter Low Output at Limiter Threshold"]
    pub ac_dac_drc_lopl: crate::Reg<ac_dac_drc_lopl::AC_DAC_DRC_LOPL_SPEC>,
    #[doc = "0x16c - DAC DRC Expander Threshold High Setting Register"]
    pub ac_dac_drc_het: crate::Reg<ac_dac_drc_het::AC_DAC_DRC_HET_SPEC>,
    #[doc = "0x170 - DAC DRC Expander Threshold Low Setting Register"]
    pub ac_dac_drc_let: crate::Reg<ac_dac_drc_let::AC_DAC_DRC_LET_SPEC>,
    #[doc = "0x174 - DAC DRC Expander Slope High Setting Register"]
    pub ac_dac_drc_hke: crate::Reg<ac_dac_drc_hke::AC_DAC_DRC_HKE_SPEC>,
    #[doc = "0x178 - DAC DRC Expander Slope Low Setting Register"]
    pub ac_dac_drc_lke: crate::Reg<ac_dac_drc_lke::AC_DAC_DRC_LKE_SPEC>,
    #[doc = "0x17c - DAC DRC Expander High Output at Expander Threshold"]
    pub ac_dac_drc_hope: crate::Reg<ac_dac_drc_hope::AC_DAC_DRC_HOPE_SPEC>,
    #[doc = "0x180 - DAC DRC Expander Low Output at Expander Threshold"]
    pub ac_dac_drc_lope: crate::Reg<ac_dac_drc_lope::AC_DAC_DRC_LOPE_SPEC>,
    #[doc = "0x184 - DAC DRC Linear Slope High Setting Register"]
    pub ac_dac_drc_hkn: crate::Reg<ac_dac_drc_hkn::AC_DAC_DRC_HKN_SPEC>,
    #[doc = "0x188 - DAC DRC Linear Slope Low Setting Register"]
    pub ac_dac_drc_lkn: crate::Reg<ac_dac_drc_lkn::AC_DAC_DRC_LKN_SPEC>,
    #[doc = "0x18c - DAC DRC Smooth filter Gain High Attack Time Coef Register"]
    pub ac_dac_drc_sfhat: crate::Reg<ac_dac_drc_sfhat::AC_DAC_DRC_SFHAT_SPEC>,
    #[doc = "0x190 - DAC DRC Smooth filter Gain Low Attack Time Coef Register"]
    pub ac_dac_drc_sflat: crate::Reg<ac_dac_drc_sflat::AC_DAC_DRC_SFLAT_SPEC>,
    #[doc = "0x194 - DAC DRC Smooth filter Gain High Release Time Coef Register"]
    pub ac_dac_drc_sfhrt: crate::Reg<ac_dac_drc_sfhrt::AC_DAC_DRC_SFHRT_SPEC>,
    #[doc = "0x198 - DAC DRC Smooth filter Gain Low Release Time Coef Register"]
    pub ac_dac_drc_sflrt: crate::Reg<ac_dac_drc_sflrt::AC_DAC_DRC_SFLRT_SPEC>,
    #[doc = "0x19c - DAC DRC MAX Gain High Setting Register"]
    pub ac_dac_drc_mxghs: crate::Reg<ac_dac_drc_mxghs::AC_DAC_DRC_MXGHS_SPEC>,
    #[doc = "0x1a0 - DAC DRC MAX Gain Low Setting Register"]
    pub ac_dac_drc_mxgls: crate::Reg<ac_dac_drc_mxgls::AC_DAC_DRC_MXGLS_SPEC>,
    #[doc = "0x1a4 - DAC DRC MIN Gain High Setting Register"]
    pub ac_dac_drc_mnghs: crate::Reg<ac_dac_drc_mnghs::AC_DAC_DRC_MNGHS_SPEC>,
    #[doc = "0x1a8 - DAC DRC MIN Gain Low Setting Register"]
    pub ac_dac_drc_mngls: crate::Reg<ac_dac_drc_mngls::AC_DAC_DRC_MNGLS_SPEC>,
    #[doc = "0x1ac - DAC DRC Expander Smooth Time High Coef Register"]
    pub ac_dac_drc_epshc: crate::Reg<ac_dac_drc_epshc::AC_DAC_DRC_EPSHC_SPEC>,
    #[doc = "0x1b0 - DAC DRC Expander Smooth Time Low Coef Register"]
    pub ac_dac_drc_epslc: crate::Reg<ac_dac_drc_epslc::AC_DAC_DRC_EPSLC_SPEC>,
    _reserved62: [u8; 0x04],
    #[doc = "0x1b8 - DAC DRC HPF Gain High Coef Register"]
    pub ac_dac_drc_hpfhgain: crate::Reg<ac_dac_drc_hpfhgain::AC_DAC_DRC_HPFHGAIN_SPEC>,
    #[doc = "0x1bc - DAC DRC HPF Gain Low Coef Register"]
    pub ac_dac_drc_hpflgain: crate::Reg<ac_dac_drc_hpflgain::AC_DAC_DRC_HPFLGAIN_SPEC>,
    _reserved64: [u8; 0x40],
    #[doc = "0x200 - ADC DRC High HPF Coef Register"]
    pub ac_adc_drc_hhpfc: crate::Reg<ac_adc_drc_hhpfc::AC_ADC_DRC_HHPFC_SPEC>,
    #[doc = "0x204 - ADC DRC Low HPF Coef Register"]
    pub ac_adc_drc_lhpfc: crate::Reg<ac_adc_drc_lhpfc::AC_ADC_DRC_LHPFC_SPEC>,
    #[doc = "0x208 - ADC DRC Control Register"]
    pub ac_adc_drc_ctrl: crate::Reg<ac_adc_drc_ctrl::AC_ADC_DRC_CTRL_SPEC>,
    #[doc = "0x20c - ADC DRC Left Peak Filter High Attack Time Coef Register"]
    pub ac_adc_drc_lpfhat: crate::Reg<ac_adc_drc_lpfhat::AC_ADC_DRC_LPFHAT_SPEC>,
    #[doc = "0x210 - ADC DRC Left Peak Filter Low Attack Time Coef Register"]
    pub ac_adc_drc_lpflat: crate::Reg<ac_adc_drc_lpflat::AC_ADC_DRC_LPFLAT_SPEC>,
    #[doc = "0x214 - ADC DRC Right Peak Filter High Attack Time Coef Register"]
    pub ac_adc_drc_rpfhat: crate::Reg<ac_adc_drc_rpfhat::AC_ADC_DRC_RPFHAT_SPEC>,
    #[doc = "0x218 - ADC DRC Right Peak Filter Low Attack Time Coef Register"]
    pub ac_adc_drc_rpflat: crate::Reg<ac_adc_drc_rpflat::AC_ADC_DRC_RPFLAT_SPEC>,
    #[doc = "0x21c - ADC DRC Left Peak Filter High Release Time Coef Register"]
    pub ac_adc_drc_lpfhrt: crate::Reg<ac_adc_drc_lpfhrt::AC_ADC_DRC_LPFHRT_SPEC>,
    #[doc = "0x220 - ADC DRC Left Peak Filter Low Release Time Coef Register"]
    pub ac_adc_drc_lpflrt: crate::Reg<ac_adc_drc_lpflrt::AC_ADC_DRC_LPFLRT_SPEC>,
    #[doc = "0x224 - ADC DRC Right Peak Filter High Release Time Coef Register"]
    pub ac_adc_drc_rpfhrt: crate::Reg<ac_adc_drc_rpfhrt::AC_ADC_DRC_RPFHRT_SPEC>,
    #[doc = "0x228 - ADC DRC Right Peak Filter Low Release Time Coef Register"]
    pub ac_adc_drc_rpflrt: crate::Reg<ac_adc_drc_rpflrt::AC_ADC_DRC_RPFLRT_SPEC>,
    #[doc = "0x22c - ADC DRC Left RMS Filter High Coef Register"]
    pub ac_adc_drc_lrmshat: crate::Reg<ac_adc_drc_lrmshat::AC_ADC_DRC_LRMSHAT_SPEC>,
    #[doc = "0x230 - ADC DRC Left RMS Filter Low Coef Register"]
    pub ac_adc_drc_lrmslat: crate::Reg<ac_adc_drc_lrmslat::AC_ADC_DRC_LRMSLAT_SPEC>,
    #[doc = "0x234 - ADC DRC Right RMS Filter High Coef Register"]
    pub ac_adc_drc_rrmshat: crate::Reg<ac_adc_drc_rrmshat::AC_ADC_DRC_RRMSHAT_SPEC>,
    #[doc = "0x238 - ADC DRC Right RMS Filter Low Coef Register"]
    pub ac_adc_drc_rrmslat: crate::Reg<ac_adc_drc_rrmslat::AC_ADC_DRC_RRMSLAT_SPEC>,
    #[doc = "0x23c - ADC DRC Compressor Threshold High Setting Register"]
    pub ac_adc_drc_hct: crate::Reg<ac_adc_drc_hct::AC_ADC_DRC_HCT_SPEC>,
    #[doc = "0x240 - ADC DRC Compressor Slope High Setting Register"]
    pub ac_adc_drc_lct: crate::Reg<ac_adc_drc_lct::AC_ADC_DRC_LCT_SPEC>,
    #[doc = "0x244 - ADC DRC Compressor Slope High Setting Register"]
    pub ac_adc_drc_hkc: crate::Reg<ac_adc_drc_hkc::AC_ADC_DRC_HKC_SPEC>,
    #[doc = "0x248 - ADC DRC Compressor Slope Low Setting Register"]
    pub ac_adc_drc_lkc: crate::Reg<ac_adc_drc_lkc::AC_ADC_DRC_LKC_SPEC>,
    #[doc = "0x24c - ADC DRC Compressor High Output at Compressor Threshold Register"]
    pub ac_adc_drc_hopc: crate::Reg<ac_adc_drc_hopc::AC_ADC_DRC_HOPC_SPEC>,
    #[doc = "0x250 - ADC DRC Compressor Low Output at Compressor Threshold Register"]
    pub ac_adc_drc_lopc: crate::Reg<ac_adc_drc_lopc::AC_ADC_DRC_LOPC_SPEC>,
    #[doc = "0x254 - ADC DRC Limiter Threshold High Setting Register"]
    pub ac_adc_drc_hlt: crate::Reg<ac_adc_drc_hlt::AC_ADC_DRC_HLT_SPEC>,
    #[doc = "0x258 - ADC DRC Limiter Threshold Low Setting Register"]
    pub ac_adc_drc_llt: crate::Reg<ac_adc_drc_llt::AC_ADC_DRC_LLT_SPEC>,
    #[doc = "0x25c - ADC DRC Limiter Slope High Setting Register"]
    pub ac_adc_drc_h_kl: crate::Reg<ac_adc_drc_h_kl::AC_ADC_DRC_H_KL_SPEC>,
    #[doc = "0x260 - ADC DRC Limiter Slope Low Setting Register"]
    pub ac_adc_drc_l_kl: crate::Reg<ac_adc_drc_l_kl::AC_ADC_DRC_L_KL_SPEC>,
    #[doc = "0x264 - ADC DRC Limiter High Output at Limiter Threshold"]
    pub ac_adc_drc_hopl: crate::Reg<ac_adc_drc_hopl::AC_ADC_DRC_HOPL_SPEC>,
    #[doc = "0x268 - ADC DRC Limiter Low Output at Limiter Threshold"]
    pub ac_adc_drc_lopl: crate::Reg<ac_adc_drc_lopl::AC_ADC_DRC_LOPL_SPEC>,
    #[doc = "0x26c - ADC DRC Expander Threshold High Setting Register"]
    pub ac_adc_drc_het: crate::Reg<ac_adc_drc_het::AC_ADC_DRC_HET_SPEC>,
    #[doc = "0x270 - ADC DRC Expander Threshold Low Setting Register"]
    pub ac_adc_drc_let: crate::Reg<ac_adc_drc_let::AC_ADC_DRC_LET_SPEC>,
    #[doc = "0x274 - ADC DRC Expander Slope High Setting Register"]
    pub ac_adc_drc_hke: crate::Reg<ac_adc_drc_hke::AC_ADC_DRC_HKE_SPEC>,
    #[doc = "0x278 - ADC DRC Expander Slope Low Setting Register"]
    pub ac_adc_drc_lke: crate::Reg<ac_adc_drc_lke::AC_ADC_DRC_LKE_SPEC>,
    #[doc = "0x27c - ADC DRC Expander High Output at Expander Threshold"]
    pub ac_adc_drc_hope: crate::Reg<ac_adc_drc_hope::AC_ADC_DRC_HOPE_SPEC>,
    #[doc = "0x280 - ADC DRC Expander Low Output at Expander Threshold"]
    pub ac_adc_drc_lope: crate::Reg<ac_adc_drc_lope::AC_ADC_DRC_LOPE_SPEC>,
    #[doc = "0x284 - ADC DRC Linear Slope High Setting Register"]
    pub ac_adc_drc_hkn: crate::Reg<ac_adc_drc_hkn::AC_ADC_DRC_HKN_SPEC>,
    #[doc = "0x288 - ADC DRC Linear Slope Low Setting Register"]
    pub ac_adc_drc_lkn: crate::Reg<ac_adc_drc_lkn::AC_ADC_DRC_LKN_SPEC>,
    #[doc = "0x28c - ADC DRC Smooth filter Gain High Attack Time Coef Register"]
    pub ac_adc_drc_sfhat: crate::Reg<ac_adc_drc_sfhat::AC_ADC_DRC_SFHAT_SPEC>,
    #[doc = "0x290 - ADC DRC Smooth filter Gain Low Attack Time Coef Register"]
    pub ac_adc_drc_sflat: crate::Reg<ac_adc_drc_sflat::AC_ADC_DRC_SFLAT_SPEC>,
    #[doc = "0x294 - ADC DRC Smooth filter Gain High Release Time Coef Register"]
    pub ac_adc_drc_sfhrt: crate::Reg<ac_adc_drc_sfhrt::AC_ADC_DRC_SFHRT_SPEC>,
    #[doc = "0x298 - ADC DRC Smooth filter Gain Low Release Time Coef Register"]
    pub ac_adc_drc_sflrt: crate::Reg<ac_adc_drc_sflrt::AC_ADC_DRC_SFLRT_SPEC>,
    #[doc = "0x29c - ADC DRC MAX Gain High Setting Register"]
    pub ac_adc_drc_mxghs: crate::Reg<ac_adc_drc_mxghs::AC_ADC_DRC_MXGHS_SPEC>,
    #[doc = "0x2a0 - ADC DRC MAX Gain Low Setting Register"]
    pub ac_adc_drc_mxgls: crate::Reg<ac_adc_drc_mxgls::AC_ADC_DRC_MXGLS_SPEC>,
    #[doc = "0x2a4 - ADC DRC MIN Gain High Setting Register"]
    pub ac_adc_drc_mnghs: crate::Reg<ac_adc_drc_mnghs::AC_ADC_DRC_MNGHS_SPEC>,
    #[doc = "0x2a8 - ADC DRC MIN Gain Low Setting Register"]
    pub ac_adc_drc_mngls: crate::Reg<ac_adc_drc_mngls::AC_ADC_DRC_MNGLS_SPEC>,
    #[doc = "0x2ac - ADC DRC Expander Smooth Time High Coef Register"]
    pub ac_adc_drc_epshc: crate::Reg<ac_adc_drc_epshc::AC_ADC_DRC_EPSHC_SPEC>,
    #[doc = "0x2b0 - ADC DRC Expander Smooth Time Low Coef Register"]
    pub ac_adc_drc_epslc: crate::Reg<ac_adc_drc_epslc::AC_ADC_DRC_EPSLC_SPEC>,
    _reserved109: [u8; 0x04],
    #[doc = "0x2b8 - ADC DRC HPF Gain High Coef Register"]
    pub ac_adc_drc_hpfhgain: crate::Reg<ac_adc_drc_hpfhgain::AC_ADC_DRC_HPFHGAIN_SPEC>,
    #[doc = "0x2bc - ADC DRC HPF Gain Low Coef Register"]
    pub ac_adc_drc_hpflgain: crate::Reg<ac_adc_drc_hpflgain::AC_ADC_DRC_HPFLGAIN_SPEC>,
    _reserved111: [u8; 0x40],
    #[doc = "0x300 - ADC1 Analog Control Register"]
    pub adc1: crate::Reg<adc1::ADC1_SPEC>,
    #[doc = "0x304 - ADC2 Analog Control Register"]
    pub adc2: crate::Reg<adc2::ADC2_SPEC>,
    #[doc = "0x308 - ADC3 Analog Control Register"]
    pub adc3: crate::Reg<adc3::ADC3_SPEC>,
    _reserved114: [u8; 0x04],
    #[doc = "0x310 - DAC Analog Control Register"]
    pub dac: crate::Reg<dac::DAC_SPEC>,
    _reserved115: [u8; 0x04],
    #[doc = "0x318 - MICBIAS Analog Control Register"]
    pub micbias: crate::Reg<micbias::MICBIAS_SPEC>,
    #[doc = "0x31c - BIAS Analog Control Register"]
    pub ramp: crate::Reg<ramp::RAMP_SPEC>,
    #[doc = "0x320 - BIAS Analog Control Register"]
    pub bias: crate::Reg<bias::BIAS_SPEC>,
    _reserved118: [u8; 0x0c],
    #[doc = "0x330 - ADC5 Analog Control Register"]
    pub adc5: crate::Reg<adc5::ADC5_SPEC>,
}
#[doc = "ac_dac_dpc register accessor: an alias for `Reg<AC_DAC_DPC_SPEC>`"]
pub type AC_DAC_DPC = crate::Reg<ac_dac_dpc::AC_DAC_DPC_SPEC>;
#[doc = "DAC Digital Part Control Register"]
pub mod ac_dac_dpc;
#[doc = "dac_vol_ctrl register accessor: an alias for `Reg<DAC_VOL_CTRL_SPEC>`"]
pub type DAC_VOL_CTRL = crate::Reg<dac_vol_ctrl::DAC_VOL_CTRL_SPEC>;
#[doc = "DAC Volume Control Register"]
pub mod dac_vol_ctrl;
#[doc = "ac_dac_fifoc register accessor: an alias for `Reg<AC_DAC_FIFOC_SPEC>`"]
pub type AC_DAC_FIFOC = crate::Reg<ac_dac_fifoc::AC_DAC_FIFOC_SPEC>;
#[doc = "DAC FIFO Control Register"]
pub mod ac_dac_fifoc;
#[doc = "ac_dac_fifos register accessor: an alias for `Reg<AC_DAC_FIFOS_SPEC>`"]
pub type AC_DAC_FIFOS = crate::Reg<ac_dac_fifos::AC_DAC_FIFOS_SPEC>;
#[doc = "DAC FIFO Status Register"]
pub mod ac_dac_fifos;
#[doc = "ac_dac_txdata register accessor: an alias for `Reg<AC_DAC_TXDATA_SPEC>`"]
pub type AC_DAC_TXDATA = crate::Reg<ac_dac_txdata::AC_DAC_TXDATA_SPEC>;
#[doc = "DAC TX DATA Register"]
pub mod ac_dac_txdata;
#[doc = "ac_dac_cnt register accessor: an alias for `Reg<AC_DAC_CNT_SPEC>`"]
pub type AC_DAC_CNT = crate::Reg<ac_dac_cnt::AC_DAC_CNT_SPEC>;
#[doc = "DAC TX FIFO Counter Register"]
pub mod ac_dac_cnt;
#[doc = "ac_dac_dg register accessor: an alias for `Reg<AC_DAC_DG_SPEC>`"]
pub type AC_DAC_DG = crate::Reg<ac_dac_dg::AC_DAC_DG_SPEC>;
#[doc = "DAC Debug Register"]
pub mod ac_dac_dg;
#[doc = "ac_adc_fifoc register accessor: an alias for `Reg<AC_ADC_FIFOC_SPEC>`"]
pub type AC_ADC_FIFOC = crate::Reg<ac_adc_fifoc::AC_ADC_FIFOC_SPEC>;
#[doc = "ADC FIFO Control Register"]
pub mod ac_adc_fifoc;
#[doc = "adc_vol_ctrl1 register accessor: an alias for `Reg<ADC_VOL_CTRL1_SPEC>`"]
pub type ADC_VOL_CTRL1 = crate::Reg<adc_vol_ctrl1::ADC_VOL_CTRL1_SPEC>;
#[doc = "ADC Volume Control1 Register"]
pub mod adc_vol_ctrl1;
#[doc = "ac_adc_fifos register accessor: an alias for `Reg<AC_ADC_FIFOS_SPEC>`"]
pub type AC_ADC_FIFOS = crate::Reg<ac_adc_fifos::AC_ADC_FIFOS_SPEC>;
#[doc = "ADC FIFO Status Register"]
pub mod ac_adc_fifos;
#[doc = "ac_adc_rxdata register accessor: an alias for `Reg<AC_ADC_RXDATA_SPEC>`"]
pub type AC_ADC_RXDATA = crate::Reg<ac_adc_rxdata::AC_ADC_RXDATA_SPEC>;
#[doc = "ADC RX Data Register"]
pub mod ac_adc_rxdata;
#[doc = "ac_adc_cnt register accessor: an alias for `Reg<AC_ADC_CNT_SPEC>`"]
pub type AC_ADC_CNT = crate::Reg<ac_adc_cnt::AC_ADC_CNT_SPEC>;
#[doc = "ADC RX Counter Register"]
pub mod ac_adc_cnt;
#[doc = "ac_adc_dg register accessor: an alias for `Reg<AC_ADC_DG_SPEC>`"]
pub type AC_ADC_DG = crate::Reg<ac_adc_dg::AC_ADC_DG_SPEC>;
#[doc = "ADC Debug Register"]
pub mod ac_adc_dg;
#[doc = "adc_dig_ctrl register accessor: an alias for `Reg<ADC_DIG_CTRL_SPEC>`"]
pub type ADC_DIG_CTRL = crate::Reg<adc_dig_ctrl::ADC_DIG_CTRL_SPEC>;
#[doc = "ADC Digtial Control Register"]
pub mod adc_dig_ctrl;
#[doc = "vra1speedup_down_ctrl register accessor: an alias for `Reg<VRA1SPEEDUP_DOWN_CTRL_SPEC>`"]
pub type VRA1SPEEDUP_DOWN_CTRL = crate::Reg<vra1speedup_down_ctrl::VRA1SPEEDUP_DOWN_CTRL_SPEC>;
#[doc = "VRA1 Speedup Down Control Register"]
pub mod vra1speedup_down_ctrl;
#[doc = "ac_dac_dap_ctrl register accessor: an alias for `Reg<AC_DAC_DAP_CTRL_SPEC>`"]
pub type AC_DAC_DAP_CTRL = crate::Reg<ac_dac_dap_ctrl::AC_DAC_DAP_CTRL_SPEC>;
#[doc = "DAC DAP Control Register"]
pub mod ac_dac_dap_ctrl;
#[doc = "ac_adc_dap_ctr register accessor: an alias for `Reg<AC_ADC_DAP_CTR_SPEC>`"]
pub type AC_ADC_DAP_CTR = crate::Reg<ac_adc_dap_ctr::AC_ADC_DAP_CTR_SPEC>;
#[doc = "ADC DAP Control Register"]
pub mod ac_adc_dap_ctr;
#[doc = "ac_dac_drc_hhpfc register accessor: an alias for `Reg<AC_DAC_DRC_HHPFC_SPEC>`"]
pub type AC_DAC_DRC_HHPFC = crate::Reg<ac_dac_drc_hhpfc::AC_DAC_DRC_HHPFC_SPEC>;
#[doc = "DAC DRC High HPF Coef Register"]
pub mod ac_dac_drc_hhpfc;
#[doc = "ac_dac_drc_lhpfc register accessor: an alias for `Reg<AC_DAC_DRC_LHPFC_SPEC>`"]
pub type AC_DAC_DRC_LHPFC = crate::Reg<ac_dac_drc_lhpfc::AC_DAC_DRC_LHPFC_SPEC>;
#[doc = "DAC DRC Low HPF Coef Register"]
pub mod ac_dac_drc_lhpfc;
#[doc = "ac_dac_drc_ctrl register accessor: an alias for `Reg<AC_DAC_DRC_CTRL_SPEC>`"]
pub type AC_DAC_DRC_CTRL = crate::Reg<ac_dac_drc_ctrl::AC_DAC_DRC_CTRL_SPEC>;
#[doc = "DAC DRC Control Register"]
pub mod ac_dac_drc_ctrl;
#[doc = "ac_dac_drc_lpfhat register accessor: an alias for `Reg<AC_DAC_DRC_LPFHAT_SPEC>`"]
pub type AC_DAC_DRC_LPFHAT = crate::Reg<ac_dac_drc_lpfhat::AC_DAC_DRC_LPFHAT_SPEC>;
#[doc = "DAC DRC Left Peak Filter High Attack Time Coef Register"]
pub mod ac_dac_drc_lpfhat;
#[doc = "ac_dac_drc_lpflat register accessor: an alias for `Reg<AC_DAC_DRC_LPFLAT_SPEC>`"]
pub type AC_DAC_DRC_LPFLAT = crate::Reg<ac_dac_drc_lpflat::AC_DAC_DRC_LPFLAT_SPEC>;
#[doc = "DAC DRC Left Peak Filter Low Attack Time Coef Register"]
pub mod ac_dac_drc_lpflat;
#[doc = "ac_dac_drc_rpfhat register accessor: an alias for `Reg<AC_DAC_DRC_RPFHAT_SPEC>`"]
pub type AC_DAC_DRC_RPFHAT = crate::Reg<ac_dac_drc_rpfhat::AC_DAC_DRC_RPFHAT_SPEC>;
#[doc = "DAC DRC Right Peak Filter High Attack Time Coef Register"]
pub mod ac_dac_drc_rpfhat;
#[doc = "ac_dac_drc_rpflat register accessor: an alias for `Reg<AC_DAC_DRC_RPFLAT_SPEC>`"]
pub type AC_DAC_DRC_RPFLAT = crate::Reg<ac_dac_drc_rpflat::AC_DAC_DRC_RPFLAT_SPEC>;
#[doc = "DAC DRC Peak Filter Low Attack Time Coef Register"]
pub mod ac_dac_drc_rpflat;
#[doc = "ac_dac_drc_lpfhrt register accessor: an alias for `Reg<AC_DAC_DRC_LPFHRT_SPEC>`"]
pub type AC_DAC_DRC_LPFHRT = crate::Reg<ac_dac_drc_lpfhrt::AC_DAC_DRC_LPFHRT_SPEC>;
#[doc = "DAC DRC Left Peak Filter High Release Time Coef Register"]
pub mod ac_dac_drc_lpfhrt;
#[doc = "ac_dac_drc_lpflrt register accessor: an alias for `Reg<AC_DAC_DRC_LPFLRT_SPEC>`"]
pub type AC_DAC_DRC_LPFLRT = crate::Reg<ac_dac_drc_lpflrt::AC_DAC_DRC_LPFLRT_SPEC>;
#[doc = "DAC DRC Left Peak Filter Low Release Time Coef Register"]
pub mod ac_dac_drc_lpflrt;
#[doc = "ac_dac_drc_rpfhrt register accessor: an alias for `Reg<AC_DAC_DRC_RPFHRT_SPEC>`"]
pub type AC_DAC_DRC_RPFHRT = crate::Reg<ac_dac_drc_rpfhrt::AC_DAC_DRC_RPFHRT_SPEC>;
#[doc = "DAC DRC Right Peak filter High Release Time Coef Register"]
pub mod ac_dac_drc_rpfhrt;
#[doc = "ac_dac_drc_rpflrt register accessor: an alias for `Reg<AC_DAC_DRC_RPFLRT_SPEC>`"]
pub type AC_DAC_DRC_RPFLRT = crate::Reg<ac_dac_drc_rpflrt::AC_DAC_DRC_RPFLRT_SPEC>;
#[doc = "DAC DRC Right Peak filter Low Release Time Coef Register"]
pub mod ac_dac_drc_rpflrt;
#[doc = "ac_dac_drc_lrmshat register accessor: an alias for `Reg<AC_DAC_DRC_LRMSHAT_SPEC>`"]
pub type AC_DAC_DRC_LRMSHAT = crate::Reg<ac_dac_drc_lrmshat::AC_DAC_DRC_LRMSHAT_SPEC>;
#[doc = "DAC DRC Left RMS Filter High Coef Register"]
pub mod ac_dac_drc_lrmshat;
#[doc = "ac_dac_drc_lrmslat register accessor: an alias for `Reg<AC_DAC_DRC_LRMSLAT_SPEC>`"]
pub type AC_DAC_DRC_LRMSLAT = crate::Reg<ac_dac_drc_lrmslat::AC_DAC_DRC_LRMSLAT_SPEC>;
#[doc = "DAC DRC Left RMS Filter Low Coef Register"]
pub mod ac_dac_drc_lrmslat;
#[doc = "ac_dac_drc_rrmshat register accessor: an alias for `Reg<AC_DAC_DRC_RRMSHAT_SPEC>`"]
pub type AC_DAC_DRC_RRMSHAT = crate::Reg<ac_dac_drc_rrmshat::AC_DAC_DRC_RRMSHAT_SPEC>;
#[doc = "DAC DRC Right RMS Filter High Coef Register"]
pub mod ac_dac_drc_rrmshat;
#[doc = "ac_dac_drc_rrmslat register accessor: an alias for `Reg<AC_DAC_DRC_RRMSLAT_SPEC>`"]
pub type AC_DAC_DRC_RRMSLAT = crate::Reg<ac_dac_drc_rrmslat::AC_DAC_DRC_RRMSLAT_SPEC>;
#[doc = "DAC DRC Right RMS Filter Low Coef Register"]
pub mod ac_dac_drc_rrmslat;
#[doc = "ac_dac_drc_hct register accessor: an alias for `Reg<AC_DAC_DRC_HCT_SPEC>`"]
pub type AC_DAC_DRC_HCT = crate::Reg<ac_dac_drc_hct::AC_DAC_DRC_HCT_SPEC>;
#[doc = "DAC DRC Compressor Threshold High Setting Register"]
pub mod ac_dac_drc_hct;
#[doc = "ac_dac_drc_lct register accessor: an alias for `Reg<AC_DAC_DRC_LCT_SPEC>`"]
pub type AC_DAC_DRC_LCT = crate::Reg<ac_dac_drc_lct::AC_DAC_DRC_LCT_SPEC>;
#[doc = "DAC DRC Compressor Slope High Setting Register"]
pub mod ac_dac_drc_lct;
#[doc = "ac_dac_drc_hkc register accessor: an alias for `Reg<AC_DAC_DRC_HKC_SPEC>`"]
pub type AC_DAC_DRC_HKC = crate::Reg<ac_dac_drc_hkc::AC_DAC_DRC_HKC_SPEC>;
#[doc = "DAC DRC Compressor Slope High Setting Register"]
pub mod ac_dac_drc_hkc;
#[doc = "ac_dac_drc_lkc register accessor: an alias for `Reg<AC_DAC_DRC_LKC_SPEC>`"]
pub type AC_DAC_DRC_LKC = crate::Reg<ac_dac_drc_lkc::AC_DAC_DRC_LKC_SPEC>;
#[doc = "DAC DRC Compressor Slope Low Setting Register"]
pub mod ac_dac_drc_lkc;
#[doc = "ac_dac_drc_hopc register accessor: an alias for `Reg<AC_DAC_DRC_HOPC_SPEC>`"]
pub type AC_DAC_DRC_HOPC = crate::Reg<ac_dac_drc_hopc::AC_DAC_DRC_HOPC_SPEC>;
#[doc = "DAC DRC Compressor High Output at Compressor Threshold Register"]
pub mod ac_dac_drc_hopc;
#[doc = "ac_dac_drc_lopc register accessor: an alias for `Reg<AC_DAC_DRC_LOPC_SPEC>`"]
pub type AC_DAC_DRC_LOPC = crate::Reg<ac_dac_drc_lopc::AC_DAC_DRC_LOPC_SPEC>;
#[doc = "DAC DRC Compressor Low Output at Compressor Threshold Register"]
pub mod ac_dac_drc_lopc;
#[doc = "ac_dac_drc_hlt register accessor: an alias for `Reg<AC_DAC_DRC_HLT_SPEC>`"]
pub type AC_DAC_DRC_HLT = crate::Reg<ac_dac_drc_hlt::AC_DAC_DRC_HLT_SPEC>;
#[doc = "DAC DRC Limiter Threshold High Setting Register"]
pub mod ac_dac_drc_hlt;
#[doc = "ac_dac_drc_llt register accessor: an alias for `Reg<AC_DAC_DRC_LLT_SPEC>`"]
pub type AC_DAC_DRC_LLT = crate::Reg<ac_dac_drc_llt::AC_DAC_DRC_LLT_SPEC>;
#[doc = "DAC DRC Limiter Threshold Low Setting Register"]
pub mod ac_dac_drc_llt;
#[doc = "ac_dac_drc_h_kl register accessor: an alias for `Reg<AC_DAC_DRC_H_KL_SPEC>`"]
pub type AC_DAC_DRC_H_KL = crate::Reg<ac_dac_drc_h_kl::AC_DAC_DRC_H_KL_SPEC>;
#[doc = "DAC DRC Limiter Slope High Setting Register"]
pub mod ac_dac_drc_h_kl;
#[doc = "ac_dac_drc_l_kl register accessor: an alias for `Reg<AC_DAC_DRC_L_KL_SPEC>`"]
pub type AC_DAC_DRC_L_KL = crate::Reg<ac_dac_drc_l_kl::AC_DAC_DRC_L_KL_SPEC>;
#[doc = "DAC DRC Limiter Slope Low Setting Register"]
pub mod ac_dac_drc_l_kl;
#[doc = "ac_dac_drc_hopl register accessor: an alias for `Reg<AC_DAC_DRC_HOPL_SPEC>`"]
pub type AC_DAC_DRC_HOPL = crate::Reg<ac_dac_drc_hopl::AC_DAC_DRC_HOPL_SPEC>;
#[doc = "DAC DRC Limiter High Output at Limiter Threshold"]
pub mod ac_dac_drc_hopl;
#[doc = "ac_dac_drc_lopl register accessor: an alias for `Reg<AC_DAC_DRC_LOPL_SPEC>`"]
pub type AC_DAC_DRC_LOPL = crate::Reg<ac_dac_drc_lopl::AC_DAC_DRC_LOPL_SPEC>;
#[doc = "DAC DRC Limiter Low Output at Limiter Threshold"]
pub mod ac_dac_drc_lopl;
#[doc = "ac_dac_drc_het register accessor: an alias for `Reg<AC_DAC_DRC_HET_SPEC>`"]
pub type AC_DAC_DRC_HET = crate::Reg<ac_dac_drc_het::AC_DAC_DRC_HET_SPEC>;
#[doc = "DAC DRC Expander Threshold High Setting Register"]
pub mod ac_dac_drc_het;
#[doc = "ac_dac_drc_let register accessor: an alias for `Reg<AC_DAC_DRC_LET_SPEC>`"]
pub type AC_DAC_DRC_LET = crate::Reg<ac_dac_drc_let::AC_DAC_DRC_LET_SPEC>;
#[doc = "DAC DRC Expander Threshold Low Setting Register"]
pub mod ac_dac_drc_let;
#[doc = "ac_dac_drc_hke register accessor: an alias for `Reg<AC_DAC_DRC_HKE_SPEC>`"]
pub type AC_DAC_DRC_HKE = crate::Reg<ac_dac_drc_hke::AC_DAC_DRC_HKE_SPEC>;
#[doc = "DAC DRC Expander Slope High Setting Register"]
pub mod ac_dac_drc_hke;
#[doc = "ac_dac_drc_lke register accessor: an alias for `Reg<AC_DAC_DRC_LKE_SPEC>`"]
pub type AC_DAC_DRC_LKE = crate::Reg<ac_dac_drc_lke::AC_DAC_DRC_LKE_SPEC>;
#[doc = "DAC DRC Expander Slope Low Setting Register"]
pub mod ac_dac_drc_lke;
#[doc = "ac_dac_drc_hope register accessor: an alias for `Reg<AC_DAC_DRC_HOPE_SPEC>`"]
pub type AC_DAC_DRC_HOPE = crate::Reg<ac_dac_drc_hope::AC_DAC_DRC_HOPE_SPEC>;
#[doc = "DAC DRC Expander High Output at Expander Threshold"]
pub mod ac_dac_drc_hope;
#[doc = "ac_dac_drc_lope register accessor: an alias for `Reg<AC_DAC_DRC_LOPE_SPEC>`"]
pub type AC_DAC_DRC_LOPE = crate::Reg<ac_dac_drc_lope::AC_DAC_DRC_LOPE_SPEC>;
#[doc = "DAC DRC Expander Low Output at Expander Threshold"]
pub mod ac_dac_drc_lope;
#[doc = "ac_dac_drc_hkn register accessor: an alias for `Reg<AC_DAC_DRC_HKN_SPEC>`"]
pub type AC_DAC_DRC_HKN = crate::Reg<ac_dac_drc_hkn::AC_DAC_DRC_HKN_SPEC>;
#[doc = "DAC DRC Linear Slope High Setting Register"]
pub mod ac_dac_drc_hkn;
#[doc = "ac_dac_drc_lkn register accessor: an alias for `Reg<AC_DAC_DRC_LKN_SPEC>`"]
pub type AC_DAC_DRC_LKN = crate::Reg<ac_dac_drc_lkn::AC_DAC_DRC_LKN_SPEC>;
#[doc = "DAC DRC Linear Slope Low Setting Register"]
pub mod ac_dac_drc_lkn;
#[doc = "ac_dac_drc_sfhat register accessor: an alias for `Reg<AC_DAC_DRC_SFHAT_SPEC>`"]
pub type AC_DAC_DRC_SFHAT = crate::Reg<ac_dac_drc_sfhat::AC_DAC_DRC_SFHAT_SPEC>;
#[doc = "DAC DRC Smooth filter Gain High Attack Time Coef Register"]
pub mod ac_dac_drc_sfhat;
#[doc = "ac_dac_drc_sflat register accessor: an alias for `Reg<AC_DAC_DRC_SFLAT_SPEC>`"]
pub type AC_DAC_DRC_SFLAT = crate::Reg<ac_dac_drc_sflat::AC_DAC_DRC_SFLAT_SPEC>;
#[doc = "DAC DRC Smooth filter Gain Low Attack Time Coef Register"]
pub mod ac_dac_drc_sflat;
#[doc = "ac_dac_drc_sfhrt register accessor: an alias for `Reg<AC_DAC_DRC_SFHRT_SPEC>`"]
pub type AC_DAC_DRC_SFHRT = crate::Reg<ac_dac_drc_sfhrt::AC_DAC_DRC_SFHRT_SPEC>;
#[doc = "DAC DRC Smooth filter Gain High Release Time Coef Register"]
pub mod ac_dac_drc_sfhrt;
#[doc = "ac_dac_drc_sflrt register accessor: an alias for `Reg<AC_DAC_DRC_SFLRT_SPEC>`"]
pub type AC_DAC_DRC_SFLRT = crate::Reg<ac_dac_drc_sflrt::AC_DAC_DRC_SFLRT_SPEC>;
#[doc = "DAC DRC Smooth filter Gain Low Release Time Coef Register"]
pub mod ac_dac_drc_sflrt;
#[doc = "ac_dac_drc_mxghs register accessor: an alias for `Reg<AC_DAC_DRC_MXGHS_SPEC>`"]
pub type AC_DAC_DRC_MXGHS = crate::Reg<ac_dac_drc_mxghs::AC_DAC_DRC_MXGHS_SPEC>;
#[doc = "DAC DRC MAX Gain High Setting Register"]
pub mod ac_dac_drc_mxghs;
#[doc = "ac_dac_drc_mxgls register accessor: an alias for `Reg<AC_DAC_DRC_MXGLS_SPEC>`"]
pub type AC_DAC_DRC_MXGLS = crate::Reg<ac_dac_drc_mxgls::AC_DAC_DRC_MXGLS_SPEC>;
#[doc = "DAC DRC MAX Gain Low Setting Register"]
pub mod ac_dac_drc_mxgls;
#[doc = "ac_dac_drc_mnghs register accessor: an alias for `Reg<AC_DAC_DRC_MNGHS_SPEC>`"]
pub type AC_DAC_DRC_MNGHS = crate::Reg<ac_dac_drc_mnghs::AC_DAC_DRC_MNGHS_SPEC>;
#[doc = "DAC DRC MIN Gain High Setting Register"]
pub mod ac_dac_drc_mnghs;
#[doc = "ac_dac_drc_mngls register accessor: an alias for `Reg<AC_DAC_DRC_MNGLS_SPEC>`"]
pub type AC_DAC_DRC_MNGLS = crate::Reg<ac_dac_drc_mngls::AC_DAC_DRC_MNGLS_SPEC>;
#[doc = "DAC DRC MIN Gain Low Setting Register"]
pub mod ac_dac_drc_mngls;
#[doc = "ac_dac_drc_epshc register accessor: an alias for `Reg<AC_DAC_DRC_EPSHC_SPEC>`"]
pub type AC_DAC_DRC_EPSHC = crate::Reg<ac_dac_drc_epshc::AC_DAC_DRC_EPSHC_SPEC>;
#[doc = "DAC DRC Expander Smooth Time High Coef Register"]
pub mod ac_dac_drc_epshc;
#[doc = "ac_dac_drc_epslc register accessor: an alias for `Reg<AC_DAC_DRC_EPSLC_SPEC>`"]
pub type AC_DAC_DRC_EPSLC = crate::Reg<ac_dac_drc_epslc::AC_DAC_DRC_EPSLC_SPEC>;
#[doc = "DAC DRC Expander Smooth Time Low Coef Register"]
pub mod ac_dac_drc_epslc;
#[doc = "ac_dac_drc_hpfhgain register accessor: an alias for `Reg<AC_DAC_DRC_HPFHGAIN_SPEC>`"]
pub type AC_DAC_DRC_HPFHGAIN = crate::Reg<ac_dac_drc_hpfhgain::AC_DAC_DRC_HPFHGAIN_SPEC>;
#[doc = "DAC DRC HPF Gain High Coef Register"]
pub mod ac_dac_drc_hpfhgain;
#[doc = "ac_dac_drc_hpflgain register accessor: an alias for `Reg<AC_DAC_DRC_HPFLGAIN_SPEC>`"]
pub type AC_DAC_DRC_HPFLGAIN = crate::Reg<ac_dac_drc_hpflgain::AC_DAC_DRC_HPFLGAIN_SPEC>;
#[doc = "DAC DRC HPF Gain Low Coef Register"]
pub mod ac_dac_drc_hpflgain;
#[doc = "ac_adc_drc_hhpfc register accessor: an alias for `Reg<AC_ADC_DRC_HHPFC_SPEC>`"]
pub type AC_ADC_DRC_HHPFC = crate::Reg<ac_adc_drc_hhpfc::AC_ADC_DRC_HHPFC_SPEC>;
#[doc = "ADC DRC High HPF Coef Register"]
pub mod ac_adc_drc_hhpfc;
#[doc = "ac_adc_drc_lhpfc register accessor: an alias for `Reg<AC_ADC_DRC_LHPFC_SPEC>`"]
pub type AC_ADC_DRC_LHPFC = crate::Reg<ac_adc_drc_lhpfc::AC_ADC_DRC_LHPFC_SPEC>;
#[doc = "ADC DRC Low HPF Coef Register"]
pub mod ac_adc_drc_lhpfc;
#[doc = "ac_adc_drc_ctrl register accessor: an alias for `Reg<AC_ADC_DRC_CTRL_SPEC>`"]
pub type AC_ADC_DRC_CTRL = crate::Reg<ac_adc_drc_ctrl::AC_ADC_DRC_CTRL_SPEC>;
#[doc = "ADC DRC Control Register"]
pub mod ac_adc_drc_ctrl;
#[doc = "ac_adc_drc_lpfhat register accessor: an alias for `Reg<AC_ADC_DRC_LPFHAT_SPEC>`"]
pub type AC_ADC_DRC_LPFHAT = crate::Reg<ac_adc_drc_lpfhat::AC_ADC_DRC_LPFHAT_SPEC>;
#[doc = "ADC DRC Left Peak Filter High Attack Time Coef Register"]
pub mod ac_adc_drc_lpfhat;
#[doc = "ac_adc_drc_lpflat register accessor: an alias for `Reg<AC_ADC_DRC_LPFLAT_SPEC>`"]
pub type AC_ADC_DRC_LPFLAT = crate::Reg<ac_adc_drc_lpflat::AC_ADC_DRC_LPFLAT_SPEC>;
#[doc = "ADC DRC Left Peak Filter Low Attack Time Coef Register"]
pub mod ac_adc_drc_lpflat;
#[doc = "ac_adc_drc_rpfhat register accessor: an alias for `Reg<AC_ADC_DRC_RPFHAT_SPEC>`"]
pub type AC_ADC_DRC_RPFHAT = crate::Reg<ac_adc_drc_rpfhat::AC_ADC_DRC_RPFHAT_SPEC>;
#[doc = "ADC DRC Right Peak Filter High Attack Time Coef Register"]
pub mod ac_adc_drc_rpfhat;
#[doc = "ac_adc_drc_rpflat register accessor: an alias for `Reg<AC_ADC_DRC_RPFLAT_SPEC>`"]
pub type AC_ADC_DRC_RPFLAT = crate::Reg<ac_adc_drc_rpflat::AC_ADC_DRC_RPFLAT_SPEC>;
#[doc = "ADC DRC Right Peak Filter Low Attack Time Coef Register"]
pub mod ac_adc_drc_rpflat;
#[doc = "ac_adc_drc_lpfhrt register accessor: an alias for `Reg<AC_ADC_DRC_LPFHRT_SPEC>`"]
pub type AC_ADC_DRC_LPFHRT = crate::Reg<ac_adc_drc_lpfhrt::AC_ADC_DRC_LPFHRT_SPEC>;
#[doc = "ADC DRC Left Peak Filter High Release Time Coef Register"]
pub mod ac_adc_drc_lpfhrt;
#[doc = "ac_adc_drc_lpflrt register accessor: an alias for `Reg<AC_ADC_DRC_LPFLRT_SPEC>`"]
pub type AC_ADC_DRC_LPFLRT = crate::Reg<ac_adc_drc_lpflrt::AC_ADC_DRC_LPFLRT_SPEC>;
#[doc = "ADC DRC Left Peak Filter Low Release Time Coef Register"]
pub mod ac_adc_drc_lpflrt;
#[doc = "ac_adc_drc_rpfhrt register accessor: an alias for `Reg<AC_ADC_DRC_RPFHRT_SPEC>`"]
pub type AC_ADC_DRC_RPFHRT = crate::Reg<ac_adc_drc_rpfhrt::AC_ADC_DRC_RPFHRT_SPEC>;
#[doc = "ADC DRC Right Peak Filter High Release Time Coef Register"]
pub mod ac_adc_drc_rpfhrt;
#[doc = "ac_adc_drc_rpflrt register accessor: an alias for `Reg<AC_ADC_DRC_RPFLRT_SPEC>`"]
pub type AC_ADC_DRC_RPFLRT = crate::Reg<ac_adc_drc_rpflrt::AC_ADC_DRC_RPFLRT_SPEC>;
#[doc = "ADC DRC Right Peak Filter Low Release Time Coef Register"]
pub mod ac_adc_drc_rpflrt;
#[doc = "ac_adc_drc_lrmshat register accessor: an alias for `Reg<AC_ADC_DRC_LRMSHAT_SPEC>`"]
pub type AC_ADC_DRC_LRMSHAT = crate::Reg<ac_adc_drc_lrmshat::AC_ADC_DRC_LRMSHAT_SPEC>;
#[doc = "ADC DRC Left RMS Filter High Coef Register"]
pub mod ac_adc_drc_lrmshat;
#[doc = "ac_adc_drc_lrmslat register accessor: an alias for `Reg<AC_ADC_DRC_LRMSLAT_SPEC>`"]
pub type AC_ADC_DRC_LRMSLAT = crate::Reg<ac_adc_drc_lrmslat::AC_ADC_DRC_LRMSLAT_SPEC>;
#[doc = "ADC DRC Left RMS Filter Low Coef Register"]
pub mod ac_adc_drc_lrmslat;
#[doc = "ac_adc_drc_rrmshat register accessor: an alias for `Reg<AC_ADC_DRC_RRMSHAT_SPEC>`"]
pub type AC_ADC_DRC_RRMSHAT = crate::Reg<ac_adc_drc_rrmshat::AC_ADC_DRC_RRMSHAT_SPEC>;
#[doc = "ADC DRC Right RMS Filter High Coef Register"]
pub mod ac_adc_drc_rrmshat;
#[doc = "ac_adc_drc_rrmslat register accessor: an alias for `Reg<AC_ADC_DRC_RRMSLAT_SPEC>`"]
pub type AC_ADC_DRC_RRMSLAT = crate::Reg<ac_adc_drc_rrmslat::AC_ADC_DRC_RRMSLAT_SPEC>;
#[doc = "ADC DRC Right RMS Filter Low Coef Register"]
pub mod ac_adc_drc_rrmslat;
#[doc = "ac_adc_drc_hct register accessor: an alias for `Reg<AC_ADC_DRC_HCT_SPEC>`"]
pub type AC_ADC_DRC_HCT = crate::Reg<ac_adc_drc_hct::AC_ADC_DRC_HCT_SPEC>;
#[doc = "ADC DRC Compressor Threshold High Setting Register"]
pub mod ac_adc_drc_hct;
#[doc = "ac_adc_drc_lct register accessor: an alias for `Reg<AC_ADC_DRC_LCT_SPEC>`"]
pub type AC_ADC_DRC_LCT = crate::Reg<ac_adc_drc_lct::AC_ADC_DRC_LCT_SPEC>;
#[doc = "ADC DRC Compressor Slope High Setting Register"]
pub mod ac_adc_drc_lct;
#[doc = "ac_adc_drc_hkc register accessor: an alias for `Reg<AC_ADC_DRC_HKC_SPEC>`"]
pub type AC_ADC_DRC_HKC = crate::Reg<ac_adc_drc_hkc::AC_ADC_DRC_HKC_SPEC>;
#[doc = "ADC DRC Compressor Slope High Setting Register"]
pub mod ac_adc_drc_hkc;
#[doc = "ac_adc_drc_lkc register accessor: an alias for `Reg<AC_ADC_DRC_LKC_SPEC>`"]
pub type AC_ADC_DRC_LKC = crate::Reg<ac_adc_drc_lkc::AC_ADC_DRC_LKC_SPEC>;
#[doc = "ADC DRC Compressor Slope Low Setting Register"]
pub mod ac_adc_drc_lkc;
#[doc = "ac_adc_drc_hopc register accessor: an alias for `Reg<AC_ADC_DRC_HOPC_SPEC>`"]
pub type AC_ADC_DRC_HOPC = crate::Reg<ac_adc_drc_hopc::AC_ADC_DRC_HOPC_SPEC>;
#[doc = "ADC DRC Compressor High Output at Compressor Threshold Register"]
pub mod ac_adc_drc_hopc;
#[doc = "ac_adc_drc_lopc register accessor: an alias for `Reg<AC_ADC_DRC_LOPC_SPEC>`"]
pub type AC_ADC_DRC_LOPC = crate::Reg<ac_adc_drc_lopc::AC_ADC_DRC_LOPC_SPEC>;
#[doc = "ADC DRC Compressor Low Output at Compressor Threshold Register"]
pub mod ac_adc_drc_lopc;
#[doc = "ac_adc_drc_hlt register accessor: an alias for `Reg<AC_ADC_DRC_HLT_SPEC>`"]
pub type AC_ADC_DRC_HLT = crate::Reg<ac_adc_drc_hlt::AC_ADC_DRC_HLT_SPEC>;
#[doc = "ADC DRC Limiter Threshold High Setting Register"]
pub mod ac_adc_drc_hlt;
#[doc = "ac_adc_drc_llt register accessor: an alias for `Reg<AC_ADC_DRC_LLT_SPEC>`"]
pub type AC_ADC_DRC_LLT = crate::Reg<ac_adc_drc_llt::AC_ADC_DRC_LLT_SPEC>;
#[doc = "ADC DRC Limiter Threshold Low Setting Register"]
pub mod ac_adc_drc_llt;
#[doc = "ac_adc_drc_h_kl register accessor: an alias for `Reg<AC_ADC_DRC_H_KL_SPEC>`"]
pub type AC_ADC_DRC_H_KL = crate::Reg<ac_adc_drc_h_kl::AC_ADC_DRC_H_KL_SPEC>;
#[doc = "ADC DRC Limiter Slope High Setting Register"]
pub mod ac_adc_drc_h_kl;
#[doc = "ac_adc_drc_l_kl register accessor: an alias for `Reg<AC_ADC_DRC_L_KL_SPEC>`"]
pub type AC_ADC_DRC_L_KL = crate::Reg<ac_adc_drc_l_kl::AC_ADC_DRC_L_KL_SPEC>;
#[doc = "ADC DRC Limiter Slope Low Setting Register"]
pub mod ac_adc_drc_l_kl;
#[doc = "ac_adc_drc_hopl register accessor: an alias for `Reg<AC_ADC_DRC_HOPL_SPEC>`"]
pub type AC_ADC_DRC_HOPL = crate::Reg<ac_adc_drc_hopl::AC_ADC_DRC_HOPL_SPEC>;
#[doc = "ADC DRC Limiter High Output at Limiter Threshold"]
pub mod ac_adc_drc_hopl;
#[doc = "ac_adc_drc_lopl register accessor: an alias for `Reg<AC_ADC_DRC_LOPL_SPEC>`"]
pub type AC_ADC_DRC_LOPL = crate::Reg<ac_adc_drc_lopl::AC_ADC_DRC_LOPL_SPEC>;
#[doc = "ADC DRC Limiter Low Output at Limiter Threshold"]
pub mod ac_adc_drc_lopl;
#[doc = "ac_adc_drc_het register accessor: an alias for `Reg<AC_ADC_DRC_HET_SPEC>`"]
pub type AC_ADC_DRC_HET = crate::Reg<ac_adc_drc_het::AC_ADC_DRC_HET_SPEC>;
#[doc = "ADC DRC Expander Threshold High Setting Register"]
pub mod ac_adc_drc_het;
#[doc = "ac_adc_drc_let register accessor: an alias for `Reg<AC_ADC_DRC_LET_SPEC>`"]
pub type AC_ADC_DRC_LET = crate::Reg<ac_adc_drc_let::AC_ADC_DRC_LET_SPEC>;
#[doc = "ADC DRC Expander Threshold Low Setting Register"]
pub mod ac_adc_drc_let;
#[doc = "ac_adc_drc_hke register accessor: an alias for `Reg<AC_ADC_DRC_HKE_SPEC>`"]
pub type AC_ADC_DRC_HKE = crate::Reg<ac_adc_drc_hke::AC_ADC_DRC_HKE_SPEC>;
#[doc = "ADC DRC Expander Slope High Setting Register"]
pub mod ac_adc_drc_hke;
#[doc = "ac_adc_drc_lke register accessor: an alias for `Reg<AC_ADC_DRC_LKE_SPEC>`"]
pub type AC_ADC_DRC_LKE = crate::Reg<ac_adc_drc_lke::AC_ADC_DRC_LKE_SPEC>;
#[doc = "ADC DRC Expander Slope Low Setting Register"]
pub mod ac_adc_drc_lke;
#[doc = "ac_adc_drc_hope register accessor: an alias for `Reg<AC_ADC_DRC_HOPE_SPEC>`"]
pub type AC_ADC_DRC_HOPE = crate::Reg<ac_adc_drc_hope::AC_ADC_DRC_HOPE_SPEC>;
#[doc = "ADC DRC Expander High Output at Expander Threshold"]
pub mod ac_adc_drc_hope;
#[doc = "ac_adc_drc_lope register accessor: an alias for `Reg<AC_ADC_DRC_LOPE_SPEC>`"]
pub type AC_ADC_DRC_LOPE = crate::Reg<ac_adc_drc_lope::AC_ADC_DRC_LOPE_SPEC>;
#[doc = "ADC DRC Expander Low Output at Expander Threshold"]
pub mod ac_adc_drc_lope;
#[doc = "ac_adc_drc_hkn register accessor: an alias for `Reg<AC_ADC_DRC_HKN_SPEC>`"]
pub type AC_ADC_DRC_HKN = crate::Reg<ac_adc_drc_hkn::AC_ADC_DRC_HKN_SPEC>;
#[doc = "ADC DRC Linear Slope High Setting Register"]
pub mod ac_adc_drc_hkn;
#[doc = "ac_adc_drc_lkn register accessor: an alias for `Reg<AC_ADC_DRC_LKN_SPEC>`"]
pub type AC_ADC_DRC_LKN = crate::Reg<ac_adc_drc_lkn::AC_ADC_DRC_LKN_SPEC>;
#[doc = "ADC DRC Linear Slope Low Setting Register"]
pub mod ac_adc_drc_lkn;
#[doc = "ac_adc_drc_sfhat register accessor: an alias for `Reg<AC_ADC_DRC_SFHAT_SPEC>`"]
pub type AC_ADC_DRC_SFHAT = crate::Reg<ac_adc_drc_sfhat::AC_ADC_DRC_SFHAT_SPEC>;
#[doc = "ADC DRC Smooth filter Gain High Attack Time Coef Register"]
pub mod ac_adc_drc_sfhat;
#[doc = "ac_adc_drc_sflat register accessor: an alias for `Reg<AC_ADC_DRC_SFLAT_SPEC>`"]
pub type AC_ADC_DRC_SFLAT = crate::Reg<ac_adc_drc_sflat::AC_ADC_DRC_SFLAT_SPEC>;
#[doc = "ADC DRC Smooth filter Gain Low Attack Time Coef Register"]
pub mod ac_adc_drc_sflat;
#[doc = "ac_adc_drc_sfhrt register accessor: an alias for `Reg<AC_ADC_DRC_SFHRT_SPEC>`"]
pub type AC_ADC_DRC_SFHRT = crate::Reg<ac_adc_drc_sfhrt::AC_ADC_DRC_SFHRT_SPEC>;
#[doc = "ADC DRC Smooth filter Gain High Release Time Coef Register"]
pub mod ac_adc_drc_sfhrt;
#[doc = "ac_adc_drc_sflrt register accessor: an alias for `Reg<AC_ADC_DRC_SFLRT_SPEC>`"]
pub type AC_ADC_DRC_SFLRT = crate::Reg<ac_adc_drc_sflrt::AC_ADC_DRC_SFLRT_SPEC>;
#[doc = "ADC DRC Smooth filter Gain Low Release Time Coef Register"]
pub mod ac_adc_drc_sflrt;
#[doc = "ac_adc_drc_mxghs register accessor: an alias for `Reg<AC_ADC_DRC_MXGHS_SPEC>`"]
pub type AC_ADC_DRC_MXGHS = crate::Reg<ac_adc_drc_mxghs::AC_ADC_DRC_MXGHS_SPEC>;
#[doc = "ADC DRC MAX Gain High Setting Register"]
pub mod ac_adc_drc_mxghs;
#[doc = "ac_adc_drc_mxgls register accessor: an alias for `Reg<AC_ADC_DRC_MXGLS_SPEC>`"]
pub type AC_ADC_DRC_MXGLS = crate::Reg<ac_adc_drc_mxgls::AC_ADC_DRC_MXGLS_SPEC>;
#[doc = "ADC DRC MAX Gain Low Setting Register"]
pub mod ac_adc_drc_mxgls;
#[doc = "ac_adc_drc_mnghs register accessor: an alias for `Reg<AC_ADC_DRC_MNGHS_SPEC>`"]
pub type AC_ADC_DRC_MNGHS = crate::Reg<ac_adc_drc_mnghs::AC_ADC_DRC_MNGHS_SPEC>;
#[doc = "ADC DRC MIN Gain High Setting Register"]
pub mod ac_adc_drc_mnghs;
#[doc = "ac_adc_drc_mngls register accessor: an alias for `Reg<AC_ADC_DRC_MNGLS_SPEC>`"]
pub type AC_ADC_DRC_MNGLS = crate::Reg<ac_adc_drc_mngls::AC_ADC_DRC_MNGLS_SPEC>;
#[doc = "ADC DRC MIN Gain Low Setting Register"]
pub mod ac_adc_drc_mngls;
#[doc = "ac_adc_drc_epshc register accessor: an alias for `Reg<AC_ADC_DRC_EPSHC_SPEC>`"]
pub type AC_ADC_DRC_EPSHC = crate::Reg<ac_adc_drc_epshc::AC_ADC_DRC_EPSHC_SPEC>;
#[doc = "ADC DRC Expander Smooth Time High Coef Register"]
pub mod ac_adc_drc_epshc;
#[doc = "ac_adc_drc_epslc register accessor: an alias for `Reg<AC_ADC_DRC_EPSLC_SPEC>`"]
pub type AC_ADC_DRC_EPSLC = crate::Reg<ac_adc_drc_epslc::AC_ADC_DRC_EPSLC_SPEC>;
#[doc = "ADC DRC Expander Smooth Time Low Coef Register"]
pub mod ac_adc_drc_epslc;
#[doc = "ac_adc_drc_hpfhgain register accessor: an alias for `Reg<AC_ADC_DRC_HPFHGAIN_SPEC>`"]
pub type AC_ADC_DRC_HPFHGAIN = crate::Reg<ac_adc_drc_hpfhgain::AC_ADC_DRC_HPFHGAIN_SPEC>;
#[doc = "ADC DRC HPF Gain High Coef Register"]
pub mod ac_adc_drc_hpfhgain;
#[doc = "ac_adc_drc_hpflgain register accessor: an alias for `Reg<AC_ADC_DRC_HPFLGAIN_SPEC>`"]
pub type AC_ADC_DRC_HPFLGAIN = crate::Reg<ac_adc_drc_hpflgain::AC_ADC_DRC_HPFLGAIN_SPEC>;
#[doc = "ADC DRC HPF Gain Low Coef Register"]
pub mod ac_adc_drc_hpflgain;
#[doc = "adc1 register accessor: an alias for `Reg<ADC1_SPEC>`"]
pub type ADC1 = crate::Reg<adc1::ADC1_SPEC>;
#[doc = "ADC1 Analog Control Register"]
pub mod adc1;
#[doc = "adc2 register accessor: an alias for `Reg<ADC2_SPEC>`"]
pub type ADC2 = crate::Reg<adc2::ADC2_SPEC>;
#[doc = "ADC2 Analog Control Register"]
pub mod adc2;
#[doc = "adc3 register accessor: an alias for `Reg<ADC3_SPEC>`"]
pub type ADC3 = crate::Reg<adc3::ADC3_SPEC>;
#[doc = "ADC3 Analog Control Register"]
pub mod adc3;
#[doc = "dac register accessor: an alias for `Reg<DAC_SPEC>`"]
pub type DAC = crate::Reg<dac::DAC_SPEC>;
#[doc = "DAC Analog Control Register"]
pub mod dac;
#[doc = "micbias register accessor: an alias for `Reg<MICBIAS_SPEC>`"]
pub type MICBIAS = crate::Reg<micbias::MICBIAS_SPEC>;
#[doc = "MICBIAS Analog Control Register"]
pub mod micbias;
#[doc = "ramp register accessor: an alias for `Reg<RAMP_SPEC>`"]
pub type RAMP = crate::Reg<ramp::RAMP_SPEC>;
#[doc = "BIAS Analog Control Register"]
pub mod ramp;
#[doc = "bias register accessor: an alias for `Reg<BIAS_SPEC>`"]
pub type BIAS = crate::Reg<bias::BIAS_SPEC>;
#[doc = "BIAS Analog Control Register"]
pub mod bias;
#[doc = "adc5 register accessor: an alias for `Reg<ADC5_SPEC>`"]
pub type ADC5 = crate::Reg<adc5::ADC5_SPEC>;
#[doc = "ADC5 Analog Control Register"]
pub mod adc5;
