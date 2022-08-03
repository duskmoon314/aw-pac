#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TV Encoder Clock Gating Register"]
    pub tve_clock_gating: TVE_CLOCK_GATING,
    #[doc = "0x04 - TV Encoder Configuration Register"]
    pub tve_configuration: TVE_CONFIGURATION,
    #[doc = "0x08 - TV Encoder DAC Register1"]
    pub tve_dac1: TVE_DAC1,
    #[doc = "0x0c - TV Encoder Notch and DAC Delay Register"]
    pub tve_notch_dac_delay: TVE_NOTCH_DAC_DELAY,
    #[doc = "0x10 - TV Encoder Chroma Frequency Register"]
    pub tve_chroma_frequency: TVE_CHROMA_FREQUENCY,
    #[doc = "0x14 - TV Encoder Front/Back Porch Register"]
    pub tve_front_back_porch: TVE_FRONT_BACK_PORCH,
    #[doc = "0x18 - TV Encoder HD Mode VSYNC Register"]
    pub tve_hd_vsync: TVE_HD_VSYNC,
    #[doc = "0x1c - TV Encoder Line Number Register"]
    pub tve_line_number: TVE_LINE_NUMBER,
    #[doc = "0x20 - TV Encoder Level Register"]
    pub tve_level: TVE_LEVEL,
    #[doc = "0x24 - TV Encoder DAC Register2"]
    pub tve_dac2: TVE_DAC2,
    _reserved10: [u8; 0x08],
    #[doc = "0x30 - TV Encoder Auto Detection Enable Register"]
    pub tve_auto_detection_enable: TVE_AUTO_DETECTION_ENABLE,
    #[doc = "0x34 - TV Encoder Auto Detection Interrupt Status Register"]
    pub tve_auto_detection_interrupt_status: TVE_AUTO_DETECTION_INTERRUPT_STATUS,
    #[doc = "0x38 - TV Encoder Auto Detection Status Register"]
    pub tve_auto_detection_status: TVE_AUTO_DETECTION_STATUS,
    #[doc = "0x3c - TV Encoder Auto Detection De-bounce Setting Register"]
    pub tve_auto_detection_debounce_setting: TVE_AUTO_DETECTION_DEBOUNCE_SETTING,
    _reserved14: [u8; 0xb8],
    #[doc = "0xf8 - TV Encoder Auto Detect Configuration Register0"]
    pub tve_auto_detect_cfg0: TVE_AUTO_DETECT_CFG0,
    #[doc = "0xfc - TV Encoder Auto Detect Configuration Register1"]
    pub tve_auto_detect_cfg1: TVE_AUTO_DETECT_CFG1,
    #[doc = "0x100 - TV Encoder Color Burst Phase Reset Configuration Register"]
    pub tve_color_burst_phase_reset_cfg: TVE_COLOR_BURST_PHASE_RESET_CFG,
    #[doc = "0x104 - TV Encoder VSYNC Number Register"]
    pub tve_vsync_number: TVE_VSYNC_NUMBER,
    #[doc = "0x108 - TV Encoder Notch Filter Frequency Register"]
    pub tve_notch_filter_frequency: TVE_NOTCH_FILTER_FREQUENCY,
    #[doc = "0x10c - TV Encoder Cb/Cr Level/Gain Register"]
    pub tve_cbcr_level_gain: TVE_CBCR_LEVEL_GAIN,
    #[doc = "0x110 - TV Encoder Tint and Color Burst Phase Register"]
    pub tve_tint_color_burst_phase: TVE_TINT_COLOR_BURST_PHASE,
    #[doc = "0x114 - TV Encoder Burst Width Register"]
    pub tve_burst_width: TVE_BURST_WIDTH,
    #[doc = "0x118 - TV Encoder Cb/Cr Gain Register"]
    pub tve_cbcr_gain: TVE_CBCR_GAIN,
    #[doc = "0x11c - TV Encoder Sync and VBI Level Register"]
    pub tve_sync_vbi_level: TVE_SYNC_VBI_LEVEL,
    #[doc = "0x120 - TV Encoder White Level Register"]
    pub tve_white_level: TVE_WHITE_LEVEL,
    #[doc = "0x124 - TV Encoder Video Active Line Register"]
    pub tve_video_active_line: TVE_VIDEO_ACTIVE_LINE,
    #[doc = "0x128 - TV Encoder Video Chroma BW and CompGain Register"]
    pub tve_video_chroma_bw_comp_gain: TVE_VIDEO_CHROMA_BW_COMP_GAIN,
    #[doc = "0x12c - TV Encoder Register"]
    pub tve_notch_width_comp_yuv_en: TVE_NOTCH_WIDTH_COMP_YUV_EN,
    #[doc = "0x130 - TV Encoder Re-sync Parameters Register"]
    pub tve_resync_parameters: TVE_RESYNC_PARAMETERS,
    #[doc = "0x134 - TV Encoder Slave Parameter Register"]
    pub tve_slave_parameter: TVE_SLAVE_PARAMETER,
    #[doc = "0x138 - TV Encoder Configuration Register0"]
    pub tve_configuration0: TVE_CONFIGURATION0,
    #[doc = "0x13c - TV Encoder Configuration Register1"]
    pub tve_configuration1: TVE_CONFIGURATION1,
    _reserved32: [u8; 0x0240],
    #[doc = "0x380 - TV Encoder Low Pass Control Register"]
    pub tve_low_pass_control: TVE_LOW_PASS_CONTROL,
    #[doc = "0x384 - TV Encoder Low Pass Filter Control Register"]
    pub tve_low_pass_filter_control: TVE_LOW_PASS_FILTER_CONTROL,
    #[doc = "0x388 - TV Encoder Low Pass Gain Register"]
    pub tve_low_pass_gain: TVE_LOW_PASS_GAIN,
    #[doc = "0x38c - TV Encoder Low Pass Gain Control Register"]
    pub tve_low_pass_gain_control: TVE_LOW_PASS_GAIN_CONTROL,
    #[doc = "0x390 - TV Encoder Low Pass Shoot Control Register"]
    pub tve_low_pass_shoot_control: TVE_LOW_PASS_SHOOT_CONTROL,
    #[doc = "0x394 - TV Encoder Low Pass Coring Register"]
    pub tve_low_pass_coring: TVE_LOW_PASS_CORING,
    _reserved38: [u8; 0x08],
    #[doc = "0x3a0 - TV Encoder Noise Reduction Register"]
    pub tve_noise_reduction: TVE_NOISE_REDUCTION,
}
#[doc = "tve_clock_gating (rw) register accessor: an alias for `Reg<TVE_CLOCK_GATING_SPEC>`"]
pub type TVE_CLOCK_GATING = crate::Reg<tve_clock_gating::TVE_CLOCK_GATING_SPEC>;
#[doc = "TV Encoder Clock Gating Register"]
pub mod tve_clock_gating;
#[doc = "tve_configuration (rw) register accessor: an alias for `Reg<TVE_CONFIGURATION_SPEC>`"]
pub type TVE_CONFIGURATION = crate::Reg<tve_configuration::TVE_CONFIGURATION_SPEC>;
#[doc = "TV Encoder Configuration Register"]
pub mod tve_configuration;
#[doc = "tve_dac1 (rw) register accessor: an alias for `Reg<TVE_DAC1_SPEC>`"]
pub type TVE_DAC1 = crate::Reg<tve_dac1::TVE_DAC1_SPEC>;
#[doc = "TV Encoder DAC Register1"]
pub mod tve_dac1;
#[doc = "tve_notch_dac_delay (rw) register accessor: an alias for `Reg<TVE_NOTCH_DAC_DELAY_SPEC>`"]
pub type TVE_NOTCH_DAC_DELAY = crate::Reg<tve_notch_dac_delay::TVE_NOTCH_DAC_DELAY_SPEC>;
#[doc = "TV Encoder Notch and DAC Delay Register"]
pub mod tve_notch_dac_delay;
#[doc = "tve_chroma_frequency (rw) register accessor: an alias for `Reg<TVE_CHROMA_FREQUENCY_SPEC>`"]
pub type TVE_CHROMA_FREQUENCY = crate::Reg<tve_chroma_frequency::TVE_CHROMA_FREQUENCY_SPEC>;
#[doc = "TV Encoder Chroma Frequency Register"]
pub mod tve_chroma_frequency;
#[doc = "tve_front_back_porch (rw) register accessor: an alias for `Reg<TVE_FRONT_BACK_PORCH_SPEC>`"]
pub type TVE_FRONT_BACK_PORCH = crate::Reg<tve_front_back_porch::TVE_FRONT_BACK_PORCH_SPEC>;
#[doc = "TV Encoder Front/Back Porch Register"]
pub mod tve_front_back_porch;
#[doc = "tve_hd_vsync (rw) register accessor: an alias for `Reg<TVE_HD_VSYNC_SPEC>`"]
pub type TVE_HD_VSYNC = crate::Reg<tve_hd_vsync::TVE_HD_VSYNC_SPEC>;
#[doc = "TV Encoder HD Mode VSYNC Register"]
pub mod tve_hd_vsync;
#[doc = "tve_line_number (rw) register accessor: an alias for `Reg<TVE_LINE_NUMBER_SPEC>`"]
pub type TVE_LINE_NUMBER = crate::Reg<tve_line_number::TVE_LINE_NUMBER_SPEC>;
#[doc = "TV Encoder Line Number Register"]
pub mod tve_line_number;
#[doc = "tve_level (rw) register accessor: an alias for `Reg<TVE_LEVEL_SPEC>`"]
pub type TVE_LEVEL = crate::Reg<tve_level::TVE_LEVEL_SPEC>;
#[doc = "TV Encoder Level Register"]
pub mod tve_level;
#[doc = "tve_dac2 (rw) register accessor: an alias for `Reg<TVE_DAC2_SPEC>`"]
pub type TVE_DAC2 = crate::Reg<tve_dac2::TVE_DAC2_SPEC>;
#[doc = "TV Encoder DAC Register2"]
pub mod tve_dac2;
#[doc = "tve_auto_detection_enable (rw) register accessor: an alias for `Reg<TVE_AUTO_DETECTION_ENABLE_SPEC>`"]
pub type TVE_AUTO_DETECTION_ENABLE =
    crate::Reg<tve_auto_detection_enable::TVE_AUTO_DETECTION_ENABLE_SPEC>;
#[doc = "TV Encoder Auto Detection Enable Register"]
pub mod tve_auto_detection_enable;
#[doc = "tve_auto_detection_interrupt_status (rw) register accessor: an alias for `Reg<TVE_AUTO_DETECTION_INTERRUPT_STATUS_SPEC>`"]
pub type TVE_AUTO_DETECTION_INTERRUPT_STATUS =
    crate::Reg<tve_auto_detection_interrupt_status::TVE_AUTO_DETECTION_INTERRUPT_STATUS_SPEC>;
#[doc = "TV Encoder Auto Detection Interrupt Status Register"]
pub mod tve_auto_detection_interrupt_status;
#[doc = "tve_auto_detection_status (rw) register accessor: an alias for `Reg<TVE_AUTO_DETECTION_STATUS_SPEC>`"]
pub type TVE_AUTO_DETECTION_STATUS =
    crate::Reg<tve_auto_detection_status::TVE_AUTO_DETECTION_STATUS_SPEC>;
#[doc = "TV Encoder Auto Detection Status Register"]
pub mod tve_auto_detection_status;
#[doc = "tve_auto_detection_debounce_setting (rw) register accessor: an alias for `Reg<TVE_AUTO_DETECTION_DEBOUNCE_SETTING_SPEC>`"]
pub type TVE_AUTO_DETECTION_DEBOUNCE_SETTING =
    crate::Reg<tve_auto_detection_debounce_setting::TVE_AUTO_DETECTION_DEBOUNCE_SETTING_SPEC>;
#[doc = "TV Encoder Auto Detection De-bounce Setting Register"]
pub mod tve_auto_detection_debounce_setting;
#[doc = "tve_auto_detect_cfg0 (rw) register accessor: an alias for `Reg<TVE_AUTO_DETECT_CFG0_SPEC>`"]
pub type TVE_AUTO_DETECT_CFG0 = crate::Reg<tve_auto_detect_cfg0::TVE_AUTO_DETECT_CFG0_SPEC>;
#[doc = "TV Encoder Auto Detect Configuration Register0"]
pub mod tve_auto_detect_cfg0;
#[doc = "tve_auto_detect_cfg1 (rw) register accessor: an alias for `Reg<TVE_AUTO_DETECT_CFG1_SPEC>`"]
pub type TVE_AUTO_DETECT_CFG1 = crate::Reg<tve_auto_detect_cfg1::TVE_AUTO_DETECT_CFG1_SPEC>;
#[doc = "TV Encoder Auto Detect Configuration Register1"]
pub mod tve_auto_detect_cfg1;
#[doc = "tve_color_burst_phase_reset_cfg (rw) register accessor: an alias for `Reg<TVE_COLOR_BURST_PHASE_RESET_CFG_SPEC>`"]
pub type TVE_COLOR_BURST_PHASE_RESET_CFG =
    crate::Reg<tve_color_burst_phase_reset_cfg::TVE_COLOR_BURST_PHASE_RESET_CFG_SPEC>;
#[doc = "TV Encoder Color Burst Phase Reset Configuration Register"]
pub mod tve_color_burst_phase_reset_cfg;
#[doc = "tve_vsync_number (rw) register accessor: an alias for `Reg<TVE_VSYNC_NUMBER_SPEC>`"]
pub type TVE_VSYNC_NUMBER = crate::Reg<tve_vsync_number::TVE_VSYNC_NUMBER_SPEC>;
#[doc = "TV Encoder VSYNC Number Register"]
pub mod tve_vsync_number;
#[doc = "tve_notch_filter_frequency (rw) register accessor: an alias for `Reg<TVE_NOTCH_FILTER_FREQUENCY_SPEC>`"]
pub type TVE_NOTCH_FILTER_FREQUENCY =
    crate::Reg<tve_notch_filter_frequency::TVE_NOTCH_FILTER_FREQUENCY_SPEC>;
#[doc = "TV Encoder Notch Filter Frequency Register"]
pub mod tve_notch_filter_frequency;
#[doc = "tve_cbcr_level_gain (rw) register accessor: an alias for `Reg<TVE_CBCR_LEVEL_GAIN_SPEC>`"]
pub type TVE_CBCR_LEVEL_GAIN = crate::Reg<tve_cbcr_level_gain::TVE_CBCR_LEVEL_GAIN_SPEC>;
#[doc = "TV Encoder Cb/Cr Level/Gain Register"]
pub mod tve_cbcr_level_gain;
#[doc = "tve_tint_color_burst_phase (rw) register accessor: an alias for `Reg<TVE_TINT_COLOR_BURST_PHASE_SPEC>`"]
pub type TVE_TINT_COLOR_BURST_PHASE =
    crate::Reg<tve_tint_color_burst_phase::TVE_TINT_COLOR_BURST_PHASE_SPEC>;
#[doc = "TV Encoder Tint and Color Burst Phase Register"]
pub mod tve_tint_color_burst_phase;
#[doc = "tve_burst_width (rw) register accessor: an alias for `Reg<TVE_BURST_WIDTH_SPEC>`"]
pub type TVE_BURST_WIDTH = crate::Reg<tve_burst_width::TVE_BURST_WIDTH_SPEC>;
#[doc = "TV Encoder Burst Width Register"]
pub mod tve_burst_width;
#[doc = "tve_cbcr_gain (rw) register accessor: an alias for `Reg<TVE_CBCR_GAIN_SPEC>`"]
pub type TVE_CBCR_GAIN = crate::Reg<tve_cbcr_gain::TVE_CBCR_GAIN_SPEC>;
#[doc = "TV Encoder Cb/Cr Gain Register"]
pub mod tve_cbcr_gain;
#[doc = "tve_sync_vbi_level (rw) register accessor: an alias for `Reg<TVE_SYNC_VBI_LEVEL_SPEC>`"]
pub type TVE_SYNC_VBI_LEVEL = crate::Reg<tve_sync_vbi_level::TVE_SYNC_VBI_LEVEL_SPEC>;
#[doc = "TV Encoder Sync and VBI Level Register"]
pub mod tve_sync_vbi_level;
#[doc = "tve_white_level (rw) register accessor: an alias for `Reg<TVE_WHITE_LEVEL_SPEC>`"]
pub type TVE_WHITE_LEVEL = crate::Reg<tve_white_level::TVE_WHITE_LEVEL_SPEC>;
#[doc = "TV Encoder White Level Register"]
pub mod tve_white_level;
#[doc = "tve_video_active_line (rw) register accessor: an alias for `Reg<TVE_VIDEO_ACTIVE_LINE_SPEC>`"]
pub type TVE_VIDEO_ACTIVE_LINE = crate::Reg<tve_video_active_line::TVE_VIDEO_ACTIVE_LINE_SPEC>;
#[doc = "TV Encoder Video Active Line Register"]
pub mod tve_video_active_line;
#[doc = "tve_video_chroma_bw_comp_gain (rw) register accessor: an alias for `Reg<TVE_VIDEO_CHROMA_BW_COMP_GAIN_SPEC>`"]
pub type TVE_VIDEO_CHROMA_BW_COMP_GAIN =
    crate::Reg<tve_video_chroma_bw_comp_gain::TVE_VIDEO_CHROMA_BW_COMP_GAIN_SPEC>;
#[doc = "TV Encoder Video Chroma BW and CompGain Register"]
pub mod tve_video_chroma_bw_comp_gain;
#[doc = "tve_notch_width_comp_yuv_en (rw) register accessor: an alias for `Reg<TVE_NOTCH_WIDTH_COMP_YUV_EN_SPEC>`"]
pub type TVE_NOTCH_WIDTH_COMP_YUV_EN =
    crate::Reg<tve_notch_width_comp_yuv_en::TVE_NOTCH_WIDTH_COMP_YUV_EN_SPEC>;
#[doc = "TV Encoder Register"]
pub mod tve_notch_width_comp_yuv_en;
#[doc = "tve_resync_parameters (rw) register accessor: an alias for `Reg<TVE_RESYNC_PARAMETERS_SPEC>`"]
pub type TVE_RESYNC_PARAMETERS = crate::Reg<tve_resync_parameters::TVE_RESYNC_PARAMETERS_SPEC>;
#[doc = "TV Encoder Re-sync Parameters Register"]
pub mod tve_resync_parameters;
#[doc = "tve_slave_parameter (rw) register accessor: an alias for `Reg<TVE_SLAVE_PARAMETER_SPEC>`"]
pub type TVE_SLAVE_PARAMETER = crate::Reg<tve_slave_parameter::TVE_SLAVE_PARAMETER_SPEC>;
#[doc = "TV Encoder Slave Parameter Register"]
pub mod tve_slave_parameter;
#[doc = "tve_configuration0 (rw) register accessor: an alias for `Reg<TVE_CONFIGURATION0_SPEC>`"]
pub type TVE_CONFIGURATION0 = crate::Reg<tve_configuration0::TVE_CONFIGURATION0_SPEC>;
#[doc = "TV Encoder Configuration Register0"]
pub mod tve_configuration0;
#[doc = "tve_configuration1 (rw) register accessor: an alias for `Reg<TVE_CONFIGURATION1_SPEC>`"]
pub type TVE_CONFIGURATION1 = crate::Reg<tve_configuration1::TVE_CONFIGURATION1_SPEC>;
#[doc = "TV Encoder Configuration Register1"]
pub mod tve_configuration1;
#[doc = "tve_low_pass_control (rw) register accessor: an alias for `Reg<TVE_LOW_PASS_CONTROL_SPEC>`"]
pub type TVE_LOW_PASS_CONTROL = crate::Reg<tve_low_pass_control::TVE_LOW_PASS_CONTROL_SPEC>;
#[doc = "TV Encoder Low Pass Control Register"]
pub mod tve_low_pass_control;
#[doc = "tve_low_pass_filter_control (rw) register accessor: an alias for `Reg<TVE_LOW_PASS_FILTER_CONTROL_SPEC>`"]
pub type TVE_LOW_PASS_FILTER_CONTROL =
    crate::Reg<tve_low_pass_filter_control::TVE_LOW_PASS_FILTER_CONTROL_SPEC>;
#[doc = "TV Encoder Low Pass Filter Control Register"]
pub mod tve_low_pass_filter_control;
#[doc = "tve_low_pass_gain (rw) register accessor: an alias for `Reg<TVE_LOW_PASS_GAIN_SPEC>`"]
pub type TVE_LOW_PASS_GAIN = crate::Reg<tve_low_pass_gain::TVE_LOW_PASS_GAIN_SPEC>;
#[doc = "TV Encoder Low Pass Gain Register"]
pub mod tve_low_pass_gain;
#[doc = "tve_low_pass_gain_control (rw) register accessor: an alias for `Reg<TVE_LOW_PASS_GAIN_CONTROL_SPEC>`"]
pub type TVE_LOW_PASS_GAIN_CONTROL =
    crate::Reg<tve_low_pass_gain_control::TVE_LOW_PASS_GAIN_CONTROL_SPEC>;
#[doc = "TV Encoder Low Pass Gain Control Register"]
pub mod tve_low_pass_gain_control;
#[doc = "tve_low_pass_shoot_control (rw) register accessor: an alias for `Reg<TVE_LOW_PASS_SHOOT_CONTROL_SPEC>`"]
pub type TVE_LOW_PASS_SHOOT_CONTROL =
    crate::Reg<tve_low_pass_shoot_control::TVE_LOW_PASS_SHOOT_CONTROL_SPEC>;
#[doc = "TV Encoder Low Pass Shoot Control Register"]
pub mod tve_low_pass_shoot_control;
#[doc = "tve_low_pass_coring (rw) register accessor: an alias for `Reg<TVE_LOW_PASS_CORING_SPEC>`"]
pub type TVE_LOW_PASS_CORING = crate::Reg<tve_low_pass_coring::TVE_LOW_PASS_CORING_SPEC>;
#[doc = "TV Encoder Low Pass Coring Register"]
pub mod tve_low_pass_coring;
#[doc = "tve_noise_reduction (rw) register accessor: an alias for `Reg<TVE_NOISE_REDUCTION_SPEC>`"]
pub type TVE_NOISE_REDUCTION = crate::Reg<tve_noise_reduction::TVE_NOISE_REDUCTION_SPEC>;
#[doc = "TV Encoder Noise Reduction Register"]
pub mod tve_noise_reduction;
