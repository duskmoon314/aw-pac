#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TVD TOP MAP Register"]
    pub tvd_top_map: crate::Reg<tvd_top_map::TVD_TOP_MAP_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - TVD 3D DMA CONTROL Register1"]
    pub tvd_3d_ctl1: crate::Reg<tvd_3d_ctl1::TVD_3D_CTL1_SPEC>,
    #[doc = "0x0c - TVD 3D DMA CONTROL Register2"]
    pub tvd_3d_ctl2: crate::Reg<tvd_3d_ctl2::TVD_3D_CTL2_SPEC>,
    #[doc = "0x10 - TVD 3D DMA CONTROL Register3"]
    pub tvd_3d_ctl3: crate::Reg<tvd_3d_ctl3::TVD_3D_CTL3_SPEC>,
    #[doc = "0x14 - TVD 3D DMA CONTROL Register4"]
    pub tvd_3d_ctl4: crate::Reg<tvd_3d_ctl4::TVD_3D_CTL4_SPEC>,
    #[doc = "0x18 - TVD 3D DMA CONTROL Register5"]
    pub tvd_3d_ctl5: crate::Reg<tvd_3d_ctl5::TVD_3D_CTL5_SPEC>,
    _reserved6: [u8; 0x08],
    #[doc = "0x24 - TVD TOP CONTROL Register"]
    pub tvd_top_ctl0: crate::Reg<tvd_top_ctl::TVD_TOP_CTL_SPEC>,
    #[doc = "0x28 - TVD ADC CONTROL Register"]
    pub tvd_adc_ctl0: crate::Reg<tvd_adc_ctl::TVD_ADC_CTL_SPEC>,
    #[doc = "0x2c - TVD ADC CONFIGURATION Register"]
    pub tvd_adc_cfg0: crate::Reg<tvd_adc_cfg::TVD_ADC_CFG_SPEC>,
    _reserved9: [u8; 0x14],
    #[doc = "0x44 - TVD TOP CONTROL Register"]
    pub tvd_top_ctl1: crate::Reg<tvd_top_ctl::TVD_TOP_CTL_SPEC>,
    #[doc = "0x48 - TVD ADC CONTROL Register"]
    pub tvd_adc_ctl1: crate::Reg<tvd_adc_ctl::TVD_ADC_CTL_SPEC>,
    #[doc = "0x4c - TVD ADC CONFIGURATION Register"]
    pub tvd_adc_cfg1: crate::Reg<tvd_adc_cfg::TVD_ADC_CFG_SPEC>,
    _reserved12: [u8; 0x14],
    #[doc = "0x64 - TVD TOP CONTROL Register"]
    pub tvd_top_ctl2: crate::Reg<tvd_top_ctl::TVD_TOP_CTL_SPEC>,
    #[doc = "0x68 - TVD ADC CONTROL Register"]
    pub tvd_adc_ctl2: crate::Reg<tvd_adc_ctl::TVD_ADC_CTL_SPEC>,
    #[doc = "0x6c - TVD ADC CONFIGURATION Register"]
    pub tvd_adc_cfg2: crate::Reg<tvd_adc_cfg::TVD_ADC_CFG_SPEC>,
    _reserved15: [u8; 0x14],
    #[doc = "0x84 - TVD TOP CONTROL Register"]
    pub tvd_top_ctl3: crate::Reg<tvd_top_ctl::TVD_TOP_CTL_SPEC>,
    #[doc = "0x88 - TVD ADC CONTROL Register"]
    pub tvd_adc_ctl3: crate::Reg<tvd_adc_ctl::TVD_ADC_CTL_SPEC>,
    #[doc = "0x8c - TVD ADC CONFIGURATION Register"]
    pub tvd_adc_cfg3: crate::Reg<tvd_adc_cfg::TVD_ADC_CFG_SPEC>,
}
#[doc = "TVD_TOP_MAP register accessor: an alias for `Reg<TVD_TOP_MAP_SPEC>`"]
pub type TVD_TOP_MAP = crate::Reg<tvd_top_map::TVD_TOP_MAP_SPEC>;
#[doc = "TVD TOP MAP Register"]
pub mod tvd_top_map;
#[doc = "TVD_3D_CTL1 register accessor: an alias for `Reg<TVD_3D_CTL1_SPEC>`"]
pub type TVD_3D_CTL1 = crate::Reg<tvd_3d_ctl1::TVD_3D_CTL1_SPEC>;
#[doc = "TVD 3D DMA CONTROL Register1"]
pub mod tvd_3d_ctl1;
#[doc = "TVD_3D_CTL2 register accessor: an alias for `Reg<TVD_3D_CTL2_SPEC>`"]
pub type TVD_3D_CTL2 = crate::Reg<tvd_3d_ctl2::TVD_3D_CTL2_SPEC>;
#[doc = "TVD 3D DMA CONTROL Register2"]
pub mod tvd_3d_ctl2;
#[doc = "TVD_3D_CTL3 register accessor: an alias for `Reg<TVD_3D_CTL3_SPEC>`"]
pub type TVD_3D_CTL3 = crate::Reg<tvd_3d_ctl3::TVD_3D_CTL3_SPEC>;
#[doc = "TVD 3D DMA CONTROL Register3"]
pub mod tvd_3d_ctl3;
#[doc = "TVD_3D_CTL4 register accessor: an alias for `Reg<TVD_3D_CTL4_SPEC>`"]
pub type TVD_3D_CTL4 = crate::Reg<tvd_3d_ctl4::TVD_3D_CTL4_SPEC>;
#[doc = "TVD 3D DMA CONTROL Register4"]
pub mod tvd_3d_ctl4;
#[doc = "TVD_3D_CTL5 register accessor: an alias for `Reg<TVD_3D_CTL5_SPEC>`"]
pub type TVD_3D_CTL5 = crate::Reg<tvd_3d_ctl5::TVD_3D_CTL5_SPEC>;
#[doc = "TVD 3D DMA CONTROL Register5"]
pub mod tvd_3d_ctl5;
#[doc = "TVD_TOP_CTL register accessor: an alias for `Reg<TVD_TOP_CTL_SPEC>`"]
pub type TVD_TOP_CTL = crate::Reg<tvd_top_ctl::TVD_TOP_CTL_SPEC>;
#[doc = "TVD TOP CONTROL Register"]
pub mod tvd_top_ctl;
#[doc = "TVD_ADC_CTL register accessor: an alias for `Reg<TVD_ADC_CTL_SPEC>`"]
pub type TVD_ADC_CTL = crate::Reg<tvd_adc_ctl::TVD_ADC_CTL_SPEC>;
#[doc = "TVD ADC CONTROL Register"]
pub mod tvd_adc_ctl;
#[doc = "TVD_ADC_CFG register accessor: an alias for `Reg<TVD_ADC_CFG_SPEC>`"]
pub type TVD_ADC_CFG = crate::Reg<tvd_adc_cfg::TVD_ADC_CFG_SPEC>;
#[doc = "TVD ADC CONFIGURATION Register"]
pub mod tvd_adc_cfg;
