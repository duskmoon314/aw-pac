#[doc = "Register `PCGR` reader"]
pub struct R(crate::R<PCGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCGR` writer"]
pub struct W(crate::W<PCGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCGR_SPEC>;
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
impl From<crate::W<PCGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCGR_SPEC>) -> Self {
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
#[doc = "PWM Clock Gating Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcgr](index.html) module"]
pub struct PCGR_SPEC;
impl crate::RegisterSpec for PCGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcgr::R](R) reader structure"]
impl crate::Readable for PCGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcgr::W](W) writer structure"]
impl crate::Writable for PCGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCGR to value 0"]
impl crate::Resettable for PCGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
