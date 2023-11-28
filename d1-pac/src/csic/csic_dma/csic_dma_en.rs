#[doc = "Register `csic_dma_en` reader"]
pub type R = crate::R<CSIC_DMA_EN_SPEC>;
#[doc = "Register `csic_dma_en` writer"]
pub type W = crate::W<CSIC_DMA_EN_SPEC>;
#[doc = "Field `bk_top_en` reader - "]
pub type BK_TOP_EN_R = crate::BitReader<BK_TOP_EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK_TOP_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<BK_TOP_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BK_TOP_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BK_TOP_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BK_TOP_EN_A {
        match self.bits {
            false => BK_TOP_EN_A::DISABLE,
            true => BK_TOP_EN_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BK_TOP_EN_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BK_TOP_EN_A::ENABLE
    }
}
#[doc = "Field `bk_top_en` writer - "]
pub type BK_TOP_EN_W<'a, REG> = crate::BitWriter<'a, REG, BK_TOP_EN_A>;
impl<'a, REG> BK_TOP_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(BK_TOP_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(BK_TOP_EN_A::ENABLE)
    }
}
#[doc = "Field `clk_cnt_en` reader - clk count per frame enable"]
pub type CLK_CNT_EN_R = crate::BitReader<CLK_CNT_EN_A>;
#[doc = "clk count per frame enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLK_CNT_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<CLK_CNT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_CNT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl CLK_CNT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLK_CNT_EN_A {
        match self.bits {
            false => CLK_CNT_EN_A::DISABLE,
            true => CLK_CNT_EN_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CLK_CNT_EN_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CLK_CNT_EN_A::ENABLE
    }
}
#[doc = "Field `clk_cnt_en` writer - clk count per frame enable"]
pub type CLK_CNT_EN_W<'a, REG> = crate::BitWriter<'a, REG, CLK_CNT_EN_A>;
impl<'a, REG> CLK_CNT_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_CNT_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_CNT_EN_A::ENABLE)
    }
}
#[doc = "Field `clk_cnt_spl` reader - Sampling time for clk counter per frame"]
pub type CLK_CNT_SPL_R = crate::BitReader<CLK_CNT_SPL_A>;
#[doc = "Sampling time for clk counter per frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLK_CNT_SPL_A {
    #[doc = "0: Sampling clock counter every frame done"]
    FRAME_DONE = 0,
    #[doc = "1: Sampling clock counter every vsync"]
    VSYNC = 1,
}
impl From<CLK_CNT_SPL_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_CNT_SPL_A) -> Self {
        variant as u8 != 0
    }
}
impl CLK_CNT_SPL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLK_CNT_SPL_A {
        match self.bits {
            false => CLK_CNT_SPL_A::FRAME_DONE,
            true => CLK_CNT_SPL_A::VSYNC,
        }
    }
    #[doc = "Sampling clock counter every frame done"]
    #[inline(always)]
    pub fn is_frame_done(&self) -> bool {
        *self == CLK_CNT_SPL_A::FRAME_DONE
    }
    #[doc = "Sampling clock counter every vsync"]
    #[inline(always)]
    pub fn is_vsync(&self) -> bool {
        *self == CLK_CNT_SPL_A::VSYNC
    }
}
#[doc = "Field `clk_cnt_spl` writer - Sampling time for clk counter per frame"]
pub type CLK_CNT_SPL_W<'a, REG> = crate::BitWriter<'a, REG, CLK_CNT_SPL_A>;
impl<'a, REG> CLK_CNT_SPL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling clock counter every frame done"]
    #[inline(always)]
    pub fn frame_done(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_CNT_SPL_A::FRAME_DONE)
    }
    #[doc = "Sampling clock counter every vsync"]
    #[inline(always)]
    pub fn vsync(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_CNT_SPL_A::VSYNC)
    }
}
#[doc = "Field `dma_en` reader - When BK_TOP_EN is enabled, setting 1 to this bit indicates the module works in DMA mode."]
pub type DMA_EN_R = crate::BitReader<DMA_EN_A>;
#[doc = "When BK_TOP_EN is enabled, setting 1 to this bit indicates the module works in DMA mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<DMA_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA_EN_A {
        match self.bits {
            false => DMA_EN_A::DISABLE,
            true => DMA_EN_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DMA_EN_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DMA_EN_A::ENABLE
    }
}
#[doc = "Field `dma_en` writer - When BK_TOP_EN is enabled, setting 1 to this bit indicates the module works in DMA mode."]
pub type DMA_EN_W<'a, REG> = crate::BitWriter<'a, REG, DMA_EN_A>;
impl<'a, REG> DMA_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_EN_A::ENABLE)
    }
}
#[doc = "Field `frame_cnt_en` reader - When BK_TOP_EN is enabled, setting 1 to this bit indicates the Frame counter starts to add."]
pub type FRAME_CNT_EN_R = crate::BitReader<FRAME_CNT_EN_A>;
#[doc = "When BK_TOP_EN is enabled, setting 1 to this bit indicates the Frame counter starts to add.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRAME_CNT_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<FRAME_CNT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FRAME_CNT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl FRAME_CNT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FRAME_CNT_EN_A {
        match self.bits {
            false => FRAME_CNT_EN_A::DISABLE,
            true => FRAME_CNT_EN_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FRAME_CNT_EN_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FRAME_CNT_EN_A::ENABLE
    }
}
#[doc = "Field `frame_cnt_en` writer - When BK_TOP_EN is enabled, setting 1 to this bit indicates the Frame counter starts to add."]
pub type FRAME_CNT_EN_W<'a, REG> = crate::BitWriter<'a, REG, FRAME_CNT_EN_A>;
impl<'a, REG> FRAME_CNT_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(FRAME_CNT_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(FRAME_CNT_EN_A::ENABLE)
    }
}
#[doc = "Field `vi_to_cnt_en` reader - Enable Video Input Timeout counter, add 1 when there is no effective video input in a 12M clock, clear to 0 when detecting effective video input."]
pub type VI_TO_CNT_EN_R = crate::BitReader<VI_TO_CNT_EN_A>;
#[doc = "Enable Video Input Timeout counter, add 1 when there is no effective video input in a 12M clock, clear to 0 when detecting effective video input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VI_TO_CNT_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<VI_TO_CNT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: VI_TO_CNT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl VI_TO_CNT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VI_TO_CNT_EN_A {
        match self.bits {
            false => VI_TO_CNT_EN_A::DISABLE,
            true => VI_TO_CNT_EN_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == VI_TO_CNT_EN_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == VI_TO_CNT_EN_A::ENABLE
    }
}
#[doc = "Field `vi_to_cnt_en` writer - Enable Video Input Timeout counter, add 1 when there is no effective video input in a 12M clock, clear to 0 when detecting effective video input."]
pub type VI_TO_CNT_EN_W<'a, REG> = crate::BitWriter<'a, REG, VI_TO_CNT_EN_A>;
impl<'a, REG> VI_TO_CNT_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(VI_TO_CNT_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(VI_TO_CNT_EN_A::ENABLE)
    }
}
#[doc = "Field `buf_addr_mode` reader - "]
pub type BUF_ADDR_MODE_R = crate::BitReader<BUF_ADDR_MODE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUF_ADDR_MODE_A {
    #[doc = "0: Buffer Address Register Mode"]
    R_EGISTER = 0,
    #[doc = "1: Buffer Address FIFO Mode"]
    FIFO = 1,
}
impl From<BUF_ADDR_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: BUF_ADDR_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl BUF_ADDR_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BUF_ADDR_MODE_A {
        match self.bits {
            false => BUF_ADDR_MODE_A::R_EGISTER,
            true => BUF_ADDR_MODE_A::FIFO,
        }
    }
    #[doc = "Buffer Address Register Mode"]
    #[inline(always)]
    pub fn is_r_egister(&self) -> bool {
        *self == BUF_ADDR_MODE_A::R_EGISTER
    }
    #[doc = "Buffer Address FIFO Mode"]
    #[inline(always)]
    pub fn is_fifo(&self) -> bool {
        *self == BUF_ADDR_MODE_A::FIFO
    }
}
#[doc = "Field `buf_addr_mode` writer - "]
pub type BUF_ADDR_MODE_W<'a, REG> = crate::BitWriter<'a, REG, BUF_ADDR_MODE_A>;
impl<'a, REG> BUF_ADDR_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Buffer Address Register Mode"]
    #[inline(always)]
    pub fn r_egister(self) -> &'a mut crate::W<REG> {
        self.variant(BUF_ADDR_MODE_A::R_EGISTER)
    }
    #[doc = "Buffer Address FIFO Mode"]
    #[inline(always)]
    pub fn fifo(self) -> &'a mut crate::W<REG> {
        self.variant(BUF_ADDR_MODE_A::FIFO)
    }
}
#[doc = "Field `flip_size_cfg_mode` reader - FLIP SIZE set by software or calculated by hardware"]
pub type FLIP_SIZE_CFG_MODE_R = crate::BitReader<FLIP_SIZE_CFG_MODE_A>;
#[doc = "FLIP SIZE set by software or calculated by hardware\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLIP_SIZE_CFG_MODE_A {
    #[doc = "0: Hardware"]
    H_ARDWARE = 0,
    #[doc = "1: Software"]
    S_OFTWARE = 1,
}
impl From<FLIP_SIZE_CFG_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: FLIP_SIZE_CFG_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl FLIP_SIZE_CFG_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FLIP_SIZE_CFG_MODE_A {
        match self.bits {
            false => FLIP_SIZE_CFG_MODE_A::H_ARDWARE,
            true => FLIP_SIZE_CFG_MODE_A::S_OFTWARE,
        }
    }
    #[doc = "Hardware"]
    #[inline(always)]
    pub fn is_h_ardware(&self) -> bool {
        *self == FLIP_SIZE_CFG_MODE_A::H_ARDWARE
    }
    #[doc = "Software"]
    #[inline(always)]
    pub fn is_s_oftware(&self) -> bool {
        *self == FLIP_SIZE_CFG_MODE_A::S_OFTWARE
    }
}
#[doc = "Field `flip_size_cfg_mode` writer - FLIP SIZE set by software or calculated by hardware"]
pub type FLIP_SIZE_CFG_MODE_W<'a, REG> = crate::BitWriter<'a, REG, FLIP_SIZE_CFG_MODE_A>;
impl<'a, REG> FLIP_SIZE_CFG_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware"]
    #[inline(always)]
    pub fn h_ardware(self) -> &'a mut crate::W<REG> {
        self.variant(FLIP_SIZE_CFG_MODE_A::H_ARDWARE)
    }
    #[doc = "Software"]
    #[inline(always)]
    pub fn s_oftware(self) -> &'a mut crate::W<REG> {
        self.variant(FLIP_SIZE_CFG_MODE_A::S_OFTWARE)
    }
}
#[doc = "Field `buf_length_cfg_mode` reader - Buffer length set by software or calculated by hardware"]
pub type BUF_LENGTH_CFG_MODE_R = crate::BitReader<BUF_LENGTH_CFG_MODE_A>;
#[doc = "Buffer length set by software or calculated by hardware\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUF_LENGTH_CFG_MODE_A {
    #[doc = "0: Hardware"]
    H_ARDWARE = 0,
    #[doc = "1: Software"]
    S_OFTWARE = 1,
}
impl From<BUF_LENGTH_CFG_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: BUF_LENGTH_CFG_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl BUF_LENGTH_CFG_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BUF_LENGTH_CFG_MODE_A {
        match self.bits {
            false => BUF_LENGTH_CFG_MODE_A::H_ARDWARE,
            true => BUF_LENGTH_CFG_MODE_A::S_OFTWARE,
        }
    }
    #[doc = "Hardware"]
    #[inline(always)]
    pub fn is_h_ardware(&self) -> bool {
        *self == BUF_LENGTH_CFG_MODE_A::H_ARDWARE
    }
    #[doc = "Software"]
    #[inline(always)]
    pub fn is_s_oftware(&self) -> bool {
        *self == BUF_LENGTH_CFG_MODE_A::S_OFTWARE
    }
}
#[doc = "Field `buf_length_cfg_mode` writer - Buffer length set by software or calculated by hardware"]
pub type BUF_LENGTH_CFG_MODE_W<'a, REG> = crate::BitWriter<'a, REG, BUF_LENGTH_CFG_MODE_A>;
impl<'a, REG> BUF_LENGTH_CFG_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware"]
    #[inline(always)]
    pub fn h_ardware(self) -> &'a mut crate::W<REG> {
        self.variant(BUF_LENGTH_CFG_MODE_A::H_ARDWARE)
    }
    #[doc = "Software"]
    #[inline(always)]
    pub fn s_oftware(self) -> &'a mut crate::W<REG> {
        self.variant(BUF_LENGTH_CFG_MODE_A::S_OFTWARE)
    }
}
#[doc = "Field `vflip_buf_addr_cfg_mode` reader - Vflip buffer address set by software or calculated by hardware"]
pub type VFLIP_BUF_ADDR_CFG_MODE_R = crate::BitReader<VFLIP_BUF_ADDR_CFG_MODE_A>;
#[doc = "Vflip buffer address set by software or calculated by hardware\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VFLIP_BUF_ADDR_CFG_MODE_A {
    #[doc = "0: Hardware"]
    H_ARDWARE = 0,
    #[doc = "1: Software"]
    S_OFTWARE = 1,
}
impl From<VFLIP_BUF_ADDR_CFG_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: VFLIP_BUF_ADDR_CFG_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl VFLIP_BUF_ADDR_CFG_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VFLIP_BUF_ADDR_CFG_MODE_A {
        match self.bits {
            false => VFLIP_BUF_ADDR_CFG_MODE_A::H_ARDWARE,
            true => VFLIP_BUF_ADDR_CFG_MODE_A::S_OFTWARE,
        }
    }
    #[doc = "Hardware"]
    #[inline(always)]
    pub fn is_h_ardware(&self) -> bool {
        *self == VFLIP_BUF_ADDR_CFG_MODE_A::H_ARDWARE
    }
    #[doc = "Software"]
    #[inline(always)]
    pub fn is_s_oftware(&self) -> bool {
        *self == VFLIP_BUF_ADDR_CFG_MODE_A::S_OFTWARE
    }
}
#[doc = "Field `vflip_buf_addr_cfg_mode` writer - Vflip buffer address set by software or calculated by hardware"]
pub type VFLIP_BUF_ADDR_CFG_MODE_W<'a, REG> = crate::BitWriter<'a, REG, VFLIP_BUF_ADDR_CFG_MODE_A>;
impl<'a, REG> VFLIP_BUF_ADDR_CFG_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware"]
    #[inline(always)]
    pub fn h_ardware(self) -> &'a mut crate::W<REG> {
        self.variant(VFLIP_BUF_ADDR_CFG_MODE_A::H_ARDWARE)
    }
    #[doc = "Software"]
    #[inline(always)]
    pub fn s_oftware(self) -> &'a mut crate::W<REG> {
        self.variant(VFLIP_BUF_ADDR_CFG_MODE_A::S_OFTWARE)
    }
}
#[doc = "Field `ver_en` reader - "]
pub type VER_EN_R = crate::BitReader;
#[doc = "Field `ver_en` writer - "]
pub type VER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bk_top_en(&self) -> BK_TOP_EN_R {
        BK_TOP_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - clk count per frame enable"]
    #[inline(always)]
    pub fn clk_cnt_en(&self) -> CLK_CNT_EN_R {
        CLK_CNT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sampling time for clk counter per frame"]
    #[inline(always)]
    pub fn clk_cnt_spl(&self) -> CLK_CNT_SPL_R {
        CLK_CNT_SPL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - When BK_TOP_EN is enabled, setting 1 to this bit indicates the module works in DMA mode."]
    #[inline(always)]
    pub fn dma_en(&self) -> DMA_EN_R {
        DMA_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When BK_TOP_EN is enabled, setting 1 to this bit indicates the Frame counter starts to add."]
    #[inline(always)]
    pub fn frame_cnt_en(&self) -> FRAME_CNT_EN_R {
        FRAME_CNT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Video Input Timeout counter, add 1 when there is no effective video input in a 12M clock, clear to 0 when detecting effective video input."]
    #[inline(always)]
    pub fn vi_to_cnt_en(&self) -> VI_TO_CNT_EN_R {
        VI_TO_CNT_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn buf_addr_mode(&self) -> BUF_ADDR_MODE_R {
        BUF_ADDR_MODE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 28 - FLIP SIZE set by software or calculated by hardware"]
    #[inline(always)]
    pub fn flip_size_cfg_mode(&self) -> FLIP_SIZE_CFG_MODE_R {
        FLIP_SIZE_CFG_MODE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Buffer length set by software or calculated by hardware"]
    #[inline(always)]
    pub fn buf_length_cfg_mode(&self) -> BUF_LENGTH_CFG_MODE_R {
        BUF_LENGTH_CFG_MODE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Vflip buffer address set by software or calculated by hardware"]
    #[inline(always)]
    pub fn vflip_buf_addr_cfg_mode(&self) -> VFLIP_BUF_ADDR_CFG_MODE_R {
        VFLIP_BUF_ADDR_CFG_MODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ver_en(&self) -> VER_EN_R {
        VER_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn bk_top_en(&mut self) -> BK_TOP_EN_W<CSIC_DMA_EN_SPEC> {
        BK_TOP_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - clk count per frame enable"]
    #[inline(always)]
    #[must_use]
    pub fn clk_cnt_en(&mut self) -> CLK_CNT_EN_W<CSIC_DMA_EN_SPEC> {
        CLK_CNT_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Sampling time for clk counter per frame"]
    #[inline(always)]
    #[must_use]
    pub fn clk_cnt_spl(&mut self) -> CLK_CNT_SPL_W<CSIC_DMA_EN_SPEC> {
        CLK_CNT_SPL_W::new(self, 2)
    }
    #[doc = "Bit 4 - When BK_TOP_EN is enabled, setting 1 to this bit indicates the module works in DMA mode."]
    #[inline(always)]
    #[must_use]
    pub fn dma_en(&mut self) -> DMA_EN_W<CSIC_DMA_EN_SPEC> {
        DMA_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - When BK_TOP_EN is enabled, setting 1 to this bit indicates the Frame counter starts to add."]
    #[inline(always)]
    #[must_use]
    pub fn frame_cnt_en(&mut self) -> FRAME_CNT_EN_W<CSIC_DMA_EN_SPEC> {
        FRAME_CNT_EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Video Input Timeout counter, add 1 when there is no effective video input in a 12M clock, clear to 0 when detecting effective video input."]
    #[inline(always)]
    #[must_use]
    pub fn vi_to_cnt_en(&mut self) -> VI_TO_CNT_EN_W<CSIC_DMA_EN_SPEC> {
        VI_TO_CNT_EN_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn buf_addr_mode(&mut self) -> BUF_ADDR_MODE_W<CSIC_DMA_EN_SPEC> {
        BUF_ADDR_MODE_W::new(self, 7)
    }
    #[doc = "Bit 28 - FLIP SIZE set by software or calculated by hardware"]
    #[inline(always)]
    #[must_use]
    pub fn flip_size_cfg_mode(&mut self) -> FLIP_SIZE_CFG_MODE_W<CSIC_DMA_EN_SPEC> {
        FLIP_SIZE_CFG_MODE_W::new(self, 28)
    }
    #[doc = "Bit 29 - Buffer length set by software or calculated by hardware"]
    #[inline(always)]
    #[must_use]
    pub fn buf_length_cfg_mode(&mut self) -> BUF_LENGTH_CFG_MODE_W<CSIC_DMA_EN_SPEC> {
        BUF_LENGTH_CFG_MODE_W::new(self, 29)
    }
    #[doc = "Bit 30 - Vflip buffer address set by software or calculated by hardware"]
    #[inline(always)]
    #[must_use]
    pub fn vflip_buf_addr_cfg_mode(&mut self) -> VFLIP_BUF_ADDR_CFG_MODE_W<CSIC_DMA_EN_SPEC> {
        VFLIP_BUF_ADDR_CFG_MODE_W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn ver_en(&mut self) -> VER_EN_W<CSIC_DMA_EN_SPEC> {
        VER_EN_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "CSIC DMA Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIC_DMA_EN_SPEC;
impl crate::RegisterSpec for CSIC_DMA_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csic_dma_en::R`](R) reader structure"]
impl crate::Readable for CSIC_DMA_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csic_dma_en::W`](W) writer structure"]
impl crate::Writable for CSIC_DMA_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_dma_en to value 0x7000_0000"]
impl crate::Resettable for CSIC_DMA_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0x7000_0000;
}
