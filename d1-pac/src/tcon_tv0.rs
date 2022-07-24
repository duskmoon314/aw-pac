#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TV Global Control Register"]
    pub tv_gctl: crate::Reg<tv_gctl::TV_GCTL_SPEC>,
    #[doc = "0x04 - TV Global Interrupt Register0"]
    pub tv_gint0: crate::Reg<tv_gint0::TV_GINT0_SPEC>,
    #[doc = "0x08 - TV Global Interrupt Register1"]
    pub tv_gint1: crate::Reg<tv_gint1::TV_GINT1_SPEC>,
    _reserved3: [u8; 0x34],
    #[doc = "0x40 - TV Source Control Register"]
    pub tv_src_ctl: crate::Reg<tv_src_ctl::TV_SRC_CTL_SPEC>,
    _reserved4: [u8; 0x44],
    #[doc = "0x88 - TV SYNC Signal Polarity Register"]
    pub tv_io_pol: crate::Reg<tv_io_pol::TV_IO_POL_SPEC>,
    #[doc = "0x8c - TV SYNC Signal IO Control Register"]
    pub tv_io_tri: crate::Reg<tv_io_tri::TV_IO_TRI_SPEC>,
    #[doc = "0x90 - TV Control Register"]
    pub tv_ctl: crate::Reg<tv_ctl::TV_CTL_SPEC>,
    #[doc = "0x94 - TV Basic Timing Register0"]
    pub tv_basic0: crate::Reg<tv_basic0::TV_BASIC0_SPEC>,
    #[doc = "0x98 - TV Basic Timing Register1"]
    pub tv_basic1: crate::Reg<tv_basic1::TV_BASIC1_SPEC>,
    #[doc = "0x9c - TV Basic Timing Register2"]
    pub tv_basic2: crate::Reg<tv_basic2::TV_BASIC2_SPEC>,
    #[doc = "0xa0 - TV Basic Timing Register3"]
    pub tv_basic3: crate::Reg<tv_basic3::TV_BASIC3_SPEC>,
    #[doc = "0xa4 - TV Basic Timing Register4"]
    pub tv_basic4: crate::Reg<tv_basic4::TV_BASIC4_SPEC>,
    #[doc = "0xa8 - TV Basic Timing Register5"]
    pub tv_basic5: crate::Reg<tv_basic5::TV_BASIC5_SPEC>,
    _reserved13: [u8; 0x50],
    #[doc = "0xfc - TV Debug Register"]
    pub tv_debug: crate::Reg<tv_debug::TV_DEBUG_SPEC>,
    #[doc = "0x100 - TV CEU Control Register"]
    pub tv_ceu_ctl: crate::Reg<tv_ceu_ctl::TV_CEU_CTL_SPEC>,
    _reserved15: [u8; 0x0c],
    #[doc = "0x110..0x13c - TV CEU Coefficient Register0"]
    pub tv_ceu_coef_mul: [crate::Reg<tv_ceu_coef_mul::TV_CEU_COEF_MUL_SPEC>; 11],
    _reserved16: [u8; 0x04],
    #[doc = "0x140..0x14c - TV CEU Coefficient Register2"]
    pub tv_ceu_coef_rang: [crate::Reg<tv_ceu_coef_rang::TV_CEU_COEF_RANG_SPEC>; 3],
    _reserved17: [u8; 0xa4],
    #[doc = "0x1f0 - TV Safe Period Register"]
    pub tv_safe_period: crate::Reg<tv_safe_period::TV_SAFE_PERIOD_SPEC>,
    _reserved18: [u8; 0x010c],
    #[doc = "0x300 - TV Fill Data Control Register"]
    pub tv_fill_ctl: crate::Reg<tv_fill_ctl::TV_FILL_CTL_SPEC>,
    #[doc = "0x304 - TV Fill Data Begin Register"]
    pub tv_fill_begin0: crate::Reg<tv_fill_begin::TV_FILL_BEGIN_SPEC>,
    #[doc = "0x308 - TV Fill Data End Register"]
    pub tv_fill_end0: crate::Reg<tv_fill_end::TV_FILL_END_SPEC>,
    #[doc = "0x30c - TV Fill Data Value Register"]
    pub tv_fill_data0: crate::Reg<tv_fill_data::TV_FILL_DATA_SPEC>,
    #[doc = "0x310 - TV Fill Data Begin Register"]
    pub tv_fill_begin1: crate::Reg<tv_fill_begin::TV_FILL_BEGIN_SPEC>,
    #[doc = "0x314 - TV Fill Data End Register"]
    pub tv_fill_end1: crate::Reg<tv_fill_end::TV_FILL_END_SPEC>,
    #[doc = "0x318 - TV Fill Data Value Register"]
    pub tv_fill_data1: crate::Reg<tv_fill_data::TV_FILL_DATA_SPEC>,
    #[doc = "0x31c - TV Fill Data Begin Register"]
    pub tv_fill_begin2: crate::Reg<tv_fill_begin::TV_FILL_BEGIN_SPEC>,
    #[doc = "0x320 - TV Fill Data End Register"]
    pub tv_fill_end2: crate::Reg<tv_fill_end::TV_FILL_END_SPEC>,
    #[doc = "0x324 - TV Fill Data Value Register"]
    pub tv_fill_data2: crate::Reg<tv_fill_data::TV_FILL_DATA_SPEC>,
    _reserved28: [u8; 0x08],
    #[doc = "0x330 - TCON Data IO Polarity Control0"]
    pub tv_data_io_pol0: crate::Reg<tv_data_io_pol0::TV_DATA_IO_POL0_SPEC>,
    #[doc = "0x334 - TCON Data IO Polarity Control1"]
    pub tv_data_io_pol1: crate::Reg<tv_data_io_pol1::TV_DATA_IO_POL1_SPEC>,
    #[doc = "0x338 - TCON Data IO Enable Control0"]
    pub tv_data_io_tri0: crate::Reg<tv_data_io_tri0::TV_DATA_IO_TRI0_SPEC>,
    #[doc = "0x33c - TCON Data IO Enable Control1"]
    pub tv_data_io_tri1: crate::Reg<tv_data_io_tri1::TV_DATA_IO_TRI1_SPEC>,
    #[doc = "0x340 - TV Pixeldepth Mode Control Register"]
    pub tv_pixeldepth_mode: crate::Reg<tv_pixeldepth_mode::TV_PIXELDEPTH_MODE_SPEC>,
}
#[doc = "tv_gctl register accessor: an alias for `Reg<TV_GCTL_SPEC>`"]
pub type TV_GCTL = crate::Reg<tv_gctl::TV_GCTL_SPEC>;
#[doc = "TV Global Control Register"]
pub mod tv_gctl;
#[doc = "tv_gint0 register accessor: an alias for `Reg<TV_GINT0_SPEC>`"]
pub type TV_GINT0 = crate::Reg<tv_gint0::TV_GINT0_SPEC>;
#[doc = "TV Global Interrupt Register0"]
pub mod tv_gint0;
#[doc = "tv_gint1 register accessor: an alias for `Reg<TV_GINT1_SPEC>`"]
pub type TV_GINT1 = crate::Reg<tv_gint1::TV_GINT1_SPEC>;
#[doc = "TV Global Interrupt Register1"]
pub mod tv_gint1;
#[doc = "tv_src_ctl register accessor: an alias for `Reg<TV_SRC_CTL_SPEC>`"]
pub type TV_SRC_CTL = crate::Reg<tv_src_ctl::TV_SRC_CTL_SPEC>;
#[doc = "TV Source Control Register"]
pub mod tv_src_ctl;
#[doc = "tv_ctl register accessor: an alias for `Reg<TV_CTL_SPEC>`"]
pub type TV_CTL = crate::Reg<tv_ctl::TV_CTL_SPEC>;
#[doc = "TV Control Register"]
pub mod tv_ctl;
#[doc = "tv_basic0 register accessor: an alias for `Reg<TV_BASIC0_SPEC>`"]
pub type TV_BASIC0 = crate::Reg<tv_basic0::TV_BASIC0_SPEC>;
#[doc = "TV Basic Timing Register0"]
pub mod tv_basic0;
#[doc = "tv_basic1 register accessor: an alias for `Reg<TV_BASIC1_SPEC>`"]
pub type TV_BASIC1 = crate::Reg<tv_basic1::TV_BASIC1_SPEC>;
#[doc = "TV Basic Timing Register1"]
pub mod tv_basic1;
#[doc = "tv_basic2 register accessor: an alias for `Reg<TV_BASIC2_SPEC>`"]
pub type TV_BASIC2 = crate::Reg<tv_basic2::TV_BASIC2_SPEC>;
#[doc = "TV Basic Timing Register2"]
pub mod tv_basic2;
#[doc = "tv_basic3 register accessor: an alias for `Reg<TV_BASIC3_SPEC>`"]
pub type TV_BASIC3 = crate::Reg<tv_basic3::TV_BASIC3_SPEC>;
#[doc = "TV Basic Timing Register3"]
pub mod tv_basic3;
#[doc = "tv_basic4 register accessor: an alias for `Reg<TV_BASIC4_SPEC>`"]
pub type TV_BASIC4 = crate::Reg<tv_basic4::TV_BASIC4_SPEC>;
#[doc = "TV Basic Timing Register4"]
pub mod tv_basic4;
#[doc = "tv_basic5 register accessor: an alias for `Reg<TV_BASIC5_SPEC>`"]
pub type TV_BASIC5 = crate::Reg<tv_basic5::TV_BASIC5_SPEC>;
#[doc = "TV Basic Timing Register5"]
pub mod tv_basic5;
#[doc = "tv_io_pol register accessor: an alias for `Reg<TV_IO_POL_SPEC>`"]
pub type TV_IO_POL = crate::Reg<tv_io_pol::TV_IO_POL_SPEC>;
#[doc = "TV SYNC Signal Polarity Register"]
pub mod tv_io_pol;
#[doc = "tv_io_tri register accessor: an alias for `Reg<TV_IO_TRI_SPEC>`"]
pub type TV_IO_TRI = crate::Reg<tv_io_tri::TV_IO_TRI_SPEC>;
#[doc = "TV SYNC Signal IO Control Register"]
pub mod tv_io_tri;
#[doc = "tv_debug register accessor: an alias for `Reg<TV_DEBUG_SPEC>`"]
pub type TV_DEBUG = crate::Reg<tv_debug::TV_DEBUG_SPEC>;
#[doc = "TV Debug Register"]
pub mod tv_debug;
#[doc = "tv_ceu_ctl register accessor: an alias for `Reg<TV_CEU_CTL_SPEC>`"]
pub type TV_CEU_CTL = crate::Reg<tv_ceu_ctl::TV_CEU_CTL_SPEC>;
#[doc = "TV CEU Control Register"]
pub mod tv_ceu_ctl;
#[doc = "tv_ceu_coef_mul register accessor: an alias for `Reg<TV_CEU_COEF_MUL_SPEC>`"]
pub type TV_CEU_COEF_MUL = crate::Reg<tv_ceu_coef_mul::TV_CEU_COEF_MUL_SPEC>;
#[doc = "TV CEU Coefficient Register0"]
pub mod tv_ceu_coef_mul;
#[doc = "tv_ceu_coef_rang register accessor: an alias for `Reg<TV_CEU_COEF_RANG_SPEC>`"]
pub type TV_CEU_COEF_RANG = crate::Reg<tv_ceu_coef_rang::TV_CEU_COEF_RANG_SPEC>;
#[doc = "TV CEU Coefficient Register2"]
pub mod tv_ceu_coef_rang;
#[doc = "tv_safe_period register accessor: an alias for `Reg<TV_SAFE_PERIOD_SPEC>`"]
pub type TV_SAFE_PERIOD = crate::Reg<tv_safe_period::TV_SAFE_PERIOD_SPEC>;
#[doc = "TV Safe Period Register"]
pub mod tv_safe_period;
#[doc = "tv_fill_ctl register accessor: an alias for `Reg<TV_FILL_CTL_SPEC>`"]
pub type TV_FILL_CTL = crate::Reg<tv_fill_ctl::TV_FILL_CTL_SPEC>;
#[doc = "TV Fill Data Control Register"]
pub mod tv_fill_ctl;
#[doc = "tv_fill_begin register accessor: an alias for `Reg<TV_FILL_BEGIN_SPEC>`"]
pub type TV_FILL_BEGIN = crate::Reg<tv_fill_begin::TV_FILL_BEGIN_SPEC>;
#[doc = "TV Fill Data Begin Register"]
pub mod tv_fill_begin;
#[doc = "tv_fill_end register accessor: an alias for `Reg<TV_FILL_END_SPEC>`"]
pub type TV_FILL_END = crate::Reg<tv_fill_end::TV_FILL_END_SPEC>;
#[doc = "TV Fill Data End Register"]
pub mod tv_fill_end;
#[doc = "tv_fill_data register accessor: an alias for `Reg<TV_FILL_DATA_SPEC>`"]
pub type TV_FILL_DATA = crate::Reg<tv_fill_data::TV_FILL_DATA_SPEC>;
#[doc = "TV Fill Data Value Register"]
pub mod tv_fill_data;
#[doc = "tv_data_io_pol0 register accessor: an alias for `Reg<TV_DATA_IO_POL0_SPEC>`"]
pub type TV_DATA_IO_POL0 = crate::Reg<tv_data_io_pol0::TV_DATA_IO_POL0_SPEC>;
#[doc = "TCON Data IO Polarity Control0"]
pub mod tv_data_io_pol0;
#[doc = "tv_data_io_pol1 register accessor: an alias for `Reg<TV_DATA_IO_POL1_SPEC>`"]
pub type TV_DATA_IO_POL1 = crate::Reg<tv_data_io_pol1::TV_DATA_IO_POL1_SPEC>;
#[doc = "TCON Data IO Polarity Control1"]
pub mod tv_data_io_pol1;
#[doc = "tv_data_io_tri0 register accessor: an alias for `Reg<TV_DATA_IO_TRI0_SPEC>`"]
pub type TV_DATA_IO_TRI0 = crate::Reg<tv_data_io_tri0::TV_DATA_IO_TRI0_SPEC>;
#[doc = "TCON Data IO Enable Control0"]
pub mod tv_data_io_tri0;
#[doc = "tv_data_io_tri1 register accessor: an alias for `Reg<TV_DATA_IO_TRI1_SPEC>`"]
pub type TV_DATA_IO_TRI1 = crate::Reg<tv_data_io_tri1::TV_DATA_IO_TRI1_SPEC>;
#[doc = "TCON Data IO Enable Control1"]
pub mod tv_data_io_tri1;
#[doc = "tv_pixeldepth_mode register accessor: an alias for `Reg<TV_PIXELDEPTH_MODE_SPEC>`"]
pub type TV_PIXELDEPTH_MODE = crate::Reg<tv_pixeldepth_mode::TV_PIXELDEPTH_MODE_SPEC>;
#[doc = "TV Pixeldepth Mode Control Register"]
pub mod tv_pixeldepth_mode;
