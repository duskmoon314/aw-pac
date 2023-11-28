#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    pll_cpu_ctrl: PLL_CPU_CTRL,
    _reserved1: [u8; 0x0c],
    pll_ddr_ctrl: PLL_DDR_CTRL,
    _reserved2: [u8; 0x0c],
    pll_peri_ctrl: PLL_PERI_CTRL,
    _reserved3: [u8; 0x1c],
    pll_video0_ctrl: PLL_VIDEO0_CTRL,
    _reserved4: [u8; 0x04],
    pll_video1_ctrl: PLL_VIDEO1_CTRL,
    _reserved5: [u8; 0x0c],
    pll_ve_ctrl: PLL_VE_CTRL,
    _reserved6: [u8; 0x1c],
    pll_audio0_ctrl: PLL_AUDIO0_CTRL,
    _reserved7: [u8; 0x04],
    pll_audio1_ctrl: PLL_AUDIO1_CTRL,
    _reserved8: [u8; 0x8c],
    pll_ddr_pat0_ctrl: PLL_DDR_PAT0_CTRL,
    pll_ddr_pat1_ctrl: PLL_DDR_PAT1_CTRL,
    _reserved10: [u8; 0x08],
    pll_peri_pat0_ctrl: PLL_PERI_PAT0_CTRL,
    pll_peri_pat1_ctrl: PLL_PERI_PAT1_CTRL,
    _reserved12: [u8; 0x18],
    pll_video0_pat0_ctrl: PLL_VIDEO0_PAT0_CTRL,
    pll_video0_pat1_ctrl: PLL_VIDEO0_PAT1_CTRL,
    pll_video1_pat0_ctrl: PLL_VIDEO1_PAT0_CTRL,
    pll_video1_pat1_ctrl: PLL_VIDEO1_PAT1_CTRL,
    _reserved16: [u8; 0x08],
    pll_ve_pat0_ctrl: PLL_VE_PAT0_CTRL,
    pll_ve_pat1_ctrl: PLL_VE_PAT1_CTRL,
    _reserved18: [u8; 0x18],
    pll_audio0_pat0_ctrl: PLL_AUDIO0_PAT0_CTRL,
    pll_audio0_pat1_ctrl: PLL_AUDIO0_PAT1_CTRL,
    pll_audio1_pat0_ctrl: PLL_AUDIO1_PAT0_CTRL,
    pll_audio1_pat1_ctrl: PLL_AUDIO1_PAT1_CTRL,
    _reserved22: [u8; 0x0178],
    pll_cpu_bias: PLL_CPU_BIAS,
    _reserved23: [u8; 0x0c],
    pll_ddr_bias: PLL_DDR_BIAS,
    _reserved24: [u8; 0x0c],
    pll_peri_bias: PLL_PERI_BIAS,
    _reserved25: [u8; 0x1c],
    pll_video0_bias: PLL_VIDEO0_BIAS,
    _reserved26: [u8; 0x04],
    pll_video1_bias: PLL_VIDEO1_BIAS,
    _reserved27: [u8; 0x0c],
    pll_ve_bias: PLL_VE_BIAS,
    _reserved28: [u8; 0x1c],
    pll_audio0_bias: PLL_AUDIO0_BIAS,
    _reserved29: [u8; 0x04],
    pll_audio1_bias: PLL_AUDIO1_BIAS,
    _reserved30: [u8; 0x7c],
    pll_cpu_tun: PLL_CPU_TUN,
    _reserved31: [u8; 0xfc],
    cpu_axi_cfg: CPU_AXI_CFG,
    cpu_gating: CPU_GATING,
    _reserved33: [u8; 0x08],
    psi_clk: PSI_CLK,
    _reserved34: [u8; 0x0c],
    apb_clk: [APB_CLK; 2],
    _reserved35: [u8; 0x18],
    mbus_clk: MBUS_CLK,
    _reserved36: [u8; 0xbc],
    de_clk: DE_CLK,
    _reserved37: [u8; 0x08],
    de_bgr: DE_BGR,
    _reserved38: [u8; 0x10],
    di_clk: DI_CLK,
    _reserved39: [u8; 0x08],
    di_bgr: DI_BGR,
    g2d_clk: G2D_CLK,
    _reserved41: [u8; 0x08],
    g2d_bgr: G2D_BGR,
    _reserved42: [u8; 0x40],
    ce_clk: CE_CLK,
    _reserved43: [u8; 0x08],
    ce_bgr: CE_BGR,
    ve_clk: VE_CLK,
    _reserved45: [u8; 0x08],
    ve_bgr: VE_BGR,
    _reserved46: [u8; 0x6c],
    dma_bgr: DMA_BGR,
    _reserved47: [u8; 0x0c],
    msgbox_bgr: MSGBOX_BGR,
    _reserved48: [u8; 0x0c],
    spinlock_bgr: SPINLOCK_BGR,
    _reserved49: [u8; 0x0c],
    hstimer_bgr: HSTIMER_BGR,
    avs_clk: AVS_CLK,
    _reserved51: [u8; 0x48],
    dbgsys_bgr: DBGSYS_BGR,
    _reserved52: [u8; 0x1c],
    pwm_bgr: PWM_BGR,
    _reserved53: [u8; 0x0c],
    iommu_bgr: IOMMU_BGR,
    _reserved54: [u8; 0x40],
    dram_clk: DRAM_CLK,
    mbus_mat_clk_gating: MBUS_MAT_CLK_GATING,
    _reserved56: [u8; 0x04],
    dram_bgr: DRAM_BGR,
    _reserved57: [u8; 0x20],
    smhc0_clk: SMHC0_CLK,
    smhc1_clk: SMHC1_CLK,
    smhc2_clk: SMHC2_CLK,
    _reserved60: [u8; 0x10],
    smhc_bgr: SMHC_BGR,
    _reserved61: [u8; 0xbc],
    uart_bgr: UART_BGR,
    _reserved62: [u8; 0x0c],
    twi_bgr: TWI_BGR,
    _reserved63: [u8; 0x20],
    spi0_clk: SPI0_CLK,
    spi1_clk: SPI1_CLK,
    _reserved65: [u8; 0x24],
    spi_bgr: SPI_BGR,
    emac_25m_clk: EMAC_25M_CLK,
    _reserved67: [u8; 0x08],
    emac_bgr: EMAC_BGR,
    _reserved68: [u8; 0x40],
    irtx_clk: IRTX_CLK,
    _reserved69: [u8; 0x08],
    irtx_bgr: IRTX_BGR,
    _reserved70: [u8; 0x1c],
    gpadc_bgr: GPADC_BGR,
    _reserved71: [u8; 0x0c],
    ths_bgr: THS_BGR,
    _reserved72: [u8; 0x10],
    i2s_clk: [I2S_CLK; 3],
    i2s2_asrc_clk: I2S2_ASRC_CLK,
    i2s_bgr: I2S_BGR,
    owa_tx_clk: OWA_TX_CLK,
    owa_rx_clk: OWA_RX_CLK,
    owa_bgr: OWA_BGR,
    _reserved78: [u8; 0x10],
    dmic_clk: DMIC_CLK,
    _reserved79: [u8; 0x08],
    dmic_bgr: DMIC_BGR,
    audio_codec_dac_clk: AUDIO_CODEC_DAC_CLK,
    audio_codec_adc_clk: AUDIO_CODEC_ADC_CLK,
    _reserved82: [u8; 0x04],
    audio_codec_bgr: AUDIO_CODEC_BGR,
    _reserved83: [u8; 0x10],
    usb0_clk: USB0_CLK,
    usb1_clk: USB1_CLK,
    _reserved85: [u8; 0x14],
    usb_bgr: USB_BGR,
    _reserved86: [u8; 0x0c],
    lradc_bgr: LRADC_BGR,
    _reserved87: [u8; 0x1c],
    dpss_top_bgr: DPSS_TOP_BGR,
    _reserved88: [u8; 0x64],
    dsi_clk: DSI_CLK,
    _reserved89: [u8; 0x24],
    dsi_bgr: DSI_BGR,
    _reserved90: [u8; 0x10],
    tconlcd_clk: TCONLCD_CLK,
    _reserved91: [u8; 0x18],
    tconlcd_bgr: TCONLCD_BGR,
    tcontv_clk: TCONTV_CLK,
    _reserved93: [u8; 0x18],
    tcontv_bgr: TCONTV_BGR,
    _reserved94: [u8; 0x0c],
    lvds_bgr: LVDS_BGR,
    tve_clk: TVE_CLK,
    _reserved96: [u8; 0x08],
    tve_bgr: TVE_BGR,
    tvd_clk: TVD_CLK,
    _reserved98: [u8; 0x18],
    tvd_bgr: TVD_BGR,
    _reserved99: [u8; 0x10],
    ledc_clk: LEDC_CLK,
    _reserved100: [u8; 0x08],
    ledc_bgr: LEDC_BGR,
    _reserved101: [u8; 0x04],
    csi_clk: CSI_CLK,
    csi_master_clk: CSI_MASTER_CLK,
    _reserved103: [u8; 0x10],
    csi_bgr: CSI_BGR,
    _reserved104: [u8; 0x30],
    tpadc_clk: TPADC_CLK,
    _reserved105: [u8; 0x08],
    tpadc_bgr: TPADC_BGR,
    _reserved106: [u8; 0x10],
    dsp_clk: DSP_CLK,
    _reserved107: [u8; 0x08],
    dsp_bgr: DSP_BGR,
    _reserved108: [u8; 0x80],
    riscv_clk: RISCV_CLK,
    riscv_gating: RISCV_GATING,
    _reserved110: [u8; 0x04],
    riscv_cfg_bgr: RISCV_CFG_BGR,
    _reserved111: [u8; 0x01f4],
    pll_lock_dbg_ctrl: PLL_LOCK_DBG_CTRL,
    fre_det_ctrl: FRE_DET_CTRL,
    fre_up_lim: FRE_UP_LIM,
    fre_down_lim: FRE_DOWN_LIM,
    _reserved115: [u8; 0x1c],
    ccu_fan_gate: CCU_FAN_GATE,
    clk27m_fan: CLK27M_FAN,
    pclk_fan: PCLK_FAN,
    ccu_fan: CCU_FAN,
}
impl RegisterBlock {
    #[doc = "0x00 - PLL_CPU Control Register"]
    #[inline(always)]
    pub const fn pll_cpu_ctrl(&self) -> &PLL_CPU_CTRL {
        &self.pll_cpu_ctrl
    }
    #[doc = "0x10 - PLL_DDR Control Register"]
    #[inline(always)]
    pub const fn pll_ddr_ctrl(&self) -> &PLL_DDR_CTRL {
        &self.pll_ddr_ctrl
    }
    #[doc = "0x20 - PLL_PERI Control Register"]
    #[inline(always)]
    pub const fn pll_peri_ctrl(&self) -> &PLL_PERI_CTRL {
        &self.pll_peri_ctrl
    }
    #[doc = "0x40 - PLL_VIDEO0 Control Register"]
    #[inline(always)]
    pub const fn pll_video0_ctrl(&self) -> &PLL_VIDEO0_CTRL {
        &self.pll_video0_ctrl
    }
    #[doc = "0x48 - PLL_VIDEO1 Control Register"]
    #[inline(always)]
    pub const fn pll_video1_ctrl(&self) -> &PLL_VIDEO1_CTRL {
        &self.pll_video1_ctrl
    }
    #[doc = "0x58 - PLL_VE Control Register"]
    #[inline(always)]
    pub const fn pll_ve_ctrl(&self) -> &PLL_VE_CTRL {
        &self.pll_ve_ctrl
    }
    #[doc = "0x78 - PLL_AUDIO0 Control Register"]
    #[inline(always)]
    pub const fn pll_audio0_ctrl(&self) -> &PLL_AUDIO0_CTRL {
        &self.pll_audio0_ctrl
    }
    #[doc = "0x80 - PLL_AUDIO1 Control Register"]
    #[inline(always)]
    pub const fn pll_audio1_ctrl(&self) -> &PLL_AUDIO1_CTRL {
        &self.pll_audio1_ctrl
    }
    #[doc = "0x110 - PLL_DDR Pattern0 Control Register"]
    #[inline(always)]
    pub const fn pll_ddr_pat0_ctrl(&self) -> &PLL_DDR_PAT0_CTRL {
        &self.pll_ddr_pat0_ctrl
    }
    #[doc = "0x114 - PLL_DDR Pattern1 Control Register"]
    #[inline(always)]
    pub const fn pll_ddr_pat1_ctrl(&self) -> &PLL_DDR_PAT1_CTRL {
        &self.pll_ddr_pat1_ctrl
    }
    #[doc = "0x120 - PLL_PERI Pattern0 Control Register"]
    #[inline(always)]
    pub const fn pll_peri_pat0_ctrl(&self) -> &PLL_PERI_PAT0_CTRL {
        &self.pll_peri_pat0_ctrl
    }
    #[doc = "0x124 - PLL_PERI Pattern1 Control Register"]
    #[inline(always)]
    pub const fn pll_peri_pat1_ctrl(&self) -> &PLL_PERI_PAT1_CTRL {
        &self.pll_peri_pat1_ctrl
    }
    #[doc = "0x140 - PLL_VIDEO0 Pattern0 Control Register"]
    #[inline(always)]
    pub const fn pll_video0_pat0_ctrl(&self) -> &PLL_VIDEO0_PAT0_CTRL {
        &self.pll_video0_pat0_ctrl
    }
    #[doc = "0x144 - PLL_VIDEO0 Pattern1 Control Register"]
    #[inline(always)]
    pub const fn pll_video0_pat1_ctrl(&self) -> &PLL_VIDEO0_PAT1_CTRL {
        &self.pll_video0_pat1_ctrl
    }
    #[doc = "0x148 - PLL_VIDEO1 Pattern0 Control Register"]
    #[inline(always)]
    pub const fn pll_video1_pat0_ctrl(&self) -> &PLL_VIDEO1_PAT0_CTRL {
        &self.pll_video1_pat0_ctrl
    }
    #[doc = "0x14c - PLL_VIDEO1 Pattern1 Control Register"]
    #[inline(always)]
    pub const fn pll_video1_pat1_ctrl(&self) -> &PLL_VIDEO1_PAT1_CTRL {
        &self.pll_video1_pat1_ctrl
    }
    #[doc = "0x158 - PLL_VE Pattern0 Control Register"]
    #[inline(always)]
    pub const fn pll_ve_pat0_ctrl(&self) -> &PLL_VE_PAT0_CTRL {
        &self.pll_ve_pat0_ctrl
    }
    #[doc = "0x15c - PLL_VE Pattern1 Control Register"]
    #[inline(always)]
    pub const fn pll_ve_pat1_ctrl(&self) -> &PLL_VE_PAT1_CTRL {
        &self.pll_ve_pat1_ctrl
    }
    #[doc = "0x178 - PLL_AUDIO0 Pattern0 Control Register"]
    #[inline(always)]
    pub const fn pll_audio0_pat0_ctrl(&self) -> &PLL_AUDIO0_PAT0_CTRL {
        &self.pll_audio0_pat0_ctrl
    }
    #[doc = "0x17c - PLL_AUDIO0 Pattern1 Control Register"]
    #[inline(always)]
    pub const fn pll_audio0_pat1_ctrl(&self) -> &PLL_AUDIO0_PAT1_CTRL {
        &self.pll_audio0_pat1_ctrl
    }
    #[doc = "0x180 - PLL_AUDIO1 Pattern0 Control Register"]
    #[inline(always)]
    pub const fn pll_audio1_pat0_ctrl(&self) -> &PLL_AUDIO1_PAT0_CTRL {
        &self.pll_audio1_pat0_ctrl
    }
    #[doc = "0x184 - PLL_AUDIO1 Pattern1 Control Register"]
    #[inline(always)]
    pub const fn pll_audio1_pat1_ctrl(&self) -> &PLL_AUDIO1_PAT1_CTRL {
        &self.pll_audio1_pat1_ctrl
    }
    #[doc = "0x300 - PLL_CPU Bias Register"]
    #[inline(always)]
    pub const fn pll_cpu_bias(&self) -> &PLL_CPU_BIAS {
        &self.pll_cpu_bias
    }
    #[doc = "0x310 - PLL_DDR Bias Register"]
    #[inline(always)]
    pub const fn pll_ddr_bias(&self) -> &PLL_DDR_BIAS {
        &self.pll_ddr_bias
    }
    #[doc = "0x320 - PLL_PERI Bias Register"]
    #[inline(always)]
    pub const fn pll_peri_bias(&self) -> &PLL_PERI_BIAS {
        &self.pll_peri_bias
    }
    #[doc = "0x340 - PLL_VIDEO0 Bias Register"]
    #[inline(always)]
    pub const fn pll_video0_bias(&self) -> &PLL_VIDEO0_BIAS {
        &self.pll_video0_bias
    }
    #[doc = "0x348 - PLL_VIDEO1 Bias Register"]
    #[inline(always)]
    pub const fn pll_video1_bias(&self) -> &PLL_VIDEO1_BIAS {
        &self.pll_video1_bias
    }
    #[doc = "0x358 - PLL_VE Bias Register"]
    #[inline(always)]
    pub const fn pll_ve_bias(&self) -> &PLL_VE_BIAS {
        &self.pll_ve_bias
    }
    #[doc = "0x378 - PLL_AUDIO0 Bias Register"]
    #[inline(always)]
    pub const fn pll_audio0_bias(&self) -> &PLL_AUDIO0_BIAS {
        &self.pll_audio0_bias
    }
    #[doc = "0x380 - PLL_AUDIO1 Bias Register"]
    #[inline(always)]
    pub const fn pll_audio1_bias(&self) -> &PLL_AUDIO1_BIAS {
        &self.pll_audio1_bias
    }
    #[doc = "0x400 - PLL_CPU Tuning Register"]
    #[inline(always)]
    pub const fn pll_cpu_tun(&self) -> &PLL_CPU_TUN {
        &self.pll_cpu_tun
    }
    #[doc = "0x500 - CPU_AXI Configuration Register"]
    #[inline(always)]
    pub const fn cpu_axi_cfg(&self) -> &CPU_AXI_CFG {
        &self.cpu_axi_cfg
    }
    #[doc = "0x504 - CPU_GATING Configuration Register"]
    #[inline(always)]
    pub const fn cpu_gating(&self) -> &CPU_GATING {
        &self.cpu_gating
    }
    #[doc = "0x510 - PSI Clock Register"]
    #[inline(always)]
    pub const fn psi_clk(&self) -> &PSI_CLK {
        &self.psi_clk
    }
    #[doc = "0x520..0x528 - APB Clock Register"]
    #[inline(always)]
    pub const fn apb_clk(&self, n: usize) -> &APB_CLK {
        &self.apb_clk[n]
    }
    #[doc = "0x520 - APB Clock Register"]
    #[inline(always)]
    pub const fn apb0_clk(&self) -> &APB_CLK {
        self.apb_clk(0)
    }
    #[doc = "0x524 - APB Clock Register"]
    #[inline(always)]
    pub const fn apb1_clk(&self) -> &APB_CLK {
        self.apb_clk(1)
    }
    #[doc = "0x540 - MBUS Clock Register"]
    #[inline(always)]
    pub const fn mbus_clk(&self) -> &MBUS_CLK {
        &self.mbus_clk
    }
    #[doc = "0x600 - DE Clock Register"]
    #[inline(always)]
    pub const fn de_clk(&self) -> &DE_CLK {
        &self.de_clk
    }
    #[doc = "0x60c - DE Bus Gating Reset Register"]
    #[inline(always)]
    pub const fn de_bgr(&self) -> &DE_BGR {
        &self.de_bgr
    }
    #[doc = "0x620 - DI Clock Register"]
    #[inline(always)]
    pub const fn di_clk(&self) -> &DI_CLK {
        &self.di_clk
    }
    #[doc = "0x62c - DI Bus Gating Reset Register"]
    #[inline(always)]
    pub const fn di_bgr(&self) -> &DI_BGR {
        &self.di_bgr
    }
    #[doc = "0x630 - G2D Clock Register"]
    #[inline(always)]
    pub const fn g2d_clk(&self) -> &G2D_CLK {
        &self.g2d_clk
    }
    #[doc = "0x63c - G2D Bus Gating Reset Register"]
    #[inline(always)]
    pub const fn g2d_bgr(&self) -> &G2D_BGR {
        &self.g2d_bgr
    }
    #[doc = "0x680 - CE Clock Register"]
    #[inline(always)]
    pub const fn ce_clk(&self) -> &CE_CLK {
        &self.ce_clk
    }
    #[doc = "0x68c - CE Bus Gating Reset Register"]
    #[inline(always)]
    pub const fn ce_bgr(&self) -> &CE_BGR {
        &self.ce_bgr
    }
    #[doc = "0x690 - VE Clock Register"]
    #[inline(always)]
    pub const fn ve_clk(&self) -> &VE_CLK {
        &self.ve_clk
    }
    #[doc = "0x69c - VE Bus Gating Reset Register"]
    #[inline(always)]
    pub const fn ve_bgr(&self) -> &VE_BGR {
        &self.ve_bgr
    }
    #[doc = "0x70c - DMA Bus Gating Reset Register"]
    #[inline(always)]
    pub const fn dma_bgr(&self) -> &DMA_BGR {
        &self.dma_bgr
    }
    #[doc = "0x71c - MSGBOX Bus Gating Reset Register"]
    #[inline(always)]
    pub const fn msgbox_bgr(&self) -> &MSGBOX_BGR {
        &self.msgbox_bgr
    }
    #[doc = "0x72c - SPINLOCK Bus Gating Reset Register"]
    #[inline(always)]
    pub const fn spinlock_bgr(&self) -> &SPINLOCK_BGR {
        &self.spinlock_bgr
    }
    #[doc = "0x73c - HSTIMER Bus Gating Reset Register"]
    #[inline(always)]
    pub const fn hstimer_bgr(&self) -> &HSTIMER_BGR {
        &self.hstimer_bgr
    }
    #[doc = "0x740 - AVS Clock Register"]
    #[inline(always)]
    pub const fn avs_clk(&self) -> &AVS_CLK {
        &self.avs_clk
    }
    #[doc = "0x78c - DBGSYS Bus Gating Reset Register"]
    #[inline(always)]
    pub const fn dbgsys_bgr(&self) -> &DBGSYS_BGR {
        &self.dbgsys_bgr
    }
    #[doc = "0x7ac - PWM Bus Gating Reset Register"]
    #[inline(always)]
    pub const fn pwm_bgr(&self) -> &PWM_BGR {
        &self.pwm_bgr
    }
    #[doc = "0x7bc - IOMMU Bus Gating Reset Register"]
    #[inline(always)]
    pub const fn iommu_bgr(&self) -> &IOMMU_BGR {
        &self.iommu_bgr
    }
    #[doc = "0x800 - DRAM Clock Register"]
    #[inline(always)]
    pub const fn dram_clk(&self) -> &DRAM_CLK {
        &self.dram_clk
    }
    #[doc = "0x804 - MBUS Master Clock Gating Register"]
    #[inline(always)]
    pub const fn mbus_mat_clk_gating(&self) -> &MBUS_MAT_CLK_GATING {
        &self.mbus_mat_clk_gating
    }
    #[doc = "0x80c - DRAM Bus Gating Reset Register"]
    #[inline(always)]
    pub const fn dram_bgr(&self) -> &DRAM_BGR {
        &self.dram_bgr
    }
    #[doc = "0x830 - SMHC0 Clock Register"]
    #[inline(always)]
    pub const fn smhc0_clk(&self) -> &SMHC0_CLK {
        &self.smhc0_clk
    }
    #[doc = "0x834 - SMHC1 Clock Register"]
    #[inline(always)]
    pub const fn smhc1_clk(&self) -> &SMHC1_CLK {
        &self.smhc1_clk
    }
    #[doc = "0x838 - SMHC2 Clock Register"]
    #[inline(always)]
    pub const fn smhc2_clk(&self) -> &SMHC2_CLK {
        &self.smhc2_clk
    }
    #[doc = "0x84c - SMHC Bus Gating Reset Register"]
    #[inline(always)]
    pub const fn smhc_bgr(&self) -> &SMHC_BGR {
        &self.smhc_bgr
    }
    #[doc = "0x90c - UART Bus Gating Reset Register"]
    #[inline(always)]
    pub const fn uart_bgr(&self) -> &UART_BGR {
        &self.uart_bgr
    }
    #[doc = "0x91c - TWI Bus Gating Reset Register"]
    #[inline(always)]
    pub const fn twi_bgr(&self) -> &TWI_BGR {
        &self.twi_bgr
    }
    #[doc = "0x940 - SPI0 Clock Register"]
    #[inline(always)]
    pub const fn spi0_clk(&self) -> &SPI0_CLK {
        &self.spi0_clk
    }
    #[doc = "0x944 - SPI1 Clock Register"]
    #[inline(always)]
    pub const fn spi1_clk(&self) -> &SPI1_CLK {
        &self.spi1_clk
    }
    #[doc = "0x96c - SPI Bus Gating Reset Register"]
    #[inline(always)]
    pub const fn spi_bgr(&self) -> &SPI_BGR {
        &self.spi_bgr
    }
    #[doc = "0x970 - EMAC_25M Clock Register"]
    #[inline(always)]
    pub const fn emac_25m_clk(&self) -> &EMAC_25M_CLK {
        &self.emac_25m_clk
    }
    #[doc = "0x97c - EMAC Bus Gating Reset Register"]
    #[inline(always)]
    pub const fn emac_bgr(&self) -> &EMAC_BGR {
        &self.emac_bgr
    }
    #[doc = "0x9c0 - IRTX Clock Register"]
    #[inline(always)]
    pub const fn irtx_clk(&self) -> &IRTX_CLK {
        &self.irtx_clk
    }
    #[doc = "0x9cc - IRTX Bus Gating Reset Register"]
    #[inline(always)]
    pub const fn irtx_bgr(&self) -> &IRTX_BGR {
        &self.irtx_bgr
    }
    #[doc = "0x9ec - GPADC Bus Gating Reset Register"]
    #[inline(always)]
    pub const fn gpadc_bgr(&self) -> &GPADC_BGR {
        &self.gpadc_bgr
    }
    #[doc = "0x9fc - THS Bus Gating Reset Register"]
    #[inline(always)]
    pub const fn ths_bgr(&self) -> &THS_BGR {
        &self.ths_bgr
    }
    #[doc = "0xa10..0xa1c - I2S Clock Register"]
    #[inline(always)]
    pub const fn i2s_clk(&self, n: usize) -> &I2S_CLK {
        &self.i2s_clk[n]
    }
    #[doc = "0xa10 - I2S Clock Register"]
    #[inline(always)]
    pub const fn i2s0_clk(&self) -> &I2S_CLK {
        self.i2s_clk(0)
    }
    #[doc = "0xa14 - I2S Clock Register"]
    #[inline(always)]
    pub const fn i2s1_clk(&self) -> &I2S_CLK {
        self.i2s_clk(1)
    }
    #[doc = "0xa18 - I2S Clock Register"]
    #[inline(always)]
    pub const fn i2s2_clk(&self) -> &I2S_CLK {
        self.i2s_clk(2)
    }
    #[doc = "0xa1c - I2S2_ASRC Clock Register"]
    #[inline(always)]
    pub const fn i2s2_asrc_clk(&self) -> &I2S2_ASRC_CLK {
        &self.i2s2_asrc_clk
    }
    #[doc = "0xa20 - I2S Bus Gating Reset Register"]
    #[inline(always)]
    pub const fn i2s_bgr(&self) -> &I2S_BGR {
        &self.i2s_bgr
    }
    #[doc = "0xa24 - OWA_TX Clock Register"]
    #[inline(always)]
    pub const fn owa_tx_clk(&self) -> &OWA_TX_CLK {
        &self.owa_tx_clk
    }
    #[doc = "0xa28 - OWA_RX Clock Register"]
    #[inline(always)]
    pub const fn owa_rx_clk(&self) -> &OWA_RX_CLK {
        &self.owa_rx_clk
    }
    #[doc = "0xa2c - OWA Bus Gating Reset Register"]
    #[inline(always)]
    pub const fn owa_bgr(&self) -> &OWA_BGR {
        &self.owa_bgr
    }
    #[doc = "0xa40 - DMIC Clock Register"]
    #[inline(always)]
    pub const fn dmic_clk(&self) -> &DMIC_CLK {
        &self.dmic_clk
    }
    #[doc = "0xa4c - DMIC Bus Gating Reset Register"]
    #[inline(always)]
    pub const fn dmic_bgr(&self) -> &DMIC_BGR {
        &self.dmic_bgr
    }
    #[doc = "0xa50 - AUDIO_CODEC_DAC Clock Register"]
    #[inline(always)]
    pub const fn audio_codec_dac_clk(&self) -> &AUDIO_CODEC_DAC_CLK {
        &self.audio_codec_dac_clk
    }
    #[doc = "0xa54 - AUDIO_CODEC_ADC Clock Register"]
    #[inline(always)]
    pub const fn audio_codec_adc_clk(&self) -> &AUDIO_CODEC_ADC_CLK {
        &self.audio_codec_adc_clk
    }
    #[doc = "0xa5c - AUDIO_CODEC Bus Gating Reset Register"]
    #[inline(always)]
    pub const fn audio_codec_bgr(&self) -> &AUDIO_CODEC_BGR {
        &self.audio_codec_bgr
    }
    #[doc = "0xa70 - USB0 Clock Register"]
    #[inline(always)]
    pub const fn usb0_clk(&self) -> &USB0_CLK {
        &self.usb0_clk
    }
    #[doc = "0xa74 - USB1 Clock Register"]
    #[inline(always)]
    pub const fn usb1_clk(&self) -> &USB1_CLK {
        &self.usb1_clk
    }
    #[doc = "0xa8c - USB Bus Gating Reset Register"]
    #[inline(always)]
    pub const fn usb_bgr(&self) -> &USB_BGR {
        &self.usb_bgr
    }
    #[doc = "0xa9c - LRADC Bus Gating Reset Register"]
    #[inline(always)]
    pub const fn lradc_bgr(&self) -> &LRADC_BGR {
        &self.lradc_bgr
    }
    #[doc = "0xabc - DPSS_TOP Bus Gating Reset Register"]
    #[inline(always)]
    pub const fn dpss_top_bgr(&self) -> &DPSS_TOP_BGR {
        &self.dpss_top_bgr
    }
    #[doc = "0xb24 - DSI Clock Register"]
    #[inline(always)]
    pub const fn dsi_clk(&self) -> &DSI_CLK {
        &self.dsi_clk
    }
    #[doc = "0xb4c - DSI Bus Gating Reset Register"]
    #[inline(always)]
    pub const fn dsi_bgr(&self) -> &DSI_BGR {
        &self.dsi_bgr
    }
    #[doc = "0xb60 - TCONLCD Clock Register"]
    #[inline(always)]
    pub const fn tconlcd_clk(&self) -> &TCONLCD_CLK {
        &self.tconlcd_clk
    }
    #[doc = "0xb7c - TCONLCD Bus Gating Reset Register"]
    #[inline(always)]
    pub const fn tconlcd_bgr(&self) -> &TCONLCD_BGR {
        &self.tconlcd_bgr
    }
    #[doc = "0xb80 - TCONTV Clock Register"]
    #[inline(always)]
    pub const fn tcontv_clk(&self) -> &TCONTV_CLK {
        &self.tcontv_clk
    }
    #[doc = "0xb9c - TCONTV Bus Gating Reset Register"]
    #[inline(always)]
    pub const fn tcontv_bgr(&self) -> &TCONTV_BGR {
        &self.tcontv_bgr
    }
    #[doc = "0xbac - LVDS Bus Gating Reset Register"]
    #[inline(always)]
    pub const fn lvds_bgr(&self) -> &LVDS_BGR {
        &self.lvds_bgr
    }
    #[doc = "0xbb0 - TVE Clock Register"]
    #[inline(always)]
    pub const fn tve_clk(&self) -> &TVE_CLK {
        &self.tve_clk
    }
    #[doc = "0xbbc - TVE Bus Gating Reset Register"]
    #[inline(always)]
    pub const fn tve_bgr(&self) -> &TVE_BGR {
        &self.tve_bgr
    }
    #[doc = "0xbc0 - TVD Clock Register"]
    #[inline(always)]
    pub const fn tvd_clk(&self) -> &TVD_CLK {
        &self.tvd_clk
    }
    #[doc = "0xbdc - TVD Bus Gating Reset Register"]
    #[inline(always)]
    pub const fn tvd_bgr(&self) -> &TVD_BGR {
        &self.tvd_bgr
    }
    #[doc = "0xbf0 - LEDC Clock Register"]
    #[inline(always)]
    pub const fn ledc_clk(&self) -> &LEDC_CLK {
        &self.ledc_clk
    }
    #[doc = "0xbfc - LEDC Bus Gating Reset Register"]
    #[inline(always)]
    pub const fn ledc_bgr(&self) -> &LEDC_BGR {
        &self.ledc_bgr
    }
    #[doc = "0xc04 - CSI Clock Register"]
    #[inline(always)]
    pub const fn csi_clk(&self) -> &CSI_CLK {
        &self.csi_clk
    }
    #[doc = "0xc08 - CSI Master Clock Register"]
    #[inline(always)]
    pub const fn csi_master_clk(&self) -> &CSI_MASTER_CLK {
        &self.csi_master_clk
    }
    #[doc = "0xc1c - CSI Bus Gating Reset Register"]
    #[inline(always)]
    pub const fn csi_bgr(&self) -> &CSI_BGR {
        &self.csi_bgr
    }
    #[doc = "0xc50 - TPADC Clock Register"]
    #[inline(always)]
    pub const fn tpadc_clk(&self) -> &TPADC_CLK {
        &self.tpadc_clk
    }
    #[doc = "0xc5c - TPADC Bus Gating Reset Register"]
    #[inline(always)]
    pub const fn tpadc_bgr(&self) -> &TPADC_BGR {
        &self.tpadc_bgr
    }
    #[doc = "0xc70 - DSP Clock Register"]
    #[inline(always)]
    pub const fn dsp_clk(&self) -> &DSP_CLK {
        &self.dsp_clk
    }
    #[doc = "0xc7c - DSP Bus Gating Reset Register"]
    #[inline(always)]
    pub const fn dsp_bgr(&self) -> &DSP_BGR {
        &self.dsp_bgr
    }
    #[doc = "0xd00 - RISC-V Clock Register"]
    #[inline(always)]
    pub const fn riscv_clk(&self) -> &RISCV_CLK {
        &self.riscv_clk
    }
    #[doc = "0xd04 - RISC-V GATING Configuration Register"]
    #[inline(always)]
    pub const fn riscv_gating(&self) -> &RISCV_GATING {
        &self.riscv_gating
    }
    #[doc = "0xd0c - RISC-V_CFG Bus Gating Reset Register"]
    #[inline(always)]
    pub const fn riscv_cfg_bgr(&self) -> &RISCV_CFG_BGR {
        &self.riscv_cfg_bgr
    }
    #[doc = "0xf04 - PLL Lock Debug Control Register"]
    #[inline(always)]
    pub const fn pll_lock_dbg_ctrl(&self) -> &PLL_LOCK_DBG_CTRL {
        &self.pll_lock_dbg_ctrl
    }
    #[doc = "0xf08 - Frequency Detect Control Register"]
    #[inline(always)]
    pub const fn fre_det_ctrl(&self) -> &FRE_DET_CTRL {
        &self.fre_det_ctrl
    }
    #[doc = "0xf0c - Frequency Up Limit Register"]
    #[inline(always)]
    pub const fn fre_up_lim(&self) -> &FRE_UP_LIM {
        &self.fre_up_lim
    }
    #[doc = "0xf10 - Frequency Down Limit Register"]
    #[inline(always)]
    pub const fn fre_down_lim(&self) -> &FRE_DOWN_LIM {
        &self.fre_down_lim
    }
    #[doc = "0xf30 - CCU FANOUT CLOCK GATE Register"]
    #[inline(always)]
    pub const fn ccu_fan_gate(&self) -> &CCU_FAN_GATE {
        &self.ccu_fan_gate
    }
    #[doc = "0xf34 - CLK27M FANOUT Register"]
    #[inline(always)]
    pub const fn clk27m_fan(&self) -> &CLK27M_FAN {
        &self.clk27m_fan
    }
    #[doc = "0xf38 - PCLK FANOUT Register"]
    #[inline(always)]
    pub const fn pclk_fan(&self) -> &PCLK_FAN {
        &self.pclk_fan
    }
    #[doc = "0xf3c - CCU FANOUT Register"]
    #[inline(always)]
    pub const fn ccu_fan(&self) -> &CCU_FAN {
        &self.ccu_fan
    }
}
#[doc = "pll_cpu_ctrl (rw) register accessor: PLL_CPU Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_cpu_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_cpu_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_cpu_ctrl`] module"]
pub type PLL_CPU_CTRL = crate::Reg<pll_cpu_ctrl::PLL_CPU_CTRL_SPEC>;
#[doc = "PLL_CPU Control Register"]
pub mod pll_cpu_ctrl;
#[doc = "pll_ddr_ctrl (rw) register accessor: PLL_DDR Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_ddr_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_ddr_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_ddr_ctrl`] module"]
pub type PLL_DDR_CTRL = crate::Reg<pll_ddr_ctrl::PLL_DDR_CTRL_SPEC>;
#[doc = "PLL_DDR Control Register"]
pub mod pll_ddr_ctrl;
#[doc = "pll_peri_ctrl (rw) register accessor: PLL_PERI Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_peri_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_peri_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_peri_ctrl`] module"]
pub type PLL_PERI_CTRL = crate::Reg<pll_peri_ctrl::PLL_PERI_CTRL_SPEC>;
#[doc = "PLL_PERI Control Register"]
pub mod pll_peri_ctrl;
#[doc = "pll_video0_ctrl (rw) register accessor: PLL_VIDEO0 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_video0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_video0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_video0_ctrl`] module"]
pub type PLL_VIDEO0_CTRL = crate::Reg<pll_video0_ctrl::PLL_VIDEO0_CTRL_SPEC>;
#[doc = "PLL_VIDEO0 Control Register"]
pub mod pll_video0_ctrl;
#[doc = "pll_video1_ctrl (rw) register accessor: PLL_VIDEO1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_video1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_video1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_video1_ctrl`] module"]
pub type PLL_VIDEO1_CTRL = crate::Reg<pll_video1_ctrl::PLL_VIDEO1_CTRL_SPEC>;
#[doc = "PLL_VIDEO1 Control Register"]
pub mod pll_video1_ctrl;
#[doc = "pll_ve_ctrl (rw) register accessor: PLL_VE Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_ve_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_ve_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_ve_ctrl`] module"]
pub type PLL_VE_CTRL = crate::Reg<pll_ve_ctrl::PLL_VE_CTRL_SPEC>;
#[doc = "PLL_VE Control Register"]
pub mod pll_ve_ctrl;
#[doc = "pll_audio0_ctrl (rw) register accessor: PLL_AUDIO0 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_audio0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_audio0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_audio0_ctrl`] module"]
pub type PLL_AUDIO0_CTRL = crate::Reg<pll_audio0_ctrl::PLL_AUDIO0_CTRL_SPEC>;
#[doc = "PLL_AUDIO0 Control Register"]
pub mod pll_audio0_ctrl;
#[doc = "pll_audio1_ctrl (rw) register accessor: PLL_AUDIO1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_audio1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_audio1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_audio1_ctrl`] module"]
pub type PLL_AUDIO1_CTRL = crate::Reg<pll_audio1_ctrl::PLL_AUDIO1_CTRL_SPEC>;
#[doc = "PLL_AUDIO1 Control Register"]
pub mod pll_audio1_ctrl;
#[doc = "pll_ddr_pat0_ctrl (rw) register accessor: PLL_DDR Pattern0 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_ddr_pat0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_ddr_pat0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_ddr_pat0_ctrl`] module"]
pub type PLL_DDR_PAT0_CTRL = crate::Reg<pll_ddr_pat0_ctrl::PLL_DDR_PAT0_CTRL_SPEC>;
#[doc = "PLL_DDR Pattern0 Control Register"]
pub mod pll_ddr_pat0_ctrl;
#[doc = "pll_ddr_pat1_ctrl (rw) register accessor: PLL_DDR Pattern1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_ddr_pat1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_ddr_pat1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_ddr_pat1_ctrl`] module"]
pub type PLL_DDR_PAT1_CTRL = crate::Reg<pll_ddr_pat1_ctrl::PLL_DDR_PAT1_CTRL_SPEC>;
#[doc = "PLL_DDR Pattern1 Control Register"]
pub mod pll_ddr_pat1_ctrl;
#[doc = "pll_peri_pat0_ctrl (rw) register accessor: PLL_PERI Pattern0 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_peri_pat0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_peri_pat0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_peri_pat0_ctrl`] module"]
pub type PLL_PERI_PAT0_CTRL = crate::Reg<pll_peri_pat0_ctrl::PLL_PERI_PAT0_CTRL_SPEC>;
#[doc = "PLL_PERI Pattern0 Control Register"]
pub mod pll_peri_pat0_ctrl;
#[doc = "pll_peri_pat1_ctrl (rw) register accessor: PLL_PERI Pattern1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_peri_pat1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_peri_pat1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_peri_pat1_ctrl`] module"]
pub type PLL_PERI_PAT1_CTRL = crate::Reg<pll_peri_pat1_ctrl::PLL_PERI_PAT1_CTRL_SPEC>;
#[doc = "PLL_PERI Pattern1 Control Register"]
pub mod pll_peri_pat1_ctrl;
#[doc = "pll_video0_pat0_ctrl (rw) register accessor: PLL_VIDEO0 Pattern0 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_video0_pat0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_video0_pat0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_video0_pat0_ctrl`] module"]
pub type PLL_VIDEO0_PAT0_CTRL = crate::Reg<pll_video0_pat0_ctrl::PLL_VIDEO0_PAT0_CTRL_SPEC>;
#[doc = "PLL_VIDEO0 Pattern0 Control Register"]
pub mod pll_video0_pat0_ctrl;
#[doc = "pll_video0_pat1_ctrl (rw) register accessor: PLL_VIDEO0 Pattern1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_video0_pat1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_video0_pat1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_video0_pat1_ctrl`] module"]
pub type PLL_VIDEO0_PAT1_CTRL = crate::Reg<pll_video0_pat1_ctrl::PLL_VIDEO0_PAT1_CTRL_SPEC>;
#[doc = "PLL_VIDEO0 Pattern1 Control Register"]
pub mod pll_video0_pat1_ctrl;
#[doc = "pll_video1_pat0_ctrl (rw) register accessor: PLL_VIDEO1 Pattern0 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_video1_pat0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_video1_pat0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_video1_pat0_ctrl`] module"]
pub type PLL_VIDEO1_PAT0_CTRL = crate::Reg<pll_video1_pat0_ctrl::PLL_VIDEO1_PAT0_CTRL_SPEC>;
#[doc = "PLL_VIDEO1 Pattern0 Control Register"]
pub mod pll_video1_pat0_ctrl;
#[doc = "pll_video1_pat1_ctrl (rw) register accessor: PLL_VIDEO1 Pattern1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_video1_pat1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_video1_pat1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_video1_pat1_ctrl`] module"]
pub type PLL_VIDEO1_PAT1_CTRL = crate::Reg<pll_video1_pat1_ctrl::PLL_VIDEO1_PAT1_CTRL_SPEC>;
#[doc = "PLL_VIDEO1 Pattern1 Control Register"]
pub mod pll_video1_pat1_ctrl;
#[doc = "pll_ve_pat0_ctrl (rw) register accessor: PLL_VE Pattern0 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_ve_pat0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_ve_pat0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_ve_pat0_ctrl`] module"]
pub type PLL_VE_PAT0_CTRL = crate::Reg<pll_ve_pat0_ctrl::PLL_VE_PAT0_CTRL_SPEC>;
#[doc = "PLL_VE Pattern0 Control Register"]
pub mod pll_ve_pat0_ctrl;
#[doc = "pll_ve_pat1_ctrl (rw) register accessor: PLL_VE Pattern1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_ve_pat1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_ve_pat1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_ve_pat1_ctrl`] module"]
pub type PLL_VE_PAT1_CTRL = crate::Reg<pll_ve_pat1_ctrl::PLL_VE_PAT1_CTRL_SPEC>;
#[doc = "PLL_VE Pattern1 Control Register"]
pub mod pll_ve_pat1_ctrl;
#[doc = "pll_audio0_pat0_ctrl (rw) register accessor: PLL_AUDIO0 Pattern0 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_audio0_pat0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_audio0_pat0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_audio0_pat0_ctrl`] module"]
pub type PLL_AUDIO0_PAT0_CTRL = crate::Reg<pll_audio0_pat0_ctrl::PLL_AUDIO0_PAT0_CTRL_SPEC>;
#[doc = "PLL_AUDIO0 Pattern0 Control Register"]
pub mod pll_audio0_pat0_ctrl;
#[doc = "pll_audio0_pat1_ctrl (rw) register accessor: PLL_AUDIO0 Pattern1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_audio0_pat1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_audio0_pat1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_audio0_pat1_ctrl`] module"]
pub type PLL_AUDIO0_PAT1_CTRL = crate::Reg<pll_audio0_pat1_ctrl::PLL_AUDIO0_PAT1_CTRL_SPEC>;
#[doc = "PLL_AUDIO0 Pattern1 Control Register"]
pub mod pll_audio0_pat1_ctrl;
#[doc = "pll_audio1_pat0_ctrl (rw) register accessor: PLL_AUDIO1 Pattern0 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_audio1_pat0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_audio1_pat0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_audio1_pat0_ctrl`] module"]
pub type PLL_AUDIO1_PAT0_CTRL = crate::Reg<pll_audio1_pat0_ctrl::PLL_AUDIO1_PAT0_CTRL_SPEC>;
#[doc = "PLL_AUDIO1 Pattern0 Control Register"]
pub mod pll_audio1_pat0_ctrl;
#[doc = "pll_audio1_pat1_ctrl (rw) register accessor: PLL_AUDIO1 Pattern1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_audio1_pat1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_audio1_pat1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_audio1_pat1_ctrl`] module"]
pub type PLL_AUDIO1_PAT1_CTRL = crate::Reg<pll_audio1_pat1_ctrl::PLL_AUDIO1_PAT1_CTRL_SPEC>;
#[doc = "PLL_AUDIO1 Pattern1 Control Register"]
pub mod pll_audio1_pat1_ctrl;
#[doc = "pll_cpu_bias (rw) register accessor: PLL_CPU Bias Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_cpu_bias::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_cpu_bias::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_cpu_bias`] module"]
pub type PLL_CPU_BIAS = crate::Reg<pll_cpu_bias::PLL_CPU_BIAS_SPEC>;
#[doc = "PLL_CPU Bias Register"]
pub mod pll_cpu_bias;
#[doc = "pll_ddr_bias (rw) register accessor: PLL_DDR Bias Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_ddr_bias::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_ddr_bias::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_ddr_bias`] module"]
pub type PLL_DDR_BIAS = crate::Reg<pll_ddr_bias::PLL_DDR_BIAS_SPEC>;
#[doc = "PLL_DDR Bias Register"]
pub mod pll_ddr_bias;
#[doc = "pll_peri_bias (rw) register accessor: PLL_PERI Bias Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_peri_bias::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_peri_bias::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_peri_bias`] module"]
pub type PLL_PERI_BIAS = crate::Reg<pll_peri_bias::PLL_PERI_BIAS_SPEC>;
#[doc = "PLL_PERI Bias Register"]
pub mod pll_peri_bias;
#[doc = "pll_video0_bias (rw) register accessor: PLL_VIDEO0 Bias Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_video0_bias::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_video0_bias::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_video0_bias`] module"]
pub type PLL_VIDEO0_BIAS = crate::Reg<pll_video0_bias::PLL_VIDEO0_BIAS_SPEC>;
#[doc = "PLL_VIDEO0 Bias Register"]
pub mod pll_video0_bias;
#[doc = "pll_video1_bias (rw) register accessor: PLL_VIDEO1 Bias Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_video1_bias::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_video1_bias::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_video1_bias`] module"]
pub type PLL_VIDEO1_BIAS = crate::Reg<pll_video1_bias::PLL_VIDEO1_BIAS_SPEC>;
#[doc = "PLL_VIDEO1 Bias Register"]
pub mod pll_video1_bias;
#[doc = "pll_ve_bias (rw) register accessor: PLL_VE Bias Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_ve_bias::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_ve_bias::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_ve_bias`] module"]
pub type PLL_VE_BIAS = crate::Reg<pll_ve_bias::PLL_VE_BIAS_SPEC>;
#[doc = "PLL_VE Bias Register"]
pub mod pll_ve_bias;
#[doc = "pll_audio0_bias (rw) register accessor: PLL_AUDIO0 Bias Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_audio0_bias::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_audio0_bias::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_audio0_bias`] module"]
pub type PLL_AUDIO0_BIAS = crate::Reg<pll_audio0_bias::PLL_AUDIO0_BIAS_SPEC>;
#[doc = "PLL_AUDIO0 Bias Register"]
pub mod pll_audio0_bias;
#[doc = "pll_audio1_bias (rw) register accessor: PLL_AUDIO1 Bias Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_audio1_bias::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_audio1_bias::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_audio1_bias`] module"]
pub type PLL_AUDIO1_BIAS = crate::Reg<pll_audio1_bias::PLL_AUDIO1_BIAS_SPEC>;
#[doc = "PLL_AUDIO1 Bias Register"]
pub mod pll_audio1_bias;
#[doc = "pll_cpu_tun (rw) register accessor: PLL_CPU Tuning Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_cpu_tun::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_cpu_tun::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_cpu_tun`] module"]
pub type PLL_CPU_TUN = crate::Reg<pll_cpu_tun::PLL_CPU_TUN_SPEC>;
#[doc = "PLL_CPU Tuning Register"]
pub mod pll_cpu_tun;
#[doc = "cpu_axi_cfg (rw) register accessor: CPU_AXI Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_axi_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_axi_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_axi_cfg`] module"]
pub type CPU_AXI_CFG = crate::Reg<cpu_axi_cfg::CPU_AXI_CFG_SPEC>;
#[doc = "CPU_AXI Configuration Register"]
pub mod cpu_axi_cfg;
#[doc = "cpu_gating (rw) register accessor: CPU_GATING Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_gating::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_gating::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_gating`] module"]
pub type CPU_GATING = crate::Reg<cpu_gating::CPU_GATING_SPEC>;
#[doc = "CPU_GATING Configuration Register"]
pub mod cpu_gating;
#[doc = "psi_clk (rw) register accessor: PSI Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psi_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psi_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psi_clk`] module"]
pub type PSI_CLK = crate::Reg<psi_clk::PSI_CLK_SPEC>;
#[doc = "PSI Clock Register"]
pub mod psi_clk;
#[doc = "apb_clk (rw) register accessor: APB Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb_clk`] module"]
pub type APB_CLK = crate::Reg<apb_clk::APB_CLK_SPEC>;
#[doc = "APB Clock Register"]
pub mod apb_clk;
#[doc = "mbus_clk (rw) register accessor: MBUS Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mbus_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mbus_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mbus_clk`] module"]
pub type MBUS_CLK = crate::Reg<mbus_clk::MBUS_CLK_SPEC>;
#[doc = "MBUS Clock Register"]
pub mod mbus_clk;
#[doc = "de_clk (rw) register accessor: DE Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`de_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`de_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@de_clk`] module"]
pub type DE_CLK = crate::Reg<de_clk::DE_CLK_SPEC>;
#[doc = "DE Clock Register"]
pub mod de_clk;
#[doc = "de_bgr (rw) register accessor: DE Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`de_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`de_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@de_bgr`] module"]
pub type DE_BGR = crate::Reg<de_bgr::DE_BGR_SPEC>;
#[doc = "DE Bus Gating Reset Register"]
pub mod de_bgr;
#[doc = "di_clk (rw) register accessor: DI Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`di_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`di_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@di_clk`] module"]
pub type DI_CLK = crate::Reg<di_clk::DI_CLK_SPEC>;
#[doc = "DI Clock Register"]
pub mod di_clk;
#[doc = "di_bgr (rw) register accessor: DI Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`di_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`di_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@di_bgr`] module"]
pub type DI_BGR = crate::Reg<di_bgr::DI_BGR_SPEC>;
#[doc = "DI Bus Gating Reset Register"]
pub mod di_bgr;
#[doc = "g2d_clk (rw) register accessor: G2D Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`g2d_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`g2d_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@g2d_clk`] module"]
pub type G2D_CLK = crate::Reg<g2d_clk::G2D_CLK_SPEC>;
#[doc = "G2D Clock Register"]
pub mod g2d_clk;
#[doc = "g2d_bgr (rw) register accessor: G2D Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`g2d_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`g2d_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@g2d_bgr`] module"]
pub type G2D_BGR = crate::Reg<g2d_bgr::G2D_BGR_SPEC>;
#[doc = "G2D Bus Gating Reset Register"]
pub mod g2d_bgr;
#[doc = "ce_clk (rw) register accessor: CE Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ce_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ce_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ce_clk`] module"]
pub type CE_CLK = crate::Reg<ce_clk::CE_CLK_SPEC>;
#[doc = "CE Clock Register"]
pub mod ce_clk;
#[doc = "ce_bgr (rw) register accessor: CE Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ce_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ce_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ce_bgr`] module"]
pub type CE_BGR = crate::Reg<ce_bgr::CE_BGR_SPEC>;
#[doc = "CE Bus Gating Reset Register"]
pub mod ce_bgr;
#[doc = "ve_clk (rw) register accessor: VE Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ve_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ve_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ve_clk`] module"]
pub type VE_CLK = crate::Reg<ve_clk::VE_CLK_SPEC>;
#[doc = "VE Clock Register"]
pub mod ve_clk;
#[doc = "ve_bgr (rw) register accessor: VE Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ve_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ve_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ve_bgr`] module"]
pub type VE_BGR = crate::Reg<ve_bgr::VE_BGR_SPEC>;
#[doc = "VE Bus Gating Reset Register"]
pub mod ve_bgr;
#[doc = "dma_bgr (rw) register accessor: DMA Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_bgr`] module"]
pub type DMA_BGR = crate::Reg<dma_bgr::DMA_BGR_SPEC>;
#[doc = "DMA Bus Gating Reset Register"]
pub mod dma_bgr;
#[doc = "msgbox_bgr (rw) register accessor: MSGBOX Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msgbox_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msgbox_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msgbox_bgr`] module"]
pub type MSGBOX_BGR = crate::Reg<msgbox_bgr::MSGBOX_BGR_SPEC>;
#[doc = "MSGBOX Bus Gating Reset Register"]
pub mod msgbox_bgr;
#[doc = "spinlock_bgr (rw) register accessor: SPINLOCK Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spinlock_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spinlock_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spinlock_bgr`] module"]
pub type SPINLOCK_BGR = crate::Reg<spinlock_bgr::SPINLOCK_BGR_SPEC>;
#[doc = "SPINLOCK Bus Gating Reset Register"]
pub mod spinlock_bgr;
#[doc = "hstimer_bgr (rw) register accessor: HSTIMER Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstimer_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstimer_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstimer_bgr`] module"]
pub type HSTIMER_BGR = crate::Reg<hstimer_bgr::HSTIMER_BGR_SPEC>;
#[doc = "HSTIMER Bus Gating Reset Register"]
pub mod hstimer_bgr;
#[doc = "avs_clk (rw) register accessor: AVS Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`avs_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`avs_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@avs_clk`] module"]
pub type AVS_CLK = crate::Reg<avs_clk::AVS_CLK_SPEC>;
#[doc = "AVS Clock Register"]
pub mod avs_clk;
#[doc = "dbgsys_bgr (rw) register accessor: DBGSYS Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgsys_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbgsys_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgsys_bgr`] module"]
pub type DBGSYS_BGR = crate::Reg<dbgsys_bgr::DBGSYS_BGR_SPEC>;
#[doc = "DBGSYS Bus Gating Reset Register"]
pub mod dbgsys_bgr;
#[doc = "pwm_bgr (rw) register accessor: PWM Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_bgr`] module"]
pub type PWM_BGR = crate::Reg<pwm_bgr::PWM_BGR_SPEC>;
#[doc = "PWM Bus Gating Reset Register"]
pub mod pwm_bgr;
#[doc = "iommu_bgr (rw) register accessor: IOMMU Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iommu_bgr`] module"]
pub type IOMMU_BGR = crate::Reg<iommu_bgr::IOMMU_BGR_SPEC>;
#[doc = "IOMMU Bus Gating Reset Register"]
pub mod iommu_bgr;
#[doc = "dram_clk (rw) register accessor: DRAM Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dram_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dram_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dram_clk`] module"]
pub type DRAM_CLK = crate::Reg<dram_clk::DRAM_CLK_SPEC>;
#[doc = "DRAM Clock Register"]
pub mod dram_clk;
#[doc = "mbus_mat_clk_gating (rw) register accessor: MBUS Master Clock Gating Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mbus_mat_clk_gating::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mbus_mat_clk_gating::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mbus_mat_clk_gating`] module"]
pub type MBUS_MAT_CLK_GATING = crate::Reg<mbus_mat_clk_gating::MBUS_MAT_CLK_GATING_SPEC>;
#[doc = "MBUS Master Clock Gating Register"]
pub mod mbus_mat_clk_gating;
#[doc = "dram_bgr (rw) register accessor: DRAM Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dram_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dram_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dram_bgr`] module"]
pub type DRAM_BGR = crate::Reg<dram_bgr::DRAM_BGR_SPEC>;
#[doc = "DRAM Bus Gating Reset Register"]
pub mod dram_bgr;
#[doc = "smhc0_clk (rw) register accessor: SMHC0 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc0_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc0_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc0_clk`] module"]
pub type SMHC0_CLK = crate::Reg<smhc0_clk::SMHC0_CLK_SPEC>;
#[doc = "SMHC0 Clock Register"]
pub mod smhc0_clk;
#[doc = "smhc1_clk (rw) register accessor: SMHC1 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc1_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc1_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc1_clk`] module"]
pub type SMHC1_CLK = crate::Reg<smhc1_clk::SMHC1_CLK_SPEC>;
#[doc = "SMHC1 Clock Register"]
pub mod smhc1_clk;
#[doc = "smhc2_clk (rw) register accessor: SMHC2 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc2_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc2_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc2_clk`] module"]
pub type SMHC2_CLK = crate::Reg<smhc2_clk::SMHC2_CLK_SPEC>;
#[doc = "SMHC2 Clock Register"]
pub mod smhc2_clk;
#[doc = "smhc_bgr (rw) register accessor: SMHC Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smhc_bgr`] module"]
pub type SMHC_BGR = crate::Reg<smhc_bgr::SMHC_BGR_SPEC>;
#[doc = "SMHC Bus Gating Reset Register"]
pub mod smhc_bgr;
#[doc = "uart_bgr (rw) register accessor: UART Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_bgr`] module"]
pub type UART_BGR = crate::Reg<uart_bgr::UART_BGR_SPEC>;
#[doc = "UART Bus Gating Reset Register"]
pub mod uart_bgr;
#[doc = "twi_bgr (rw) register accessor: TWI Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twi_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twi_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twi_bgr`] module"]
pub type TWI_BGR = crate::Reg<twi_bgr::TWI_BGR_SPEC>;
#[doc = "TWI Bus Gating Reset Register"]
pub mod twi_bgr;
#[doc = "spi0_clk (rw) register accessor: SPI0 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi0_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi0_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0_clk`] module"]
pub type SPI0_CLK = crate::Reg<spi0_clk::SPI0_CLK_SPEC>;
#[doc = "SPI0 Clock Register"]
pub mod spi0_clk;
#[doc = "spi1_clk (rw) register accessor: SPI1 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi1_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi1_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1_clk`] module"]
pub type SPI1_CLK = crate::Reg<spi1_clk::SPI1_CLK_SPEC>;
#[doc = "SPI1 Clock Register"]
pub mod spi1_clk;
#[doc = "spi_bgr (rw) register accessor: SPI Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_bgr`] module"]
pub type SPI_BGR = crate::Reg<spi_bgr::SPI_BGR_SPEC>;
#[doc = "SPI Bus Gating Reset Register"]
pub mod spi_bgr;
#[doc = "emac_25m_clk (rw) register accessor: EMAC_25M Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_25m_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emac_25m_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emac_25m_clk`] module"]
pub type EMAC_25M_CLK = crate::Reg<emac_25m_clk::EMAC_25M_CLK_SPEC>;
#[doc = "EMAC_25M Clock Register"]
pub mod emac_25m_clk;
#[doc = "emac_bgr (rw) register accessor: EMAC Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emac_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emac_bgr`] module"]
pub type EMAC_BGR = crate::Reg<emac_bgr::EMAC_BGR_SPEC>;
#[doc = "EMAC Bus Gating Reset Register"]
pub mod emac_bgr;
#[doc = "irtx_clk (rw) register accessor: IRTX Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irtx_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irtx_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irtx_clk`] module"]
pub type IRTX_CLK = crate::Reg<irtx_clk::IRTX_CLK_SPEC>;
#[doc = "IRTX Clock Register"]
pub mod irtx_clk;
#[doc = "irtx_bgr (rw) register accessor: IRTX Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irtx_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irtx_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irtx_bgr`] module"]
pub type IRTX_BGR = crate::Reg<irtx_bgr::IRTX_BGR_SPEC>;
#[doc = "IRTX Bus Gating Reset Register"]
pub mod irtx_bgr;
#[doc = "gpadc_bgr (rw) register accessor: GPADC Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpadc_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpadc_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_bgr`] module"]
pub type GPADC_BGR = crate::Reg<gpadc_bgr::GPADC_BGR_SPEC>;
#[doc = "GPADC Bus Gating Reset Register"]
pub mod gpadc_bgr;
#[doc = "ths_bgr (rw) register accessor: THS Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ths_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ths_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ths_bgr`] module"]
pub type THS_BGR = crate::Reg<ths_bgr::THS_BGR_SPEC>;
#[doc = "THS Bus Gating Reset Register"]
pub mod ths_bgr;
#[doc = "i2s_clk (rw) register accessor: I2S Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_clk`] module"]
pub type I2S_CLK = crate::Reg<i2s_clk::I2S_CLK_SPEC>;
#[doc = "I2S Clock Register"]
pub mod i2s_clk;
#[doc = "i2s2_asrc_clk (rw) register accessor: I2S2_ASRC Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s2_asrc_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s2_asrc_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s2_asrc_clk`] module"]
pub type I2S2_ASRC_CLK = crate::Reg<i2s2_asrc_clk::I2S2_ASRC_CLK_SPEC>;
#[doc = "I2S2_ASRC Clock Register"]
pub mod i2s2_asrc_clk;
#[doc = "i2s_bgr (rw) register accessor: I2S Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_bgr`] module"]
pub type I2S_BGR = crate::Reg<i2s_bgr::I2S_BGR_SPEC>;
#[doc = "I2S Bus Gating Reset Register"]
pub mod i2s_bgr;
#[doc = "owa_tx_clk (rw) register accessor: OWA_TX Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`owa_tx_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`owa_tx_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@owa_tx_clk`] module"]
pub type OWA_TX_CLK = crate::Reg<owa_tx_clk::OWA_TX_CLK_SPEC>;
#[doc = "OWA_TX Clock Register"]
pub mod owa_tx_clk;
#[doc = "owa_rx_clk (rw) register accessor: OWA_RX Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`owa_rx_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`owa_rx_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@owa_rx_clk`] module"]
pub type OWA_RX_CLK = crate::Reg<owa_rx_clk::OWA_RX_CLK_SPEC>;
#[doc = "OWA_RX Clock Register"]
pub mod owa_rx_clk;
#[doc = "owa_bgr (rw) register accessor: OWA Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`owa_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`owa_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@owa_bgr`] module"]
pub type OWA_BGR = crate::Reg<owa_bgr::OWA_BGR_SPEC>;
#[doc = "OWA Bus Gating Reset Register"]
pub mod owa_bgr;
#[doc = "dmic_clk (rw) register accessor: DMIC Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmic_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmic_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmic_clk`] module"]
pub type DMIC_CLK = crate::Reg<dmic_clk::DMIC_CLK_SPEC>;
#[doc = "DMIC Clock Register"]
pub mod dmic_clk;
#[doc = "dmic_bgr (rw) register accessor: DMIC Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmic_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmic_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmic_bgr`] module"]
pub type DMIC_BGR = crate::Reg<dmic_bgr::DMIC_BGR_SPEC>;
#[doc = "DMIC Bus Gating Reset Register"]
pub mod dmic_bgr;
#[doc = "audio_codec_dac_clk (rw) register accessor: AUDIO_CODEC_DAC Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`audio_codec_dac_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`audio_codec_dac_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audio_codec_dac_clk`] module"]
pub type AUDIO_CODEC_DAC_CLK = crate::Reg<audio_codec_dac_clk::AUDIO_CODEC_DAC_CLK_SPEC>;
#[doc = "AUDIO_CODEC_DAC Clock Register"]
pub mod audio_codec_dac_clk;
#[doc = "audio_codec_adc_clk (rw) register accessor: AUDIO_CODEC_ADC Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`audio_codec_adc_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`audio_codec_adc_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audio_codec_adc_clk`] module"]
pub type AUDIO_CODEC_ADC_CLK = crate::Reg<audio_codec_adc_clk::AUDIO_CODEC_ADC_CLK_SPEC>;
#[doc = "AUDIO_CODEC_ADC Clock Register"]
pub mod audio_codec_adc_clk;
#[doc = "audio_codec_bgr (rw) register accessor: AUDIO_CODEC Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`audio_codec_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`audio_codec_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audio_codec_bgr`] module"]
pub type AUDIO_CODEC_BGR = crate::Reg<audio_codec_bgr::AUDIO_CODEC_BGR_SPEC>;
#[doc = "AUDIO_CODEC Bus Gating Reset Register"]
pub mod audio_codec_bgr;
#[doc = "usb0_clk (rw) register accessor: USB0 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0_clk`] module"]
pub type USB0_CLK = crate::Reg<usb0_clk::USB0_CLK_SPEC>;
#[doc = "USB0 Clock Register"]
pub mod usb0_clk;
#[doc = "usb1_clk (rw) register accessor: USB1 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb1_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb1_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb1_clk`] module"]
pub type USB1_CLK = crate::Reg<usb1_clk::USB1_CLK_SPEC>;
#[doc = "USB1 Clock Register"]
pub mod usb1_clk;
#[doc = "usb_bgr (rw) register accessor: USB Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_bgr`] module"]
pub type USB_BGR = crate::Reg<usb_bgr::USB_BGR_SPEC>;
#[doc = "USB Bus Gating Reset Register"]
pub mod usb_bgr;
#[doc = "lradc_bgr (rw) register accessor: LRADC Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lradc_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lradc_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lradc_bgr`] module"]
pub type LRADC_BGR = crate::Reg<lradc_bgr::LRADC_BGR_SPEC>;
#[doc = "LRADC Bus Gating Reset Register"]
pub mod lradc_bgr;
#[doc = "dpss_top_bgr (rw) register accessor: DPSS_TOP Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpss_top_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpss_top_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpss_top_bgr`] module"]
pub type DPSS_TOP_BGR = crate::Reg<dpss_top_bgr::DPSS_TOP_BGR_SPEC>;
#[doc = "DPSS_TOP Bus Gating Reset Register"]
pub mod dpss_top_bgr;
#[doc = "dsi_clk (rw) register accessor: DSI Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_clk`] module"]
pub type DSI_CLK = crate::Reg<dsi_clk::DSI_CLK_SPEC>;
#[doc = "DSI Clock Register"]
pub mod dsi_clk;
#[doc = "dsi_bgr (rw) register accessor: DSI Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_bgr`] module"]
pub type DSI_BGR = crate::Reg<dsi_bgr::DSI_BGR_SPEC>;
#[doc = "DSI Bus Gating Reset Register"]
pub mod dsi_bgr;
#[doc = "tconlcd_clk (rw) register accessor: TCONLCD Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tconlcd_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tconlcd_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tconlcd_clk`] module"]
pub type TCONLCD_CLK = crate::Reg<tconlcd_clk::TCONLCD_CLK_SPEC>;
#[doc = "TCONLCD Clock Register"]
pub mod tconlcd_clk;
#[doc = "tconlcd_bgr (rw) register accessor: TCONLCD Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tconlcd_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tconlcd_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tconlcd_bgr`] module"]
pub type TCONLCD_BGR = crate::Reg<tconlcd_bgr::TCONLCD_BGR_SPEC>;
#[doc = "TCONLCD Bus Gating Reset Register"]
pub mod tconlcd_bgr;
#[doc = "tcontv_clk (rw) register accessor: TCONTV Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcontv_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcontv_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcontv_clk`] module"]
pub type TCONTV_CLK = crate::Reg<tcontv_clk::TCONTV_CLK_SPEC>;
#[doc = "TCONTV Clock Register"]
pub mod tcontv_clk;
#[doc = "tcontv_bgr (rw) register accessor: TCONTV Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcontv_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcontv_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcontv_bgr`] module"]
pub type TCONTV_BGR = crate::Reg<tcontv_bgr::TCONTV_BGR_SPEC>;
#[doc = "TCONTV Bus Gating Reset Register"]
pub mod tcontv_bgr;
#[doc = "lvds_bgr (rw) register accessor: LVDS Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lvds_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lvds_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lvds_bgr`] module"]
pub type LVDS_BGR = crate::Reg<lvds_bgr::LVDS_BGR_SPEC>;
#[doc = "LVDS Bus Gating Reset Register"]
pub mod lvds_bgr;
#[doc = "tve_clk (rw) register accessor: TVE Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_clk`] module"]
pub type TVE_CLK = crate::Reg<tve_clk::TVE_CLK_SPEC>;
#[doc = "TVE Clock Register"]
pub mod tve_clk;
#[doc = "tve_bgr (rw) register accessor: TVE Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_bgr`] module"]
pub type TVE_BGR = crate::Reg<tve_bgr::TVE_BGR_SPEC>;
#[doc = "TVE Bus Gating Reset Register"]
pub mod tve_bgr;
#[doc = "tvd_clk (rw) register accessor: TVD Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_clk`] module"]
pub type TVD_CLK = crate::Reg<tvd_clk::TVD_CLK_SPEC>;
#[doc = "TVD Clock Register"]
pub mod tvd_clk;
#[doc = "tvd_bgr (rw) register accessor: TVD Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tvd_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tvd_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_bgr`] module"]
pub type TVD_BGR = crate::Reg<tvd_bgr::TVD_BGR_SPEC>;
#[doc = "TVD Bus Gating Reset Register"]
pub mod tvd_bgr;
#[doc = "ledc_clk (rw) register accessor: LEDC Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ledc_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ledc_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ledc_clk`] module"]
pub type LEDC_CLK = crate::Reg<ledc_clk::LEDC_CLK_SPEC>;
#[doc = "LEDC Clock Register"]
pub mod ledc_clk;
#[doc = "ledc_bgr (rw) register accessor: LEDC Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ledc_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ledc_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ledc_bgr`] module"]
pub type LEDC_BGR = crate::Reg<ledc_bgr::LEDC_BGR_SPEC>;
#[doc = "LEDC Bus Gating Reset Register"]
pub mod ledc_bgr;
#[doc = "csi_clk (rw) register accessor: CSI Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csi_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csi_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi_clk`] module"]
pub type CSI_CLK = crate::Reg<csi_clk::CSI_CLK_SPEC>;
#[doc = "CSI Clock Register"]
pub mod csi_clk;
#[doc = "csi_master_clk (rw) register accessor: CSI Master Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csi_master_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csi_master_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi_master_clk`] module"]
pub type CSI_MASTER_CLK = crate::Reg<csi_master_clk::CSI_MASTER_CLK_SPEC>;
#[doc = "CSI Master Clock Register"]
pub mod csi_master_clk;
#[doc = "csi_bgr (rw) register accessor: CSI Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csi_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csi_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi_bgr`] module"]
pub type CSI_BGR = crate::Reg<csi_bgr::CSI_BGR_SPEC>;
#[doc = "CSI Bus Gating Reset Register"]
pub mod csi_bgr;
#[doc = "tpadc_clk (rw) register accessor: TPADC Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tpadc_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tpadc_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tpadc_clk`] module"]
pub type TPADC_CLK = crate::Reg<tpadc_clk::TPADC_CLK_SPEC>;
#[doc = "TPADC Clock Register"]
pub mod tpadc_clk;
#[doc = "tpadc_bgr (rw) register accessor: TPADC Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tpadc_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tpadc_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tpadc_bgr`] module"]
pub type TPADC_BGR = crate::Reg<tpadc_bgr::TPADC_BGR_SPEC>;
#[doc = "TPADC Bus Gating Reset Register"]
pub mod tpadc_bgr;
#[doc = "dsp_clk (rw) register accessor: DSP Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsp_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp_clk`] module"]
pub type DSP_CLK = crate::Reg<dsp_clk::DSP_CLK_SPEC>;
#[doc = "DSP Clock Register"]
pub mod dsp_clk;
#[doc = "dsp_bgr (rw) register accessor: DSP Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsp_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp_bgr`] module"]
pub type DSP_BGR = crate::Reg<dsp_bgr::DSP_BGR_SPEC>;
#[doc = "DSP Bus Gating Reset Register"]
pub mod dsp_bgr;
#[doc = "riscv_clk (rw) register accessor: RISC-V Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`riscv_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`riscv_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@riscv_clk`] module"]
pub type RISCV_CLK = crate::Reg<riscv_clk::RISCV_CLK_SPEC>;
#[doc = "RISC-V Clock Register"]
pub mod riscv_clk;
#[doc = "riscv_gating (rw) register accessor: RISC-V GATING Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`riscv_gating::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`riscv_gating::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@riscv_gating`] module"]
pub type RISCV_GATING = crate::Reg<riscv_gating::RISCV_GATING_SPEC>;
#[doc = "RISC-V GATING Configuration Register"]
pub mod riscv_gating;
#[doc = "riscv_cfg_bgr (rw) register accessor: RISC-V_CFG Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`riscv_cfg_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`riscv_cfg_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@riscv_cfg_bgr`] module"]
pub type RISCV_CFG_BGR = crate::Reg<riscv_cfg_bgr::RISCV_CFG_BGR_SPEC>;
#[doc = "RISC-V_CFG Bus Gating Reset Register"]
pub mod riscv_cfg_bgr;
#[doc = "pll_lock_dbg_ctrl (rw) register accessor: PLL Lock Debug Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_lock_dbg_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_lock_dbg_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_lock_dbg_ctrl`] module"]
pub type PLL_LOCK_DBG_CTRL = crate::Reg<pll_lock_dbg_ctrl::PLL_LOCK_DBG_CTRL_SPEC>;
#[doc = "PLL Lock Debug Control Register"]
pub mod pll_lock_dbg_ctrl;
#[doc = "fre_det_ctrl (rw) register accessor: Frequency Detect Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fre_det_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fre_det_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fre_det_ctrl`] module"]
pub type FRE_DET_CTRL = crate::Reg<fre_det_ctrl::FRE_DET_CTRL_SPEC>;
#[doc = "Frequency Detect Control Register"]
pub mod fre_det_ctrl;
#[doc = "fre_up_lim (rw) register accessor: Frequency Up Limit Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fre_up_lim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fre_up_lim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fre_up_lim`] module"]
pub type FRE_UP_LIM = crate::Reg<fre_up_lim::FRE_UP_LIM_SPEC>;
#[doc = "Frequency Up Limit Register"]
pub mod fre_up_lim;
#[doc = "fre_down_lim (rw) register accessor: Frequency Down Limit Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fre_down_lim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fre_down_lim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fre_down_lim`] module"]
pub type FRE_DOWN_LIM = crate::Reg<fre_down_lim::FRE_DOWN_LIM_SPEC>;
#[doc = "Frequency Down Limit Register"]
pub mod fre_down_lim;
#[doc = "ccu_fan_gate (rw) register accessor: CCU FANOUT CLOCK GATE Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccu_fan_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccu_fan_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccu_fan_gate`] module"]
pub type CCU_FAN_GATE = crate::Reg<ccu_fan_gate::CCU_FAN_GATE_SPEC>;
#[doc = "CCU FANOUT CLOCK GATE Register"]
pub mod ccu_fan_gate;
#[doc = "clk27m_fan (rw) register accessor: CLK27M FANOUT Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk27m_fan::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk27m_fan::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk27m_fan`] module"]
pub type CLK27M_FAN = crate::Reg<clk27m_fan::CLK27M_FAN_SPEC>;
#[doc = "CLK27M FANOUT Register"]
pub mod clk27m_fan;
#[doc = "pclk_fan (rw) register accessor: PCLK FANOUT Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pclk_fan::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pclk_fan::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pclk_fan`] module"]
pub type PCLK_FAN = crate::Reg<pclk_fan::PCLK_FAN_SPEC>;
#[doc = "PCLK FANOUT Register"]
pub mod pclk_fan;
#[doc = "ccu_fan (rw) register accessor: CCU FANOUT Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccu_fan::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccu_fan::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccu_fan`] module"]
pub type CCU_FAN = crate::Reg<ccu_fan::CCU_FAN_SPEC>;
#[doc = "CCU FANOUT Register"]
pub mod ccu_fan;
