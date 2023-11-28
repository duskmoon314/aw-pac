#[doc = "Register `ppr%s` reader"]
pub type R = crate::R<PPR_SPEC>;
#[doc = "Register `ppr%s` writer"]
pub type W = crate::W<PPR_SPEC>;
#[doc = "Field `pwm_act_cycle` reader - Number of the active cycles in the PWM clock.\n\nN: N cycles"]
pub type PWM_ACT_CYCLE_R = crate::FieldReader<u16>;
#[doc = "Field `pwm_act_cycle` writer - Number of the active cycles in the PWM clock.\n\nN: N cycles"]
pub type PWM_ACT_CYCLE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `pwm_entire_cycle` reader - Number of the entire cycles in the PWM clock.\n\nN: N + 1 cycles"]
pub type PWM_ENTIRE_CYCLE_R = crate::FieldReader<u16>;
#[doc = "Field `pwm_entire_cycle` writer - Number of the entire cycles in the PWM clock.\n\nN: N + 1 cycles"]
pub type PWM_ENTIRE_CYCLE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of the active cycles in the PWM clock.\n\nN: N cycles"]
    #[inline(always)]
    pub fn pwm_act_cycle(&self) -> PWM_ACT_CYCLE_R {
        PWM_ACT_CYCLE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Number of the entire cycles in the PWM clock.\n\nN: N + 1 cycles"]
    #[inline(always)]
    pub fn pwm_entire_cycle(&self) -> PWM_ENTIRE_CYCLE_R {
        PWM_ENTIRE_CYCLE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of the active cycles in the PWM clock.\n\nN: N cycles"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_act_cycle(&mut self) -> PWM_ACT_CYCLE_W<PPR_SPEC> {
        PWM_ACT_CYCLE_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Number of the entire cycles in the PWM clock.\n\nN: N + 1 cycles"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_entire_cycle(&mut self) -> PWM_ENTIRE_CYCLE_W<PPR_SPEC> {
        PWM_ENTIRE_CYCLE_W::new(self, 16)
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
#[doc = "PWM Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PPR_SPEC;
impl crate::RegisterSpec for PPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppr::R`](R) reader structure"]
impl crate::Readable for PPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ppr::W`](W) writer structure"]
impl crate::Writable for PPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ppr%s to value 0"]
impl crate::Resettable for PPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
