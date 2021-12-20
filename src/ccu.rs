#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PLL_CPU Control Register"]
    pub pll_cpu_ctrl: crate::Reg<pll_cpu_ctrl::PLL_CPU_CTRL_SPEC>,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - PLL_DDR Control Register"]
    pub pll_ddr_ctrl: crate::Reg<pll_ddr_ctrl::PLL_DDR_CTRL_SPEC>,
    _reserved2: [u8; 0x0c],
    #[doc = "0x20 - PLL_PERI Control Register"]
    pub pll_peri_ctrl: crate::Reg<pll_peri_ctrl::PLL_PERI_CTRL_SPEC>,
    _reserved3: [u8; 0x1c],
    #[doc = "0x40 - PLL_VIDEO0 Control Register"]
    pub pll_video0_ctrl: crate::Reg<pll_video0_ctrl::PLL_VIDEO0_CTRL_SPEC>,
    _reserved4: [u8; 0x04],
    #[doc = "0x48 - PLL_VIDEO1 Control Register"]
    pub pll_video1_ctrl: crate::Reg<pll_video1_ctrl::PLL_VIDEO1_CTRL_SPEC>,
    _reserved5: [u8; 0x0c],
    #[doc = "0x58 - PLL_VE Control Register"]
    pub pll_ve_ctrl: crate::Reg<pll_ve_ctrl::PLL_VE_CTRL_SPEC>,
    _reserved6: [u8; 0x1c],
    #[doc = "0x78 - PLL_AUDIO0 Control Register"]
    pub pll_audio0_ctrl: crate::Reg<pll_audio0_ctrl::PLL_AUDIO0_CTRL_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x80 - PLL_AUDIO1 Control Register"]
    pub pll_audio1_ctrl: crate::Reg<pll_audio1_ctrl::PLL_AUDIO1_CTRL_SPEC>,
    _reserved8: [u8; 0x8c],
    #[doc = "0x110 - PLL_DDR Pattern0 Control Register"]
    pub pll_ddr_pat0_ctrl: crate::Reg<pll_ddr_pat0_ctrl::PLL_DDR_PAT0_CTRL_SPEC>,
    #[doc = "0x114 - PLL_DDR Pattern1 Control Register"]
    pub pll_ddr_pat1_ctrl: crate::Reg<pll_ddr_pat1_ctrl::PLL_DDR_PAT1_CTRL_SPEC>,
    _reserved10: [u8; 0x08],
    #[doc = "0x120 - PLL_PERI Pattern0 Control Register"]
    pub pll_peri_pat0_ctrl: crate::Reg<pll_peri_pat0_ctrl::PLL_PERI_PAT0_CTRL_SPEC>,
    #[doc = "0x124 - PLL_PERI Pattern1 Control Register"]
    pub pll_peri_pat1_ctrl: crate::Reg<pll_peri_pat1_ctrl::PLL_PERI_PAT1_CTRL_SPEC>,
    _reserved12: [u8; 0x18],
    #[doc = "0x140 - PLL_VIDEO0 Pattern0 Control Register"]
    pub pll_video0_pat0_ctrl: crate::Reg<pll_video0_pat0_ctrl::PLL_VIDEO0_PAT0_CTRL_SPEC>,
    #[doc = "0x144 - PLL_VIDEO0 Pattern1 Control Register"]
    pub pll_video0_pat1_ctrl: crate::Reg<pll_video0_pat1_ctrl::PLL_VIDEO0_PAT1_CTRL_SPEC>,
    #[doc = "0x148 - PLL_VIDEO1 Pattern0 Control Register"]
    pub pll_video1_pat0_ctrl: crate::Reg<pll_video1_pat0_ctrl::PLL_VIDEO1_PAT0_CTRL_SPEC>,
    #[doc = "0x14c - PLL_VIDEO1 Pattern1 Control Register"]
    pub pll_video1_pat1_ctrl: crate::Reg<pll_video1_pat1_ctrl::PLL_VIDEO1_PAT1_CTRL_SPEC>,
    _reserved16: [u8; 0x08],
    #[doc = "0x158 - PLL_VE Pattern0 Control Register"]
    pub pll_ve_pat0_ctrl: crate::Reg<pll_ve_pat0_ctrl::PLL_VE_PAT0_CTRL_SPEC>,
    #[doc = "0x15c - PLL_VE Pattern1 Control Register"]
    pub pll_ve_pat1_ctrl: crate::Reg<pll_ve_pat1_ctrl::PLL_VE_PAT1_CTRL_SPEC>,
    _reserved18: [u8; 0x18],
    #[doc = "0x178 - PLL_AUDIO0 Pattern0 Control Register"]
    pub pll_audio0_pat0_ctrl: crate::Reg<pll_audio0_pat0_ctrl::PLL_AUDIO0_PAT0_CTRL_SPEC>,
    #[doc = "0x17c - PLL_AUDIO0 Pattern1 Control Register"]
    pub pll_audio0_pat1_ctrl: crate::Reg<pll_audio0_pat1_ctrl::PLL_AUDIO0_PAT1_CTRL_SPEC>,
    #[doc = "0x180 - PLL_AUDIO1 Pattern0 Control Register"]
    pub pll_audio1_pat0_ctrl: crate::Reg<pll_audio1_pat0_ctrl::PLL_AUDIO1_PAT0_CTRL_SPEC>,
    #[doc = "0x184 - PLL_AUDIO1 Pattern1 Control Register"]
    pub pll_audio1_pat1_ctrl: crate::Reg<pll_audio1_pat1_ctrl::PLL_AUDIO1_PAT1_CTRL_SPEC>,
    _reserved22: [u8; 0x0178],
    #[doc = "0x300 - PLL_CPU Bias Register"]
    pub pll_cpu_bias: crate::Reg<pll_cpu_bias::PLL_CPU_BIAS_SPEC>,
    _reserved23: [u8; 0x0c],
    #[doc = "0x310 - PLL_DDR Bias Register"]
    pub pll_ddr_bias: crate::Reg<pll_ddr_bias::PLL_DDR_BIAS_SPEC>,
    _reserved24: [u8; 0x0c],
    #[doc = "0x320 - PLL_PERI Bias Register"]
    pub pll_peri_bias: crate::Reg<pll_peri_bias::PLL_PERI_BIAS_SPEC>,
    _reserved25: [u8; 0x1c],
    #[doc = "0x340 - PLL_VIDEO0 Bias Register"]
    pub pll_video0_bias: crate::Reg<pll_video0_bias::PLL_VIDEO0_BIAS_SPEC>,
    _reserved26: [u8; 0x04],
    #[doc = "0x348 - PLL_VIDEO1 Bias Register"]
    pub pll_video1_bias: crate::Reg<pll_video1_bias::PLL_VIDEO1_BIAS_SPEC>,
    _reserved27: [u8; 0x0c],
    #[doc = "0x358 - PLL_VE Bias Register"]
    pub pll_ve_bias: crate::Reg<pll_ve_bias::PLL_VE_BIAS_SPEC>,
    _reserved28: [u8; 0x1c],
    #[doc = "0x378 - PLL_AUDIO0 Bias Register"]
    pub pll_audio0_bias: crate::Reg<pll_audio0_bias::PLL_AUDIO0_BIAS_SPEC>,
    _reserved29: [u8; 0x04],
    #[doc = "0x380 - PLL_AUDIO1 Bias Register"]
    pub pll_audio1_bias: crate::Reg<pll_audio1_bias::PLL_AUDIO1_BIAS_SPEC>,
    _reserved30: [u8; 0x7c],
    #[doc = "0x400 - PLL_CPU Tuning Register"]
    pub pll_cpu_tun: crate::Reg<pll_cpu_tun::PLL_CPU_TUN_SPEC>,
    _reserved31: [u8; 0xfc],
    #[doc = "0x500 - CPU_AXI Configuration Register"]
    pub cpu_axi_cfg: crate::Reg<cpu_axi_cfg::CPU_AXI_CFG_SPEC>,
    #[doc = "0x504 - CPU_GATING Configuration Register"]
    pub cpu_gating: crate::Reg<cpu_gating::CPU_GATING_SPEC>,
    _reserved33: [u8; 0x08],
    #[doc = "0x510 - PSI Clock Register"]
    pub psi_clk: crate::Reg<psi_clk::PSI_CLK_SPEC>,
    _reserved34: [u8; 0x0c],
    #[doc = "0x520 - APB0 Clock Register"]
    pub apb0_clk: crate::Reg<apb0_clk::APB0_CLK_SPEC>,
    #[doc = "0x524 - APB1 Clock Register"]
    pub apb1_clk: crate::Reg<apb1_clk::APB1_CLK_SPEC>,
    _reserved36: [u8; 0x18],
    #[doc = "0x540 - MBUS Clock Register"]
    pub mbus_clk: crate::Reg<mbus_clk::MBUS_CLK_SPEC>,
    _reserved37: [u8; 0xbc],
    #[doc = "0x600 - DE Clock Register"]
    pub de_clk: crate::Reg<de_clk::DE_CLK_SPEC>,
    _reserved38: [u8; 0x08],
    #[doc = "0x60c - DE Bus Gating Reset Register"]
    pub de_bgr: crate::Reg<de_bgr::DE_BGR_SPEC>,
    _reserved39: [u8; 0x10],
    #[doc = "0x620 - DI Clock Register"]
    pub di_clk: crate::Reg<di_clk::DI_CLK_SPEC>,
    _reserved40: [u8; 0x08],
    #[doc = "0x62c - DI Bus Gating Reset Register"]
    pub di_bgr: crate::Reg<di_bgr::DI_BGR_SPEC>,
    #[doc = "0x630 - G2D Clock Register"]
    pub g2d_clk: crate::Reg<g2d_clk::G2D_CLK_SPEC>,
    _reserved42: [u8; 0x08],
    #[doc = "0x63c - G2D Bus Gating Reset Register"]
    pub g2d_bgr: crate::Reg<g2d_bgr::G2D_BGR_SPEC>,
    _reserved43: [u8; 0x40],
    #[doc = "0x680 - CE Clock Register"]
    pub ce_clk: crate::Reg<ce_clk::CE_CLK_SPEC>,
    _reserved44: [u8; 0x08],
    #[doc = "0x68c - CE Bus Gating Reset Register"]
    pub ce_bgr: crate::Reg<ce_bgr::CE_BGR_SPEC>,
    #[doc = "0x690 - VE Clock Register"]
    pub ve_clk: crate::Reg<ve_clk::VE_CLK_SPEC>,
    _reserved46: [u8; 0x08],
    #[doc = "0x69c - VE Bus Gating Reset Register"]
    pub ve_bgr: crate::Reg<ve_bgr::VE_BGR_SPEC>,
    _reserved47: [u8; 0x6c],
    #[doc = "0x70c - DMA Bus Gating Reset Register"]
    pub dma_bgr: crate::Reg<dma_bgr::DMA_BGR_SPEC>,
    _reserved48: [u8; 0x0c],
    #[doc = "0x71c - MSGBOX Bus Gating Reset Register"]
    pub msgbox_bgr: crate::Reg<msgbox_bgr::MSGBOX_BGR_SPEC>,
    _reserved49: [u8; 0x0c],
    #[doc = "0x72c - SPINLOCK Bus Gating Reset Register"]
    pub spinlock_bgr: crate::Reg<spinlock_bgr::SPINLOCK_BGR_SPEC>,
    _reserved50: [u8; 0x0c],
    #[doc = "0x73c - HSTIMER Bus Gating Reset Register"]
    pub hstimer_bgr: crate::Reg<hstimer_bgr::HSTIMER_BGR_SPEC>,
    #[doc = "0x740 - AVS Clock Register"]
    pub avs_clk: crate::Reg<avs_clk::AVS_CLK_SPEC>,
    _reserved52: [u8; 0x48],
    #[doc = "0x78c - DBGSYS Bus Gating Reset Register"]
    pub dbgsys_bgr: crate::Reg<dbgsys_bgr::DBGSYS_BGR_SPEC>,
    _reserved53: [u8; 0x1c],
    #[doc = "0x7ac - PWM Bus Gating Reset Register"]
    pub pwm_bgr: crate::Reg<pwm_bgr::PWM_BGR_SPEC>,
    _reserved54: [u8; 0x0c],
    #[doc = "0x7bc - IOMMU Bus Gating Reset Register"]
    pub iommu_bgr: crate::Reg<iommu_bgr::IOMMU_BGR_SPEC>,
    _reserved55: [u8; 0x40],
    #[doc = "0x800 - DRAM Clock Register"]
    pub dram_clk: crate::Reg<dram_clk::DRAM_CLK_SPEC>,
    #[doc = "0x804 - MBUS Master Clock Gating Register"]
    pub mbus_mat_clk_gating: crate::Reg<mbus_mat_clk_gating::MBUS_MAT_CLK_GATING_SPEC>,
    _reserved57: [u8; 0x04],
    #[doc = "0x80c - DRAM Bus Gating Reset Register"]
    pub dram_bgr: crate::Reg<dram_bgr::DRAM_BGR_SPEC>,
    _reserved58: [u8; 0x20],
    #[doc = "0x830 - SMHC0 Clock Register"]
    pub smhc0_clk: crate::Reg<smhc0_clk::SMHC0_CLK_SPEC>,
    #[doc = "0x834 - SMHC1 Clock Register"]
    pub smhc1_clk: crate::Reg<smhc1_clk::SMHC1_CLK_SPEC>,
    #[doc = "0x838 - SMHC2 Clock Register"]
    pub smhc2_clk: crate::Reg<smhc2_clk::SMHC2_CLK_SPEC>,
    _reserved61: [u8; 0x10],
    #[doc = "0x84c - SMHC Bus Gating Reset Register"]
    pub smhc_bgr: crate::Reg<smhc_bgr::SMHC_BGR_SPEC>,
    _reserved62: [u8; 0xbc],
    #[doc = "0x90c - UART Bus Gating Reset Register"]
    pub uart_bgr: crate::Reg<uart_bgr::UART_BGR_SPEC>,
    _reserved63: [u8; 0x0c],
    #[doc = "0x91c - TWI Bus Gating Reset Register"]
    pub twi_bgr: crate::Reg<twi_bgr::TWI_BGR_SPEC>,
    _reserved64: [u8; 0x20],
    #[doc = "0x940 - SPI0 Clock Register"]
    pub spi0_clk: crate::Reg<spi0_clk::SPI0_CLK_SPEC>,
    #[doc = "0x944 - SPI1 Clock Register"]
    pub spi1_clk: crate::Reg<spi1_clk::SPI1_CLK_SPEC>,
    _reserved66: [u8; 0x24],
    #[doc = "0x96c - SPI Bus Gating Reset Register"]
    pub spi_bgr: crate::Reg<spi_bgr::SPI_BGR_SPEC>,
    #[doc = "0x970 - EMAC_25M Clock Register"]
    pub emac_25m_clk: crate::Reg<emac_25m_clk::EMAC_25M_CLK_SPEC>,
    _reserved68: [u8; 0x08],
    #[doc = "0x97c - EMAC Bus Gating Reset Register"]
    pub emac_bgr: crate::Reg<emac_bgr::EMAC_BGR_SPEC>,
    _reserved69: [u8; 0x40],
    #[doc = "0x9c0 - IRTX Clock Register"]
    pub irtx_clk: crate::Reg<irtx_clk::IRTX_CLK_SPEC>,
    _reserved70: [u8; 0x08],
    #[doc = "0x9cc - IRTX Bus Gating Reset Register"]
    pub irtx_bgr: crate::Reg<irtx_bgr::IRTX_BGR_SPEC>,
    _reserved71: [u8; 0x1c],
    #[doc = "0x9ec - GPADC Bus Gating Reset Register"]
    pub gpadc_bgr: crate::Reg<gpadc_bgr::GPADC_BGR_SPEC>,
    _reserved72: [u8; 0x0c],
    #[doc = "0x9fc - THS Bus Gating Reset Register"]
    pub ths_bgr: crate::Reg<ths_bgr::THS_BGR_SPEC>,
    _reserved73: [u8; 0x10],
    #[doc = "0xa10 - I2S0 Clock Register"]
    pub i2s0_clk: crate::Reg<i2s0_clk::I2S0_CLK_SPEC>,
    #[doc = "0xa14 - I2S1 Clock Register"]
    pub i2s1_clk: crate::Reg<i2s1_clk::I2S1_CLK_SPEC>,
    #[doc = "0xa18 - I2S2 Clock Register"]
    pub i2s2_clk: crate::Reg<i2s2_clk::I2S2_CLK_SPEC>,
    #[doc = "0xa1c - I2S2_ASRC Clock Register"]
    pub i2s2_asrc_clk: crate::Reg<i2s2_asrc_clk::I2S2_ASRC_CLK_SPEC>,
    #[doc = "0xa20 - I2S Bus Gating Reset Register"]
    pub i2s_bgr: crate::Reg<i2s_bgr::I2S_BGR_SPEC>,
    #[doc = "0xa24 - OWA_TX Clock Register"]
    pub owa_tx_clk: crate::Reg<owa_tx_clk::OWA_TX_CLK_SPEC>,
    #[doc = "0xa28 - OWA_RX Clock Register"]
    pub owa_rx_clk: crate::Reg<owa_rx_clk::OWA_RX_CLK_SPEC>,
    #[doc = "0xa2c - OWA Bus Gating Reset Register"]
    pub owa_bgr: crate::Reg<owa_bgr::OWA_BGR_SPEC>,
    _reserved81: [u8; 0x10],
    #[doc = "0xa40 - DMIC Clock Register"]
    pub dmic_clk: crate::Reg<dmic_clk::DMIC_CLK_SPEC>,
    _reserved82: [u8; 0x08],
    #[doc = "0xa4c - DMIC Bus Gating Reset Register"]
    pub dmic_bgr: crate::Reg<dmic_bgr::DMIC_BGR_SPEC>,
    #[doc = "0xa50 - AUDIO_CODEC_DAC Clock Register"]
    pub audio_codec_dac_clk: crate::Reg<audio_codec_dac_clk::AUDIO_CODEC_DAC_CLK_SPEC>,
    #[doc = "0xa54 - AUDIO_CODEC_ADC Clock Register"]
    pub audio_codec_adc_clk: crate::Reg<audio_codec_adc_clk::AUDIO_CODEC_ADC_CLK_SPEC>,
    _reserved85: [u8; 0x04],
    #[doc = "0xa5c - AUDIO_CODEC Bus Gating Reset Register"]
    pub audio_codec_bgr: crate::Reg<audio_codec_bgr::AUDIO_CODEC_BGR_SPEC>,
    _reserved86: [u8; 0x10],
    #[doc = "0xa70 - USB0 Clock Register"]
    pub usb0_clk: crate::Reg<usb0_clk::USB0_CLK_SPEC>,
    #[doc = "0xa74 - USB1 Clock Register"]
    pub usb1_clk: crate::Reg<usb1_clk::USB1_CLK_SPEC>,
    _reserved88: [u8; 0x14],
    #[doc = "0xa8c - USB Bus Gating Reset Register"]
    pub usb_bgr: crate::Reg<usb_bgr::USB_BGR_SPEC>,
    _reserved89: [u8; 0x2c],
    #[doc = "0xabc - DPSS_TOP Bus Gating Reset Register"]
    pub dpss_top_bgr: crate::Reg<dpss_top_bgr::DPSS_TOP_BGR_SPEC>,
    _reserved90: [u8; 0x64],
    #[doc = "0xb24 - DSI Clock Register"]
    pub dsi_clk: crate::Reg<dsi_clk::DSI_CLK_SPEC>,
    _reserved91: [u8; 0x24],
    #[doc = "0xb4c - DSI Bus Gating Reset Register"]
    pub dsi_bgr: crate::Reg<dsi_bgr::DSI_BGR_SPEC>,
    _reserved92: [u8; 0x10],
    #[doc = "0xb60 - TCONLCD Clock Register"]
    pub tconlcd_clk: crate::Reg<tconlcd_clk::TCONLCD_CLK_SPEC>,
    _reserved93: [u8; 0x18],
    #[doc = "0xb7c - TCONLCD Bus Gating Reset Register"]
    pub tconlcd_bgr: crate::Reg<tconlcd_bgr::TCONLCD_BGR_SPEC>,
    #[doc = "0xb80 - TCONTV Clock Register"]
    pub tcontv_clk: crate::Reg<tcontv_clk::TCONTV_CLK_SPEC>,
    _reserved95: [u8; 0x18],
    #[doc = "0xb9c - TCONTV Bus Gating Reset Register"]
    pub tcontv_bgr: crate::Reg<tcontv_bgr::TCONTV_BGR_SPEC>,
    _reserved96: [u8; 0x0c],
    #[doc = "0xbac - LVDS Bus Gating Reset Register"]
    pub lvds_bgr: crate::Reg<lvds_bgr::LVDS_BGR_SPEC>,
    #[doc = "0xbb0 - TVE Clock Register"]
    pub tve_clk: crate::Reg<tve_clk::TVE_CLK_SPEC>,
    _reserved98: [u8; 0x08],
    #[doc = "0xbbc - TVE Bus Gating Reset Register"]
    pub tve_bgr: crate::Reg<tve_bgr::TVE_BGR_SPEC>,
    #[doc = "0xbc0 - TVD Clock Register"]
    pub tvd_clk: crate::Reg<tvd_clk::TVD_CLK_SPEC>,
    _reserved100: [u8; 0x18],
    #[doc = "0xbdc - TVD Bus Gating Reset Register"]
    pub tvd_bgr: crate::Reg<tvd_bgr::TVD_BGR_SPEC>,
    _reserved101: [u8; 0x10],
    #[doc = "0xbf0 - LEDC Clock Register"]
    pub ledc_clk: crate::Reg<ledc_clk::LEDC_CLK_SPEC>,
    _reserved102: [u8; 0x08],
    #[doc = "0xbfc - LEDC Bus Gating Reset Register"]
    pub ledc_bgr: crate::Reg<ledc_bgr::LEDC_BGR_SPEC>,
    _reserved103: [u8; 0x04],
    #[doc = "0xc04 - CSI Clock Register"]
    pub csi_clk: crate::Reg<csi_clk::CSI_CLK_SPEC>,
    #[doc = "0xc08 - CSI Master Clock Register"]
    pub csi_master_clk: crate::Reg<csi_master_clk::CSI_MASTER_CLK_SPEC>,
    _reserved105: [u8; 0x10],
    #[doc = "0xc1c - CSI Bus Gating Reset Register"]
    pub csi_bgr: crate::Reg<csi_bgr::CSI_BGR_SPEC>,
    _reserved106: [u8; 0x30],
    #[doc = "0xc50 - TPADC Clock Register"]
    pub tpadc_clk: crate::Reg<tpadc_clk::TPADC_CLK_SPEC>,
    _reserved107: [u8; 0x08],
    #[doc = "0xc5c - TPADC Bus Gating Reset Register"]
    pub tpadc_bgr: crate::Reg<tpadc_bgr::TPADC_BGR_SPEC>,
    _reserved108: [u8; 0x10],
    #[doc = "0xc70 - DSP Clock Register"]
    pub dsp_clk: crate::Reg<dsp_clk::DSP_CLK_SPEC>,
    _reserved109: [u8; 0x08],
    #[doc = "0xc7c - DSP Bus Gating Reset Register"]
    pub dsp_bgr: crate::Reg<dsp_bgr::DSP_BGR_SPEC>,
    _reserved110: [u8; 0x80],
    #[doc = "0xd00 - RISC-V Clock Register"]
    pub riscv_clk: crate::Reg<riscv_clk::RISCV_CLK_SPEC>,
    #[doc = "0xd04 - RISC-V GATING Configuration Register"]
    pub riscv_gating: crate::Reg<riscv_gating::RISCV_GATING_SPEC>,
    _reserved112: [u8; 0x04],
    #[doc = "0xd0c - RISC-V_CFG Bus Gating Reset Register"]
    pub riscv_cfg_bgr: crate::Reg<riscv_cfg_bgr::RISCV_CFG_BGR_SPEC>,
    _reserved113: [u8; 0x01f4],
    #[doc = "0xf04 - PLL Lock Debug Control Register"]
    pub pll_lock_dbg_ctrl: crate::Reg<pll_lock_dbg_ctrl::PLL_LOCK_DBG_CTRL_SPEC>,
    #[doc = "0xf08 - Frequency Detect Control Register"]
    pub fre_det_ctrl: crate::Reg<fre_det_ctrl::FRE_DET_CTRL_SPEC>,
    #[doc = "0xf0c - Frequency Up Limit Register"]
    pub fre_up_lim: crate::Reg<fre_up_lim::FRE_UP_LIM_SPEC>,
    #[doc = "0xf10 - Frequency Down Limit Register"]
    pub fre_down_lim: crate::Reg<fre_down_lim::FRE_DOWN_LIM_SPEC>,
    _reserved117: [u8; 0x1c],
    #[doc = "0xf30 - CCU FANOUT CLOCK GATE Register"]
    pub ccu_fan_gate: crate::Reg<ccu_fan_gate::CCU_FAN_GATE_SPEC>,
    #[doc = "0xf34 - CLK27M FANOUT Register"]
    pub clk27m_fan: crate::Reg<clk27m_fan::CLK27M_FAN_SPEC>,
    #[doc = "0xf38 - PCLK FANOUT Register"]
    pub pclk_fan: crate::Reg<pclk_fan::PCLK_FAN_SPEC>,
    #[doc = "0xf3c - CCU FANOUT Register"]
    pub ccu_fan: crate::Reg<ccu_fan::CCU_FAN_SPEC>,
}
#[doc = "PLL_CPU_CTRL register accessor: an alias for `Reg<PLL_CPU_CTRL_SPEC>`"]
pub type PLL_CPU_CTRL = crate::Reg<pll_cpu_ctrl::PLL_CPU_CTRL_SPEC>;
#[doc = "PLL_CPU Control Register"]
pub mod pll_cpu_ctrl;
#[doc = "PLL_DDR_CTRL register accessor: an alias for `Reg<PLL_DDR_CTRL_SPEC>`"]
pub type PLL_DDR_CTRL = crate::Reg<pll_ddr_ctrl::PLL_DDR_CTRL_SPEC>;
#[doc = "PLL_DDR Control Register"]
pub mod pll_ddr_ctrl;
#[doc = "PLL_PERI_CTRL register accessor: an alias for `Reg<PLL_PERI_CTRL_SPEC>`"]
pub type PLL_PERI_CTRL = crate::Reg<pll_peri_ctrl::PLL_PERI_CTRL_SPEC>;
#[doc = "PLL_PERI Control Register"]
pub mod pll_peri_ctrl;
#[doc = "PLL_VIDEO0_CTRL register accessor: an alias for `Reg<PLL_VIDEO0_CTRL_SPEC>`"]
pub type PLL_VIDEO0_CTRL = crate::Reg<pll_video0_ctrl::PLL_VIDEO0_CTRL_SPEC>;
#[doc = "PLL_VIDEO0 Control Register"]
pub mod pll_video0_ctrl;
#[doc = "PLL_VIDEO1_CTRL register accessor: an alias for `Reg<PLL_VIDEO1_CTRL_SPEC>`"]
pub type PLL_VIDEO1_CTRL = crate::Reg<pll_video1_ctrl::PLL_VIDEO1_CTRL_SPEC>;
#[doc = "PLL_VIDEO1 Control Register"]
pub mod pll_video1_ctrl;
#[doc = "PLL_VE_CTRL register accessor: an alias for `Reg<PLL_VE_CTRL_SPEC>`"]
pub type PLL_VE_CTRL = crate::Reg<pll_ve_ctrl::PLL_VE_CTRL_SPEC>;
#[doc = "PLL_VE Control Register"]
pub mod pll_ve_ctrl;
#[doc = "PLL_AUDIO0_CTRL register accessor: an alias for `Reg<PLL_AUDIO0_CTRL_SPEC>`"]
pub type PLL_AUDIO0_CTRL = crate::Reg<pll_audio0_ctrl::PLL_AUDIO0_CTRL_SPEC>;
#[doc = "PLL_AUDIO0 Control Register"]
pub mod pll_audio0_ctrl;
#[doc = "PLL_AUDIO1_CTRL register accessor: an alias for `Reg<PLL_AUDIO1_CTRL_SPEC>`"]
pub type PLL_AUDIO1_CTRL = crate::Reg<pll_audio1_ctrl::PLL_AUDIO1_CTRL_SPEC>;
#[doc = "PLL_AUDIO1 Control Register"]
pub mod pll_audio1_ctrl;
#[doc = "PLL_DDR_PAT0_CTRL register accessor: an alias for `Reg<PLL_DDR_PAT0_CTRL_SPEC>`"]
pub type PLL_DDR_PAT0_CTRL = crate::Reg<pll_ddr_pat0_ctrl::PLL_DDR_PAT0_CTRL_SPEC>;
#[doc = "PLL_DDR Pattern0 Control Register"]
pub mod pll_ddr_pat0_ctrl;
#[doc = "PLL_DDR_PAT1_CTRL register accessor: an alias for `Reg<PLL_DDR_PAT1_CTRL_SPEC>`"]
pub type PLL_DDR_PAT1_CTRL = crate::Reg<pll_ddr_pat1_ctrl::PLL_DDR_PAT1_CTRL_SPEC>;
#[doc = "PLL_DDR Pattern1 Control Register"]
pub mod pll_ddr_pat1_ctrl;
#[doc = "PLL_PERI_PAT0_CTRL register accessor: an alias for `Reg<PLL_PERI_PAT0_CTRL_SPEC>`"]
pub type PLL_PERI_PAT0_CTRL = crate::Reg<pll_peri_pat0_ctrl::PLL_PERI_PAT0_CTRL_SPEC>;
#[doc = "PLL_PERI Pattern0 Control Register"]
pub mod pll_peri_pat0_ctrl;
#[doc = "PLL_PERI_PAT1_CTRL register accessor: an alias for `Reg<PLL_PERI_PAT1_CTRL_SPEC>`"]
pub type PLL_PERI_PAT1_CTRL = crate::Reg<pll_peri_pat1_ctrl::PLL_PERI_PAT1_CTRL_SPEC>;
#[doc = "PLL_PERI Pattern1 Control Register"]
pub mod pll_peri_pat1_ctrl;
#[doc = "PLL_VIDEO0_PAT0_CTRL register accessor: an alias for `Reg<PLL_VIDEO0_PAT0_CTRL_SPEC>`"]
pub type PLL_VIDEO0_PAT0_CTRL = crate::Reg<pll_video0_pat0_ctrl::PLL_VIDEO0_PAT0_CTRL_SPEC>;
#[doc = "PLL_VIDEO0 Pattern0 Control Register"]
pub mod pll_video0_pat0_ctrl;
#[doc = "PLL_VIDEO0_PAT1_CTRL register accessor: an alias for `Reg<PLL_VIDEO0_PAT1_CTRL_SPEC>`"]
pub type PLL_VIDEO0_PAT1_CTRL = crate::Reg<pll_video0_pat1_ctrl::PLL_VIDEO0_PAT1_CTRL_SPEC>;
#[doc = "PLL_VIDEO0 Pattern1 Control Register"]
pub mod pll_video0_pat1_ctrl;
#[doc = "PLL_VIDEO1_PAT0_CTRL register accessor: an alias for `Reg<PLL_VIDEO1_PAT0_CTRL_SPEC>`"]
pub type PLL_VIDEO1_PAT0_CTRL = crate::Reg<pll_video1_pat0_ctrl::PLL_VIDEO1_PAT0_CTRL_SPEC>;
#[doc = "PLL_VIDEO1 Pattern0 Control Register"]
pub mod pll_video1_pat0_ctrl;
#[doc = "PLL_VIDEO1_PAT1_CTRL register accessor: an alias for `Reg<PLL_VIDEO1_PAT1_CTRL_SPEC>`"]
pub type PLL_VIDEO1_PAT1_CTRL = crate::Reg<pll_video1_pat1_ctrl::PLL_VIDEO1_PAT1_CTRL_SPEC>;
#[doc = "PLL_VIDEO1 Pattern1 Control Register"]
pub mod pll_video1_pat1_ctrl;
#[doc = "PLL_VE_PAT0_CTRL register accessor: an alias for `Reg<PLL_VE_PAT0_CTRL_SPEC>`"]
pub type PLL_VE_PAT0_CTRL = crate::Reg<pll_ve_pat0_ctrl::PLL_VE_PAT0_CTRL_SPEC>;
#[doc = "PLL_VE Pattern0 Control Register"]
pub mod pll_ve_pat0_ctrl;
#[doc = "PLL_VE_PAT1_CTRL register accessor: an alias for `Reg<PLL_VE_PAT1_CTRL_SPEC>`"]
pub type PLL_VE_PAT1_CTRL = crate::Reg<pll_ve_pat1_ctrl::PLL_VE_PAT1_CTRL_SPEC>;
#[doc = "PLL_VE Pattern1 Control Register"]
pub mod pll_ve_pat1_ctrl;
#[doc = "PLL_AUDIO0_PAT0_CTRL register accessor: an alias for `Reg<PLL_AUDIO0_PAT0_CTRL_SPEC>`"]
pub type PLL_AUDIO0_PAT0_CTRL = crate::Reg<pll_audio0_pat0_ctrl::PLL_AUDIO0_PAT0_CTRL_SPEC>;
#[doc = "PLL_AUDIO0 Pattern0 Control Register"]
pub mod pll_audio0_pat0_ctrl;
#[doc = "PLL_AUDIO0_PAT1_CTRL register accessor: an alias for `Reg<PLL_AUDIO0_PAT1_CTRL_SPEC>`"]
pub type PLL_AUDIO0_PAT1_CTRL = crate::Reg<pll_audio0_pat1_ctrl::PLL_AUDIO0_PAT1_CTRL_SPEC>;
#[doc = "PLL_AUDIO0 Pattern1 Control Register"]
pub mod pll_audio0_pat1_ctrl;
#[doc = "PLL_AUDIO1_PAT0_CTRL register accessor: an alias for `Reg<PLL_AUDIO1_PAT0_CTRL_SPEC>`"]
pub type PLL_AUDIO1_PAT0_CTRL = crate::Reg<pll_audio1_pat0_ctrl::PLL_AUDIO1_PAT0_CTRL_SPEC>;
#[doc = "PLL_AUDIO1 Pattern0 Control Register"]
pub mod pll_audio1_pat0_ctrl;
#[doc = "PLL_AUDIO1_PAT1_CTRL register accessor: an alias for `Reg<PLL_AUDIO1_PAT1_CTRL_SPEC>`"]
pub type PLL_AUDIO1_PAT1_CTRL = crate::Reg<pll_audio1_pat1_ctrl::PLL_AUDIO1_PAT1_CTRL_SPEC>;
#[doc = "PLL_AUDIO1 Pattern1 Control Register"]
pub mod pll_audio1_pat1_ctrl;
#[doc = "PLL_CPU_BIAS register accessor: an alias for `Reg<PLL_CPU_BIAS_SPEC>`"]
pub type PLL_CPU_BIAS = crate::Reg<pll_cpu_bias::PLL_CPU_BIAS_SPEC>;
#[doc = "PLL_CPU Bias Register"]
pub mod pll_cpu_bias;
#[doc = "PLL_DDR_BIAS register accessor: an alias for `Reg<PLL_DDR_BIAS_SPEC>`"]
pub type PLL_DDR_BIAS = crate::Reg<pll_ddr_bias::PLL_DDR_BIAS_SPEC>;
#[doc = "PLL_DDR Bias Register"]
pub mod pll_ddr_bias;
#[doc = "PLL_PERI_BIAS register accessor: an alias for `Reg<PLL_PERI_BIAS_SPEC>`"]
pub type PLL_PERI_BIAS = crate::Reg<pll_peri_bias::PLL_PERI_BIAS_SPEC>;
#[doc = "PLL_PERI Bias Register"]
pub mod pll_peri_bias;
#[doc = "PLL_VIDEO0_BIAS register accessor: an alias for `Reg<PLL_VIDEO0_BIAS_SPEC>`"]
pub type PLL_VIDEO0_BIAS = crate::Reg<pll_video0_bias::PLL_VIDEO0_BIAS_SPEC>;
#[doc = "PLL_VIDEO0 Bias Register"]
pub mod pll_video0_bias;
#[doc = "PLL_VIDEO1_BIAS register accessor: an alias for `Reg<PLL_VIDEO1_BIAS_SPEC>`"]
pub type PLL_VIDEO1_BIAS = crate::Reg<pll_video1_bias::PLL_VIDEO1_BIAS_SPEC>;
#[doc = "PLL_VIDEO1 Bias Register"]
pub mod pll_video1_bias;
#[doc = "PLL_VE_BIAS register accessor: an alias for `Reg<PLL_VE_BIAS_SPEC>`"]
pub type PLL_VE_BIAS = crate::Reg<pll_ve_bias::PLL_VE_BIAS_SPEC>;
#[doc = "PLL_VE Bias Register"]
pub mod pll_ve_bias;
#[doc = "PLL_AUDIO0_BIAS register accessor: an alias for `Reg<PLL_AUDIO0_BIAS_SPEC>`"]
pub type PLL_AUDIO0_BIAS = crate::Reg<pll_audio0_bias::PLL_AUDIO0_BIAS_SPEC>;
#[doc = "PLL_AUDIO0 Bias Register"]
pub mod pll_audio0_bias;
#[doc = "PLL_AUDIO1_BIAS register accessor: an alias for `Reg<PLL_AUDIO1_BIAS_SPEC>`"]
pub type PLL_AUDIO1_BIAS = crate::Reg<pll_audio1_bias::PLL_AUDIO1_BIAS_SPEC>;
#[doc = "PLL_AUDIO1 Bias Register"]
pub mod pll_audio1_bias;
#[doc = "PLL_CPU_TUN register accessor: an alias for `Reg<PLL_CPU_TUN_SPEC>`"]
pub type PLL_CPU_TUN = crate::Reg<pll_cpu_tun::PLL_CPU_TUN_SPEC>;
#[doc = "PLL_CPU Tuning Register"]
pub mod pll_cpu_tun;
#[doc = "CPU_AXI_CFG register accessor: an alias for `Reg<CPU_AXI_CFG_SPEC>`"]
pub type CPU_AXI_CFG = crate::Reg<cpu_axi_cfg::CPU_AXI_CFG_SPEC>;
#[doc = "CPU_AXI Configuration Register"]
pub mod cpu_axi_cfg;
#[doc = "CPU_GATING register accessor: an alias for `Reg<CPU_GATING_SPEC>`"]
pub type CPU_GATING = crate::Reg<cpu_gating::CPU_GATING_SPEC>;
#[doc = "CPU_GATING Configuration Register"]
pub mod cpu_gating;
#[doc = "PSI_CLK register accessor: an alias for `Reg<PSI_CLK_SPEC>`"]
pub type PSI_CLK = crate::Reg<psi_clk::PSI_CLK_SPEC>;
#[doc = "PSI Clock Register"]
pub mod psi_clk;
#[doc = "APB0_CLK register accessor: an alias for `Reg<APB0_CLK_SPEC>`"]
pub type APB0_CLK = crate::Reg<apb0_clk::APB0_CLK_SPEC>;
#[doc = "APB0 Clock Register"]
pub mod apb0_clk;
#[doc = "APB1_CLK register accessor: an alias for `Reg<APB1_CLK_SPEC>`"]
pub type APB1_CLK = crate::Reg<apb1_clk::APB1_CLK_SPEC>;
#[doc = "APB1 Clock Register"]
pub mod apb1_clk;
#[doc = "MBUS_CLK register accessor: an alias for `Reg<MBUS_CLK_SPEC>`"]
pub type MBUS_CLK = crate::Reg<mbus_clk::MBUS_CLK_SPEC>;
#[doc = "MBUS Clock Register"]
pub mod mbus_clk;
#[doc = "DE_CLK register accessor: an alias for `Reg<DE_CLK_SPEC>`"]
pub type DE_CLK = crate::Reg<de_clk::DE_CLK_SPEC>;
#[doc = "DE Clock Register"]
pub mod de_clk;
#[doc = "DE_BGR register accessor: an alias for `Reg<DE_BGR_SPEC>`"]
pub type DE_BGR = crate::Reg<de_bgr::DE_BGR_SPEC>;
#[doc = "DE Bus Gating Reset Register"]
pub mod de_bgr;
#[doc = "DI_CLK register accessor: an alias for `Reg<DI_CLK_SPEC>`"]
pub type DI_CLK = crate::Reg<di_clk::DI_CLK_SPEC>;
#[doc = "DI Clock Register"]
pub mod di_clk;
#[doc = "DI_BGR register accessor: an alias for `Reg<DI_BGR_SPEC>`"]
pub type DI_BGR = crate::Reg<di_bgr::DI_BGR_SPEC>;
#[doc = "DI Bus Gating Reset Register"]
pub mod di_bgr;
#[doc = "G2D_CLK register accessor: an alias for `Reg<G2D_CLK_SPEC>`"]
pub type G2D_CLK = crate::Reg<g2d_clk::G2D_CLK_SPEC>;
#[doc = "G2D Clock Register"]
pub mod g2d_clk;
#[doc = "G2D_BGR register accessor: an alias for `Reg<G2D_BGR_SPEC>`"]
pub type G2D_BGR = crate::Reg<g2d_bgr::G2D_BGR_SPEC>;
#[doc = "G2D Bus Gating Reset Register"]
pub mod g2d_bgr;
#[doc = "CE_CLK register accessor: an alias for `Reg<CE_CLK_SPEC>`"]
pub type CE_CLK = crate::Reg<ce_clk::CE_CLK_SPEC>;
#[doc = "CE Clock Register"]
pub mod ce_clk;
#[doc = "CE_BGR register accessor: an alias for `Reg<CE_BGR_SPEC>`"]
pub type CE_BGR = crate::Reg<ce_bgr::CE_BGR_SPEC>;
#[doc = "CE Bus Gating Reset Register"]
pub mod ce_bgr;
#[doc = "VE_CLK register accessor: an alias for `Reg<VE_CLK_SPEC>`"]
pub type VE_CLK = crate::Reg<ve_clk::VE_CLK_SPEC>;
#[doc = "VE Clock Register"]
pub mod ve_clk;
#[doc = "VE_BGR register accessor: an alias for `Reg<VE_BGR_SPEC>`"]
pub type VE_BGR = crate::Reg<ve_bgr::VE_BGR_SPEC>;
#[doc = "VE Bus Gating Reset Register"]
pub mod ve_bgr;
#[doc = "DMA_BGR register accessor: an alias for `Reg<DMA_BGR_SPEC>`"]
pub type DMA_BGR = crate::Reg<dma_bgr::DMA_BGR_SPEC>;
#[doc = "DMA Bus Gating Reset Register"]
pub mod dma_bgr;
#[doc = "MSGBOX_BGR register accessor: an alias for `Reg<MSGBOX_BGR_SPEC>`"]
pub type MSGBOX_BGR = crate::Reg<msgbox_bgr::MSGBOX_BGR_SPEC>;
#[doc = "MSGBOX Bus Gating Reset Register"]
pub mod msgbox_bgr;
#[doc = "SPINLOCK_BGR register accessor: an alias for `Reg<SPINLOCK_BGR_SPEC>`"]
pub type SPINLOCK_BGR = crate::Reg<spinlock_bgr::SPINLOCK_BGR_SPEC>;
#[doc = "SPINLOCK Bus Gating Reset Register"]
pub mod spinlock_bgr;
#[doc = "HSTIMER_BGR register accessor: an alias for `Reg<HSTIMER_BGR_SPEC>`"]
pub type HSTIMER_BGR = crate::Reg<hstimer_bgr::HSTIMER_BGR_SPEC>;
#[doc = "HSTIMER Bus Gating Reset Register"]
pub mod hstimer_bgr;
#[doc = "AVS_CLK register accessor: an alias for `Reg<AVS_CLK_SPEC>`"]
pub type AVS_CLK = crate::Reg<avs_clk::AVS_CLK_SPEC>;
#[doc = "AVS Clock Register"]
pub mod avs_clk;
#[doc = "DBGSYS_BGR register accessor: an alias for `Reg<DBGSYS_BGR_SPEC>`"]
pub type DBGSYS_BGR = crate::Reg<dbgsys_bgr::DBGSYS_BGR_SPEC>;
#[doc = "DBGSYS Bus Gating Reset Register"]
pub mod dbgsys_bgr;
#[doc = "PWM_BGR register accessor: an alias for `Reg<PWM_BGR_SPEC>`"]
pub type PWM_BGR = crate::Reg<pwm_bgr::PWM_BGR_SPEC>;
#[doc = "PWM Bus Gating Reset Register"]
pub mod pwm_bgr;
#[doc = "IOMMU_BGR register accessor: an alias for `Reg<IOMMU_BGR_SPEC>`"]
pub type IOMMU_BGR = crate::Reg<iommu_bgr::IOMMU_BGR_SPEC>;
#[doc = "IOMMU Bus Gating Reset Register"]
pub mod iommu_bgr;
#[doc = "DRAM_CLK register accessor: an alias for `Reg<DRAM_CLK_SPEC>`"]
pub type DRAM_CLK = crate::Reg<dram_clk::DRAM_CLK_SPEC>;
#[doc = "DRAM Clock Register"]
pub mod dram_clk;
#[doc = "MBUS_MAT_CLK_GATING register accessor: an alias for `Reg<MBUS_MAT_CLK_GATING_SPEC>`"]
pub type MBUS_MAT_CLK_GATING = crate::Reg<mbus_mat_clk_gating::MBUS_MAT_CLK_GATING_SPEC>;
#[doc = "MBUS Master Clock Gating Register"]
pub mod mbus_mat_clk_gating;
#[doc = "DRAM_BGR register accessor: an alias for `Reg<DRAM_BGR_SPEC>`"]
pub type DRAM_BGR = crate::Reg<dram_bgr::DRAM_BGR_SPEC>;
#[doc = "DRAM Bus Gating Reset Register"]
pub mod dram_bgr;
#[doc = "SMHC0_CLK register accessor: an alias for `Reg<SMHC0_CLK_SPEC>`"]
pub type SMHC0_CLK = crate::Reg<smhc0_clk::SMHC0_CLK_SPEC>;
#[doc = "SMHC0 Clock Register"]
pub mod smhc0_clk;
#[doc = "SMHC1_CLK register accessor: an alias for `Reg<SMHC1_CLK_SPEC>`"]
pub type SMHC1_CLK = crate::Reg<smhc1_clk::SMHC1_CLK_SPEC>;
#[doc = "SMHC1 Clock Register"]
pub mod smhc1_clk;
#[doc = "SMHC2_CLK register accessor: an alias for `Reg<SMHC2_CLK_SPEC>`"]
pub type SMHC2_CLK = crate::Reg<smhc2_clk::SMHC2_CLK_SPEC>;
#[doc = "SMHC2 Clock Register"]
pub mod smhc2_clk;
#[doc = "SMHC_BGR register accessor: an alias for `Reg<SMHC_BGR_SPEC>`"]
pub type SMHC_BGR = crate::Reg<smhc_bgr::SMHC_BGR_SPEC>;
#[doc = "SMHC Bus Gating Reset Register"]
pub mod smhc_bgr;
#[doc = "UART_BGR register accessor: an alias for `Reg<UART_BGR_SPEC>`"]
pub type UART_BGR = crate::Reg<uart_bgr::UART_BGR_SPEC>;
#[doc = "UART Bus Gating Reset Register"]
pub mod uart_bgr;
#[doc = "TWI_BGR register accessor: an alias for `Reg<TWI_BGR_SPEC>`"]
pub type TWI_BGR = crate::Reg<twi_bgr::TWI_BGR_SPEC>;
#[doc = "TWI Bus Gating Reset Register"]
pub mod twi_bgr;
#[doc = "SPI0_CLK register accessor: an alias for `Reg<SPI0_CLK_SPEC>`"]
pub type SPI0_CLK = crate::Reg<spi0_clk::SPI0_CLK_SPEC>;
#[doc = "SPI0 Clock Register"]
pub mod spi0_clk;
#[doc = "SPI1_CLK register accessor: an alias for `Reg<SPI1_CLK_SPEC>`"]
pub type SPI1_CLK = crate::Reg<spi1_clk::SPI1_CLK_SPEC>;
#[doc = "SPI1 Clock Register"]
pub mod spi1_clk;
#[doc = "SPI_BGR register accessor: an alias for `Reg<SPI_BGR_SPEC>`"]
pub type SPI_BGR = crate::Reg<spi_bgr::SPI_BGR_SPEC>;
#[doc = "SPI Bus Gating Reset Register"]
pub mod spi_bgr;
#[doc = "EMAC_25M_CLK register accessor: an alias for `Reg<EMAC_25M_CLK_SPEC>`"]
pub type EMAC_25M_CLK = crate::Reg<emac_25m_clk::EMAC_25M_CLK_SPEC>;
#[doc = "EMAC_25M Clock Register"]
pub mod emac_25m_clk;
#[doc = "EMAC_BGR register accessor: an alias for `Reg<EMAC_BGR_SPEC>`"]
pub type EMAC_BGR = crate::Reg<emac_bgr::EMAC_BGR_SPEC>;
#[doc = "EMAC Bus Gating Reset Register"]
pub mod emac_bgr;
#[doc = "IRTX_CLK register accessor: an alias for `Reg<IRTX_CLK_SPEC>`"]
pub type IRTX_CLK = crate::Reg<irtx_clk::IRTX_CLK_SPEC>;
#[doc = "IRTX Clock Register"]
pub mod irtx_clk;
#[doc = "IRTX_BGR register accessor: an alias for `Reg<IRTX_BGR_SPEC>`"]
pub type IRTX_BGR = crate::Reg<irtx_bgr::IRTX_BGR_SPEC>;
#[doc = "IRTX Bus Gating Reset Register"]
pub mod irtx_bgr;
#[doc = "GPADC_BGR register accessor: an alias for `Reg<GPADC_BGR_SPEC>`"]
pub type GPADC_BGR = crate::Reg<gpadc_bgr::GPADC_BGR_SPEC>;
#[doc = "GPADC Bus Gating Reset Register"]
pub mod gpadc_bgr;
#[doc = "THS_BGR register accessor: an alias for `Reg<THS_BGR_SPEC>`"]
pub type THS_BGR = crate::Reg<ths_bgr::THS_BGR_SPEC>;
#[doc = "THS Bus Gating Reset Register"]
pub mod ths_bgr;
#[doc = "I2S0_CLK register accessor: an alias for `Reg<I2S0_CLK_SPEC>`"]
pub type I2S0_CLK = crate::Reg<i2s0_clk::I2S0_CLK_SPEC>;
#[doc = "I2S0 Clock Register"]
pub mod i2s0_clk;
#[doc = "I2S1_CLK register accessor: an alias for `Reg<I2S1_CLK_SPEC>`"]
pub type I2S1_CLK = crate::Reg<i2s1_clk::I2S1_CLK_SPEC>;
#[doc = "I2S1 Clock Register"]
pub mod i2s1_clk;
#[doc = "I2S2_CLK register accessor: an alias for `Reg<I2S2_CLK_SPEC>`"]
pub type I2S2_CLK = crate::Reg<i2s2_clk::I2S2_CLK_SPEC>;
#[doc = "I2S2 Clock Register"]
pub mod i2s2_clk;
#[doc = "I2S2_ASRC_CLK register accessor: an alias for `Reg<I2S2_ASRC_CLK_SPEC>`"]
pub type I2S2_ASRC_CLK = crate::Reg<i2s2_asrc_clk::I2S2_ASRC_CLK_SPEC>;
#[doc = "I2S2_ASRC Clock Register"]
pub mod i2s2_asrc_clk;
#[doc = "I2S_BGR register accessor: an alias for `Reg<I2S_BGR_SPEC>`"]
pub type I2S_BGR = crate::Reg<i2s_bgr::I2S_BGR_SPEC>;
#[doc = "I2S Bus Gating Reset Register"]
pub mod i2s_bgr;
#[doc = "OWA_TX_CLK register accessor: an alias for `Reg<OWA_TX_CLK_SPEC>`"]
pub type OWA_TX_CLK = crate::Reg<owa_tx_clk::OWA_TX_CLK_SPEC>;
#[doc = "OWA_TX Clock Register"]
pub mod owa_tx_clk;
#[doc = "OWA_RX_CLK register accessor: an alias for `Reg<OWA_RX_CLK_SPEC>`"]
pub type OWA_RX_CLK = crate::Reg<owa_rx_clk::OWA_RX_CLK_SPEC>;
#[doc = "OWA_RX Clock Register"]
pub mod owa_rx_clk;
#[doc = "OWA_BGR register accessor: an alias for `Reg<OWA_BGR_SPEC>`"]
pub type OWA_BGR = crate::Reg<owa_bgr::OWA_BGR_SPEC>;
#[doc = "OWA Bus Gating Reset Register"]
pub mod owa_bgr;
#[doc = "DMIC_CLK register accessor: an alias for `Reg<DMIC_CLK_SPEC>`"]
pub type DMIC_CLK = crate::Reg<dmic_clk::DMIC_CLK_SPEC>;
#[doc = "DMIC Clock Register"]
pub mod dmic_clk;
#[doc = "DMIC_BGR register accessor: an alias for `Reg<DMIC_BGR_SPEC>`"]
pub type DMIC_BGR = crate::Reg<dmic_bgr::DMIC_BGR_SPEC>;
#[doc = "DMIC Bus Gating Reset Register"]
pub mod dmic_bgr;
#[doc = "AUDIO_CODEC_DAC_CLK register accessor: an alias for `Reg<AUDIO_CODEC_DAC_CLK_SPEC>`"]
pub type AUDIO_CODEC_DAC_CLK = crate::Reg<audio_codec_dac_clk::AUDIO_CODEC_DAC_CLK_SPEC>;
#[doc = "AUDIO_CODEC_DAC Clock Register"]
pub mod audio_codec_dac_clk;
#[doc = "AUDIO_CODEC_ADC_CLK register accessor: an alias for `Reg<AUDIO_CODEC_ADC_CLK_SPEC>`"]
pub type AUDIO_CODEC_ADC_CLK = crate::Reg<audio_codec_adc_clk::AUDIO_CODEC_ADC_CLK_SPEC>;
#[doc = "AUDIO_CODEC_ADC Clock Register"]
pub mod audio_codec_adc_clk;
#[doc = "AUDIO_CODEC_BGR register accessor: an alias for `Reg<AUDIO_CODEC_BGR_SPEC>`"]
pub type AUDIO_CODEC_BGR = crate::Reg<audio_codec_bgr::AUDIO_CODEC_BGR_SPEC>;
#[doc = "AUDIO_CODEC Bus Gating Reset Register"]
pub mod audio_codec_bgr;
#[doc = "USB0_CLK register accessor: an alias for `Reg<USB0_CLK_SPEC>`"]
pub type USB0_CLK = crate::Reg<usb0_clk::USB0_CLK_SPEC>;
#[doc = "USB0 Clock Register"]
pub mod usb0_clk;
#[doc = "USB1_CLK register accessor: an alias for `Reg<USB1_CLK_SPEC>`"]
pub type USB1_CLK = crate::Reg<usb1_clk::USB1_CLK_SPEC>;
#[doc = "USB1 Clock Register"]
pub mod usb1_clk;
#[doc = "USB_BGR register accessor: an alias for `Reg<USB_BGR_SPEC>`"]
pub type USB_BGR = crate::Reg<usb_bgr::USB_BGR_SPEC>;
#[doc = "USB Bus Gating Reset Register"]
pub mod usb_bgr;
#[doc = "DPSS_TOP_BGR register accessor: an alias for `Reg<DPSS_TOP_BGR_SPEC>`"]
pub type DPSS_TOP_BGR = crate::Reg<dpss_top_bgr::DPSS_TOP_BGR_SPEC>;
#[doc = "DPSS_TOP Bus Gating Reset Register"]
pub mod dpss_top_bgr;
#[doc = "DSI_CLK register accessor: an alias for `Reg<DSI_CLK_SPEC>`"]
pub type DSI_CLK = crate::Reg<dsi_clk::DSI_CLK_SPEC>;
#[doc = "DSI Clock Register"]
pub mod dsi_clk;
#[doc = "DSI_BGR register accessor: an alias for `Reg<DSI_BGR_SPEC>`"]
pub type DSI_BGR = crate::Reg<dsi_bgr::DSI_BGR_SPEC>;
#[doc = "DSI Bus Gating Reset Register"]
pub mod dsi_bgr;
#[doc = "TCONLCD_CLK register accessor: an alias for `Reg<TCONLCD_CLK_SPEC>`"]
pub type TCONLCD_CLK = crate::Reg<tconlcd_clk::TCONLCD_CLK_SPEC>;
#[doc = "TCONLCD Clock Register"]
pub mod tconlcd_clk;
#[doc = "TCONLCD_BGR register accessor: an alias for `Reg<TCONLCD_BGR_SPEC>`"]
pub type TCONLCD_BGR = crate::Reg<tconlcd_bgr::TCONLCD_BGR_SPEC>;
#[doc = "TCONLCD Bus Gating Reset Register"]
pub mod tconlcd_bgr;
#[doc = "TCONTV_CLK register accessor: an alias for `Reg<TCONTV_CLK_SPEC>`"]
pub type TCONTV_CLK = crate::Reg<tcontv_clk::TCONTV_CLK_SPEC>;
#[doc = "TCONTV Clock Register"]
pub mod tcontv_clk;
#[doc = "TCONTV_BGR register accessor: an alias for `Reg<TCONTV_BGR_SPEC>`"]
pub type TCONTV_BGR = crate::Reg<tcontv_bgr::TCONTV_BGR_SPEC>;
#[doc = "TCONTV Bus Gating Reset Register"]
pub mod tcontv_bgr;
#[doc = "LVDS_BGR register accessor: an alias for `Reg<LVDS_BGR_SPEC>`"]
pub type LVDS_BGR = crate::Reg<lvds_bgr::LVDS_BGR_SPEC>;
#[doc = "LVDS Bus Gating Reset Register"]
pub mod lvds_bgr;
#[doc = "TVE_CLK register accessor: an alias for `Reg<TVE_CLK_SPEC>`"]
pub type TVE_CLK = crate::Reg<tve_clk::TVE_CLK_SPEC>;
#[doc = "TVE Clock Register"]
pub mod tve_clk;
#[doc = "TVE_BGR register accessor: an alias for `Reg<TVE_BGR_SPEC>`"]
pub type TVE_BGR = crate::Reg<tve_bgr::TVE_BGR_SPEC>;
#[doc = "TVE Bus Gating Reset Register"]
pub mod tve_bgr;
#[doc = "TVD_CLK register accessor: an alias for `Reg<TVD_CLK_SPEC>`"]
pub type TVD_CLK = crate::Reg<tvd_clk::TVD_CLK_SPEC>;
#[doc = "TVD Clock Register"]
pub mod tvd_clk;
#[doc = "TVD_BGR register accessor: an alias for `Reg<TVD_BGR_SPEC>`"]
pub type TVD_BGR = crate::Reg<tvd_bgr::TVD_BGR_SPEC>;
#[doc = "TVD Bus Gating Reset Register"]
pub mod tvd_bgr;
#[doc = "LEDC_CLK register accessor: an alias for `Reg<LEDC_CLK_SPEC>`"]
pub type LEDC_CLK = crate::Reg<ledc_clk::LEDC_CLK_SPEC>;
#[doc = "LEDC Clock Register"]
pub mod ledc_clk;
#[doc = "LEDC_BGR register accessor: an alias for `Reg<LEDC_BGR_SPEC>`"]
pub type LEDC_BGR = crate::Reg<ledc_bgr::LEDC_BGR_SPEC>;
#[doc = "LEDC Bus Gating Reset Register"]
pub mod ledc_bgr;
#[doc = "CSI_CLK register accessor: an alias for `Reg<CSI_CLK_SPEC>`"]
pub type CSI_CLK = crate::Reg<csi_clk::CSI_CLK_SPEC>;
#[doc = "CSI Clock Register"]
pub mod csi_clk;
#[doc = "CSI_MASTER_CLK register accessor: an alias for `Reg<CSI_MASTER_CLK_SPEC>`"]
pub type CSI_MASTER_CLK = crate::Reg<csi_master_clk::CSI_MASTER_CLK_SPEC>;
#[doc = "CSI Master Clock Register"]
pub mod csi_master_clk;
#[doc = "CSI_BGR register accessor: an alias for `Reg<CSI_BGR_SPEC>`"]
pub type CSI_BGR = crate::Reg<csi_bgr::CSI_BGR_SPEC>;
#[doc = "CSI Bus Gating Reset Register"]
pub mod csi_bgr;
#[doc = "TPADC_CLK register accessor: an alias for `Reg<TPADC_CLK_SPEC>`"]
pub type TPADC_CLK = crate::Reg<tpadc_clk::TPADC_CLK_SPEC>;
#[doc = "TPADC Clock Register"]
pub mod tpadc_clk;
#[doc = "TPADC_BGR register accessor: an alias for `Reg<TPADC_BGR_SPEC>`"]
pub type TPADC_BGR = crate::Reg<tpadc_bgr::TPADC_BGR_SPEC>;
#[doc = "TPADC Bus Gating Reset Register"]
pub mod tpadc_bgr;
#[doc = "DSP_CLK register accessor: an alias for `Reg<DSP_CLK_SPEC>`"]
pub type DSP_CLK = crate::Reg<dsp_clk::DSP_CLK_SPEC>;
#[doc = "DSP Clock Register"]
pub mod dsp_clk;
#[doc = "DSP_BGR register accessor: an alias for `Reg<DSP_BGR_SPEC>`"]
pub type DSP_BGR = crate::Reg<dsp_bgr::DSP_BGR_SPEC>;
#[doc = "DSP Bus Gating Reset Register"]
pub mod dsp_bgr;
#[doc = "RISCV_CLK register accessor: an alias for `Reg<RISCV_CLK_SPEC>`"]
pub type RISCV_CLK = crate::Reg<riscv_clk::RISCV_CLK_SPEC>;
#[doc = "RISC-V Clock Register"]
pub mod riscv_clk;
#[doc = "RISCV_GATING register accessor: an alias for `Reg<RISCV_GATING_SPEC>`"]
pub type RISCV_GATING = crate::Reg<riscv_gating::RISCV_GATING_SPEC>;
#[doc = "RISC-V GATING Configuration Register"]
pub mod riscv_gating;
#[doc = "RISCV_CFG_BGR register accessor: an alias for `Reg<RISCV_CFG_BGR_SPEC>`"]
pub type RISCV_CFG_BGR = crate::Reg<riscv_cfg_bgr::RISCV_CFG_BGR_SPEC>;
#[doc = "RISC-V_CFG Bus Gating Reset Register"]
pub mod riscv_cfg_bgr;
#[doc = "PLL_LOCK_DBG_CTRL register accessor: an alias for `Reg<PLL_LOCK_DBG_CTRL_SPEC>`"]
pub type PLL_LOCK_DBG_CTRL = crate::Reg<pll_lock_dbg_ctrl::PLL_LOCK_DBG_CTRL_SPEC>;
#[doc = "PLL Lock Debug Control Register"]
pub mod pll_lock_dbg_ctrl;
#[doc = "FRE_DET_CTRL register accessor: an alias for `Reg<FRE_DET_CTRL_SPEC>`"]
pub type FRE_DET_CTRL = crate::Reg<fre_det_ctrl::FRE_DET_CTRL_SPEC>;
#[doc = "Frequency Detect Control Register"]
pub mod fre_det_ctrl;
#[doc = "FRE_UP_LIM register accessor: an alias for `Reg<FRE_UP_LIM_SPEC>`"]
pub type FRE_UP_LIM = crate::Reg<fre_up_lim::FRE_UP_LIM_SPEC>;
#[doc = "Frequency Up Limit Register"]
pub mod fre_up_lim;
#[doc = "FRE_DOWN_LIM register accessor: an alias for `Reg<FRE_DOWN_LIM_SPEC>`"]
pub type FRE_DOWN_LIM = crate::Reg<fre_down_lim::FRE_DOWN_LIM_SPEC>;
#[doc = "Frequency Down Limit Register"]
pub mod fre_down_lim;
#[doc = "CCU_FAN_GATE register accessor: an alias for `Reg<CCU_FAN_GATE_SPEC>`"]
pub type CCU_FAN_GATE = crate::Reg<ccu_fan_gate::CCU_FAN_GATE_SPEC>;
#[doc = "CCU FANOUT CLOCK GATE Register"]
pub mod ccu_fan_gate;
#[doc = "CLK27M_FAN register accessor: an alias for `Reg<CLK27M_FAN_SPEC>`"]
pub type CLK27M_FAN = crate::Reg<clk27m_fan::CLK27M_FAN_SPEC>;
#[doc = "CLK27M FANOUT Register"]
pub mod clk27m_fan;
#[doc = "PCLK_FAN register accessor: an alias for `Reg<PCLK_FAN_SPEC>`"]
pub type PCLK_FAN = crate::Reg<pclk_fan::PCLK_FAN_SPEC>;
#[doc = "PCLK FANOUT Register"]
pub mod pclk_fan;
#[doc = "CCU_FAN register accessor: an alias for `Reg<CCU_FAN_SPEC>`"]
pub type CCU_FAN = crate::Reg<ccu_fan::CCU_FAN_SPEC>;
#[doc = "CCU FANOUT Register"]
pub mod ccu_fan;
