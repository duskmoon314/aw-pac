#[doc = "Register `RXDMA_WADDRH` reader"]
pub struct R(crate::R<RXDMA_WADDRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXDMA_WADDRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXDMA_WADDRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXDMA_WADDRH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXDMA_WADDRH` writer"]
pub struct W(crate::W<RXDMA_WADDRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXDMA_WADDRH_SPEC>;
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
impl From<crate::W<RXDMA_WADDRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXDMA_WADDRH_SPEC>) -> Self {
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
#[doc = "UART RXDMA Write Address High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdma_waddrh](index.html) module"]
pub struct RXDMA_WADDRH_SPEC;
impl crate::RegisterSpec for RXDMA_WADDRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxdma_waddrh::R](R) reader structure"]
impl crate::Readable for RXDMA_WADDRH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxdma_waddrh::W](W) writer structure"]
impl crate::Writable for RXDMA_WADDRH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXDMA_WADDRH to value 0"]
impl crate::Resettable for RXDMA_WADDRH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
