#[doc = "Register `tv_ceu_coef_rang%s` reader"]
pub struct R(crate::R<TV_CEU_COEF_RANG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TV_CEU_COEF_RANG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TV_CEU_COEF_RANG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TV_CEU_COEF_RANG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tv_ceu_coef_rang%s` writer"]
pub struct W(crate::W<TV_CEU_COEF_RANG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TV_CEU_COEF_RANG_SPEC>;
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
impl From<crate::W<TV_CEU_COEF_RANG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TV_CEU_COEF_RANG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ceu_coef_range_max` reader - Unsigned 10-bit Value, range of \\[0, 1023\\]"]
pub type CEU_COEF_RANGE_MAX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ceu_coef_range_max` writer - Unsigned 10-bit Value, range of \\[0, 1023\\]"]
pub type CEU_COEF_RANGE_MAX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TV_CEU_COEF_RANG_SPEC, u16, u16, 10, O>;
#[doc = "Field `ceu_coef_range_min` reader - Unsigned 10-bit Value, range of \\[0, 1023\\]"]
pub type CEU_COEF_RANGE_MIN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ceu_coef_range_min` writer - Unsigned 10-bit Value, range of \\[0, 1023\\]"]
pub type CEU_COEF_RANGE_MIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TV_CEU_COEF_RANG_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Unsigned 10-bit Value, range of \\[0, 1023\\]"]
    #[inline(always)]
    pub fn ceu_coef_range_max(&self) -> CEU_COEF_RANGE_MAX_R {
        CEU_COEF_RANGE_MAX_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Unsigned 10-bit Value, range of \\[0, 1023\\]"]
    #[inline(always)]
    pub fn ceu_coef_range_min(&self) -> CEU_COEF_RANGE_MIN_R {
        CEU_COEF_RANGE_MIN_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Unsigned 10-bit Value, range of \\[0, 1023\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ceu_coef_range_max(&mut self) -> CEU_COEF_RANGE_MAX_W<0> {
        CEU_COEF_RANGE_MAX_W::new(self)
    }
    #[doc = "Bits 16:25 - Unsigned 10-bit Value, range of \\[0, 1023\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ceu_coef_range_min(&mut self) -> CEU_COEF_RANGE_MIN_W<16> {
        CEU_COEF_RANGE_MIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV CEU Coefficient Register2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tv_ceu_coef_rang](index.html) module"]
pub struct TV_CEU_COEF_RANG_SPEC;
impl crate::RegisterSpec for TV_CEU_COEF_RANG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tv_ceu_coef_rang::R](R) reader structure"]
impl crate::Readable for TV_CEU_COEF_RANG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tv_ceu_coef_rang::W](W) writer structure"]
impl crate::Writable for TV_CEU_COEF_RANG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tv_ceu_coef_rang%s to value 0"]
impl crate::Resettable for TV_CEU_COEF_RANG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
