#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    tv_gctl: TV_GCTL,
    tv_gint0: TV_GINT0,
    tv_gint1: TV_GINT1,
    _reserved3: [u8; 0x34],
    tv_src_ctl: TV_SRC_CTL,
    _reserved4: [u8; 0x44],
    tv_io_pol: TV_IO_POL,
    tv_io_tri: TV_IO_TRI,
    tv_ctl: TV_CTL,
    tv_basic0: TV_BASIC0,
    tv_basic1: TV_BASIC1,
    tv_basic2: TV_BASIC2,
    tv_basic3: TV_BASIC3,
    tv_basic4: TV_BASIC4,
    tv_basic5: TV_BASIC5,
    _reserved13: [u8; 0x50],
    tv_debug: TV_DEBUG,
    tv_ceu_ctl: TV_CEU_CTL,
    _reserved15: [u8; 0x0c],
    tv_ceu_coef_mul: [TV_CEU_COEF_MUL; 11],
    _reserved16: [u8; 0x04],
    tv_ceu_coef_rang: [TV_CEU_COEF_RANG; 3],
    _reserved17: [u8; 0xa4],
    tv_safe_period: TV_SAFE_PERIOD,
    _reserved18: [u8; 0x010c],
    tv_fill_ctl: TV_FILL_CTL,
    tv_fill_begin: (),
    _reserved20: [u8; 0x04],
    tv_fill_end: (),
    _reserved21: [u8; 0x04],
    tv_fill_data: (),
    _reserved22: [u8; 0x24],
    tv_data_io_pol0: TV_DATA_IO_POL0,
    tv_data_io_pol1: TV_DATA_IO_POL1,
    tv_data_io_tri0: TV_DATA_IO_TRI0,
    tv_data_io_tri1: TV_DATA_IO_TRI1,
    tv_pixeldepth_mode: TV_PIXELDEPTH_MODE,
}
impl RegisterBlock {
    #[doc = "0x00 - TV Global Control Register"]
    #[inline(always)]
    pub const fn tv_gctl(&self) -> &TV_GCTL {
        &self.tv_gctl
    }
    #[doc = "0x04 - TV Global Interrupt Register0"]
    #[inline(always)]
    pub const fn tv_gint0(&self) -> &TV_GINT0 {
        &self.tv_gint0
    }
    #[doc = "0x08 - TV Global Interrupt Register1"]
    #[inline(always)]
    pub const fn tv_gint1(&self) -> &TV_GINT1 {
        &self.tv_gint1
    }
    #[doc = "0x40 - TV Source Control Register"]
    #[inline(always)]
    pub const fn tv_src_ctl(&self) -> &TV_SRC_CTL {
        &self.tv_src_ctl
    }
    #[doc = "0x88 - TV SYNC Signal Polarity Register"]
    #[inline(always)]
    pub const fn tv_io_pol(&self) -> &TV_IO_POL {
        &self.tv_io_pol
    }
    #[doc = "0x8c - TV SYNC Signal IO Control Register"]
    #[inline(always)]
    pub const fn tv_io_tri(&self) -> &TV_IO_TRI {
        &self.tv_io_tri
    }
    #[doc = "0x90 - TV Control Register"]
    #[inline(always)]
    pub const fn tv_ctl(&self) -> &TV_CTL {
        &self.tv_ctl
    }
    #[doc = "0x94 - TV Basic Timing Register0"]
    #[inline(always)]
    pub const fn tv_basic0(&self) -> &TV_BASIC0 {
        &self.tv_basic0
    }
    #[doc = "0x98 - TV Basic Timing Register1"]
    #[inline(always)]
    pub const fn tv_basic1(&self) -> &TV_BASIC1 {
        &self.tv_basic1
    }
    #[doc = "0x9c - TV Basic Timing Register2"]
    #[inline(always)]
    pub const fn tv_basic2(&self) -> &TV_BASIC2 {
        &self.tv_basic2
    }
    #[doc = "0xa0 - TV Basic Timing Register3"]
    #[inline(always)]
    pub const fn tv_basic3(&self) -> &TV_BASIC3 {
        &self.tv_basic3
    }
    #[doc = "0xa4 - TV Basic Timing Register4"]
    #[inline(always)]
    pub const fn tv_basic4(&self) -> &TV_BASIC4 {
        &self.tv_basic4
    }
    #[doc = "0xa8 - TV Basic Timing Register5"]
    #[inline(always)]
    pub const fn tv_basic5(&self) -> &TV_BASIC5 {
        &self.tv_basic5
    }
    #[doc = "0xfc - TV Debug Register"]
    #[inline(always)]
    pub const fn tv_debug(&self) -> &TV_DEBUG {
        &self.tv_debug
    }
    #[doc = "0x100 - TV CEU Control Register"]
    #[inline(always)]
    pub const fn tv_ceu_ctl(&self) -> &TV_CEU_CTL {
        &self.tv_ceu_ctl
    }
    #[doc = "0x110..0x13c - TV CEU Coefficient Register0"]
    #[inline(always)]
    pub const fn tv_ceu_coef_mul(&self, n: usize) -> &TV_CEU_COEF_MUL {
        &self.tv_ceu_coef_mul[n]
    }
    #[doc = "0x140..0x14c - TV CEU Coefficient Register2"]
    #[inline(always)]
    pub const fn tv_ceu_coef_rang(&self, n: usize) -> &TV_CEU_COEF_RANG {
        &self.tv_ceu_coef_rang[n]
    }
    #[doc = "0x1f0 - TV Safe Period Register"]
    #[inline(always)]
    pub const fn tv_safe_period(&self) -> &TV_SAFE_PERIOD {
        &self.tv_safe_period
    }
    #[doc = "0x300 - TV Fill Data Control Register"]
    #[inline(always)]
    pub const fn tv_fill_ctl(&self) -> &TV_FILL_CTL {
        &self.tv_fill_ctl
    }
    #[doc = "0x304..0x310 - TV Fill Data Begin Register"]
    #[inline(always)]
    pub const fn tv_fill_begin(&self, n: usize) -> &TV_FILL_BEGIN {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(772)
                .add(12 * n)
                .cast()
        }
    }
    #[doc = "0x308..0x314 - TV Fill Data End Register"]
    #[inline(always)]
    pub const fn tv_fill_end(&self, n: usize) -> &TV_FILL_END {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(776)
                .add(12 * n)
                .cast()
        }
    }
    #[doc = "0x30c..0x318 - TV Fill Data Value Register"]
    #[inline(always)]
    pub const fn tv_fill_data(&self, n: usize) -> &TV_FILL_DATA {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(780)
                .add(12 * n)
                .cast()
        }
    }
    #[doc = "0x330 - TCON Data IO Polarity Control0"]
    #[inline(always)]
    pub const fn tv_data_io_pol0(&self) -> &TV_DATA_IO_POL0 {
        &self.tv_data_io_pol0
    }
    #[doc = "0x334 - TCON Data IO Polarity Control1"]
    #[inline(always)]
    pub const fn tv_data_io_pol1(&self) -> &TV_DATA_IO_POL1 {
        &self.tv_data_io_pol1
    }
    #[doc = "0x338 - TCON Data IO Enable Control0"]
    #[inline(always)]
    pub const fn tv_data_io_tri0(&self) -> &TV_DATA_IO_TRI0 {
        &self.tv_data_io_tri0
    }
    #[doc = "0x33c - TCON Data IO Enable Control1"]
    #[inline(always)]
    pub const fn tv_data_io_tri1(&self) -> &TV_DATA_IO_TRI1 {
        &self.tv_data_io_tri1
    }
    #[doc = "0x340 - TV Pixeldepth Mode Control Register"]
    #[inline(always)]
    pub const fn tv_pixeldepth_mode(&self) -> &TV_PIXELDEPTH_MODE {
        &self.tv_pixeldepth_mode
    }
}
#[doc = "tv_gctl (rw) register accessor: TV Global Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_gctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_gctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tv_gctl`] module"]
pub type TV_GCTL = crate::Reg<tv_gctl::TV_GCTL_SPEC>;
#[doc = "TV Global Control Register"]
pub mod tv_gctl;
#[doc = "tv_gint0 (rw) register accessor: TV Global Interrupt Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_gint0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_gint0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tv_gint0`] module"]
pub type TV_GINT0 = crate::Reg<tv_gint0::TV_GINT0_SPEC>;
#[doc = "TV Global Interrupt Register0"]
pub mod tv_gint0;
#[doc = "tv_gint1 (rw) register accessor: TV Global Interrupt Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_gint1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_gint1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tv_gint1`] module"]
pub type TV_GINT1 = crate::Reg<tv_gint1::TV_GINT1_SPEC>;
#[doc = "TV Global Interrupt Register1"]
pub mod tv_gint1;
#[doc = "tv_src_ctl (rw) register accessor: TV Source Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_src_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_src_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tv_src_ctl`] module"]
pub type TV_SRC_CTL = crate::Reg<tv_src_ctl::TV_SRC_CTL_SPEC>;
#[doc = "TV Source Control Register"]
pub mod tv_src_ctl;
#[doc = "tv_ctl (rw) register accessor: TV Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tv_ctl`] module"]
pub type TV_CTL = crate::Reg<tv_ctl::TV_CTL_SPEC>;
#[doc = "TV Control Register"]
pub mod tv_ctl;
#[doc = "tv_basic0 (rw) register accessor: TV Basic Timing Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_basic0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_basic0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tv_basic0`] module"]
pub type TV_BASIC0 = crate::Reg<tv_basic0::TV_BASIC0_SPEC>;
#[doc = "TV Basic Timing Register0"]
pub mod tv_basic0;
#[doc = "tv_basic1 (rw) register accessor: TV Basic Timing Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_basic1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_basic1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tv_basic1`] module"]
pub type TV_BASIC1 = crate::Reg<tv_basic1::TV_BASIC1_SPEC>;
#[doc = "TV Basic Timing Register1"]
pub mod tv_basic1;
#[doc = "tv_basic2 (rw) register accessor: TV Basic Timing Register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_basic2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_basic2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tv_basic2`] module"]
pub type TV_BASIC2 = crate::Reg<tv_basic2::TV_BASIC2_SPEC>;
#[doc = "TV Basic Timing Register2"]
pub mod tv_basic2;
#[doc = "tv_basic3 (rw) register accessor: TV Basic Timing Register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_basic3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_basic3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tv_basic3`] module"]
pub type TV_BASIC3 = crate::Reg<tv_basic3::TV_BASIC3_SPEC>;
#[doc = "TV Basic Timing Register3"]
pub mod tv_basic3;
#[doc = "tv_basic4 (rw) register accessor: TV Basic Timing Register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_basic4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_basic4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tv_basic4`] module"]
pub type TV_BASIC4 = crate::Reg<tv_basic4::TV_BASIC4_SPEC>;
#[doc = "TV Basic Timing Register4"]
pub mod tv_basic4;
#[doc = "tv_basic5 (rw) register accessor: TV Basic Timing Register5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_basic5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_basic5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tv_basic5`] module"]
pub type TV_BASIC5 = crate::Reg<tv_basic5::TV_BASIC5_SPEC>;
#[doc = "TV Basic Timing Register5"]
pub mod tv_basic5;
#[doc = "tv_io_pol (rw) register accessor: TV SYNC Signal Polarity Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_io_pol::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_io_pol::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tv_io_pol`] module"]
pub type TV_IO_POL = crate::Reg<tv_io_pol::TV_IO_POL_SPEC>;
#[doc = "TV SYNC Signal Polarity Register"]
pub mod tv_io_pol;
#[doc = "tv_io_tri (rw) register accessor: TV SYNC Signal IO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_io_tri::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_io_tri::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tv_io_tri`] module"]
pub type TV_IO_TRI = crate::Reg<tv_io_tri::TV_IO_TRI_SPEC>;
#[doc = "TV SYNC Signal IO Control Register"]
pub mod tv_io_tri;
#[doc = "tv_debug (rw) register accessor: TV Debug Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_debug::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_debug::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tv_debug`] module"]
pub type TV_DEBUG = crate::Reg<tv_debug::TV_DEBUG_SPEC>;
#[doc = "TV Debug Register"]
pub mod tv_debug;
#[doc = "tv_ceu_ctl (rw) register accessor: TV CEU Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_ceu_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_ceu_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tv_ceu_ctl`] module"]
pub type TV_CEU_CTL = crate::Reg<tv_ceu_ctl::TV_CEU_CTL_SPEC>;
#[doc = "TV CEU Control Register"]
pub mod tv_ceu_ctl;
#[doc = "tv_ceu_coef_mul (rw) register accessor: TV CEU Coefficient Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_ceu_coef_mul::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_ceu_coef_mul::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tv_ceu_coef_mul`] module"]
pub type TV_CEU_COEF_MUL = crate::Reg<tv_ceu_coef_mul::TV_CEU_COEF_MUL_SPEC>;
#[doc = "TV CEU Coefficient Register0"]
pub mod tv_ceu_coef_mul;
#[doc = "tv_ceu_coef_rang (rw) register accessor: TV CEU Coefficient Register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_ceu_coef_rang::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_ceu_coef_rang::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tv_ceu_coef_rang`] module"]
pub type TV_CEU_COEF_RANG = crate::Reg<tv_ceu_coef_rang::TV_CEU_COEF_RANG_SPEC>;
#[doc = "TV CEU Coefficient Register2"]
pub mod tv_ceu_coef_rang;
#[doc = "tv_safe_period (rw) register accessor: TV Safe Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_safe_period::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_safe_period::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tv_safe_period`] module"]
pub type TV_SAFE_PERIOD = crate::Reg<tv_safe_period::TV_SAFE_PERIOD_SPEC>;
#[doc = "TV Safe Period Register"]
pub mod tv_safe_period;
#[doc = "tv_fill_ctl (rw) register accessor: TV Fill Data Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_fill_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_fill_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tv_fill_ctl`] module"]
pub type TV_FILL_CTL = crate::Reg<tv_fill_ctl::TV_FILL_CTL_SPEC>;
#[doc = "TV Fill Data Control Register"]
pub mod tv_fill_ctl;
#[doc = "tv_fill_begin (rw) register accessor: TV Fill Data Begin Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_fill_begin::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_fill_begin::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tv_fill_begin`] module"]
pub type TV_FILL_BEGIN = crate::Reg<tv_fill_begin::TV_FILL_BEGIN_SPEC>;
#[doc = "TV Fill Data Begin Register"]
pub mod tv_fill_begin;
#[doc = "tv_fill_end (rw) register accessor: TV Fill Data End Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_fill_end::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_fill_end::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tv_fill_end`] module"]
pub type TV_FILL_END = crate::Reg<tv_fill_end::TV_FILL_END_SPEC>;
#[doc = "TV Fill Data End Register"]
pub mod tv_fill_end;
#[doc = "tv_fill_data (rw) register accessor: TV Fill Data Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_fill_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_fill_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tv_fill_data`] module"]
pub type TV_FILL_DATA = crate::Reg<tv_fill_data::TV_FILL_DATA_SPEC>;
#[doc = "TV Fill Data Value Register"]
pub mod tv_fill_data;
#[doc = "tv_data_io_pol0 (rw) register accessor: TCON Data IO Polarity Control0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_data_io_pol0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_data_io_pol0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tv_data_io_pol0`] module"]
pub type TV_DATA_IO_POL0 = crate::Reg<tv_data_io_pol0::TV_DATA_IO_POL0_SPEC>;
#[doc = "TCON Data IO Polarity Control0"]
pub mod tv_data_io_pol0;
#[doc = "tv_data_io_pol1 (rw) register accessor: TCON Data IO Polarity Control1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_data_io_pol1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_data_io_pol1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tv_data_io_pol1`] module"]
pub type TV_DATA_IO_POL1 = crate::Reg<tv_data_io_pol1::TV_DATA_IO_POL1_SPEC>;
#[doc = "TCON Data IO Polarity Control1"]
pub mod tv_data_io_pol1;
#[doc = "tv_data_io_tri0 (rw) register accessor: TCON Data IO Enable Control0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_data_io_tri0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_data_io_tri0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tv_data_io_tri0`] module"]
pub type TV_DATA_IO_TRI0 = crate::Reg<tv_data_io_tri0::TV_DATA_IO_TRI0_SPEC>;
#[doc = "TCON Data IO Enable Control0"]
pub mod tv_data_io_tri0;
#[doc = "tv_data_io_tri1 (rw) register accessor: TCON Data IO Enable Control1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_data_io_tri1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_data_io_tri1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tv_data_io_tri1`] module"]
pub type TV_DATA_IO_TRI1 = crate::Reg<tv_data_io_tri1::TV_DATA_IO_TRI1_SPEC>;
#[doc = "TCON Data IO Enable Control1"]
pub mod tv_data_io_tri1;
#[doc = "tv_pixeldepth_mode (rw) register accessor: TV Pixeldepth Mode Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_pixeldepth_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_pixeldepth_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tv_pixeldepth_mode`] module"]
pub type TV_PIXELDEPTH_MODE = crate::Reg<tv_pixeldepth_mode::TV_PIXELDEPTH_MODE_SPEC>;
#[doc = "TV Pixeldepth Mode Control Register"]
pub mod tv_pixeldepth_mode;
