#[doc = "Register `tve_low_pass_coring` reader"]
pub struct R(crate::R<TVE_LOW_PASS_CORING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVE_LOW_PASS_CORING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVE_LOW_PASS_CORING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVE_LOW_PASS_CORING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tve_low_pass_coring` writer"]
pub struct W(crate::W<TVE_LOW_PASS_CORING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVE_LOW_PASS_CORING_SPEC>;
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
impl From<crate::W<TVE_LOW_PASS_CORING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVE_LOW_PASS_CORING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `corthr` reader - Coring threshold."]
pub type CORTHR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `corthr` writer - Coring threshold."]
pub type CORTHR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TVE_LOW_PASS_CORING_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Coring threshold."]
    #[inline(always)]
    pub fn corthr(&self) -> CORTHR_R {
        CORTHR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Coring threshold."]
    #[inline(always)]
    #[must_use]
    pub fn corthr(&mut self) -> CORTHR_W<0> {
        CORTHR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Encoder Low Pass Coring Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tve_low_pass_coring](index.html) module"]
pub struct TVE_LOW_PASS_CORING_SPEC;
impl crate::RegisterSpec for TVE_LOW_PASS_CORING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tve_low_pass_coring::R](R) reader structure"]
impl crate::Readable for TVE_LOW_PASS_CORING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tve_low_pass_coring::W](W) writer structure"]
impl crate::Writable for TVE_LOW_PASS_CORING_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_low_pass_coring to value 0"]
impl crate::Resettable for TVE_LOW_PASS_CORING_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
