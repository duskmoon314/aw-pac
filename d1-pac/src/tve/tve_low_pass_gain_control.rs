#[doc = "Register `tve_low_pass_gain_control` reader"]
pub struct R(crate::R<TVE_LOW_PASS_GAIN_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVE_LOW_PASS_GAIN_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVE_LOW_PASS_GAIN_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVE_LOW_PASS_GAIN_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tve_low_pass_gain_control` writer"]
pub struct W(crate::W<TVE_LOW_PASS_GAIN_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVE_LOW_PASS_GAIN_CONTROL_SPEC>;
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
impl From<crate::W<TVE_LOW_PASS_GAIN_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVE_LOW_PASS_GAIN_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `beta` reader - Gain control: large gain limitation."]
pub type BETA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `beta` writer - Gain control: large gain limitation."]
pub type BETA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TVE_LOW_PASS_GAIN_CONTROL_SPEC, u8, u8, 5, O>;
#[doc = "Field `dif_up` reader - Gain control: limitation threshold."]
pub type DIF_UP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dif_up` writer - Gain control: limitation threshold."]
pub type DIF_UP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TVE_LOW_PASS_GAIN_CONTROL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:4 - Gain control: large gain limitation."]
    #[inline(always)]
    pub fn beta(&self) -> BETA_R {
        BETA_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:23 - Gain control: limitation threshold."]
    #[inline(always)]
    pub fn dif_up(&self) -> DIF_UP_R {
        DIF_UP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Gain control: large gain limitation."]
    #[inline(always)]
    #[must_use]
    pub fn beta(&mut self) -> BETA_W<0> {
        BETA_W::new(self)
    }
    #[doc = "Bits 16:23 - Gain control: limitation threshold."]
    #[inline(always)]
    #[must_use]
    pub fn dif_up(&mut self) -> DIF_UP_W<16> {
        DIF_UP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Encoder Low Pass Gain Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tve_low_pass_gain_control](index.html) module"]
pub struct TVE_LOW_PASS_GAIN_CONTROL_SPEC;
impl crate::RegisterSpec for TVE_LOW_PASS_GAIN_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tve_low_pass_gain_control::R](R) reader structure"]
impl crate::Readable for TVE_LOW_PASS_GAIN_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tve_low_pass_gain_control::W](W) writer structure"]
impl crate::Writable for TVE_LOW_PASS_GAIN_CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_low_pass_gain_control to value 0"]
impl crate::Resettable for TVE_LOW_PASS_GAIN_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
