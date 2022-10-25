#[doc = "Register `ppr%s` reader"]
pub struct R(crate::R<PPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ppr%s` writer"]
pub struct W(crate::W<PPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPR_SPEC>;
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
impl From<crate::W<PPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pwm_act_cycle` reader - Number of the active cycles in the PWM clock.\n\nN: N cycles"]
pub type PWM_ACT_CYCLE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `pwm_act_cycle` writer - Number of the active cycles in the PWM clock.\n\nN: N cycles"]
pub type PWM_ACT_CYCLE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PPR_SPEC, u16, u16, 16, O>;
#[doc = "Field `pwm_entire_cycle` reader - Number of the entire cycles in the PWM clock.\n\nN: N + 1 cycles"]
pub type PWM_ENTIRE_CYCLE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `pwm_entire_cycle` writer - Number of the entire cycles in the PWM clock.\n\nN: N + 1 cycles"]
pub type PWM_ENTIRE_CYCLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PPR_SPEC, u16, u16, 16, O>;
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
    pub fn pwm_act_cycle(&mut self) -> PWM_ACT_CYCLE_W<0> {
        PWM_ACT_CYCLE_W::new(self)
    }
    #[doc = "Bits 16:31 - Number of the entire cycles in the PWM clock.\n\nN: N + 1 cycles"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_entire_cycle(&mut self) -> PWM_ENTIRE_CYCLE_W<16> {
        PWM_ENTIRE_CYCLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Period Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppr](index.html) module"]
pub struct PPR_SPEC;
impl crate::RegisterSpec for PPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ppr::R](R) reader structure"]
impl crate::Readable for PPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ppr::W](W) writer structure"]
impl crate::Writable for PPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ppr%s to value 0"]
impl crate::Resettable for PPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
