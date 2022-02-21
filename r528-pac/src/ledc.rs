#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - LEDC Control Register"]
    pub ledc_ctrl: crate::Reg<ledc_ctrl::LEDC_CTRL_SPEC>,
    #[doc = "0x04 - LEDC T0 T1 Timing Control Register"]
    pub led_t01_timing_ctrl: crate::Reg<led_t01_timing_ctrl::LED_T01_TIMING_CTRL_SPEC>,
    #[doc = "0x08 - LEDC Data Finish Counter Register"]
    pub ledc_data_finish_cnt: crate::Reg<ledc_data_finish_cnt::LEDC_DATA_FINISH_CNT_SPEC>,
    #[doc = "0x0c - LEDC Reset Timing Control Register"]
    pub led_reset_timing_ctrl: crate::Reg<led_reset_timing_ctrl::LED_RESET_TIMING_CTRL_SPEC>,
    #[doc = "0x10 - LEDC Wait Time0 Control Register"]
    pub ledc_wait_time0_ctrl: crate::Reg<ledc_wait_time0_ctrl::LEDC_WAIT_TIME0_CTRL_SPEC>,
    #[doc = "0x14 - LEDC Data Register"]
    pub ledc_data: crate::Reg<ledc_data::LEDC_DATA_SPEC>,
    #[doc = "0x18 - LEDC DMA Control Register"]
    pub ledc_dma_ctrl: crate::Reg<ledc_dma_ctrl::LEDC_DMA_CTRL_SPEC>,
    #[doc = "0x1c - LEDC Interrupt Control Register"]
    pub ledc_int_ctrl: crate::Reg<ledc_int_ctrl::LEDC_INT_CTRL_SPEC>,
    #[doc = "0x20 - LEDC Interrupt Status Register"]
    pub ledc_int_sts: crate::Reg<ledc_int_sts::LEDC_INT_STS_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0x28 - LEDC Wait Time1 Control Register"]
    pub ledc_wait_time1_ctrl: crate::Reg<ledc_wait_time1_ctrl::LEDC_WAIT_TIME1_CTRL_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x30..0xb0 - LEDC FIFO Data Register"]
    pub ledc_fifo_data: [crate::Reg<ledc_fifo_data::LEDC_FIFO_DATA_SPEC>; 32],
}
#[doc = "LEDC_CTRL register accessor: an alias for `Reg<LEDC_CTRL_SPEC>`"]
pub type LEDC_CTRL = crate::Reg<ledc_ctrl::LEDC_CTRL_SPEC>;
#[doc = "LEDC Control Register"]
pub mod ledc_ctrl;
#[doc = "LED_T01_TIMING_CTRL register accessor: an alias for `Reg<LED_T01_TIMING_CTRL_SPEC>`"]
pub type LED_T01_TIMING_CTRL = crate::Reg<led_t01_timing_ctrl::LED_T01_TIMING_CTRL_SPEC>;
#[doc = "LEDC T0 T1 Timing Control Register"]
pub mod led_t01_timing_ctrl;
#[doc = "LEDC_DATA_FINISH_CNT register accessor: an alias for `Reg<LEDC_DATA_FINISH_CNT_SPEC>`"]
pub type LEDC_DATA_FINISH_CNT = crate::Reg<ledc_data_finish_cnt::LEDC_DATA_FINISH_CNT_SPEC>;
#[doc = "LEDC Data Finish Counter Register"]
pub mod ledc_data_finish_cnt;
#[doc = "LED_RESET_TIMING_CTRL register accessor: an alias for `Reg<LED_RESET_TIMING_CTRL_SPEC>`"]
pub type LED_RESET_TIMING_CTRL = crate::Reg<led_reset_timing_ctrl::LED_RESET_TIMING_CTRL_SPEC>;
#[doc = "LEDC Reset Timing Control Register"]
pub mod led_reset_timing_ctrl;
#[doc = "LEDC_WAIT_TIME0_CTRL register accessor: an alias for `Reg<LEDC_WAIT_TIME0_CTRL_SPEC>`"]
pub type LEDC_WAIT_TIME0_CTRL = crate::Reg<ledc_wait_time0_ctrl::LEDC_WAIT_TIME0_CTRL_SPEC>;
#[doc = "LEDC Wait Time0 Control Register"]
pub mod ledc_wait_time0_ctrl;
#[doc = "LEDC_DATA register accessor: an alias for `Reg<LEDC_DATA_SPEC>`"]
pub type LEDC_DATA = crate::Reg<ledc_data::LEDC_DATA_SPEC>;
#[doc = "LEDC Data Register"]
pub mod ledc_data;
#[doc = "LEDC_DMA_CTRL register accessor: an alias for `Reg<LEDC_DMA_CTRL_SPEC>`"]
pub type LEDC_DMA_CTRL = crate::Reg<ledc_dma_ctrl::LEDC_DMA_CTRL_SPEC>;
#[doc = "LEDC DMA Control Register"]
pub mod ledc_dma_ctrl;
#[doc = "LEDC_INT_CTRL register accessor: an alias for `Reg<LEDC_INT_CTRL_SPEC>`"]
pub type LEDC_INT_CTRL = crate::Reg<ledc_int_ctrl::LEDC_INT_CTRL_SPEC>;
#[doc = "LEDC Interrupt Control Register"]
pub mod ledc_int_ctrl;
#[doc = "LEDC_INT_STS register accessor: an alias for `Reg<LEDC_INT_STS_SPEC>`"]
pub type LEDC_INT_STS = crate::Reg<ledc_int_sts::LEDC_INT_STS_SPEC>;
#[doc = "LEDC Interrupt Status Register"]
pub mod ledc_int_sts;
#[doc = "LEDC_WAIT_TIME1_CTRL register accessor: an alias for `Reg<LEDC_WAIT_TIME1_CTRL_SPEC>`"]
pub type LEDC_WAIT_TIME1_CTRL = crate::Reg<ledc_wait_time1_ctrl::LEDC_WAIT_TIME1_CTRL_SPEC>;
#[doc = "LEDC Wait Time1 Control Register"]
pub mod ledc_wait_time1_ctrl;
#[doc = "LEDC_FIFO_DATA register accessor: an alias for `Reg<LEDC_FIFO_DATA_SPEC>`"]
pub type LEDC_FIFO_DATA = crate::Reg<ledc_fifo_data::LEDC_FIFO_DATA_SPEC>;
#[doc = "LEDC FIFO Data Register"]
pub mod ledc_fifo_data;
