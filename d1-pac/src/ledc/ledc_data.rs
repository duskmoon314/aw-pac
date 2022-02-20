#[doc = "Register `LEDC_DATA` reader"]
pub struct R(crate::R<LEDC_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LEDC_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LEDC_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LEDC_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LEDC_DATA` writer"]
pub struct W(crate::W<LEDC_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LEDC_DATA_SPEC>;
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
impl From<crate::W<LEDC_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LEDC_DATA_SPEC>) -> Self {
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
#[doc = "LEDC Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_data](index.html) module"]
pub struct LEDC_DATA_SPEC;
impl crate::RegisterSpec for LEDC_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ledc_data::R](R) reader structure"]
impl crate::Readable for LEDC_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ledc_data::W](W) writer structure"]
impl crate::Writable for LEDC_DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LEDC_DATA to value 0"]
impl crate::Resettable for LEDC_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
