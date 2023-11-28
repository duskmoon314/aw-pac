#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    tvd_top_map: TVD_TOP_MAP,
    _reserved1: [u8; 0x04],
    tvd_3d_ctl1: TVD_3D_CTL1,
    tvd_3d_ctl2: TVD_3D_CTL2,
    tvd_3d_ctl3: TVD_3D_CTL3,
    tvd_3d_ctl4: TVD_3D_CTL4,
    tvd_3d_ctl5: TVD_3D_CTL5,
    _reserved6: [u8; 0x08],
    tvd_top_ctl: (),
    _reserved7: [u8; 0x04],
    tvd_adc_ctl: (),
    _reserved8: [u8; 0x04],
    tvd_adc_cfg: (),
}
impl RegisterBlock {
    #[doc = "0x00 - TVD TOP MAP Register"]
    #[inline(always)]
    pub const fn tvd_top_map(&self) -> &TVD_TOP_MAP {
        &self.tvd_top_map
    }
    #[doc = "0x08 - TVD 3D DMA CONTROL Register1"]
    #[inline(always)]
    pub const fn tvd_3d_ctl1(&self) -> &TVD_3D_CTL1 {
        &self.tvd_3d_ctl1
    }
    #[doc = "0x0c - TVD 3D DMA CONTROL Register2"]
    #[inline(always)]
    pub const fn tvd_3d_ctl2(&self) -> &TVD_3D_CTL2 {
        &self.tvd_3d_ctl2
    }
    #[doc = "0x10 - TVD 3D DMA CONTROL Register3"]
    #[inline(always)]
    pub const fn tvd_3d_ctl3(&self) -> &TVD_3D_CTL3 {
        &self.tvd_3d_ctl3
    }
    #[doc = "0x14 - TVD 3D DMA CONTROL Register4"]
    #[inline(always)]
    pub const fn tvd_3d_ctl4(&self) -> &TVD_3D_CTL4 {
        &self.tvd_3d_ctl4
    }
    #[doc = "0x18 - TVD 3D DMA CONTROL Register5"]
    #[inline(always)]
    pub const fn tvd_3d_ctl5(&self) -> &TVD_3D_CTL5 {
        &self.tvd_3d_ctl5
    }
    #[doc = "0x24..0x34 - TVD TOP CONTROL Register"]
    #[inline(always)]
    pub const fn tvd_top_ctl(&self, n: usize) -> &TVD_TOP_CTL {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(36)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "0x28..0x38 - TVD ADC CONTROL Register"]
    #[inline(always)]
    pub const fn tvd_adc_ctl(&self, n: usize) -> &TVD_ADC_CTL {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(40)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "0x2c..0x3c - TVD ADC CONFIGURATION Register"]
    #[inline(always)]
    pub const fn tvd_adc_cfg(&self, n: usize) -> &TVD_ADC_CFG {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(44)
                .add(32 * n)
                .cast()
        }
    }
}
#[doc = "tvd_top_map (rw) register accessor: TVD TOP MAP Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_top_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_top_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_top_map`] module"]
pub type TVD_TOP_MAP = crate::Reg<tvd_top_map::TVD_TOP_MAP_SPEC>;
#[doc = "TVD TOP MAP Register"]
pub mod tvd_top_map;
#[doc = "tvd_3d_ctl1 (rw) register accessor: TVD 3D DMA CONTROL Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_3d_ctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_3d_ctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_3d_ctl1`] module"]
pub type TVD_3D_CTL1 = crate::Reg<tvd_3d_ctl1::TVD_3D_CTL1_SPEC>;
#[doc = "TVD 3D DMA CONTROL Register1"]
pub mod tvd_3d_ctl1;
#[doc = "tvd_3d_ctl2 (rw) register accessor: TVD 3D DMA CONTROL Register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_3d_ctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_3d_ctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_3d_ctl2`] module"]
pub type TVD_3D_CTL2 = crate::Reg<tvd_3d_ctl2::TVD_3D_CTL2_SPEC>;
#[doc = "TVD 3D DMA CONTROL Register2"]
pub mod tvd_3d_ctl2;
#[doc = "tvd_3d_ctl3 (rw) register accessor: TVD 3D DMA CONTROL Register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_3d_ctl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_3d_ctl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_3d_ctl3`] module"]
pub type TVD_3D_CTL3 = crate::Reg<tvd_3d_ctl3::TVD_3D_CTL3_SPEC>;
#[doc = "TVD 3D DMA CONTROL Register3"]
pub mod tvd_3d_ctl3;
#[doc = "tvd_3d_ctl4 (rw) register accessor: TVD 3D DMA CONTROL Register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_3d_ctl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_3d_ctl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_3d_ctl4`] module"]
pub type TVD_3D_CTL4 = crate::Reg<tvd_3d_ctl4::TVD_3D_CTL4_SPEC>;
#[doc = "TVD 3D DMA CONTROL Register4"]
pub mod tvd_3d_ctl4;
#[doc = "tvd_3d_ctl5 (rw) register accessor: TVD 3D DMA CONTROL Register5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_3d_ctl5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_3d_ctl5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_3d_ctl5`] module"]
pub type TVD_3D_CTL5 = crate::Reg<tvd_3d_ctl5::TVD_3D_CTL5_SPEC>;
#[doc = "TVD 3D DMA CONTROL Register5"]
pub mod tvd_3d_ctl5;
#[doc = "tvd_top_ctl (rw) register accessor: TVD TOP CONTROL Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_top_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_top_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_top_ctl`] module"]
pub type TVD_TOP_CTL = crate::Reg<tvd_top_ctl::TVD_TOP_CTL_SPEC>;
#[doc = "TVD TOP CONTROL Register"]
pub mod tvd_top_ctl;
#[doc = "tvd_adc_ctl (rw) register accessor: TVD ADC CONTROL Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_adc_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_adc_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_adc_ctl`] module"]
pub type TVD_ADC_CTL = crate::Reg<tvd_adc_ctl::TVD_ADC_CTL_SPEC>;
#[doc = "TVD ADC CONTROL Register"]
pub mod tvd_adc_ctl;
#[doc = "tvd_adc_cfg (rw) register accessor: TVD ADC CONFIGURATION Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_adc_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_adc_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_adc_cfg`] module"]
pub type TVD_ADC_CFG = crate::Reg<tvd_adc_cfg::TVD_ADC_CFG_SPEC>;
#[doc = "TVD ADC CONFIGURATION Register"]
pub mod tvd_adc_cfg;
