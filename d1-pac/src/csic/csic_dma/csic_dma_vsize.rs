#[doc = "Register `csic_dma_vsize` reader"]
pub struct R(crate::R<CSIC_DMA_VSIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIC_DMA_VSIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIC_DMA_VSIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIC_DMA_VSIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csic_dma_vsize` writer"]
pub struct W(crate::W<CSIC_DMA_VSIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIC_DMA_VSIZE_SPEC>;
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
impl From<crate::W<CSIC_DMA_VSIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIC_DMA_VSIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ver_start` reader - Vertical line start\n\nData is valid from this line."]
pub type VER_START_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ver_start` writer - Vertical line start\n\nData is valid from this line."]
pub type VER_START_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSIC_DMA_VSIZE_SPEC, u16, u16, 13, O>;
#[doc = "Field `ver_len` reader - When BK_TOP_EN is enabled, DMA_EN is enabled, these bits indicate Valid line number of a frame in DMA mode."]
pub type VER_LEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ver_len` writer - When BK_TOP_EN is enabled, DMA_EN is enabled, these bits indicate Valid line number of a frame in DMA mode."]
pub type VER_LEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSIC_DMA_VSIZE_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 0:12 - Vertical line start\n\nData is valid from this line."]
    #[inline(always)]
    pub fn ver_start(&self) -> VER_START_R {
        VER_START_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - When BK_TOP_EN is enabled, DMA_EN is enabled, these bits indicate Valid line number of a frame in DMA mode."]
    #[inline(always)]
    pub fn ver_len(&self) -> VER_LEN_R {
        VER_LEN_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Vertical line start\n\nData is valid from this line."]
    #[inline(always)]
    #[must_use]
    pub fn ver_start(&mut self) -> VER_START_W<0> {
        VER_START_W::new(self)
    }
    #[doc = "Bits 16:28 - When BK_TOP_EN is enabled, DMA_EN is enabled, these bits indicate Valid line number of a frame in DMA mode."]
    #[inline(always)]
    #[must_use]
    pub fn ver_len(&mut self) -> VER_LEN_W<16> {
        VER_LEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSIC DMA Vertical Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csic_dma_vsize](index.html) module"]
pub struct CSIC_DMA_VSIZE_SPEC;
impl crate::RegisterSpec for CSIC_DMA_VSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csic_dma_vsize::R](R) reader structure"]
impl crate::Readable for CSIC_DMA_VSIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csic_dma_vsize::W](W) writer structure"]
impl crate::Writable for CSIC_DMA_VSIZE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_dma_vsize to value 0x02d0_0000"]
impl crate::Resettable for CSIC_DMA_VSIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0x02d0_0000;
}
