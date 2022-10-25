#[doc = "Register `dmac_pau%s` reader"]
pub struct R(crate::R<DMAC_PAU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC_PAU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC_PAU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC_PAU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dmac_pau%s` writer"]
pub struct W(crate::W<DMAC_PAU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAC_PAU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DMAC_PAU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAC_PAU_SPEC>) -> Self {
        W(writer)
    }
}
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
    pub fn variant(&self) -> DMA_PAUSE_A {
        match self.bits {
            false => DMA_PAUSE_A::RESUME,
            true => DMA_PAUSE_A::PAUSE,
        }
    }
    #[doc = "Checks if the value of the field is `RESUME`"]
    #[inline(always)]
    pub fn is_resume(&self) -> bool {
        *self == DMA_PAUSE_A::RESUME
    }
    #[doc = "Checks if the value of the field is `PAUSE`"]
    #[inline(always)]
    pub fn is_pause(&self) -> bool {
        *self == DMA_PAUSE_A::PAUSE
    }
}
#[doc = "Field `dma_pause` writer - Pause the DMA Channel Transfer Data"]
pub type DMA_PAUSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAC_PAU_SPEC, DMA_PAUSE_A, O>;
impl<'a, const O: u8> DMA_PAUSE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut W {
        self.variant(DMA_PAUSE_A::RESUME)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pause(self) -> &'a mut W {
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
    pub fn dma_pause(&mut self) -> DMA_PAUSE_W<0> {
        DMA_PAUSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Channel Pause Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_pau](index.html) module"]
pub struct DMAC_PAU_SPEC;
impl crate::RegisterSpec for DMAC_PAU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac_pau::R](R) reader structure"]
impl crate::Readable for DMAC_PAU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmac_pau::W](W) writer structure"]
impl crate::Writable for DMAC_PAU_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dmac_pau%s to value 0"]
impl crate::Resettable for DMAC_PAU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
