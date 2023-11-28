#[doc = "Register `pcntr%s` reader"]
pub type R = crate::R<PCNTR_SPEC>;
#[doc = "Register `pcntr%s` writer"]
pub type W = crate::W<PCNTR_SPEC>;
#[doc = "Field `pwm_counter_status` reader - On PWM output or capture input, reading this register could get the current value of the PWM 16-bit up-counter."]
pub type PWM_COUNTER_STATUS_R = crate::FieldReader<u16>;
#[doc = "Field `pwm_counter_start` reader - PWM counter value is set for phase control."]
pub type PWM_COUNTER_START_R = crate::FieldReader<u16>;
#[doc = "Field `pwm_counter_start` writer - PWM counter value is set for phase control."]
pub type PWM_COUNTER_START_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
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
    pub fn pwm_counter_start(&mut self) -> PWM_COUNTER_START_W<PCNTR_SPEC> {
        PWM_COUNTER_START_W::new(self, 16)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PWM Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcntr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcntr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCNTR_SPEC;
impl crate::RegisterSpec for PCNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcntr::R`](R) reader structure"]
impl crate::Readable for PCNTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcntr::W`](W) writer structure"]
impl crate::Writable for PCNTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pcntr%s to value 0"]
impl crate::Resettable for PCNTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
