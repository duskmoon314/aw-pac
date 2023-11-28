#[doc = "Register `i2s_pcm_int` reader"]
pub type R = crate::R<I2S_PCM_INT_SPEC>;
#[doc = "Register `i2s_pcm_int` writer"]
pub type W = crate::W<I2S_PCM_INT_SPEC>;
#[doc = "Field `rxai_en` reader - RXFIFO Data Available Interrupt Enable"]
pub type RXAI_EN_R = crate::BitReader<RXAI_EN_A>;
#[doc = "RXFIFO Data Available Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXAI_EN_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<RXAI_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RXAI_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RXAI_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXAI_EN_A {
        match self.bits {
            false => RXAI_EN_A::DISABLE,
            true => RXAI_EN_A::ENABLE,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RXAI_EN_A::DISABLE
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RXAI_EN_A::ENABLE
    }
}
#[doc = "Field `rxai_en` writer - RXFIFO Data Available Interrupt Enable"]
pub type RXAI_EN_W<'a, REG> = crate::BitWriter<'a, REG, RXAI_EN_A>;
impl<'a, REG> RXAI_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RXAI_EN_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RXAI_EN_A::ENABLE)
    }
}
#[doc = "Field `rxoi_en` reader - RXFIFO Overrun Interrupt Enable"]
pub type RXOI_EN_R = crate::BitReader<RXOI_EN_A>;
#[doc = "RXFIFO Overrun Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXOI_EN_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<RXOI_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RXOI_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RXOI_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXOI_EN_A {
        match self.bits {
            false => RXOI_EN_A::DISABLE,
            true => RXOI_EN_A::ENABLE,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RXOI_EN_A::DISABLE
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RXOI_EN_A::ENABLE
    }
}
#[doc = "Field `rxoi_en` writer - RXFIFO Overrun Interrupt Enable"]
pub type RXOI_EN_W<'a, REG> = crate::BitWriter<'a, REG, RXOI_EN_A>;
impl<'a, REG> RXOI_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RXOI_EN_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RXOI_EN_A::ENABLE)
    }
}
#[doc = "Field `rxui_en` reader - RXFIFO Underrun Interrupt Enable"]
pub type RXUI_EN_R = crate::BitReader<RXUI_EN_A>;
#[doc = "RXFIFO Underrun Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXUI_EN_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<RXUI_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RXUI_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RXUI_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXUI_EN_A {
        match self.bits {
            false => RXUI_EN_A::DISABLE,
            true => RXUI_EN_A::ENABLE,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RXUI_EN_A::DISABLE
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RXUI_EN_A::ENABLE
    }
}
#[doc = "Field `rxui_en` writer - RXFIFO Underrun Interrupt Enable"]
pub type RXUI_EN_W<'a, REG> = crate::BitWriter<'a, REG, RXUI_EN_A>;
impl<'a, REG> RXUI_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RXUI_EN_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RXUI_EN_A::ENABLE)
    }
}
#[doc = "Field `rx_drq` reader - RXFIFO Data Available DRQ Enable"]
pub type RX_DRQ_R = crate::BitReader<RX_DRQ_A>;
#[doc = "RXFIFO Data Available DRQ Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_DRQ_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<RX_DRQ_A> for bool {
    #[inline(always)]
    fn from(variant: RX_DRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_DRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RX_DRQ_A {
        match self.bits {
            false => RX_DRQ_A::DISABLE,
            true => RX_DRQ_A::ENABLE,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RX_DRQ_A::DISABLE
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RX_DRQ_A::ENABLE
    }
}
#[doc = "Field `rx_drq` writer - RXFIFO Data Available DRQ Enable"]
pub type RX_DRQ_W<'a, REG> = crate::BitWriter<'a, REG, RX_DRQ_A>;
impl<'a, REG> RX_DRQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RX_DRQ_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RX_DRQ_A::ENABLE)
    }
}
#[doc = "Field `txei_en` reader - TXFIFO Empty Interrupt Enable"]
pub type TXEI_EN_R = crate::BitReader<TXEI_EN_A>;
#[doc = "TXFIFO Empty Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXEI_EN_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<TXEI_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TXEI_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TXEI_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXEI_EN_A {
        match self.bits {
            false => TXEI_EN_A::DISABLE,
            true => TXEI_EN_A::ENABLE,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TXEI_EN_A::DISABLE
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TXEI_EN_A::ENABLE
    }
}
#[doc = "Field `txei_en` writer - TXFIFO Empty Interrupt Enable"]
pub type TXEI_EN_W<'a, REG> = crate::BitWriter<'a, REG, TXEI_EN_A>;
impl<'a, REG> TXEI_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TXEI_EN_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TXEI_EN_A::ENABLE)
    }
}
#[doc = "Field `txoi_en` reader - TXFIFO Overrun Interrupt Enable"]
pub type TXOI_EN_R = crate::BitReader<TXOI_EN_A>;
#[doc = "TXFIFO Overrun Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXOI_EN_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<TXOI_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TXOI_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TXOI_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXOI_EN_A {
        match self.bits {
            false => TXOI_EN_A::DISABLE,
            true => TXOI_EN_A::ENABLE,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TXOI_EN_A::DISABLE
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TXOI_EN_A::ENABLE
    }
}
#[doc = "Field `txoi_en` writer - TXFIFO Overrun Interrupt Enable"]
pub type TXOI_EN_W<'a, REG> = crate::BitWriter<'a, REG, TXOI_EN_A>;
impl<'a, REG> TXOI_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TXOI_EN_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TXOI_EN_A::ENABLE)
    }
}
#[doc = "Field `txui_en` reader - TXFIFO Underrun Interrupt Enable"]
pub type TXUI_EN_R = crate::BitReader<TXUI_EN_A>;
#[doc = "TXFIFO Underrun Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXUI_EN_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<TXUI_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TXUI_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TXUI_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXUI_EN_A {
        match self.bits {
            false => TXUI_EN_A::DISABLE,
            true => TXUI_EN_A::ENABLE,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TXUI_EN_A::DISABLE
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TXUI_EN_A::ENABLE
    }
}
#[doc = "Field `txui_en` writer - TXFIFO Underrun Interrupt Enable"]
pub type TXUI_EN_W<'a, REG> = crate::BitWriter<'a, REG, TXUI_EN_A>;
impl<'a, REG> TXUI_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TXUI_EN_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TXUI_EN_A::ENABLE)
    }
}
#[doc = "Field `tx_drq` reader - TXFIFO Empty DRQ Enable"]
pub type TX_DRQ_R = crate::BitReader<TX_DRQ_A>;
#[doc = "TXFIFO Empty DRQ Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_DRQ_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<TX_DRQ_A> for bool {
    #[inline(always)]
    fn from(variant: TX_DRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_DRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TX_DRQ_A {
        match self.bits {
            false => TX_DRQ_A::DISABLE,
            true => TX_DRQ_A::ENABLE,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TX_DRQ_A::DISABLE
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TX_DRQ_A::ENABLE
    }
}
#[doc = "Field `tx_drq` writer - TXFIFO Empty DRQ Enable"]
pub type TX_DRQ_W<'a, REG> = crate::BitWriter<'a, REG, TX_DRQ_A>;
impl<'a, REG> TX_DRQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TX_DRQ_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TX_DRQ_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - RXFIFO Data Available Interrupt Enable"]
    #[inline(always)]
    pub fn rxai_en(&self) -> RXAI_EN_R {
        RXAI_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RXFIFO Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn rxoi_en(&self) -> RXOI_EN_R {
        RXOI_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RXFIFO Underrun Interrupt Enable"]
    #[inline(always)]
    pub fn rxui_en(&self) -> RXUI_EN_R {
        RXUI_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RXFIFO Data Available DRQ Enable"]
    #[inline(always)]
    pub fn rx_drq(&self) -> RX_DRQ_R {
        RX_DRQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TXFIFO Empty Interrupt Enable"]
    #[inline(always)]
    pub fn txei_en(&self) -> TXEI_EN_R {
        TXEI_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TXFIFO Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn txoi_en(&self) -> TXOI_EN_R {
        TXOI_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TXFIFO Underrun Interrupt Enable"]
    #[inline(always)]
    pub fn txui_en(&self) -> TXUI_EN_R {
        TXUI_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TXFIFO Empty DRQ Enable"]
    #[inline(always)]
    pub fn tx_drq(&self) -> TX_DRQ_R {
        TX_DRQ_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXFIFO Data Available Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxai_en(&mut self) -> RXAI_EN_W<I2S_PCM_INT_SPEC> {
        RXAI_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - RXFIFO Overrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxoi_en(&mut self) -> RXOI_EN_W<I2S_PCM_INT_SPEC> {
        RXOI_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - RXFIFO Underrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxui_en(&mut self) -> RXUI_EN_W<I2S_PCM_INT_SPEC> {
        RXUI_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - RXFIFO Data Available DRQ Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rx_drq(&mut self) -> RX_DRQ_W<I2S_PCM_INT_SPEC> {
        RX_DRQ_W::new(self, 3)
    }
    #[doc = "Bit 4 - TXFIFO Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txei_en(&mut self) -> TXEI_EN_W<I2S_PCM_INT_SPEC> {
        TXEI_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - TXFIFO Overrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txoi_en(&mut self) -> TXOI_EN_W<I2S_PCM_INT_SPEC> {
        TXOI_EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - TXFIFO Underrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txui_en(&mut self) -> TXUI_EN_W<I2S_PCM_INT_SPEC> {
        TXUI_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - TXFIFO Empty DRQ Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tx_drq(&mut self) -> TX_DRQ_W<I2S_PCM_INT_SPEC> {
        TX_DRQ_W::new(self, 7)
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
#[doc = "I2S/PCM DMA and Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_int::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_int::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2S_PCM_INT_SPEC;
impl crate::RegisterSpec for I2S_PCM_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_pcm_int::R`](R) reader structure"]
impl crate::Readable for I2S_PCM_INT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2s_pcm_int::W`](W) writer structure"]
impl crate::Writable for I2S_PCM_INT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets i2s_pcm_int to value 0"]
impl crate::Resettable for I2S_PCM_INT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
