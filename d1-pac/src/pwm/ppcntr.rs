#[doc = "Register `ppcntr%s` reader"]
pub type R = crate::R<PPCNTR_SPEC>;
#[doc = "Field `pwm_pul_counter_status` reader - On PWM output, reading this register could get the current value of the PWM pulse counter."]
pub type PWM_PUL_COUNTER_STATUS_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - On PWM output, reading this register could get the current value of the PWM pulse counter."]
    #[inline(always)]
    pub fn pwm_pul_counter_status(&self) -> PWM_PUL_COUNTER_STATUS_R {
        PWM_PUL_COUNTER_STATUS_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "PWM Pulse Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppcntr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PPCNTR_SPEC;
impl crate::RegisterSpec for PPCNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppcntr::R`](R) reader structure"]
impl crate::Readable for PPCNTR_SPEC {}
#[doc = "`reset()` method sets ppcntr%s to value 0"]
impl crate::Resettable for PPCNTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
