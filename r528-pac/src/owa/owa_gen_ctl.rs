#[doc = "Register `OWA_GEN_CTL` reader"]
pub struct R(crate::R<OWA_GEN_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OWA_GEN_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OWA_GEN_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OWA_GEN_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OWA_GEN_CTL` writer"]
pub struct W(crate::W<OWA_GEN_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OWA_GEN_CTL_SPEC>;
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
impl From<crate::W<OWA_GEN_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OWA_GEN_CTL_SPEC>) -> Self {
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
#[doc = "OWA General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [owa_gen_ctl](index.html) module"]
pub struct OWA_GEN_CTL_SPEC;
impl crate::RegisterSpec for OWA_GEN_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [owa_gen_ctl::R](R) reader structure"]
impl crate::Readable for OWA_GEN_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [owa_gen_ctl::W](W) writer structure"]
impl crate::Writable for OWA_GEN_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OWA_GEN_CTL to value 0"]
impl crate::Resettable for OWA_GEN_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
