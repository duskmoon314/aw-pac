#[doc = "Register `IRTX_BGR` reader"]
pub struct R(crate::R<IRTX_BGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRTX_BGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRTX_BGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRTX_BGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRTX_BGR` writer"]
pub struct W(crate::W<IRTX_BGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRTX_BGR_SPEC>;
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
impl From<crate::W<IRTX_BGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRTX_BGR_SPEC>) -> Self {
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
#[doc = "IRTX Bus Gating Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irtx_bgr](index.html) module"]
pub struct IRTX_BGR_SPEC;
impl crate::RegisterSpec for IRTX_BGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irtx_bgr::R](R) reader structure"]
impl crate::Readable for IRTX_BGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irtx_bgr::W](W) writer structure"]
impl crate::Writable for IRTX_BGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IRTX_BGR to value 0"]
impl crate::Resettable for IRTX_BGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
