#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TV Encoder Clock Gating Register"]
    pub tve_000_reg: crate::Reg<tve_000_reg::TVE_000_REG_SPEC>,
    #[doc = "0x04 - TV Encoder Configuration Register"]
    pub tve_004_reg: crate::Reg<tve_004_reg::TVE_004_REG_SPEC>,
    #[doc = "0x08 - TV Encoder DAC Register1"]
    pub tve_008_reg: crate::Reg<tve_008_reg::TVE_008_REG_SPEC>,
    #[doc = "0x0c - TV Encoder Notch and DAC Delay Register"]
    pub tve_00c_reg: crate::Reg<tve_00c_reg::TVE_00C_REG_SPEC>,
    #[doc = "0x10 - TV Encoder Chroma Frequency Register"]
    pub tve_010_reg: crate::Reg<tve_010_reg::TVE_010_REG_SPEC>,
    #[doc = "0x14 - TV Encoder Front/Back Porch Register"]
    pub tve_014_reg: crate::Reg<tve_014_reg::TVE_014_REG_SPEC>,
    #[doc = "0x18 - TV Encoder HD Mode VSYNC Register"]
    pub tve_018_reg: crate::Reg<tve_018_reg::TVE_018_REG_SPEC>,
    #[doc = "0x1c - TV Encoder Line Number Register"]
    pub tve_01c_reg: crate::Reg<tve_01c_reg::TVE_01C_REG_SPEC>,
    #[doc = "0x20 - TV Encoder Level Register"]
    pub tve_020_reg: crate::Reg<tve_020_reg::TVE_020_REG_SPEC>,
    #[doc = "0x24 - TV Encoder DAC Register2"]
    pub tve_024_reg: crate::Reg<tve_024_reg::TVE_024_REG_SPEC>,
    _reserved10: [u8; 0x08],
    #[doc = "0x30 - TV Encoder Auto Detection Enable Register"]
    pub tve_030_reg: crate::Reg<tve_030_reg::TVE_030_REG_SPEC>,
    #[doc = "0x34 - TV Encoder Auto Detection Interrupt Status Register"]
    pub tve_034_reg: crate::Reg<tve_034_reg::TVE_034_REG_SPEC>,
    #[doc = "0x38 - TV Encoder Auto Detection Status Register"]
    pub tve_038_reg: crate::Reg<tve_038_reg::TVE_038_REG_SPEC>,
    #[doc = "0x3c - TV Encoder Auto Detection De-bounce Setting Register"]
    pub tve_03c_reg: crate::Reg<tve_03c_reg::TVE_03C_REG_SPEC>,
    _reserved14: [u8; 0xb8],
    #[doc = "0xf8 - TV Encoder Auto Detect Configuration Register0"]
    pub tve_0f8_reg: crate::Reg<tve_0f8_reg::TVE_0F8_REG_SPEC>,
    #[doc = "0xfc - TV Encoder Auto Detect Configuration Register1"]
    pub tve_0fc_reg: crate::Reg<tve_0fc_reg::TVE_0FC_REG_SPEC>,
    #[doc = "0x100 - TV Encoder Color Burst Phase Reset Configuration Register"]
    pub tve_100_reg: crate::Reg<tve_100_reg::TVE_100_REG_SPEC>,
    #[doc = "0x104 - TV Encoder VSYNC Number Register"]
    pub tve_104_reg: crate::Reg<tve_104_reg::TVE_104_REG_SPEC>,
    #[doc = "0x108 - TV Encoder Notch Filter Frequency Register"]
    pub tve_108_reg: crate::Reg<tve_108_reg::TVE_108_REG_SPEC>,
    #[doc = "0x10c - TV Encoder Cb/Cr Level/Gain Register"]
    pub tve_10c_reg: crate::Reg<tve_10c_reg::TVE_10C_REG_SPEC>,
    #[doc = "0x110 - TV Encoder Tint and Color Burst Phase Register"]
    pub tve_110_reg: crate::Reg<tve_110_reg::TVE_110_REG_SPEC>,
    #[doc = "0x114 - TV Encoder Burst Width Register"]
    pub tve_114_reg: crate::Reg<tve_114_reg::TVE_114_REG_SPEC>,
    #[doc = "0x118 - TV Encoder Cb/Cr Gain Register"]
    pub tve_118_reg: crate::Reg<tve_118_reg::TVE_118_REG_SPEC>,
    #[doc = "0x11c - TV Encoder Sync and VBI Level Register"]
    pub tve_11c_reg: crate::Reg<tve_11c_reg::TVE_11C_REG_SPEC>,
    #[doc = "0x120 - TV Encoder White Level Register"]
    pub tve_120_reg: crate::Reg<tve_120_reg::TVE_120_REG_SPEC>,
    #[doc = "0x124 - TV Encoder Video Active Line Register"]
    pub tve_124_reg: crate::Reg<tve_124_reg::TVE_124_REG_SPEC>,
    #[doc = "0x128 - TV Encoder Video Chroma BW and CompGain Register"]
    pub tve_128_reg: crate::Reg<tve_128_reg::TVE_128_REG_SPEC>,
    #[doc = "0x12c - TV Encoder Register"]
    pub tve_12c_reg: crate::Reg<tve_12c_reg::TVE_12C_REG_SPEC>,
    #[doc = "0x130 - TV Encoder Re-sync Parameters Register"]
    pub tve_130_reg: crate::Reg<tve_130_reg::TVE_130_REG_SPEC>,
    #[doc = "0x134 - TV Encoder Slave Parameter Register"]
    pub tve_134_reg: crate::Reg<tve_134_reg::TVE_134_REG_SPEC>,
    #[doc = "0x138 - TV Encoder Configuration Register0"]
    pub tve_138_reg: crate::Reg<tve_138_reg::TVE_138_REG_SPEC>,
    #[doc = "0x13c - TV Encoder Configuration Register1"]
    pub tve_13c_reg: crate::Reg<tve_13c_reg::TVE_13C_REG_SPEC>,
    _reserved32: [u8; 0x0240],
    #[doc = "0x380 - TV Encoder Low Pass Control Register"]
    pub tve_380_reg: crate::Reg<tve_380_reg::TVE_380_REG_SPEC>,
    #[doc = "0x384 - TV Encoder Low Pass Filter Control Register"]
    pub tve_384_reg: crate::Reg<tve_384_reg::TVE_384_REG_SPEC>,
}
#[doc = "TVE_000_REG register accessor: an alias for `Reg<TVE_000_REG_SPEC>`"]
pub type TVE_000_REG = crate::Reg<tve_000_reg::TVE_000_REG_SPEC>;
#[doc = "TV Encoder Clock Gating Register"]
pub mod tve_000_reg;
#[doc = "TVE_004_REG register accessor: an alias for `Reg<TVE_004_REG_SPEC>`"]
pub type TVE_004_REG = crate::Reg<tve_004_reg::TVE_004_REG_SPEC>;
#[doc = "TV Encoder Configuration Register"]
pub mod tve_004_reg;
#[doc = "TVE_008_REG register accessor: an alias for `Reg<TVE_008_REG_SPEC>`"]
pub type TVE_008_REG = crate::Reg<tve_008_reg::TVE_008_REG_SPEC>;
#[doc = "TV Encoder DAC Register1"]
pub mod tve_008_reg;
#[doc = "TVE_00C_REG register accessor: an alias for `Reg<TVE_00C_REG_SPEC>`"]
pub type TVE_00C_REG = crate::Reg<tve_00c_reg::TVE_00C_REG_SPEC>;
#[doc = "TV Encoder Notch and DAC Delay Register"]
pub mod tve_00c_reg;
#[doc = "TVE_010_REG register accessor: an alias for `Reg<TVE_010_REG_SPEC>`"]
pub type TVE_010_REG = crate::Reg<tve_010_reg::TVE_010_REG_SPEC>;
#[doc = "TV Encoder Chroma Frequency Register"]
pub mod tve_010_reg;
#[doc = "TVE_014_REG register accessor: an alias for `Reg<TVE_014_REG_SPEC>`"]
pub type TVE_014_REG = crate::Reg<tve_014_reg::TVE_014_REG_SPEC>;
#[doc = "TV Encoder Front/Back Porch Register"]
pub mod tve_014_reg;
#[doc = "TVE_018_REG register accessor: an alias for `Reg<TVE_018_REG_SPEC>`"]
pub type TVE_018_REG = crate::Reg<tve_018_reg::TVE_018_REG_SPEC>;
#[doc = "TV Encoder HD Mode VSYNC Register"]
pub mod tve_018_reg;
#[doc = "TVE_01C_REG register accessor: an alias for `Reg<TVE_01C_REG_SPEC>`"]
pub type TVE_01C_REG = crate::Reg<tve_01c_reg::TVE_01C_REG_SPEC>;
#[doc = "TV Encoder Line Number Register"]
pub mod tve_01c_reg;
#[doc = "TVE_020_REG register accessor: an alias for `Reg<TVE_020_REG_SPEC>`"]
pub type TVE_020_REG = crate::Reg<tve_020_reg::TVE_020_REG_SPEC>;
#[doc = "TV Encoder Level Register"]
pub mod tve_020_reg;
#[doc = "TVE_024_REG register accessor: an alias for `Reg<TVE_024_REG_SPEC>`"]
pub type TVE_024_REG = crate::Reg<tve_024_reg::TVE_024_REG_SPEC>;
#[doc = "TV Encoder DAC Register2"]
pub mod tve_024_reg;
#[doc = "TVE_030_REG register accessor: an alias for `Reg<TVE_030_REG_SPEC>`"]
pub type TVE_030_REG = crate::Reg<tve_030_reg::TVE_030_REG_SPEC>;
#[doc = "TV Encoder Auto Detection Enable Register"]
pub mod tve_030_reg;
#[doc = "TVE_034_REG register accessor: an alias for `Reg<TVE_034_REG_SPEC>`"]
pub type TVE_034_REG = crate::Reg<tve_034_reg::TVE_034_REG_SPEC>;
#[doc = "TV Encoder Auto Detection Interrupt Status Register"]
pub mod tve_034_reg;
#[doc = "TVE_038_REG register accessor: an alias for `Reg<TVE_038_REG_SPEC>`"]
pub type TVE_038_REG = crate::Reg<tve_038_reg::TVE_038_REG_SPEC>;
#[doc = "TV Encoder Auto Detection Status Register"]
pub mod tve_038_reg;
#[doc = "TVE_03C_REG register accessor: an alias for `Reg<TVE_03C_REG_SPEC>`"]
pub type TVE_03C_REG = crate::Reg<tve_03c_reg::TVE_03C_REG_SPEC>;
#[doc = "TV Encoder Auto Detection De-bounce Setting Register"]
pub mod tve_03c_reg;
#[doc = "TVE_0F8_REG register accessor: an alias for `Reg<TVE_0F8_REG_SPEC>`"]
pub type TVE_0F8_REG = crate::Reg<tve_0f8_reg::TVE_0F8_REG_SPEC>;
#[doc = "TV Encoder Auto Detect Configuration Register0"]
pub mod tve_0f8_reg;
#[doc = "TVE_0FC_REG register accessor: an alias for `Reg<TVE_0FC_REG_SPEC>`"]
pub type TVE_0FC_REG = crate::Reg<tve_0fc_reg::TVE_0FC_REG_SPEC>;
#[doc = "TV Encoder Auto Detect Configuration Register1"]
pub mod tve_0fc_reg;
#[doc = "TVE_100_REG register accessor: an alias for `Reg<TVE_100_REG_SPEC>`"]
pub type TVE_100_REG = crate::Reg<tve_100_reg::TVE_100_REG_SPEC>;
#[doc = "TV Encoder Color Burst Phase Reset Configuration Register"]
pub mod tve_100_reg;
#[doc = "TVE_104_REG register accessor: an alias for `Reg<TVE_104_REG_SPEC>`"]
pub type TVE_104_REG = crate::Reg<tve_104_reg::TVE_104_REG_SPEC>;
#[doc = "TV Encoder VSYNC Number Register"]
pub mod tve_104_reg;
#[doc = "TVE_108_REG register accessor: an alias for `Reg<TVE_108_REG_SPEC>`"]
pub type TVE_108_REG = crate::Reg<tve_108_reg::TVE_108_REG_SPEC>;
#[doc = "TV Encoder Notch Filter Frequency Register"]
pub mod tve_108_reg;
#[doc = "TVE_10C_REG register accessor: an alias for `Reg<TVE_10C_REG_SPEC>`"]
pub type TVE_10C_REG = crate::Reg<tve_10c_reg::TVE_10C_REG_SPEC>;
#[doc = "TV Encoder Cb/Cr Level/Gain Register"]
pub mod tve_10c_reg;
#[doc = "TVE_110_REG register accessor: an alias for `Reg<TVE_110_REG_SPEC>`"]
pub type TVE_110_REG = crate::Reg<tve_110_reg::TVE_110_REG_SPEC>;
#[doc = "TV Encoder Tint and Color Burst Phase Register"]
pub mod tve_110_reg;
#[doc = "TVE_114_REG register accessor: an alias for `Reg<TVE_114_REG_SPEC>`"]
pub type TVE_114_REG = crate::Reg<tve_114_reg::TVE_114_REG_SPEC>;
#[doc = "TV Encoder Burst Width Register"]
pub mod tve_114_reg;
#[doc = "TVE_118_REG register accessor: an alias for `Reg<TVE_118_REG_SPEC>`"]
pub type TVE_118_REG = crate::Reg<tve_118_reg::TVE_118_REG_SPEC>;
#[doc = "TV Encoder Cb/Cr Gain Register"]
pub mod tve_118_reg;
#[doc = "TVE_11C_REG register accessor: an alias for `Reg<TVE_11C_REG_SPEC>`"]
pub type TVE_11C_REG = crate::Reg<tve_11c_reg::TVE_11C_REG_SPEC>;
#[doc = "TV Encoder Sync and VBI Level Register"]
pub mod tve_11c_reg;
#[doc = "TVE_120_REG register accessor: an alias for `Reg<TVE_120_REG_SPEC>`"]
pub type TVE_120_REG = crate::Reg<tve_120_reg::TVE_120_REG_SPEC>;
#[doc = "TV Encoder White Level Register"]
pub mod tve_120_reg;
#[doc = "TVE_124_REG register accessor: an alias for `Reg<TVE_124_REG_SPEC>`"]
pub type TVE_124_REG = crate::Reg<tve_124_reg::TVE_124_REG_SPEC>;
#[doc = "TV Encoder Video Active Line Register"]
pub mod tve_124_reg;
#[doc = "TVE_128_REG register accessor: an alias for `Reg<TVE_128_REG_SPEC>`"]
pub type TVE_128_REG = crate::Reg<tve_128_reg::TVE_128_REG_SPEC>;
#[doc = "TV Encoder Video Chroma BW and CompGain Register"]
pub mod tve_128_reg;
#[doc = "TVE_12C_REG register accessor: an alias for `Reg<TVE_12C_REG_SPEC>`"]
pub type TVE_12C_REG = crate::Reg<tve_12c_reg::TVE_12C_REG_SPEC>;
#[doc = "TV Encoder Register"]
pub mod tve_12c_reg;
#[doc = "TVE_130_REG register accessor: an alias for `Reg<TVE_130_REG_SPEC>`"]
pub type TVE_130_REG = crate::Reg<tve_130_reg::TVE_130_REG_SPEC>;
#[doc = "TV Encoder Re-sync Parameters Register"]
pub mod tve_130_reg;
#[doc = "TVE_134_REG register accessor: an alias for `Reg<TVE_134_REG_SPEC>`"]
pub type TVE_134_REG = crate::Reg<tve_134_reg::TVE_134_REG_SPEC>;
#[doc = "TV Encoder Slave Parameter Register"]
pub mod tve_134_reg;
#[doc = "TVE_138_REG register accessor: an alias for `Reg<TVE_138_REG_SPEC>`"]
pub type TVE_138_REG = crate::Reg<tve_138_reg::TVE_138_REG_SPEC>;
#[doc = "TV Encoder Configuration Register0"]
pub mod tve_138_reg;
#[doc = "TVE_13C_REG register accessor: an alias for `Reg<TVE_13C_REG_SPEC>`"]
pub type TVE_13C_REG = crate::Reg<tve_13c_reg::TVE_13C_REG_SPEC>;
#[doc = "TV Encoder Configuration Register1"]
pub mod tve_13c_reg;
#[doc = "TVE_380_REG register accessor: an alias for `Reg<TVE_380_REG_SPEC>`"]
pub type TVE_380_REG = crate::Reg<tve_380_reg::TVE_380_REG_SPEC>;
#[doc = "TV Encoder Low Pass Control Register"]
pub mod tve_380_reg;
#[doc = "TVE_384_REG register accessor: an alias for `Reg<TVE_384_REG_SPEC>`"]
pub type TVE_384_REG = crate::Reg<tve_384_reg::TVE_384_REG_SPEC>;
#[doc = "TV Encoder Low Pass Filter Control Register"]
pub mod tve_384_reg;
