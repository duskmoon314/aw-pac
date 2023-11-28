#[doc = "Register `dma_req_en` reader"]
pub type R = crate::R<DMA_REQ_EN_SPEC>;
#[doc = "Register `dma_req_en` writer"]
pub type W = crate::W<DMA_REQ_EN_SPEC>;
#[doc = "Field `rx_req_enable` reader - DMA RX REQ Enable"]
pub type RX_REQ_ENABLE_R = crate::BitReader<RX_REQ_ENABLE_A>;
#[doc = "DMA RX REQ Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_REQ_ENABLE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RX_REQ_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: RX_REQ_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_REQ_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RX_REQ_ENABLE_A {
        match self.bits {
            false => RX_REQ_ENABLE_A::DISABLE,
            true => RX_REQ_ENABLE_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RX_REQ_ENABLE_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RX_REQ_ENABLE_A::ENABLE
    }
}
#[doc = "Field `rx_req_enable` writer - DMA RX REQ Enable"]
pub type RX_REQ_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, RX_REQ_ENABLE_A>;
impl<'a, REG> RX_REQ_ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RX_REQ_ENABLE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RX_REQ_ENABLE_A::ENABLE)
    }
}
#[doc = "Field `tx_req_enable` reader - DMA TX REQ Enable"]
pub type TX_REQ_ENABLE_R = crate::BitReader<TX_REQ_ENABLE_A>;
#[doc = "DMA TX REQ Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_REQ_ENABLE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TX_REQ_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: TX_REQ_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_REQ_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TX_REQ_ENABLE_A {
        match self.bits {
            false => TX_REQ_ENABLE_A::DISABLE,
            true => TX_REQ_ENABLE_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TX_REQ_ENABLE_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TX_REQ_ENABLE_A::ENABLE
    }
}
#[doc = "Field `tx_req_enable` writer - DMA TX REQ Enable"]
pub type TX_REQ_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, TX_REQ_ENABLE_A>;
impl<'a, REG> TX_REQ_ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TX_REQ_ENABLE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TX_REQ_ENABLE_A::ENABLE)
    }
}
#[doc = "Field `timeout_enable` reader - DMA Timeout Enable"]
pub type TIMEOUT_ENABLE_R = crate::BitReader<TIMEOUT_ENABLE_A>;
#[doc = "DMA Timeout Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMEOUT_ENABLE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TIMEOUT_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: TIMEOUT_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMEOUT_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIMEOUT_ENABLE_A {
        match self.bits {
            false => TIMEOUT_ENABLE_A::DISABLE,
            true => TIMEOUT_ENABLE_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TIMEOUT_ENABLE_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TIMEOUT_ENABLE_A::ENABLE
    }
}
#[doc = "Field `timeout_enable` writer - DMA Timeout Enable"]
pub type TIMEOUT_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, TIMEOUT_ENABLE_A>;
impl<'a, REG> TIMEOUT_ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TIMEOUT_ENABLE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TIMEOUT_ENABLE_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - DMA RX REQ Enable"]
    #[inline(always)]
    pub fn rx_req_enable(&self) -> RX_REQ_ENABLE_R {
        RX_REQ_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA TX REQ Enable"]
    #[inline(always)]
    pub fn tx_req_enable(&self) -> TX_REQ_ENABLE_R {
        TX_REQ_ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Timeout Enable"]
    #[inline(always)]
    pub fn timeout_enable(&self) -> TIMEOUT_ENABLE_R {
        TIMEOUT_ENABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA RX REQ Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rx_req_enable(&mut self) -> RX_REQ_ENABLE_W<DMA_REQ_EN_SPEC> {
        RX_REQ_ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA TX REQ Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tx_req_enable(&mut self) -> TX_REQ_ENABLE_W<DMA_REQ_EN_SPEC> {
        TX_REQ_ENABLE_W::new(self, 1)
    }
    #[doc = "Bit 2 - DMA Timeout Enable"]
    #[inline(always)]
    #[must_use]
    pub fn timeout_enable(&mut self) -> TIMEOUT_ENABLE_W<DMA_REQ_EN_SPEC> {
        TIMEOUT_ENABLE_W::new(self, 2)
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
#[doc = "UART DMA Request Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_req_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_req_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_REQ_EN_SPEC;
impl crate::RegisterSpec for DMA_REQ_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_req_en::R`](R) reader structure"]
impl crate::Readable for DMA_REQ_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_req_en::W`](W) writer structure"]
impl crate::Writable for DMA_REQ_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dma_req_en to value 0"]
impl crate::Resettable for DMA_REQ_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
