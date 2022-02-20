#[doc = "Register `E_HCSPPORTROUTE` reader"]
pub struct R(crate::R<E_HCSPPORTROUTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<E_HCSPPORTROUTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<E_HCSPPORTROUTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<E_HCSPPORTROUTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `E_HCSPPORTROUTE` writer"]
pub struct W(crate::W<E_HCSPPORTROUTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<E_HCSPPORTROUTE_SPEC>;
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
impl From<crate::W<E_HCSPPORTROUTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<E_HCSPPORTROUTE_SPEC>) -> Self {
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
#[doc = "EHCI Companion Port Route Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [e_hcspportroute](index.html) module"]
pub struct E_HCSPPORTROUTE_SPEC;
impl crate::RegisterSpec for E_HCSPPORTROUTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [e_hcspportroute::R](R) reader structure"]
impl crate::Readable for E_HCSPPORTROUTE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [e_hcspportroute::W](W) writer structure"]
impl crate::Writable for E_HCSPPORTROUTE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets E_HCSPPORTROUTE to value 0"]
impl crate::Resettable for E_HCSPPORTROUTE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
