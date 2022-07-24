#[doc = "Register `csic_dma1_input_sel` reader"]
pub struct R(crate::R<CSIC_DMA1_INPUT_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIC_DMA1_INPUT_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIC_DMA1_INPUT_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIC_DMA1_INPUT_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csic_dma1_input_sel` writer"]
pub struct W(crate::W<CSIC_DMA1_INPUT_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIC_DMA1_INPUT_SEL_SPEC>;
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
impl From<crate::W<CSIC_DMA1_INPUT_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIC_DMA1_INPUT_SEL_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSIC DMA1 Input Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csic_dma1_input_sel](index.html) module"]
pub struct CSIC_DMA1_INPUT_SEL_SPEC;
impl crate::RegisterSpec for CSIC_DMA1_INPUT_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csic_dma1_input_sel::R](R) reader structure"]
impl crate::Readable for CSIC_DMA1_INPUT_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csic_dma1_input_sel::W](W) writer structure"]
impl crate::Writable for CSIC_DMA1_INPUT_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets csic_dma1_input_sel to value 0"]
impl crate::Resettable for CSIC_DMA1_INPUT_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
