#[doc = "Register `tve_low_pass_shoot_control` reader"]
pub struct R(crate::R<TVE_LOW_PASS_SHOOT_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVE_LOW_PASS_SHOOT_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVE_LOW_PASS_SHOOT_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVE_LOW_PASS_SHOOT_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tve_low_pass_shoot_control` writer"]
pub struct W(crate::W<TVE_LOW_PASS_SHOOT_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVE_LOW_PASS_SHOOT_CONTROL_SPEC>;
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
impl From<crate::W<TVE_LOW_PASS_SHOOT_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVE_LOW_PASS_SHOOT_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `neg_gain` reader - Undershoot gain control."]
pub type NEG_GAIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `neg_gain` writer - Undershoot gain control."]
pub type NEG_GAIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TVE_LOW_PASS_SHOOT_CONTROL_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Undershoot gain control."]
    #[inline(always)]
    pub fn neg_gain(&self) -> NEG_GAIN_R {
        NEG_GAIN_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Undershoot gain control."]
    #[inline(always)]
    #[must_use]
    pub fn neg_gain(&mut self) -> NEG_GAIN_W<0> {
        NEG_GAIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Encoder Low Pass Shoot Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tve_low_pass_shoot_control](index.html) module"]
pub struct TVE_LOW_PASS_SHOOT_CONTROL_SPEC;
impl crate::RegisterSpec for TVE_LOW_PASS_SHOOT_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tve_low_pass_shoot_control::R](R) reader structure"]
impl crate::Readable for TVE_LOW_PASS_SHOOT_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tve_low_pass_shoot_control::W](W) writer structure"]
impl crate::Writable for TVE_LOW_PASS_SHOOT_CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_low_pass_shoot_control to value 0"]
impl crate::Resettable for TVE_LOW_PASS_SHOOT_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
