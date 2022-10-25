#[doc = "Register `pcntr%s` reader"]
pub struct R(crate::R<PCNTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCNTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCNTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCNTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pcntr%s` writer"]
pub struct W(crate::W<PCNTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCNTR_SPEC>;
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
impl From<crate::W<PCNTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCNTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pwm_counter_status` reader - On PWM output or capture input, reading this register could get the current value of the PWM 16-bit up-counter."]
pub type PWM_COUNTER_STATUS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `pwm_counter_start` reader - PWM counter value is set for phase control."]
pub type PWM_COUNTER_START_R = crate::FieldReader<u16, u16>;
#[doc = "Field `pwm_counter_start` writer - PWM counter value is set for phase control."]
pub type PWM_COUNTER_START_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PCNTR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - On PWM output or capture input, reading this register could get the current value of the PWM 16-bit up-counter."]
    #[inline(always)]
    pub fn pwm_counter_status(&self) -> PWM_COUNTER_STATUS_R {
        PWM_COUNTER_STATUS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - PWM counter value is set for phase control."]
    #[inline(always)]
    pub fn pwm_counter_start(&self) -> PWM_COUNTER_START_R {
        PWM_COUNTER_START_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - PWM counter value is set for phase control."]
    #[inline(always)]
    #[must_use]
    pub fn pwm_counter_start(&mut self) -> PWM_COUNTER_START_W<16> {
        PWM_COUNTER_START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcntr](index.html) module"]
pub struct PCNTR_SPEC;
impl crate::RegisterSpec for PCNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcntr::R](R) reader structure"]
impl crate::Readable for PCNTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcntr::W](W) writer structure"]
impl crate::Writable for PCNTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pcntr%s to value 0"]
impl crate::Resettable for PCNTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
