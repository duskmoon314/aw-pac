#[doc = "Register `dmac_en%s` reader"]
pub type R = crate::R<DMAC_EN_SPEC>;
#[doc = "Register `dmac_en%s` writer"]
pub type W = crate::W<DMAC_EN_SPEC>;
#[doc = "Field `dma_en` reader - DMA Channel Enable"]
pub type DMA_EN_R = crate::BitReader<DMA_EN_A>;
#[doc = "DMA Channel Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_EN_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
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
            false => DMA_EN_A::DISABLED,
            true => DMA_EN_A::ENABLED,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA_EN_A::DISABLED
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA_EN_A::ENABLED
    }
}
#[doc = "Field `dma_en` writer - DMA Channel Enable"]
pub type DMA_EN_W<'a, REG> = crate::BitWriter<'a, REG, DMA_EN_A>;
impl<'a, REG> DMA_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_EN_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_EN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - DMA Channel Enable"]
    #[inline(always)]
    pub fn dma_en(&self) -> DMA_EN_R {
        DMA_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Channel Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_en(&mut self) -> DMA_EN_W<DMAC_EN_SPEC> {
        DMA_EN_W::new(self, 0)
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
#[doc = "DMAC Channel Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAC_EN_SPEC;
impl crate::RegisterSpec for DMAC_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_en::R`](R) reader structure"]
impl crate::Readable for DMAC_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmac_en::W`](W) writer structure"]
impl crate::Writable for DMAC_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dmac_en%s to value 0"]
impl crate::Resettable for DMAC_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
