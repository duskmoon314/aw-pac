#[doc = "Register `ppcntr%s` reader"]
pub struct R(crate::R<PPCNTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPCNTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPCNTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPCNTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `pwm_pul_counter_status` reader - On PWM output, reading this register could get the current value of the PWM pulse counter."]
pub type PWM_PUL_COUNTER_STATUS_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - On PWM output, reading this register could get the current value of the PWM pulse counter."]
    #[inline(always)]
    pub fn pwm_pul_counter_status(&self) -> PWM_PUL_COUNTER_STATUS_R {
        PWM_PUL_COUNTER_STATUS_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "PWM Pulse Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppcntr](index.html) module"]
pub struct PPCNTR_SPEC;
impl crate::RegisterSpec for PPCNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ppcntr::R](R) reader structure"]
impl crate::Readable for PPCNTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ppcntr%s to value 0"]
impl crate::Resettable for PPCNTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
