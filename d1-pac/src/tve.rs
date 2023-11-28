#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    tve_clock_gating: TVE_CLOCK_GATING,
    tve_configuration: TVE_CONFIGURATION,
    tve_dac1: TVE_DAC1,
    tve_notch_dac_delay: TVE_NOTCH_DAC_DELAY,
    tve_chroma_frequency: TVE_CHROMA_FREQUENCY,
    tve_front_back_porch: TVE_FRONT_BACK_PORCH,
    tve_hd_vsync: TVE_HD_VSYNC,
    tve_line_number: TVE_LINE_NUMBER,
    tve_level: TVE_LEVEL,
    tve_dac2: TVE_DAC2,
    _reserved10: [u8; 0x08],
    tve_auto_detection_enable: TVE_AUTO_DETECTION_ENABLE,
    tve_auto_detection_interrupt_status: TVE_AUTO_DETECTION_INTERRUPT_STATUS,
    tve_auto_detection_status: TVE_AUTO_DETECTION_STATUS,
    tve_auto_detection_debounce_setting: TVE_AUTO_DETECTION_DEBOUNCE_SETTING,
    _reserved14: [u8; 0xb8],
    tve_auto_detect_cfg0: TVE_AUTO_DETECT_CFG0,
    tve_auto_detect_cfg1: TVE_AUTO_DETECT_CFG1,
    tve_color_burst_phase_reset_cfg: TVE_COLOR_BURST_PHASE_RESET_CFG,
    tve_vsync_number: TVE_VSYNC_NUMBER,
    tve_notch_filter_frequency: TVE_NOTCH_FILTER_FREQUENCY,
    tve_cbcr_level_gain: TVE_CBCR_LEVEL_GAIN,
    tve_tint_color_burst_phase: TVE_TINT_COLOR_BURST_PHASE,
    tve_burst_width: TVE_BURST_WIDTH,
    tve_cbcr_gain: TVE_CBCR_GAIN,
    tve_sync_vbi_level: TVE_SYNC_VBI_LEVEL,
    tve_white_level: TVE_WHITE_LEVEL,
    tve_video_active_line: TVE_VIDEO_ACTIVE_LINE,
    tve_video_chroma_bw_comp_gain: TVE_VIDEO_CHROMA_BW_COMP_GAIN,
    tve_notch_width_comp_yuv_en: TVE_NOTCH_WIDTH_COMP_YUV_EN,
    tve_resync_parameters: TVE_RESYNC_PARAMETERS,
    tve_slave_parameter: TVE_SLAVE_PARAMETER,
    tve_configuration0: TVE_CONFIGURATION0,
    tve_configuration1: TVE_CONFIGURATION1,
    _reserved32: [u8; 0x0240],
    tve_low_pass_control: TVE_LOW_PASS_CONTROL,
    tve_low_pass_filter_control: TVE_LOW_PASS_FILTER_CONTROL,
    tve_low_pass_gain: TVE_LOW_PASS_GAIN,
    tve_low_pass_gain_control: TVE_LOW_PASS_GAIN_CONTROL,
    tve_low_pass_shoot_control: TVE_LOW_PASS_SHOOT_CONTROL,
    tve_low_pass_coring: TVE_LOW_PASS_CORING,
    _reserved38: [u8; 0x08],
    tve_noise_reduction: TVE_NOISE_REDUCTION,
}
impl RegisterBlock {
    #[doc = "0x00 - TV Encoder Clock Gating Register"]
    #[inline(always)]
    pub const fn tve_clock_gating(&self) -> &TVE_CLOCK_GATING {
        &self.tve_clock_gating
    }
    #[doc = "0x04 - TV Encoder Configuration Register"]
    #[inline(always)]
    pub const fn tve_configuration(&self) -> &TVE_CONFIGURATION {
        &self.tve_configuration
    }
    #[doc = "0x08 - TV Encoder DAC Register1"]
    #[inline(always)]
    pub const fn tve_dac1(&self) -> &TVE_DAC1 {
        &self.tve_dac1
    }
    #[doc = "0x0c - TV Encoder Notch and DAC Delay Register"]
    #[inline(always)]
    pub const fn tve_notch_dac_delay(&self) -> &TVE_NOTCH_DAC_DELAY {
        &self.tve_notch_dac_delay
    }
    #[doc = "0x10 - TV Encoder Chroma Frequency Register"]
    #[inline(always)]
    pub const fn tve_chroma_frequency(&self) -> &TVE_CHROMA_FREQUENCY {
        &self.tve_chroma_frequency
    }
    #[doc = "0x14 - TV Encoder Front/Back Porch Register"]
    #[inline(always)]
    pub const fn tve_front_back_porch(&self) -> &TVE_FRONT_BACK_PORCH {
        &self.tve_front_back_porch
    }
    #[doc = "0x18 - TV Encoder HD Mode VSYNC Register"]
    #[inline(always)]
    pub const fn tve_hd_vsync(&self) -> &TVE_HD_VSYNC {
        &self.tve_hd_vsync
    }
    #[doc = "0x1c - TV Encoder Line Number Register"]
    #[inline(always)]
    pub const fn tve_line_number(&self) -> &TVE_LINE_NUMBER {
        &self.tve_line_number
    }
    #[doc = "0x20 - TV Encoder Level Register"]
    #[inline(always)]
    pub const fn tve_level(&self) -> &TVE_LEVEL {
        &self.tve_level
    }
    #[doc = "0x24 - TV Encoder DAC Register2"]
    #[inline(always)]
    pub const fn tve_dac2(&self) -> &TVE_DAC2 {
        &self.tve_dac2
    }
    #[doc = "0x30 - TV Encoder Auto Detection Enable Register"]
    #[inline(always)]
    pub const fn tve_auto_detection_enable(&self) -> &TVE_AUTO_DETECTION_ENABLE {
        &self.tve_auto_detection_enable
    }
    #[doc = "0x34 - TV Encoder Auto Detection Interrupt Status Register"]
    #[inline(always)]
    pub const fn tve_auto_detection_interrupt_status(
        &self,
    ) -> &TVE_AUTO_DETECTION_INTERRUPT_STATUS {
        &self.tve_auto_detection_interrupt_status
    }
    #[doc = "0x38 - TV Encoder Auto Detection Status Register"]
    #[inline(always)]
    pub const fn tve_auto_detection_status(&self) -> &TVE_AUTO_DETECTION_STATUS {
        &self.tve_auto_detection_status
    }
    #[doc = "0x3c - TV Encoder Auto Detection De-bounce Setting Register"]
    #[inline(always)]
    pub const fn tve_auto_detection_debounce_setting(
        &self,
    ) -> &TVE_AUTO_DETECTION_DEBOUNCE_SETTING {
        &self.tve_auto_detection_debounce_setting
    }
    #[doc = "0xf8 - TV Encoder Auto Detect Configuration Register0"]
    #[inline(always)]
    pub const fn tve_auto_detect_cfg0(&self) -> &TVE_AUTO_DETECT_CFG0 {
        &self.tve_auto_detect_cfg0
    }
    #[doc = "0xfc - TV Encoder Auto Detect Configuration Register1"]
    #[inline(always)]
    pub const fn tve_auto_detect_cfg1(&self) -> &TVE_AUTO_DETECT_CFG1 {
        &self.tve_auto_detect_cfg1
    }
    #[doc = "0x100 - TV Encoder Color Burst Phase Reset Configuration Register"]
    #[inline(always)]
    pub const fn tve_color_burst_phase_reset_cfg(&self) -> &TVE_COLOR_BURST_PHASE_RESET_CFG {
        &self.tve_color_burst_phase_reset_cfg
    }
    #[doc = "0x104 - TV Encoder VSYNC Number Register"]
    #[inline(always)]
    pub const fn tve_vsync_number(&self) -> &TVE_VSYNC_NUMBER {
        &self.tve_vsync_number
    }
    #[doc = "0x108 - TV Encoder Notch Filter Frequency Register"]
    #[inline(always)]
    pub const fn tve_notch_filter_frequency(&self) -> &TVE_NOTCH_FILTER_FREQUENCY {
        &self.tve_notch_filter_frequency
    }
    #[doc = "0x10c - TV Encoder Cb/Cr Level/Gain Register"]
    #[inline(always)]
    pub const fn tve_cbcr_level_gain(&self) -> &TVE_CBCR_LEVEL_GAIN {
        &self.tve_cbcr_level_gain
    }
    #[doc = "0x110 - TV Encoder Tint and Color Burst Phase Register"]
    #[inline(always)]
    pub const fn tve_tint_color_burst_phase(&self) -> &TVE_TINT_COLOR_BURST_PHASE {
        &self.tve_tint_color_burst_phase
    }
    #[doc = "0x114 - TV Encoder Burst Width Register"]
    #[inline(always)]
    pub const fn tve_burst_width(&self) -> &TVE_BURST_WIDTH {
        &self.tve_burst_width
    }
    #[doc = "0x118 - TV Encoder Cb/Cr Gain Register"]
    #[inline(always)]
    pub const fn tve_cbcr_gain(&self) -> &TVE_CBCR_GAIN {
        &self.tve_cbcr_gain
    }
    #[doc = "0x11c - TV Encoder Sync and VBI Level Register"]
    #[inline(always)]
    pub const fn tve_sync_vbi_level(&self) -> &TVE_SYNC_VBI_LEVEL {
        &self.tve_sync_vbi_level
    }
    #[doc = "0x120 - TV Encoder White Level Register"]
    #[inline(always)]
    pub const fn tve_white_level(&self) -> &TVE_WHITE_LEVEL {
        &self.tve_white_level
    }
    #[doc = "0x124 - TV Encoder Video Active Line Register"]
    #[inline(always)]
    pub const fn tve_video_active_line(&self) -> &TVE_VIDEO_ACTIVE_LINE {
        &self.tve_video_active_line
    }
    #[doc = "0x128 - TV Encoder Video Chroma BW and CompGain Register"]
    #[inline(always)]
    pub const fn tve_video_chroma_bw_comp_gain(&self) -> &TVE_VIDEO_CHROMA_BW_COMP_GAIN {
        &self.tve_video_chroma_bw_comp_gain
    }
    #[doc = "0x12c - TV Encoder Register"]
    #[inline(always)]
    pub const fn tve_notch_width_comp_yuv_en(&self) -> &TVE_NOTCH_WIDTH_COMP_YUV_EN {
        &self.tve_notch_width_comp_yuv_en
    }
    #[doc = "0x130 - TV Encoder Re-sync Parameters Register"]
    #[inline(always)]
    pub const fn tve_resync_parameters(&self) -> &TVE_RESYNC_PARAMETERS {
        &self.tve_resync_parameters
    }
    #[doc = "0x134 - TV Encoder Slave Parameter Register"]
    #[inline(always)]
    pub const fn tve_slave_parameter(&self) -> &TVE_SLAVE_PARAMETER {
        &self.tve_slave_parameter
    }
    #[doc = "0x138 - TV Encoder Configuration Register0"]
    #[inline(always)]
    pub const fn tve_configuration0(&self) -> &TVE_CONFIGURATION0 {
        &self.tve_configuration0
    }
    #[doc = "0x13c - TV Encoder Configuration Register1"]
    #[inline(always)]
    pub const fn tve_configuration1(&self) -> &TVE_CONFIGURATION1 {
        &self.tve_configuration1
    }
    #[doc = "0x380 - TV Encoder Low Pass Control Register"]
    #[inline(always)]
    pub const fn tve_low_pass_control(&self) -> &TVE_LOW_PASS_CONTROL {
        &self.tve_low_pass_control
    }
    #[doc = "0x384 - TV Encoder Low Pass Filter Control Register"]
    #[inline(always)]
    pub const fn tve_low_pass_filter_control(&self) -> &TVE_LOW_PASS_FILTER_CONTROL {
        &self.tve_low_pass_filter_control
    }
    #[doc = "0x388 - TV Encoder Low Pass Gain Register"]
    #[inline(always)]
    pub const fn tve_low_pass_gain(&self) -> &TVE_LOW_PASS_GAIN {
        &self.tve_low_pass_gain
    }
    #[doc = "0x38c - TV Encoder Low Pass Gain Control Register"]
    #[inline(always)]
    pub const fn tve_low_pass_gain_control(&self) -> &TVE_LOW_PASS_GAIN_CONTROL {
        &self.tve_low_pass_gain_control
    }
    #[doc = "0x390 - TV Encoder Low Pass Shoot Control Register"]
    #[inline(always)]
    pub const fn tve_low_pass_shoot_control(&self) -> &TVE_LOW_PASS_SHOOT_CONTROL {
        &self.tve_low_pass_shoot_control
    }
    #[doc = "0x394 - TV Encoder Low Pass Coring Register"]
    #[inline(always)]
    pub const fn tve_low_pass_coring(&self) -> &TVE_LOW_PASS_CORING {
        &self.tve_low_pass_coring
    }
    #[doc = "0x3a0 - TV Encoder Noise Reduction Register"]
    #[inline(always)]
    pub const fn tve_noise_reduction(&self) -> &TVE_NOISE_REDUCTION {
        &self.tve_noise_reduction
    }
}
#[doc = "tve_clock_gating (rw) register accessor: TV Encoder Clock Gating Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_clock_gating::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_clock_gating::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_clock_gating`] module"]
pub type TVE_CLOCK_GATING = crate::Reg<tve_clock_gating::TVE_CLOCK_GATING_SPEC>;
#[doc = "TV Encoder Clock Gating Register"]
pub mod tve_clock_gating;
#[doc = "tve_configuration (rw) register accessor: TV Encoder Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_configuration::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_configuration::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_configuration`] module"]
pub type TVE_CONFIGURATION = crate::Reg<tve_configuration::TVE_CONFIGURATION_SPEC>;
#[doc = "TV Encoder Configuration Register"]
pub mod tve_configuration;
#[doc = "tve_dac1 (rw) register accessor: TV Encoder DAC Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_dac1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_dac1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_dac1`] module"]
pub type TVE_DAC1 = crate::Reg<tve_dac1::TVE_DAC1_SPEC>;
#[doc = "TV Encoder DAC Register1"]
pub mod tve_dac1;
#[doc = "tve_notch_dac_delay (rw) register accessor: TV Encoder Notch and DAC Delay Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_notch_dac_delay::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_notch_dac_delay::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_notch_dac_delay`] module"]
pub type TVE_NOTCH_DAC_DELAY = crate::Reg<tve_notch_dac_delay::TVE_NOTCH_DAC_DELAY_SPEC>;
#[doc = "TV Encoder Notch and DAC Delay Register"]
pub mod tve_notch_dac_delay;
#[doc = "tve_chroma_frequency (rw) register accessor: TV Encoder Chroma Frequency Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_chroma_frequency::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_chroma_frequency::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_chroma_frequency`] module"]
pub type TVE_CHROMA_FREQUENCY = crate::Reg<tve_chroma_frequency::TVE_CHROMA_FREQUENCY_SPEC>;
#[doc = "TV Encoder Chroma Frequency Register"]
pub mod tve_chroma_frequency;
#[doc = "tve_front_back_porch (rw) register accessor: TV Encoder Front/Back Porch Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_front_back_porch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_front_back_porch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_front_back_porch`] module"]
pub type TVE_FRONT_BACK_PORCH = crate::Reg<tve_front_back_porch::TVE_FRONT_BACK_PORCH_SPEC>;
#[doc = "TV Encoder Front/Back Porch Register"]
pub mod tve_front_back_porch;
#[doc = "tve_hd_vsync (rw) register accessor: TV Encoder HD Mode VSYNC Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_hd_vsync::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_hd_vsync::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_hd_vsync`] module"]
pub type TVE_HD_VSYNC = crate::Reg<tve_hd_vsync::TVE_HD_VSYNC_SPEC>;
#[doc = "TV Encoder HD Mode VSYNC Register"]
pub mod tve_hd_vsync;
#[doc = "tve_line_number (rw) register accessor: TV Encoder Line Number Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_line_number::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_line_number::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_line_number`] module"]
pub type TVE_LINE_NUMBER = crate::Reg<tve_line_number::TVE_LINE_NUMBER_SPEC>;
#[doc = "TV Encoder Line Number Register"]
pub mod tve_line_number;
#[doc = "tve_level (rw) register accessor: TV Encoder Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_level::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_level::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_level`] module"]
pub type TVE_LEVEL = crate::Reg<tve_level::TVE_LEVEL_SPEC>;
#[doc = "TV Encoder Level Register"]
pub mod tve_level;
#[doc = "tve_dac2 (rw) register accessor: TV Encoder DAC Register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_dac2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_dac2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_dac2`] module"]
pub type TVE_DAC2 = crate::Reg<tve_dac2::TVE_DAC2_SPEC>;
#[doc = "TV Encoder DAC Register2"]
pub mod tve_dac2;
#[doc = "tve_auto_detection_enable (rw) register accessor: TV Encoder Auto Detection Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_auto_detection_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_auto_detection_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_auto_detection_enable`] module"]
pub type TVE_AUTO_DETECTION_ENABLE =
    crate::Reg<tve_auto_detection_enable::TVE_AUTO_DETECTION_ENABLE_SPEC>;
#[doc = "TV Encoder Auto Detection Enable Register"]
pub mod tve_auto_detection_enable;
#[doc = "tve_auto_detection_interrupt_status (rw) register accessor: TV Encoder Auto Detection Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_auto_detection_interrupt_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_auto_detection_interrupt_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_auto_detection_interrupt_status`] module"]
pub type TVE_AUTO_DETECTION_INTERRUPT_STATUS =
    crate::Reg<tve_auto_detection_interrupt_status::TVE_AUTO_DETECTION_INTERRUPT_STATUS_SPEC>;
#[doc = "TV Encoder Auto Detection Interrupt Status Register"]
pub mod tve_auto_detection_interrupt_status;
#[doc = "tve_auto_detection_status (rw) register accessor: TV Encoder Auto Detection Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_auto_detection_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_auto_detection_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_auto_detection_status`] module"]
pub type TVE_AUTO_DETECTION_STATUS =
    crate::Reg<tve_auto_detection_status::TVE_AUTO_DETECTION_STATUS_SPEC>;
#[doc = "TV Encoder Auto Detection Status Register"]
pub mod tve_auto_detection_status;
#[doc = "tve_auto_detection_debounce_setting (rw) register accessor: TV Encoder Auto Detection De-bounce Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_auto_detection_debounce_setting::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_auto_detection_debounce_setting::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_auto_detection_debounce_setting`] module"]
pub type TVE_AUTO_DETECTION_DEBOUNCE_SETTING =
    crate::Reg<tve_auto_detection_debounce_setting::TVE_AUTO_DETECTION_DEBOUNCE_SETTING_SPEC>;
#[doc = "TV Encoder Auto Detection De-bounce Setting Register"]
pub mod tve_auto_detection_debounce_setting;
#[doc = "tve_auto_detect_cfg0 (rw) register accessor: TV Encoder Auto Detect Configuration Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_auto_detect_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_auto_detect_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_auto_detect_cfg0`] module"]
pub type TVE_AUTO_DETECT_CFG0 = crate::Reg<tve_auto_detect_cfg0::TVE_AUTO_DETECT_CFG0_SPEC>;
#[doc = "TV Encoder Auto Detect Configuration Register0"]
pub mod tve_auto_detect_cfg0;
#[doc = "tve_auto_detect_cfg1 (rw) register accessor: TV Encoder Auto Detect Configuration Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_auto_detect_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_auto_detect_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_auto_detect_cfg1`] module"]
pub type TVE_AUTO_DETECT_CFG1 = crate::Reg<tve_auto_detect_cfg1::TVE_AUTO_DETECT_CFG1_SPEC>;
#[doc = "TV Encoder Auto Detect Configuration Register1"]
pub mod tve_auto_detect_cfg1;
#[doc = "tve_color_burst_phase_reset_cfg (rw) register accessor: TV Encoder Color Burst Phase Reset Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_color_burst_phase_reset_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_color_burst_phase_reset_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_color_burst_phase_reset_cfg`] module"]
pub type TVE_COLOR_BURST_PHASE_RESET_CFG =
    crate::Reg<tve_color_burst_phase_reset_cfg::TVE_COLOR_BURST_PHASE_RESET_CFG_SPEC>;
#[doc = "TV Encoder Color Burst Phase Reset Configuration Register"]
pub mod tve_color_burst_phase_reset_cfg;
#[doc = "tve_vsync_number (rw) register accessor: TV Encoder VSYNC Number Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_vsync_number::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_vsync_number::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_vsync_number`] module"]
pub type TVE_VSYNC_NUMBER = crate::Reg<tve_vsync_number::TVE_VSYNC_NUMBER_SPEC>;
#[doc = "TV Encoder VSYNC Number Register"]
pub mod tve_vsync_number;
#[doc = "tve_notch_filter_frequency (rw) register accessor: TV Encoder Notch Filter Frequency Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_notch_filter_frequency::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_notch_filter_frequency::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_notch_filter_frequency`] module"]
pub type TVE_NOTCH_FILTER_FREQUENCY =
    crate::Reg<tve_notch_filter_frequency::TVE_NOTCH_FILTER_FREQUENCY_SPEC>;
#[doc = "TV Encoder Notch Filter Frequency Register"]
pub mod tve_notch_filter_frequency;
#[doc = "tve_cbcr_level_gain (rw) register accessor: TV Encoder Cb/Cr Level/Gain Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_cbcr_level_gain::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_cbcr_level_gain::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_cbcr_level_gain`] module"]
pub type TVE_CBCR_LEVEL_GAIN = crate::Reg<tve_cbcr_level_gain::TVE_CBCR_LEVEL_GAIN_SPEC>;
#[doc = "TV Encoder Cb/Cr Level/Gain Register"]
pub mod tve_cbcr_level_gain;
#[doc = "tve_tint_color_burst_phase (rw) register accessor: TV Encoder Tint and Color Burst Phase Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_tint_color_burst_phase::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_tint_color_burst_phase::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_tint_color_burst_phase`] module"]
pub type TVE_TINT_COLOR_BURST_PHASE =
    crate::Reg<tve_tint_color_burst_phase::TVE_TINT_COLOR_BURST_PHASE_SPEC>;
#[doc = "TV Encoder Tint and Color Burst Phase Register"]
pub mod tve_tint_color_burst_phase;
#[doc = "tve_burst_width (rw) register accessor: TV Encoder Burst Width Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_burst_width::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_burst_width::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_burst_width`] module"]
pub type TVE_BURST_WIDTH = crate::Reg<tve_burst_width::TVE_BURST_WIDTH_SPEC>;
#[doc = "TV Encoder Burst Width Register"]
pub mod tve_burst_width;
#[doc = "tve_cbcr_gain (rw) register accessor: TV Encoder Cb/Cr Gain Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_cbcr_gain::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_cbcr_gain::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_cbcr_gain`] module"]
pub type TVE_CBCR_GAIN = crate::Reg<tve_cbcr_gain::TVE_CBCR_GAIN_SPEC>;
#[doc = "TV Encoder Cb/Cr Gain Register"]
pub mod tve_cbcr_gain;
#[doc = "tve_sync_vbi_level (rw) register accessor: TV Encoder Sync and VBI Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_sync_vbi_level::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_sync_vbi_level::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_sync_vbi_level`] module"]
pub type TVE_SYNC_VBI_LEVEL = crate::Reg<tve_sync_vbi_level::TVE_SYNC_VBI_LEVEL_SPEC>;
#[doc = "TV Encoder Sync and VBI Level Register"]
pub mod tve_sync_vbi_level;
#[doc = "tve_white_level (rw) register accessor: TV Encoder White Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_white_level::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_white_level::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_white_level`] module"]
pub type TVE_WHITE_LEVEL = crate::Reg<tve_white_level::TVE_WHITE_LEVEL_SPEC>;
#[doc = "TV Encoder White Level Register"]
pub mod tve_white_level;
#[doc = "tve_video_active_line (rw) register accessor: TV Encoder Video Active Line Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_video_active_line::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_video_active_line::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_video_active_line`] module"]
pub type TVE_VIDEO_ACTIVE_LINE = crate::Reg<tve_video_active_line::TVE_VIDEO_ACTIVE_LINE_SPEC>;
#[doc = "TV Encoder Video Active Line Register"]
pub mod tve_video_active_line;
#[doc = "tve_video_chroma_bw_comp_gain (rw) register accessor: TV Encoder Video Chroma BW and CompGain Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_video_chroma_bw_comp_gain::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_video_chroma_bw_comp_gain::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_video_chroma_bw_comp_gain`] module"]
pub type TVE_VIDEO_CHROMA_BW_COMP_GAIN =
    crate::Reg<tve_video_chroma_bw_comp_gain::TVE_VIDEO_CHROMA_BW_COMP_GAIN_SPEC>;
#[doc = "TV Encoder Video Chroma BW and CompGain Register"]
pub mod tve_video_chroma_bw_comp_gain;
#[doc = "tve_notch_width_comp_yuv_en (rw) register accessor: TV Encoder Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_notch_width_comp_yuv_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_notch_width_comp_yuv_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_notch_width_comp_yuv_en`] module"]
pub type TVE_NOTCH_WIDTH_COMP_YUV_EN =
    crate::Reg<tve_notch_width_comp_yuv_en::TVE_NOTCH_WIDTH_COMP_YUV_EN_SPEC>;
#[doc = "TV Encoder Register"]
pub mod tve_notch_width_comp_yuv_en;
#[doc = "tve_resync_parameters (rw) register accessor: TV Encoder Re-sync Parameters Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_resync_parameters::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_resync_parameters::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_resync_parameters`] module"]
pub type TVE_RESYNC_PARAMETERS = crate::Reg<tve_resync_parameters::TVE_RESYNC_PARAMETERS_SPEC>;
#[doc = "TV Encoder Re-sync Parameters Register"]
pub mod tve_resync_parameters;
#[doc = "tve_slave_parameter (rw) register accessor: TV Encoder Slave Parameter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_slave_parameter::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_slave_parameter::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_slave_parameter`] module"]
pub type TVE_SLAVE_PARAMETER = crate::Reg<tve_slave_parameter::TVE_SLAVE_PARAMETER_SPEC>;
#[doc = "TV Encoder Slave Parameter Register"]
pub mod tve_slave_parameter;
#[doc = "tve_configuration0 (rw) register accessor: TV Encoder Configuration Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_configuration0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_configuration0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_configuration0`] module"]
pub type TVE_CONFIGURATION0 = crate::Reg<tve_configuration0::TVE_CONFIGURATION0_SPEC>;
#[doc = "TV Encoder Configuration Register0"]
pub mod tve_configuration0;
#[doc = "tve_configuration1 (rw) register accessor: TV Encoder Configuration Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_configuration1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_configuration1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_configuration1`] module"]
pub type TVE_CONFIGURATION1 = crate::Reg<tve_configuration1::TVE_CONFIGURATION1_SPEC>;
#[doc = "TV Encoder Configuration Register1"]
pub mod tve_configuration1;
#[doc = "tve_low_pass_control (rw) register accessor: TV Encoder Low Pass Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_low_pass_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_low_pass_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_low_pass_control`] module"]
pub type TVE_LOW_PASS_CONTROL = crate::Reg<tve_low_pass_control::TVE_LOW_PASS_CONTROL_SPEC>;
#[doc = "TV Encoder Low Pass Control Register"]
pub mod tve_low_pass_control;
#[doc = "tve_low_pass_filter_control (rw) register accessor: TV Encoder Low Pass Filter Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_low_pass_filter_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_low_pass_filter_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_low_pass_filter_control`] module"]
pub type TVE_LOW_PASS_FILTER_CONTROL =
    crate::Reg<tve_low_pass_filter_control::TVE_LOW_PASS_FILTER_CONTROL_SPEC>;
#[doc = "TV Encoder Low Pass Filter Control Register"]
pub mod tve_low_pass_filter_control;
#[doc = "tve_low_pass_gain (rw) register accessor: TV Encoder Low Pass Gain Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_low_pass_gain::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_low_pass_gain::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_low_pass_gain`] module"]
pub type TVE_LOW_PASS_GAIN = crate::Reg<tve_low_pass_gain::TVE_LOW_PASS_GAIN_SPEC>;
#[doc = "TV Encoder Low Pass Gain Register"]
pub mod tve_low_pass_gain;
#[doc = "tve_low_pass_gain_control (rw) register accessor: TV Encoder Low Pass Gain Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_low_pass_gain_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_low_pass_gain_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_low_pass_gain_control`] module"]
pub type TVE_LOW_PASS_GAIN_CONTROL =
    crate::Reg<tve_low_pass_gain_control::TVE_LOW_PASS_GAIN_CONTROL_SPEC>;
#[doc = "TV Encoder Low Pass Gain Control Register"]
pub mod tve_low_pass_gain_control;
#[doc = "tve_low_pass_shoot_control (rw) register accessor: TV Encoder Low Pass Shoot Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_low_pass_shoot_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_low_pass_shoot_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_low_pass_shoot_control`] module"]
pub type TVE_LOW_PASS_SHOOT_CONTROL =
    crate::Reg<tve_low_pass_shoot_control::TVE_LOW_PASS_SHOOT_CONTROL_SPEC>;
#[doc = "TV Encoder Low Pass Shoot Control Register"]
pub mod tve_low_pass_shoot_control;
#[doc = "tve_low_pass_coring (rw) register accessor: TV Encoder Low Pass Coring Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_low_pass_coring::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_low_pass_coring::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_low_pass_coring`] module"]
pub type TVE_LOW_PASS_CORING = crate::Reg<tve_low_pass_coring::TVE_LOW_PASS_CORING_SPEC>;
#[doc = "TV Encoder Low Pass Coring Register"]
pub mod tve_low_pass_coring;
#[doc = "tve_noise_reduction (rw) register accessor: TV Encoder Noise Reduction Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_noise_reduction::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_noise_reduction::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_noise_reduction`] module"]
pub type TVE_NOISE_REDUCTION = crate::Reg<tve_noise_reduction::TVE_NOISE_REDUCTION_SPEC>;
#[doc = "TV Encoder Noise Reduction Register"]
pub mod tve_noise_reduction;
