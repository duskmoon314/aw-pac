#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    tp_ctrl0: TP_CTRL0,
    tp_ctrl1: TP_CTRL1,
    tp_ctrl2: TP_CTRL2,
    tp_ctrl3: TP_CTRL3,
    tp_int_fifo_ctrl: TP_INT_FIFO_CTRL,
    tp_int_fifo_stat: TP_INT_FIFO_STAT,
    _reserved6: [u8; 0x04],
    tp_cali_data: TP_CALI_DATA,
    _reserved7: [u8; 0x04],
    tp_data: TP_DATA,
}
impl RegisterBlock {
    #[doc = "0x00 - TP Control Register 0"]
    #[inline(always)]
    pub const fn tp_ctrl0(&self) -> &TP_CTRL0 {
        &self.tp_ctrl0
    }
    #[doc = "0x04 - TP Control Register 1"]
    #[inline(always)]
    pub const fn tp_ctrl1(&self) -> &TP_CTRL1 {
        &self.tp_ctrl1
    }
    #[doc = "0x08 - TP Control Register 2"]
    #[inline(always)]
    pub const fn tp_ctrl2(&self) -> &TP_CTRL2 {
        &self.tp_ctrl2
    }
    #[doc = "0x0c - TP Control Register 3"]
    #[inline(always)]
    pub const fn tp_ctrl3(&self) -> &TP_CTRL3 {
        &self.tp_ctrl3
    }
    #[doc = "0x10 - TP Interrupt FIFO Control Register"]
    #[inline(always)]
    pub const fn tp_int_fifo_ctrl(&self) -> &TP_INT_FIFO_CTRL {
        &self.tp_int_fifo_ctrl
    }
    #[doc = "0x14 - TP Interrupt FIFO Status Register"]
    #[inline(always)]
    pub const fn tp_int_fifo_stat(&self) -> &TP_INT_FIFO_STAT {
        &self.tp_int_fifo_stat
    }
    #[doc = "0x1c - TP Calibration Data Register"]
    #[inline(always)]
    pub const fn tp_cali_data(&self) -> &TP_CALI_DATA {
        &self.tp_cali_data
    }
    #[doc = "0x24 - TP Data Register"]
    #[inline(always)]
    pub const fn tp_data(&self) -> &TP_DATA {
        &self.tp_data
    }
}
#[doc = "tp_ctrl0 (rw) register accessor: TP Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tp_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tp_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tp_ctrl0`] module"]
pub type TP_CTRL0 = crate::Reg<tp_ctrl0::TP_CTRL0_SPEC>;
#[doc = "TP Control Register 0"]
pub mod tp_ctrl0;
#[doc = "tp_ctrl1 (rw) register accessor: TP Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tp_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tp_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tp_ctrl1`] module"]
pub type TP_CTRL1 = crate::Reg<tp_ctrl1::TP_CTRL1_SPEC>;
#[doc = "TP Control Register 1"]
pub mod tp_ctrl1;
#[doc = "tp_ctrl2 (rw) register accessor: TP Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tp_ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tp_ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tp_ctrl2`] module"]
pub type TP_CTRL2 = crate::Reg<tp_ctrl2::TP_CTRL2_SPEC>;
#[doc = "TP Control Register 2"]
pub mod tp_ctrl2;
#[doc = "tp_ctrl3 (rw) register accessor: TP Control Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tp_ctrl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tp_ctrl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tp_ctrl3`] module"]
pub type TP_CTRL3 = crate::Reg<tp_ctrl3::TP_CTRL3_SPEC>;
#[doc = "TP Control Register 3"]
pub mod tp_ctrl3;
#[doc = "tp_int_fifo_ctrl (rw) register accessor: TP Interrupt FIFO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tp_int_fifo_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tp_int_fifo_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tp_int_fifo_ctrl`] module"]
pub type TP_INT_FIFO_CTRL = crate::Reg<tp_int_fifo_ctrl::TP_INT_FIFO_CTRL_SPEC>;
#[doc = "TP Interrupt FIFO Control Register"]
pub mod tp_int_fifo_ctrl;
#[doc = "tp_int_fifo_stat (rw) register accessor: TP Interrupt FIFO Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tp_int_fifo_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tp_int_fifo_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tp_int_fifo_stat`] module"]
pub type TP_INT_FIFO_STAT = crate::Reg<tp_int_fifo_stat::TP_INT_FIFO_STAT_SPEC>;
#[doc = "TP Interrupt FIFO Status Register"]
pub mod tp_int_fifo_stat;
#[doc = "tp_cali_data (rw) register accessor: TP Calibration Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tp_cali_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tp_cali_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tp_cali_data`] module"]
pub type TP_CALI_DATA = crate::Reg<tp_cali_data::TP_CALI_DATA_SPEC>;
#[doc = "TP Calibration Data Register"]
pub mod tp_cali_data;
#[doc = "tp_data (r) register accessor: TP Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tp_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tp_data`] module"]
pub type TP_DATA = crate::Reg<tp_data::TP_DATA_SPEC>;
#[doc = "TP Data Register"]
pub mod tp_data;
