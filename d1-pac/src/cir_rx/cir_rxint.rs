#[doc = "Register `cir_rxint` reader"]
pub type R = crate::R<CIR_RXINT_SPEC>;
#[doc = "Register `cir_rxint` writer"]
pub type W = crate::W<CIR_RXINT_SPEC>;
#[doc = "Field `roi_en` reader - Receiver FIFO Overrun Interrupt Enable"]
pub type ROI_EN_R = crate::BitReader<ROI_EN_A>;
#[doc = "Receiver FIFO Overrun Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROI_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<ROI_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ROI_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ROI_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ROI_EN_A {
        match self.bits {
            false => ROI_EN_A::DISABLE,
            true => ROI_EN_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ROI_EN_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ROI_EN_A::ENABLE
    }
}
#[doc = "Field `roi_en` writer - Receiver FIFO Overrun Interrupt Enable"]
pub type ROI_EN_W<'a, REG> = crate::BitWriter<'a, REG, ROI_EN_A>;
impl<'a, REG> ROI_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ROI_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ROI_EN_A::ENABLE)
    }
}
#[doc = "Field `rpei_en` reader - Receiver Packet End Interrupt Enable"]
pub type RPEI_EN_R = crate::BitReader<RPEI_EN_A>;
#[doc = "Receiver Packet End Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPEI_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<RPEI_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RPEI_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RPEI_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RPEI_EN_A {
        match self.bits {
            false => RPEI_EN_A::DISABLE,
            true => RPEI_EN_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RPEI_EN_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RPEI_EN_A::ENABLE
    }
}
#[doc = "Field `rpei_en` writer - Receiver Packet End Interrupt Enable"]
pub type RPEI_EN_W<'a, REG> = crate::BitWriter<'a, REG, RPEI_EN_A>;
impl<'a, REG> RPEI_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RPEI_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RPEI_EN_A::ENABLE)
    }
}
#[doc = "Field `rai_en` reader - RX FIFO Available Interrupt Enable"]
pub type RAI_EN_R = crate::BitReader<RAI_EN_A>;
#[doc = "RX FIFO Available Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAI_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<RAI_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RAI_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RAI_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RAI_EN_A {
        match self.bits {
            false => RAI_EN_A::DISABLE,
            true => RAI_EN_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RAI_EN_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RAI_EN_A::ENABLE
    }
}
#[doc = "Field `rai_en` writer - RX FIFO Available Interrupt Enable"]
pub type RAI_EN_W<'a, REG> = crate::BitWriter<'a, REG, RAI_EN_A>;
impl<'a, REG> RAI_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RAI_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RAI_EN_A::ENABLE)
    }
}
#[doc = "Field `drq_en` reader - RX FIFO DMA Enable"]
pub type DRQ_EN_R = crate::BitReader<DRQ_EN_A>;
#[doc = "RX FIFO DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRQ_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<DRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DRQ_EN_A {
        match self.bits {
            false => DRQ_EN_A::DISABLE,
            true => DRQ_EN_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DRQ_EN_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DRQ_EN_A::ENABLE
    }
}
#[doc = "Field `drq_en` writer - RX FIFO DMA Enable"]
pub type DRQ_EN_W<'a, REG> = crate::BitWriter<'a, REG, DRQ_EN_A>;
impl<'a, REG> DRQ_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DRQ_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DRQ_EN_A::ENABLE)
    }
}
#[doc = "Field `ral` reader - RX FIFO available received byte level for interrupt and DMA request\n\nTRIGGER_LEVEL = RAL + 1"]
pub type RAL_R = crate::FieldReader;
#[doc = "Field `ral` writer - RX FIFO available received byte level for interrupt and DMA request\n\nTRIGGER_LEVEL = RAL + 1"]
pub type RAL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Receiver FIFO Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn roi_en(&self) -> ROI_EN_R {
        ROI_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receiver Packet End Interrupt Enable"]
    #[inline(always)]
    pub fn rpei_en(&self) -> RPEI_EN_R {
        RPEI_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - RX FIFO Available Interrupt Enable"]
    #[inline(always)]
    pub fn rai_en(&self) -> RAI_EN_R {
        RAI_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RX FIFO DMA Enable"]
    #[inline(always)]
    pub fn drq_en(&self) -> DRQ_EN_R {
        DRQ_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:13 - RX FIFO available received byte level for interrupt and DMA request\n\nTRIGGER_LEVEL = RAL + 1"]
    #[inline(always)]
    pub fn ral(&self) -> RAL_R {
        RAL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Receiver FIFO Overrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn roi_en(&mut self) -> ROI_EN_W<CIR_RXINT_SPEC> {
        ROI_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Receiver Packet End Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rpei_en(&mut self) -> RPEI_EN_W<CIR_RXINT_SPEC> {
        RPEI_EN_W::new(self, 1)
    }
    #[doc = "Bit 4 - RX FIFO Available Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rai_en(&mut self) -> RAI_EN_W<CIR_RXINT_SPEC> {
        RAI_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - RX FIFO DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn drq_en(&mut self) -> DRQ_EN_W<CIR_RXINT_SPEC> {
        DRQ_EN_W::new(self, 5)
    }
    #[doc = "Bits 8:13 - RX FIFO available received byte level for interrupt and DMA request\n\nTRIGGER_LEVEL = RAL + 1"]
    #[inline(always)]
    #[must_use]
    pub fn ral(&mut self) -> RAL_W<CIR_RXINT_SPEC> {
        RAL_W::new(self, 8)
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
#[doc = "CIR Receiver Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cir_rxint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cir_rxint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIR_RXINT_SPEC;
impl crate::RegisterSpec for CIR_RXINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cir_rxint::R`](R) reader structure"]
impl crate::Readable for CIR_RXINT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cir_rxint::W`](W) writer structure"]
impl crate::Writable for CIR_RXINT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cir_rxint to value 0"]
impl crate::Resettable for CIR_RXINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
