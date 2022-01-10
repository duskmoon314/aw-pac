#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TV Global Control Register"]
    pub tv_gctl_reg: crate::Reg<tv_gctl_reg::TV_GCTL_REG_SPEC>,
    #[doc = "0x04 - TV Global Interrupt Register0"]
    pub tv_gint0_reg: crate::Reg<tv_gint0_reg::TV_GINT0_REG_SPEC>,
    #[doc = "0x08 - TV Global Interrupt Register1"]
    pub tv_gint1_reg: crate::Reg<tv_gint1_reg::TV_GINT1_REG_SPEC>,
    _reserved3: [u8; 0x34],
    #[doc = "0x40 - TV Source Control Register"]
    pub tv_src_ctl_reg: crate::Reg<tv_src_ctl_reg::TV_SRC_CTL_REG_SPEC>,
    _reserved4: [u8; 0x44],
    #[doc = "0x88 - TV SYNC Signal Polarity Register"]
    pub tv_io_pol_reg: crate::Reg<tv_io_pol_reg::TV_IO_POL_REG_SPEC>,
    #[doc = "0x8c - TV SYNC Signal IO Control Register"]
    pub tv_io_tri_reg: crate::Reg<tv_io_tri_reg::TV_IO_TRI_REG_SPEC>,
    #[doc = "0x90 - TV Control Register"]
    pub tv_ctl_reg: crate::Reg<tv_ctl_reg::TV_CTL_REG_SPEC>,
    #[doc = "0x94 - TV Basic Timing Register0"]
    pub tv_basic0_reg: crate::Reg<tv_basic0_reg::TV_BASIC0_REG_SPEC>,
    #[doc = "0x98 - TV Basic Timing Register1"]
    pub tv_basic1_reg: crate::Reg<tv_basic1_reg::TV_BASIC1_REG_SPEC>,
    #[doc = "0x9c - TV Basic Timing Register2"]
    pub tv_basic2_reg: crate::Reg<tv_basic2_reg::TV_BASIC2_REG_SPEC>,
    #[doc = "0xa0 - TV Basic Timing Register3"]
    pub tv_basic3_reg: crate::Reg<tv_basic3_reg::TV_BASIC3_REG_SPEC>,
    #[doc = "0xa4 - TV Basic Timing Register4"]
    pub tv_basic4_reg: crate::Reg<tv_basic4_reg::TV_BASIC4_REG_SPEC>,
    #[doc = "0xa8 - TV Basic Timing Register5"]
    pub tv_basic5_reg: crate::Reg<tv_basic5_reg::TV_BASIC5_REG_SPEC>,
    _reserved13: [u8; 0x50],
    #[doc = "0xfc - TV Debug Register"]
    pub tv_debug_reg: crate::Reg<tv_debug_reg::TV_DEBUG_REG_SPEC>,
    #[doc = "0x100 - TV CEU Control Register"]
    pub tv_ceu_ctl_reg: crate::Reg<tv_ceu_ctl_reg::TV_CEU_CTL_REG_SPEC>,
    _reserved15: [u8; 0x0c],
    #[doc = "0x110..0x13c - TV CEU Coefficient Register0"]
    pub tv_ceu_coef_mul_reg: [crate::Reg<tv_ceu_coef_mul_reg::TV_CEU_COEF_MUL_REG_SPEC>; 11],
    _reserved16: [u8; 0x04],
    #[doc = "0x140..0x14c - TV CEU Coefficient Register2"]
    pub tv_ceu_coef_rang_reg: [crate::Reg<tv_ceu_coef_rang_reg::TV_CEU_COEF_RANG_REG_SPEC>; 3],
    _reserved17: [u8; 0xa4],
    #[doc = "0x1f0 - TV Safe Period Register"]
    pub tv_safe_period_reg: crate::Reg<tv_safe_period_reg::TV_SAFE_PERIOD_REG_SPEC>,
    _reserved18: [u8; 0x010c],
    #[doc = "0x300 - TV Fill Data Control Register"]
    pub tv_fill_ctl_reg: crate::Reg<tv_fill_ctl_reg::TV_FILL_CTL_REG_SPEC>,
    #[doc = "0x304 - TV Fill Data Begin Register"]
    pub tv_fill_begin_reg0: crate::Reg<tv_fill_begin_reg::TV_FILL_BEGIN_REG_SPEC>,
    #[doc = "0x308 - TV Fill Data End Register"]
    pub tv_fill_end_reg0: crate::Reg<tv_fill_end_reg::TV_FILL_END_REG_SPEC>,
    #[doc = "0x30c - TV Fill Data Value Register"]
    pub tv_fill_data_reg0: crate::Reg<tv_fill_data_reg::TV_FILL_DATA_REG_SPEC>,
    #[doc = "0x310 - TV Fill Data Begin Register"]
    pub tv_fill_begin_reg1: crate::Reg<tv_fill_begin_reg::TV_FILL_BEGIN_REG_SPEC>,
    #[doc = "0x314 - TV Fill Data End Register"]
    pub tv_fill_end_reg1: crate::Reg<tv_fill_end_reg::TV_FILL_END_REG_SPEC>,
    #[doc = "0x318 - TV Fill Data Value Register"]
    pub tv_fill_data_reg1: crate::Reg<tv_fill_data_reg::TV_FILL_DATA_REG_SPEC>,
    #[doc = "0x31c - TV Fill Data Begin Register"]
    pub tv_fill_begin_reg2: crate::Reg<tv_fill_begin_reg::TV_FILL_BEGIN_REG_SPEC>,
    #[doc = "0x320 - TV Fill Data End Register"]
    pub tv_fill_end_reg2: crate::Reg<tv_fill_end_reg::TV_FILL_END_REG_SPEC>,
    #[doc = "0x324 - TV Fill Data Value Register"]
    pub tv_fill_data_reg2: crate::Reg<tv_fill_data_reg::TV_FILL_DATA_REG_SPEC>,
    _reserved28: [u8; 0x08],
    #[doc = "0x330 - TCON Data IO Polarity Control0"]
    pub tv_data_io_pol0_reg: crate::Reg<tv_data_io_pol0_reg::TV_DATA_IO_POL0_REG_SPEC>,
    #[doc = "0x334 - TCON Data IO Polarity Control1"]
    pub tv_data_io_pol1_reg: crate::Reg<tv_data_io_pol1_reg::TV_DATA_IO_POL1_REG_SPEC>,
    #[doc = "0x338 - TCON Data IO Enable Control0"]
    pub tv_data_io_tri0_reg: crate::Reg<tv_data_io_tri0_reg::TV_DATA_IO_TRI0_REG_SPEC>,
    #[doc = "0x33c - TCON Data IO Enable Control1"]
    pub tv_data_io_tri1_reg: crate::Reg<tv_data_io_tri1_reg::TV_DATA_IO_TRI1_REG_SPEC>,
    #[doc = "0x340 - TV Pixeldepth Mode Control Register"]
    pub tv_pixeldepth_mode_reg: crate::Reg<tv_pixeldepth_mode_reg::TV_PIXELDEPTH_MODE_REG_SPEC>,
}
#[doc = "TV_GCTL_REG register accessor: an alias for `Reg<TV_GCTL_REG_SPEC>`"]
pub type TV_GCTL_REG = crate::Reg<tv_gctl_reg::TV_GCTL_REG_SPEC>;
#[doc = "TV Global Control Register"]
pub mod tv_gctl_reg;
#[doc = "TV_GINT0_REG register accessor: an alias for `Reg<TV_GINT0_REG_SPEC>`"]
pub type TV_GINT0_REG = crate::Reg<tv_gint0_reg::TV_GINT0_REG_SPEC>;
#[doc = "TV Global Interrupt Register0"]
pub mod tv_gint0_reg;
#[doc = "TV_GINT1_REG register accessor: an alias for `Reg<TV_GINT1_REG_SPEC>`"]
pub type TV_GINT1_REG = crate::Reg<tv_gint1_reg::TV_GINT1_REG_SPEC>;
#[doc = "TV Global Interrupt Register1"]
pub mod tv_gint1_reg;
#[doc = "TV_SRC_CTL_REG register accessor: an alias for `Reg<TV_SRC_CTL_REG_SPEC>`"]
pub type TV_SRC_CTL_REG = crate::Reg<tv_src_ctl_reg::TV_SRC_CTL_REG_SPEC>;
#[doc = "TV Source Control Register"]
pub mod tv_src_ctl_reg;
#[doc = "TV_CTL_REG register accessor: an alias for `Reg<TV_CTL_REG_SPEC>`"]
pub type TV_CTL_REG = crate::Reg<tv_ctl_reg::TV_CTL_REG_SPEC>;
#[doc = "TV Control Register"]
pub mod tv_ctl_reg;
#[doc = "TV_BASIC0_REG register accessor: an alias for `Reg<TV_BASIC0_REG_SPEC>`"]
pub type TV_BASIC0_REG = crate::Reg<tv_basic0_reg::TV_BASIC0_REG_SPEC>;
#[doc = "TV Basic Timing Register0"]
pub mod tv_basic0_reg;
#[doc = "TV_BASIC1_REG register accessor: an alias for `Reg<TV_BASIC1_REG_SPEC>`"]
pub type TV_BASIC1_REG = crate::Reg<tv_basic1_reg::TV_BASIC1_REG_SPEC>;
#[doc = "TV Basic Timing Register1"]
pub mod tv_basic1_reg;
#[doc = "TV_BASIC2_REG register accessor: an alias for `Reg<TV_BASIC2_REG_SPEC>`"]
pub type TV_BASIC2_REG = crate::Reg<tv_basic2_reg::TV_BASIC2_REG_SPEC>;
#[doc = "TV Basic Timing Register2"]
pub mod tv_basic2_reg;
#[doc = "TV_BASIC3_REG register accessor: an alias for `Reg<TV_BASIC3_REG_SPEC>`"]
pub type TV_BASIC3_REG = crate::Reg<tv_basic3_reg::TV_BASIC3_REG_SPEC>;
#[doc = "TV Basic Timing Register3"]
pub mod tv_basic3_reg;
#[doc = "TV_BASIC4_REG register accessor: an alias for `Reg<TV_BASIC4_REG_SPEC>`"]
pub type TV_BASIC4_REG = crate::Reg<tv_basic4_reg::TV_BASIC4_REG_SPEC>;
#[doc = "TV Basic Timing Register4"]
pub mod tv_basic4_reg;
#[doc = "TV_BASIC5_REG register accessor: an alias for `Reg<TV_BASIC5_REG_SPEC>`"]
pub type TV_BASIC5_REG = crate::Reg<tv_basic5_reg::TV_BASIC5_REG_SPEC>;
#[doc = "TV Basic Timing Register5"]
pub mod tv_basic5_reg;
#[doc = "TV_IO_POL_REG register accessor: an alias for `Reg<TV_IO_POL_REG_SPEC>`"]
pub type TV_IO_POL_REG = crate::Reg<tv_io_pol_reg::TV_IO_POL_REG_SPEC>;
#[doc = "TV SYNC Signal Polarity Register"]
pub mod tv_io_pol_reg;
#[doc = "TV_IO_TRI_REG register accessor: an alias for `Reg<TV_IO_TRI_REG_SPEC>`"]
pub type TV_IO_TRI_REG = crate::Reg<tv_io_tri_reg::TV_IO_TRI_REG_SPEC>;
#[doc = "TV SYNC Signal IO Control Register"]
pub mod tv_io_tri_reg;
#[doc = "TV_DEBUG_REG register accessor: an alias for `Reg<TV_DEBUG_REG_SPEC>`"]
pub type TV_DEBUG_REG = crate::Reg<tv_debug_reg::TV_DEBUG_REG_SPEC>;
#[doc = "TV Debug Register"]
pub mod tv_debug_reg;
#[doc = "TV_CEU_CTL_REG register accessor: an alias for `Reg<TV_CEU_CTL_REG_SPEC>`"]
pub type TV_CEU_CTL_REG = crate::Reg<tv_ceu_ctl_reg::TV_CEU_CTL_REG_SPEC>;
#[doc = "TV CEU Control Register"]
pub mod tv_ceu_ctl_reg;
#[doc = "TV_CEU_COEF_MUL_REG register accessor: an alias for `Reg<TV_CEU_COEF_MUL_REG_SPEC>`"]
pub type TV_CEU_COEF_MUL_REG = crate::Reg<tv_ceu_coef_mul_reg::TV_CEU_COEF_MUL_REG_SPEC>;
#[doc = "TV CEU Coefficient Register0"]
pub mod tv_ceu_coef_mul_reg;
#[doc = "TV_CEU_COEF_RANG_REG register accessor: an alias for `Reg<TV_CEU_COEF_RANG_REG_SPEC>`"]
pub type TV_CEU_COEF_RANG_REG = crate::Reg<tv_ceu_coef_rang_reg::TV_CEU_COEF_RANG_REG_SPEC>;
#[doc = "TV CEU Coefficient Register2"]
pub mod tv_ceu_coef_rang_reg;
#[doc = "TV_SAFE_PERIOD_REG register accessor: an alias for `Reg<TV_SAFE_PERIOD_REG_SPEC>`"]
pub type TV_SAFE_PERIOD_REG = crate::Reg<tv_safe_period_reg::TV_SAFE_PERIOD_REG_SPEC>;
#[doc = "TV Safe Period Register"]
pub mod tv_safe_period_reg;
#[doc = "TV_FILL_CTL_REG register accessor: an alias for `Reg<TV_FILL_CTL_REG_SPEC>`"]
pub type TV_FILL_CTL_REG = crate::Reg<tv_fill_ctl_reg::TV_FILL_CTL_REG_SPEC>;
#[doc = "TV Fill Data Control Register"]
pub mod tv_fill_ctl_reg;
#[doc = "TV_FILL_BEGIN_REG register accessor: an alias for `Reg<TV_FILL_BEGIN_REG_SPEC>`"]
pub type TV_FILL_BEGIN_REG = crate::Reg<tv_fill_begin_reg::TV_FILL_BEGIN_REG_SPEC>;
#[doc = "TV Fill Data Begin Register"]
pub mod tv_fill_begin_reg;
#[doc = "TV_FILL_END_REG register accessor: an alias for `Reg<TV_FILL_END_REG_SPEC>`"]
pub type TV_FILL_END_REG = crate::Reg<tv_fill_end_reg::TV_FILL_END_REG_SPEC>;
#[doc = "TV Fill Data End Register"]
pub mod tv_fill_end_reg;
#[doc = "TV_FILL_DATA_REG register accessor: an alias for `Reg<TV_FILL_DATA_REG_SPEC>`"]
pub type TV_FILL_DATA_REG = crate::Reg<tv_fill_data_reg::TV_FILL_DATA_REG_SPEC>;
#[doc = "TV Fill Data Value Register"]
pub mod tv_fill_data_reg;
#[doc = "TV_DATA_IO_POL0_REG register accessor: an alias for `Reg<TV_DATA_IO_POL0_REG_SPEC>`"]
pub type TV_DATA_IO_POL0_REG = crate::Reg<tv_data_io_pol0_reg::TV_DATA_IO_POL0_REG_SPEC>;
#[doc = "TCON Data IO Polarity Control0"]
pub mod tv_data_io_pol0_reg;
#[doc = "TV_DATA_IO_POL1_REG register accessor: an alias for `Reg<TV_DATA_IO_POL1_REG_SPEC>`"]
pub type TV_DATA_IO_POL1_REG = crate::Reg<tv_data_io_pol1_reg::TV_DATA_IO_POL1_REG_SPEC>;
#[doc = "TCON Data IO Polarity Control1"]
pub mod tv_data_io_pol1_reg;
#[doc = "TV_DATA_IO_TRI0_REG register accessor: an alias for `Reg<TV_DATA_IO_TRI0_REG_SPEC>`"]
pub type TV_DATA_IO_TRI0_REG = crate::Reg<tv_data_io_tri0_reg::TV_DATA_IO_TRI0_REG_SPEC>;
#[doc = "TCON Data IO Enable Control0"]
pub mod tv_data_io_tri0_reg;
#[doc = "TV_DATA_IO_TRI1_REG register accessor: an alias for `Reg<TV_DATA_IO_TRI1_REG_SPEC>`"]
pub type TV_DATA_IO_TRI1_REG = crate::Reg<tv_data_io_tri1_reg::TV_DATA_IO_TRI1_REG_SPEC>;
#[doc = "TCON Data IO Enable Control1"]
pub mod tv_data_io_tri1_reg;
#[doc = "TV_PIXELDEPTH_MODE_REG register accessor: an alias for `Reg<TV_PIXELDEPTH_MODE_REG_SPEC>`"]
pub type TV_PIXELDEPTH_MODE_REG = crate::Reg<tv_pixeldepth_mode_reg::TV_PIXELDEPTH_MODE_REG_SPEC>;
#[doc = "TV Pixeldepth Mode Control Register"]
pub mod tv_pixeldepth_mode_reg;
