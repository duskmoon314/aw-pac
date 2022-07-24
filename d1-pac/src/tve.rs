#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TV Encoder Clock Gating Register"]
    pub tve_000: crate::Reg<tve_000::TVE_000_SPEC>,
    #[doc = "0x04 - TV Encoder Configuration Register"]
    pub tve_004: crate::Reg<tve_004::TVE_004_SPEC>,
    #[doc = "0x08 - TV Encoder DAC Register1"]
    pub tve_008: crate::Reg<tve_008::TVE_008_SPEC>,
    #[doc = "0x0c - TV Encoder Notch and DAC Delay Register"]
    pub tve_00c: crate::Reg<tve_00c::TVE_00C_SPEC>,
    #[doc = "0x10 - TV Encoder Chroma Frequency Register"]
    pub tve_010: crate::Reg<tve_010::TVE_010_SPEC>,
    #[doc = "0x14 - TV Encoder Front/Back Porch Register"]
    pub tve_014: crate::Reg<tve_014::TVE_014_SPEC>,
    #[doc = "0x18 - TV Encoder HD Mode VSYNC Register"]
    pub tve_018: crate::Reg<tve_018::TVE_018_SPEC>,
    #[doc = "0x1c - TV Encoder Line Number Register"]
    pub tve_01c: crate::Reg<tve_01c::TVE_01C_SPEC>,
    #[doc = "0x20 - TV Encoder Level Register"]
    pub tve_020: crate::Reg<tve_020::TVE_020_SPEC>,
    #[doc = "0x24 - TV Encoder DAC Register2"]
    pub tve_024: crate::Reg<tve_024::TVE_024_SPEC>,
    _reserved10: [u8; 0x08],
    #[doc = "0x30 - TV Encoder Auto Detection Enable Register"]
    pub tve_030: crate::Reg<tve_030::TVE_030_SPEC>,
    #[doc = "0x34 - TV Encoder Auto Detection Interrupt Status Register"]
    pub tve_034: crate::Reg<tve_034::TVE_034_SPEC>,
    #[doc = "0x38 - TV Encoder Auto Detection Status Register"]
    pub tve_038: crate::Reg<tve_038::TVE_038_SPEC>,
    #[doc = "0x3c - TV Encoder Auto Detection De-bounce Setting Register"]
    pub tve_03c: crate::Reg<tve_03c::TVE_03C_SPEC>,
    _reserved14: [u8; 0xb8],
    #[doc = "0xf8 - TV Encoder Auto Detect Configuration Register0"]
    pub tve_0f8: crate::Reg<tve_0f8::TVE_0F8_SPEC>,
    #[doc = "0xfc - TV Encoder Auto Detect Configuration Register1"]
    pub tve_0fc: crate::Reg<tve_0fc::TVE_0FC_SPEC>,
    #[doc = "0x100 - TV Encoder Color Burst Phase Reset Configuration Register"]
    pub tve_100: crate::Reg<tve_100::TVE_100_SPEC>,
    #[doc = "0x104 - TV Encoder VSYNC Number Register"]
    pub tve_104: crate::Reg<tve_104::TVE_104_SPEC>,
    #[doc = "0x108 - TV Encoder Notch Filter Frequency Register"]
    pub tve_108: crate::Reg<tve_108::TVE_108_SPEC>,
    #[doc = "0x10c - TV Encoder Cb/Cr Level/Gain Register"]
    pub tve_10c: crate::Reg<tve_10c::TVE_10C_SPEC>,
    #[doc = "0x110 - TV Encoder Tint and Color Burst Phase Register"]
    pub tve_110: crate::Reg<tve_110::TVE_110_SPEC>,
    #[doc = "0x114 - TV Encoder Burst Width Register"]
    pub tve_114: crate::Reg<tve_114::TVE_114_SPEC>,
    #[doc = "0x118 - TV Encoder Cb/Cr Gain Register"]
    pub tve_118: crate::Reg<tve_118::TVE_118_SPEC>,
    #[doc = "0x11c - TV Encoder Sync and VBI Level Register"]
    pub tve_11c: crate::Reg<tve_11c::TVE_11C_SPEC>,
    #[doc = "0x120 - TV Encoder White Level Register"]
    pub tve_120: crate::Reg<tve_120::TVE_120_SPEC>,
    #[doc = "0x124 - TV Encoder Video Active Line Register"]
    pub tve_124: crate::Reg<tve_124::TVE_124_SPEC>,
    #[doc = "0x128 - TV Encoder Video Chroma BW and CompGain Register"]
    pub tve_128: crate::Reg<tve_128::TVE_128_SPEC>,
    #[doc = "0x12c - TV Encoder Register"]
    pub tve_12c: crate::Reg<tve_12c::TVE_12C_SPEC>,
    #[doc = "0x130 - TV Encoder Re-sync Parameters Register"]
    pub tve_130: crate::Reg<tve_130::TVE_130_SPEC>,
    #[doc = "0x134 - TV Encoder Slave Parameter Register"]
    pub tve_134: crate::Reg<tve_134::TVE_134_SPEC>,
    #[doc = "0x138 - TV Encoder Configuration Register0"]
    pub tve_138: crate::Reg<tve_138::TVE_138_SPEC>,
    #[doc = "0x13c - TV Encoder Configuration Register1"]
    pub tve_13c: crate::Reg<tve_13c::TVE_13C_SPEC>,
    _reserved32: [u8; 0x0240],
    #[doc = "0x380 - TV Encoder Low Pass Control Register"]
    pub tve_380: crate::Reg<tve_380::TVE_380_SPEC>,
    #[doc = "0x384 - TV Encoder Low Pass Filter Control Register"]
    pub tve_384: crate::Reg<tve_384::TVE_384_SPEC>,
}
#[doc = "tve_000 register accessor: an alias for `Reg<TVE_000_SPEC>`"]
pub type TVE_000 = crate::Reg<tve_000::TVE_000_SPEC>;
#[doc = "TV Encoder Clock Gating Register"]
pub mod tve_000;
#[doc = "tve_004 register accessor: an alias for `Reg<TVE_004_SPEC>`"]
pub type TVE_004 = crate::Reg<tve_004::TVE_004_SPEC>;
#[doc = "TV Encoder Configuration Register"]
pub mod tve_004;
#[doc = "tve_008 register accessor: an alias for `Reg<TVE_008_SPEC>`"]
pub type TVE_008 = crate::Reg<tve_008::TVE_008_SPEC>;
#[doc = "TV Encoder DAC Register1"]
pub mod tve_008;
#[doc = "tve_00c register accessor: an alias for `Reg<TVE_00C_SPEC>`"]
pub type TVE_00C = crate::Reg<tve_00c::TVE_00C_SPEC>;
#[doc = "TV Encoder Notch and DAC Delay Register"]
pub mod tve_00c;
#[doc = "tve_010 register accessor: an alias for `Reg<TVE_010_SPEC>`"]
pub type TVE_010 = crate::Reg<tve_010::TVE_010_SPEC>;
#[doc = "TV Encoder Chroma Frequency Register"]
pub mod tve_010;
#[doc = "tve_014 register accessor: an alias for `Reg<TVE_014_SPEC>`"]
pub type TVE_014 = crate::Reg<tve_014::TVE_014_SPEC>;
#[doc = "TV Encoder Front/Back Porch Register"]
pub mod tve_014;
#[doc = "tve_018 register accessor: an alias for `Reg<TVE_018_SPEC>`"]
pub type TVE_018 = crate::Reg<tve_018::TVE_018_SPEC>;
#[doc = "TV Encoder HD Mode VSYNC Register"]
pub mod tve_018;
#[doc = "tve_01c register accessor: an alias for `Reg<TVE_01C_SPEC>`"]
pub type TVE_01C = crate::Reg<tve_01c::TVE_01C_SPEC>;
#[doc = "TV Encoder Line Number Register"]
pub mod tve_01c;
#[doc = "tve_020 register accessor: an alias for `Reg<TVE_020_SPEC>`"]
pub type TVE_020 = crate::Reg<tve_020::TVE_020_SPEC>;
#[doc = "TV Encoder Level Register"]
pub mod tve_020;
#[doc = "tve_024 register accessor: an alias for `Reg<TVE_024_SPEC>`"]
pub type TVE_024 = crate::Reg<tve_024::TVE_024_SPEC>;
#[doc = "TV Encoder DAC Register2"]
pub mod tve_024;
#[doc = "tve_030 register accessor: an alias for `Reg<TVE_030_SPEC>`"]
pub type TVE_030 = crate::Reg<tve_030::TVE_030_SPEC>;
#[doc = "TV Encoder Auto Detection Enable Register"]
pub mod tve_030;
#[doc = "tve_034 register accessor: an alias for `Reg<TVE_034_SPEC>`"]
pub type TVE_034 = crate::Reg<tve_034::TVE_034_SPEC>;
#[doc = "TV Encoder Auto Detection Interrupt Status Register"]
pub mod tve_034;
#[doc = "tve_038 register accessor: an alias for `Reg<TVE_038_SPEC>`"]
pub type TVE_038 = crate::Reg<tve_038::TVE_038_SPEC>;
#[doc = "TV Encoder Auto Detection Status Register"]
pub mod tve_038;
#[doc = "tve_03c register accessor: an alias for `Reg<TVE_03C_SPEC>`"]
pub type TVE_03C = crate::Reg<tve_03c::TVE_03C_SPEC>;
#[doc = "TV Encoder Auto Detection De-bounce Setting Register"]
pub mod tve_03c;
#[doc = "tve_0f8 register accessor: an alias for `Reg<TVE_0F8_SPEC>`"]
pub type TVE_0F8 = crate::Reg<tve_0f8::TVE_0F8_SPEC>;
#[doc = "TV Encoder Auto Detect Configuration Register0"]
pub mod tve_0f8;
#[doc = "tve_0fc register accessor: an alias for `Reg<TVE_0FC_SPEC>`"]
pub type TVE_0FC = crate::Reg<tve_0fc::TVE_0FC_SPEC>;
#[doc = "TV Encoder Auto Detect Configuration Register1"]
pub mod tve_0fc;
#[doc = "tve_100 register accessor: an alias for `Reg<TVE_100_SPEC>`"]
pub type TVE_100 = crate::Reg<tve_100::TVE_100_SPEC>;
#[doc = "TV Encoder Color Burst Phase Reset Configuration Register"]
pub mod tve_100;
#[doc = "tve_104 register accessor: an alias for `Reg<TVE_104_SPEC>`"]
pub type TVE_104 = crate::Reg<tve_104::TVE_104_SPEC>;
#[doc = "TV Encoder VSYNC Number Register"]
pub mod tve_104;
#[doc = "tve_108 register accessor: an alias for `Reg<TVE_108_SPEC>`"]
pub type TVE_108 = crate::Reg<tve_108::TVE_108_SPEC>;
#[doc = "TV Encoder Notch Filter Frequency Register"]
pub mod tve_108;
#[doc = "tve_10c register accessor: an alias for `Reg<TVE_10C_SPEC>`"]
pub type TVE_10C = crate::Reg<tve_10c::TVE_10C_SPEC>;
#[doc = "TV Encoder Cb/Cr Level/Gain Register"]
pub mod tve_10c;
#[doc = "tve_110 register accessor: an alias for `Reg<TVE_110_SPEC>`"]
pub type TVE_110 = crate::Reg<tve_110::TVE_110_SPEC>;
#[doc = "TV Encoder Tint and Color Burst Phase Register"]
pub mod tve_110;
#[doc = "tve_114 register accessor: an alias for `Reg<TVE_114_SPEC>`"]
pub type TVE_114 = crate::Reg<tve_114::TVE_114_SPEC>;
#[doc = "TV Encoder Burst Width Register"]
pub mod tve_114;
#[doc = "tve_118 register accessor: an alias for `Reg<TVE_118_SPEC>`"]
pub type TVE_118 = crate::Reg<tve_118::TVE_118_SPEC>;
#[doc = "TV Encoder Cb/Cr Gain Register"]
pub mod tve_118;
#[doc = "tve_11c register accessor: an alias for `Reg<TVE_11C_SPEC>`"]
pub type TVE_11C = crate::Reg<tve_11c::TVE_11C_SPEC>;
#[doc = "TV Encoder Sync and VBI Level Register"]
pub mod tve_11c;
#[doc = "tve_120 register accessor: an alias for `Reg<TVE_120_SPEC>`"]
pub type TVE_120 = crate::Reg<tve_120::TVE_120_SPEC>;
#[doc = "TV Encoder White Level Register"]
pub mod tve_120;
#[doc = "tve_124 register accessor: an alias for `Reg<TVE_124_SPEC>`"]
pub type TVE_124 = crate::Reg<tve_124::TVE_124_SPEC>;
#[doc = "TV Encoder Video Active Line Register"]
pub mod tve_124;
#[doc = "tve_128 register accessor: an alias for `Reg<TVE_128_SPEC>`"]
pub type TVE_128 = crate::Reg<tve_128::TVE_128_SPEC>;
#[doc = "TV Encoder Video Chroma BW and CompGain Register"]
pub mod tve_128;
#[doc = "tve_12c register accessor: an alias for `Reg<TVE_12C_SPEC>`"]
pub type TVE_12C = crate::Reg<tve_12c::TVE_12C_SPEC>;
#[doc = "TV Encoder Register"]
pub mod tve_12c;
#[doc = "tve_130 register accessor: an alias for `Reg<TVE_130_SPEC>`"]
pub type TVE_130 = crate::Reg<tve_130::TVE_130_SPEC>;
#[doc = "TV Encoder Re-sync Parameters Register"]
pub mod tve_130;
#[doc = "tve_134 register accessor: an alias for `Reg<TVE_134_SPEC>`"]
pub type TVE_134 = crate::Reg<tve_134::TVE_134_SPEC>;
#[doc = "TV Encoder Slave Parameter Register"]
pub mod tve_134;
#[doc = "tve_138 register accessor: an alias for `Reg<TVE_138_SPEC>`"]
pub type TVE_138 = crate::Reg<tve_138::TVE_138_SPEC>;
#[doc = "TV Encoder Configuration Register0"]
pub mod tve_138;
#[doc = "tve_13c register accessor: an alias for `Reg<TVE_13C_SPEC>`"]
pub type TVE_13C = crate::Reg<tve_13c::TVE_13C_SPEC>;
#[doc = "TV Encoder Configuration Register1"]
pub mod tve_13c;
#[doc = "tve_380 register accessor: an alias for `Reg<TVE_380_SPEC>`"]
pub type TVE_380 = crate::Reg<tve_380::TVE_380_SPEC>;
#[doc = "TV Encoder Low Pass Control Register"]
pub mod tve_380;
#[doc = "tve_384 register accessor: an alias for `Reg<TVE_384_SPEC>`"]
pub type TVE_384 = crate::Reg<tve_384::TVE_384_SPEC>;
#[doc = "TV Encoder Low Pass Filter Control Register"]
pub mod tve_384;
