#[doc = "Register `dmac_pau%s` reader"]
pub type R = crate::R<DMAC_PAU_SPEC>;
#[doc = "Register `dmac_pau%s` writer"]
pub type W = crate::W<DMAC_PAU_SPEC>;
#[doc = "Field `dma_pause` reader - Pause the DMA Channel Transfer Data"]
pub type DMA_PAUSE_R = crate::BitReader<DMA_PAUSE_A>;
#[doc = "Pause the DMA Channel Transfer Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_PAUSE_A {
    #[doc = "0: `0`"]
    RESUME = 0,
    #[doc = "1: `1`"]
    PAUSE = 1,
}
impl From<DMA_PAUSE_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_PAUSE_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_PAUSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA_PAUSE_A {
        match self.bits {
            false => DMA_PAUSE_A::RESUME,
            true => DMA_PAUSE_A::PAUSE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_resume(&self) -> bool {
        *self == DMA_PAUSE_A::RESUME
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pause(&self) -> bool {
        *self == DMA_PAUSE_A::PAUSE
    }
}
#[doc = "Field `dma_pause` writer - Pause the DMA Channel Transfer Data"]
pub type DMA_PAUSE_W<'a, REG> = crate::BitWriter<'a, REG, DMA_PAUSE_A>;
impl<'a, REG> DMA_PAUSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_PAUSE_A::RESUME)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pause(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_PAUSE_A::PAUSE)
    }
}
impl R {
    #[doc = "Bit 0 - Pause the DMA Channel Transfer Data"]
    #[inline(always)]
    pub fn dma_pause(&self) -> DMA_PAUSE_R {
        DMA_PAUSE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pause the DMA Channel Transfer Data"]
    #[inline(always)]
    #[must_use]
    pub fn dma_pause(&mut self) -> DMA_PAUSE_W<DMAC_PAU_SPEC> {
        DMA_PAUSE_W::new(self, 0)
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
#[doc = "DMAC Channel Pause Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_pau::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac_pau::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAC_PAU_SPEC;
impl crate::RegisterSpec for DMAC_PAU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_pau::R`](R) reader structure"]
impl crate::Readable for DMAC_PAU_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmac_pau::W`](W) writer structure"]
impl crate::Writable for DMAC_PAU_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dmac_pau%s to value 0"]
impl crate::Resettable for DMAC_PAU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
