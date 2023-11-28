#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ledc_ctrl: LEDC_CTRL,
    led_t01_timing_ctrl: LED_T01_TIMING_CTRL,
    ledc_data_finish_cnt: LEDC_DATA_FINISH_CNT,
    led_reset_timing_ctrl: LED_RESET_TIMING_CTRL,
    ledc_wait_time0_ctrl: LEDC_WAIT_TIME0_CTRL,
    ledc_data: LEDC_DATA,
    ledc_dma_ctrl: LEDC_DMA_CTRL,
    ledc_int_ctrl: LEDC_INT_CTRL,
    ledc_int_sts: LEDC_INT_STS,
    _reserved9: [u8; 0x04],
    ledc_wait_time1_ctrl: LEDC_WAIT_TIME1_CTRL,
    _reserved10: [u8; 0x04],
    ledc_fifo_data: [LEDC_FIFO_DATA; 32],
}
impl RegisterBlock {
    #[doc = "0x00 - LEDC Control Register"]
    #[inline(always)]
    pub const fn ledc_ctrl(&self) -> &LEDC_CTRL {
        &self.ledc_ctrl
    }
    #[doc = "0x04 - LEDC T0 T1 Timing Control Register"]
    #[inline(always)]
    pub const fn led_t01_timing_ctrl(&self) -> &LED_T01_TIMING_CTRL {
        &self.led_t01_timing_ctrl
    }
    #[doc = "0x08 - LEDC Data Finish Counter Register"]
    #[inline(always)]
    pub const fn ledc_data_finish_cnt(&self) -> &LEDC_DATA_FINISH_CNT {
        &self.ledc_data_finish_cnt
    }
    #[doc = "0x0c - LEDC Reset Timing Control Register"]
    #[inline(always)]
    pub const fn led_reset_timing_ctrl(&self) -> &LED_RESET_TIMING_CTRL {
        &self.led_reset_timing_ctrl
    }
    #[doc = "0x10 - LEDC Wait Time0 Control Register"]
    #[inline(always)]
    pub const fn ledc_wait_time0_ctrl(&self) -> &LEDC_WAIT_TIME0_CTRL {
        &self.ledc_wait_time0_ctrl
    }
    #[doc = "0x14 - LEDC Data Register"]
    #[inline(always)]
    pub const fn ledc_data(&self) -> &LEDC_DATA {
        &self.ledc_data
    }
    #[doc = "0x18 - LEDC DMA Control Register"]
    #[inline(always)]
    pub const fn ledc_dma_ctrl(&self) -> &LEDC_DMA_CTRL {
        &self.ledc_dma_ctrl
    }
    #[doc = "0x1c - LEDC Interrupt Control Register"]
    #[inline(always)]
    pub const fn ledc_int_ctrl(&self) -> &LEDC_INT_CTRL {
        &self.ledc_int_ctrl
    }
    #[doc = "0x20 - LEDC Interrupt Status Register"]
    #[inline(always)]
    pub const fn ledc_int_sts(&self) -> &LEDC_INT_STS {
        &self.ledc_int_sts
    }
    #[doc = "0x28 - LEDC Wait Time1 Control Register"]
    #[inline(always)]
    pub const fn ledc_wait_time1_ctrl(&self) -> &LEDC_WAIT_TIME1_CTRL {
        &self.ledc_wait_time1_ctrl
    }
    #[doc = "0x30..0xb0 - LEDC FIFO Data Register"]
    #[inline(always)]
    pub const fn ledc_fifo_data(&self, n: usize) -> &LEDC_FIFO_DATA {
        &self.ledc_fifo_data[n]
    }
}
#[doc = "ledc_ctrl (rw) register accessor: LEDC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ledc_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ledc_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ledc_ctrl`] module"]
pub type LEDC_CTRL = crate::Reg<ledc_ctrl::LEDC_CTRL_SPEC>;
#[doc = "LEDC Control Register"]
pub mod ledc_ctrl;
#[doc = "led_t01_timing_ctrl (rw) register accessor: LEDC T0 T1 Timing Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`led_t01_timing_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`led_t01_timing_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@led_t01_timing_ctrl`] module"]
pub type LED_T01_TIMING_CTRL = crate::Reg<led_t01_timing_ctrl::LED_T01_TIMING_CTRL_SPEC>;
#[doc = "LEDC T0 T1 Timing Control Register"]
pub mod led_t01_timing_ctrl;
#[doc = "ledc_data_finish_cnt (rw) register accessor: LEDC Data Finish Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ledc_data_finish_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ledc_data_finish_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ledc_data_finish_cnt`] module"]
pub type LEDC_DATA_FINISH_CNT = crate::Reg<ledc_data_finish_cnt::LEDC_DATA_FINISH_CNT_SPEC>;
#[doc = "LEDC Data Finish Counter Register"]
pub mod ledc_data_finish_cnt;
#[doc = "led_reset_timing_ctrl (rw) register accessor: LEDC Reset Timing Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`led_reset_timing_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`led_reset_timing_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@led_reset_timing_ctrl`] module"]
pub type LED_RESET_TIMING_CTRL = crate::Reg<led_reset_timing_ctrl::LED_RESET_TIMING_CTRL_SPEC>;
#[doc = "LEDC Reset Timing Control Register"]
pub mod led_reset_timing_ctrl;
#[doc = "ledc_wait_time0_ctrl (rw) register accessor: LEDC Wait Time0 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ledc_wait_time0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ledc_wait_time0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ledc_wait_time0_ctrl`] module"]
pub type LEDC_WAIT_TIME0_CTRL = crate::Reg<ledc_wait_time0_ctrl::LEDC_WAIT_TIME0_CTRL_SPEC>;
#[doc = "LEDC Wait Time0 Control Register"]
pub mod ledc_wait_time0_ctrl;
#[doc = "ledc_data (w) register accessor: LEDC Data Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ledc_data::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ledc_data`] module"]
pub type LEDC_DATA = crate::Reg<ledc_data::LEDC_DATA_SPEC>;
#[doc = "LEDC Data Register"]
pub mod ledc_data;
#[doc = "ledc_dma_ctrl (rw) register accessor: LEDC DMA Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ledc_dma_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ledc_dma_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ledc_dma_ctrl`] module"]
pub type LEDC_DMA_CTRL = crate::Reg<ledc_dma_ctrl::LEDC_DMA_CTRL_SPEC>;
#[doc = "LEDC DMA Control Register"]
pub mod ledc_dma_ctrl;
#[doc = "ledc_int_ctrl (rw) register accessor: LEDC Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ledc_int_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ledc_int_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ledc_int_ctrl`] module"]
pub type LEDC_INT_CTRL = crate::Reg<ledc_int_ctrl::LEDC_INT_CTRL_SPEC>;
#[doc = "LEDC Interrupt Control Register"]
pub mod ledc_int_ctrl;
#[doc = "ledc_int_sts (rw) register accessor: LEDC Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ledc_int_sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ledc_int_sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ledc_int_sts`] module"]
pub type LEDC_INT_STS = crate::Reg<ledc_int_sts::LEDC_INT_STS_SPEC>;
#[doc = "LEDC Interrupt Status Register"]
pub mod ledc_int_sts;
#[doc = "ledc_wait_time1_ctrl (rw) register accessor: LEDC Wait Time1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ledc_wait_time1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ledc_wait_time1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ledc_wait_time1_ctrl`] module"]
pub type LEDC_WAIT_TIME1_CTRL = crate::Reg<ledc_wait_time1_ctrl::LEDC_WAIT_TIME1_CTRL_SPEC>;
#[doc = "LEDC Wait Time1 Control Register"]
pub mod ledc_wait_time1_ctrl;
#[doc = "ledc_fifo_data (r) register accessor: LEDC FIFO Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ledc_fifo_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ledc_fifo_data`] module"]
pub type LEDC_FIFO_DATA = crate::Reg<ledc_fifo_data::LEDC_FIFO_DATA_SPEC>;
#[doc = "LEDC FIFO Data Register"]
pub mod ledc_fifo_data;
