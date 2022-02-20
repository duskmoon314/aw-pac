#[doc = "Register `E_HCIVERSION` reader"]
pub struct R(crate::R<E_HCIVERSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<E_HCIVERSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<E_HCIVERSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<E_HCIVERSION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `E_HCIVERSION` writer"]
pub struct W(crate::W<E_HCIVERSION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<E_HCIVERSION_SPEC>;
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
impl From<crate::W<E_HCIVERSION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<E_HCIVERSION_SPEC>) -> Self {
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
#[doc = "EHCI Host Interface Version Number Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [e_hciversion](index.html) module"]
pub struct E_HCIVERSION_SPEC;
impl crate::RegisterSpec for E_HCIVERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [e_hciversion::R](R) reader structure"]
impl crate::Readable for E_HCIVERSION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [e_hciversion::W](W) writer structure"]
impl crate::Writable for E_HCIVERSION_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets E_HCIVERSION to value 0"]
impl crate::Resettable for E_HCIVERSION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
