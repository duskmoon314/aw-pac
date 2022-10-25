#[doc = "Register `dmac_en%s` reader"]
pub struct R(crate::R<DMAC_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dmac_en%s` writer"]
pub struct W(crate::W<DMAC_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAC_EN_SPEC>;
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
impl From<crate::W<DMAC_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAC_EN_SPEC>) -> Self {
        W(writer)
    }
}
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
    pub fn variant(&self) -> DMA_EN_A {
        match self.bits {
            false => DMA_EN_A::DISABLED,
            true => DMA_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA_EN_A::ENABLED
    }
}
#[doc = "Field `dma_en` writer - DMA Channel Enable"]
pub type DMA_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAC_EN_SPEC, DMA_EN_A, O>;
impl<'a, const O: u8> DMA_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA_EN_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
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
    pub fn dma_en(&mut self) -> DMA_EN_W<0> {
        DMA_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Channel Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_en](index.html) module"]
pub struct DMAC_EN_SPEC;
impl crate::RegisterSpec for DMAC_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac_en::R](R) reader structure"]
impl crate::Readable for DMAC_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmac_en::W](W) writer structure"]
impl crate::Writable for DMAC_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dmac_en%s to value 0"]
impl crate::Resettable for DMAC_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
