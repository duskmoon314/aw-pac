#[doc = "Register `rxdma_saddrl` reader"]
pub struct R(crate::R<RXDMA_SADDRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXDMA_SADDRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXDMA_SADDRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXDMA_SADDRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rxdma_saddrl` writer"]
pub struct W(crate::W<RXDMA_SADDRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXDMA_SADDRL_SPEC>;
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
impl From<crate::W<RXDMA_SADDRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXDMA_SADDRL_SPEC>) -> Self {
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
#[doc = "UART RXDMA Buffer Start Address Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdma_saddrl](index.html) module"]
pub struct RXDMA_SADDRL_SPEC;
impl crate::RegisterSpec for RXDMA_SADDRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxdma_saddrl::R](R) reader structure"]
impl crate::Readable for RXDMA_SADDRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxdma_saddrl::W](W) writer structure"]
impl crate::Writable for RXDMA_SADDRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rxdma_saddrl to value 0"]
impl crate::Resettable for RXDMA_SADDRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
