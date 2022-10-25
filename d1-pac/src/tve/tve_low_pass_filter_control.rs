#[doc = "Register `tve_low_pass_filter_control` reader"]
pub struct R(crate::R<TVE_LOW_PASS_FILTER_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVE_LOW_PASS_FILTER_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVE_LOW_PASS_FILTER_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVE_LOW_PASS_FILTER_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tve_low_pass_filter_control` writer"]
pub struct W(crate::W<TVE_LOW_PASS_FILTER_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVE_LOW_PASS_FILTER_CONTROL_SPEC>;
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
impl From<crate::W<TVE_LOW_PASS_FILTER_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVE_LOW_PASS_FILTER_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `bp1_ratio` reader - Default band-pass filter1 ratio\n\nIn two complement, the range is from -31 to 31."]
pub type BP1_RATIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `bp1_ratio` writer - Default band-pass filter1 ratio\n\nIn two complement, the range is from -31 to 31."]
pub type BP1_RATIO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TVE_LOW_PASS_FILTER_CONTROL_SPEC, u8, u8, 6, O>;
#[doc = "Field `bp0_ratio` reader - Default band-pass filter0 ratio\n\nIn two complement, the range is from -31 to 31."]
pub type BP0_RATIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `bp0_ratio` writer - Default band-pass filter0 ratio\n\nIn two complement, the range is from -31 to 31."]
pub type BP0_RATIO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TVE_LOW_PASS_FILTER_CONTROL_SPEC, u8, u8, 6, O>;
#[doc = "Field `hp_ratio` reader - Default high-pass filter ratio\n\nIn two complement, the range is from -31 to 31."]
pub type HP_RATIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `hp_ratio` writer - Default high-pass filter ratio\n\nIn two complement, the range is from -31 to 31."]
pub type HP_RATIO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TVE_LOW_PASS_FILTER_CONTROL_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Default band-pass filter1 ratio\n\nIn two complement, the range is from -31 to 31."]
    #[inline(always)]
    pub fn bp1_ratio(&self) -> BP1_RATIO_R {
        BP1_RATIO_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Default band-pass filter0 ratio\n\nIn two complement, the range is from -31 to 31."]
    #[inline(always)]
    pub fn bp0_ratio(&self) -> BP0_RATIO_R {
        BP0_RATIO_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Default high-pass filter ratio\n\nIn two complement, the range is from -31 to 31."]
    #[inline(always)]
    pub fn hp_ratio(&self) -> HP_RATIO_R {
        HP_RATIO_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Default band-pass filter1 ratio\n\nIn two complement, the range is from -31 to 31."]
    #[inline(always)]
    #[must_use]
    pub fn bp1_ratio(&mut self) -> BP1_RATIO_W<0> {
        BP1_RATIO_W::new(self)
    }
    #[doc = "Bits 8:13 - Default band-pass filter0 ratio\n\nIn two complement, the range is from -31 to 31."]
    #[inline(always)]
    #[must_use]
    pub fn bp0_ratio(&mut self) -> BP0_RATIO_W<8> {
        BP0_RATIO_W::new(self)
    }
    #[doc = "Bits 16:21 - Default high-pass filter ratio\n\nIn two complement, the range is from -31 to 31."]
    #[inline(always)]
    #[must_use]
    pub fn hp_ratio(&mut self) -> HP_RATIO_W<16> {
        HP_RATIO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Encoder Low Pass Filter Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tve_low_pass_filter_control](index.html) module"]
pub struct TVE_LOW_PASS_FILTER_CONTROL_SPEC;
impl crate::RegisterSpec for TVE_LOW_PASS_FILTER_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tve_low_pass_filter_control::R](R) reader structure"]
impl crate::Readable for TVE_LOW_PASS_FILTER_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tve_low_pass_filter_control::W](W) writer structure"]
impl crate::Writable for TVE_LOW_PASS_FILTER_CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_low_pass_filter_control to value 0"]
impl crate::Resettable for TVE_LOW_PASS_FILTER_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
