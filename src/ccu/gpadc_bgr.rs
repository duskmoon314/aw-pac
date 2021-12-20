#[doc = "Register `GPADC_BGR` reader"]
pub struct R(crate::R<GPADC_BGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPADC_BGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPADC_BGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPADC_BGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPADC_BGR` writer"]
pub struct W(crate::W<GPADC_BGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPADC_BGR_SPEC>;
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
impl From<crate::W<GPADC_BGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPADC_BGR_SPEC>) -> Self {
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
#[doc = "GPADC Bus Gating Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_bgr](index.html) module"]
pub struct GPADC_BGR_SPEC;
impl crate::RegisterSpec for GPADC_BGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpadc_bgr::R](R) reader structure"]
impl crate::Readable for GPADC_BGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpadc_bgr::W](W) writer structure"]
impl crate::Writable for GPADC_BGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPADC_BGR to value 0"]
impl crate::Resettable for GPADC_BGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
