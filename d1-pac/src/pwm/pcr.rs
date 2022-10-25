#[doc = "Register `pcr%s` reader"]
pub struct R(crate::R<PCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pcr%s` writer"]
pub struct W(crate::W<PCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCR_SPEC>;
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
impl From<crate::W<PCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pwm_prescal_k` reader - PWM pre-scale K, actual pre-scale is (K + 1)"]
pub type PWM_PRESCAL_K_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pwm_prescal_k` writer - PWM pre-scale K, actual pre-scale is (K + 1)"]
pub type PWM_PRESCAL_K_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `pwm_act_sta` reader - PWM Active State"]
pub type PWM_ACT_STA_R = crate::BitReader<PWM_ACT_STA_A>;
#[doc = "PWM Active State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWM_ACT_STA_A {
    #[doc = "0: low level"]
    LOW = 0,
    #[doc = "1: high level"]
    HIGH = 1,
}
impl From<PWM_ACT_STA_A> for bool {
    #[inline(always)]
    fn from(variant: PWM_ACT_STA_A) -> Self {
        variant as u8 != 0
    }
}
impl PWM_ACT_STA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM_ACT_STA_A {
        match self.bits {
            false => PWM_ACT_STA_A::LOW,
            true => PWM_ACT_STA_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PWM_ACT_STA_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PWM_ACT_STA_A::HIGH
    }
}
#[doc = "Field `pwm_act_sta` writer - PWM Active State"]
pub type PWM_ACT_STA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, PWM_ACT_STA_A, O>;
impl<'a, const O: u8> PWM_ACT_STA_W<'a, O> {
    #[doc = "low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PWM_ACT_STA_A::LOW)
    }
    #[doc = "high level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PWM_ACT_STA_A::HIGH)
    }
}
#[doc = "Field `pwm_mode` reader - PWM Output Mode Select"]
pub type PWM_MODE_R = crate::BitReader<PWM_MODE_A>;
#[doc = "PWM Output Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWM_MODE_A {
    #[doc = "0: Cycle mode"]
    CYCLE = 0,
    #[doc = "1: Pulse mode"]
    PULSE = 1,
}
impl From<PWM_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: PWM_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl PWM_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM_MODE_A {
        match self.bits {
            false => PWM_MODE_A::CYCLE,
            true => PWM_MODE_A::PULSE,
        }
    }
    #[doc = "Checks if the value of the field is `CYCLE`"]
    #[inline(always)]
    pub fn is_cycle(&self) -> bool {
        *self == PWM_MODE_A::CYCLE
    }
    #[doc = "Checks if the value of the field is `PULSE`"]
    #[inline(always)]
    pub fn is_pulse(&self) -> bool {
        *self == PWM_MODE_A::PULSE
    }
}
#[doc = "Field `pwm_mode` writer - PWM Output Mode Select"]
pub type PWM_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, PWM_MODE_A, O>;
impl<'a, const O: u8> PWM_MODE_W<'a, O> {
    #[doc = "Cycle mode"]
    #[inline(always)]
    pub fn cycle(self) -> &'a mut W {
        self.variant(PWM_MODE_A::CYCLE)
    }
    #[doc = "Pulse mode"]
    #[inline(always)]
    pub fn pulse(self) -> &'a mut W {
        self.variant(PWM_MODE_A::PULSE)
    }
}
#[doc = "Field `pwm_pul_start` reader - PWM Pulse Output Start"]
pub type PWM_PUL_START_R = crate::BitReader<PWM_PUL_START_A>;
#[doc = "PWM Pulse Output Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWM_PUL_START_A {
    #[doc = "0: `0`"]
    NO_EFFECT = 0,
    #[doc = "1: Output pulse for PWM_CYCLE_NM + 1"]
    START = 1,
}
impl From<PWM_PUL_START_A> for bool {
    #[inline(always)]
    fn from(variant: PWM_PUL_START_A) -> Self {
        variant as u8 != 0
    }
}
impl PWM_PUL_START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM_PUL_START_A {
        match self.bits {
            false => PWM_PUL_START_A::NO_EFFECT,
            true => PWM_PUL_START_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == PWM_PUL_START_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == PWM_PUL_START_A::START
    }
}
#[doc = "Field `pwm_pul_start` writer - PWM Pulse Output Start"]
pub type PWM_PUL_START_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, PWM_PUL_START_A, O>;
impl<'a, const O: u8> PWM_PUL_START_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(PWM_PUL_START_A::NO_EFFECT)
    }
    #[doc = "Output pulse for PWM_CYCLE_NM + 1"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(PWM_PUL_START_A::START)
    }
}
#[doc = "Field `pwm_period_rdy` reader - PWM Period Register Ready"]
pub type PWM_PERIOD_RDY_R = crate::BitReader<PWM_PERIOD_RDY_A>;
#[doc = "PWM Period Register Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWM_PERIOD_RDY_A {
    #[doc = "0: PWM period register is ready to write"]
    READY = 0,
    #[doc = "1: PWM period register is busy"]
    BUSY = 1,
}
impl From<PWM_PERIOD_RDY_A> for bool {
    #[inline(always)]
    fn from(variant: PWM_PERIOD_RDY_A) -> Self {
        variant as u8 != 0
    }
}
impl PWM_PERIOD_RDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM_PERIOD_RDY_A {
        match self.bits {
            false => PWM_PERIOD_RDY_A::READY,
            true => PWM_PERIOD_RDY_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == PWM_PERIOD_RDY_A::READY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == PWM_PERIOD_RDY_A::BUSY
    }
}
#[doc = "Field `pwm_pul_num` reader - In pulse mode, the PWM outputs pulse for PWM_CYCLE_NUM + 1 times and then stops"]
pub type PWM_PUL_NUM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `pwm_pul_num` writer - In pulse mode, the PWM outputs pulse for PWM_CYCLE_NUM + 1 times and then stops"]
pub type PWM_PUL_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:7 - PWM pre-scale K, actual pre-scale is (K + 1)"]
    #[inline(always)]
    pub fn pwm_prescal_k(&self) -> PWM_PRESCAL_K_R {
        PWM_PRESCAL_K_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PWM Active State"]
    #[inline(always)]
    pub fn pwm_act_sta(&self) -> PWM_ACT_STA_R {
        PWM_ACT_STA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PWM Output Mode Select"]
    #[inline(always)]
    pub fn pwm_mode(&self) -> PWM_MODE_R {
        PWM_MODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PWM Pulse Output Start"]
    #[inline(always)]
    pub fn pwm_pul_start(&self) -> PWM_PUL_START_R {
        PWM_PUL_START_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PWM Period Register Ready"]
    #[inline(always)]
    pub fn pwm_period_rdy(&self) -> PWM_PERIOD_RDY_R {
        PWM_PERIOD_RDY_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:31 - In pulse mode, the PWM outputs pulse for PWM_CYCLE_NUM + 1 times and then stops"]
    #[inline(always)]
    pub fn pwm_pul_num(&self) -> PWM_PUL_NUM_R {
        PWM_PUL_NUM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - PWM pre-scale K, actual pre-scale is (K + 1)"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_prescal_k(&mut self) -> PWM_PRESCAL_K_W<0> {
        PWM_PRESCAL_K_W::new(self)
    }
    #[doc = "Bit 8 - PWM Active State"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_act_sta(&mut self) -> PWM_ACT_STA_W<8> {
        PWM_ACT_STA_W::new(self)
    }
    #[doc = "Bit 9 - PWM Output Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_mode(&mut self) -> PWM_MODE_W<9> {
        PWM_MODE_W::new(self)
    }
    #[doc = "Bit 10 - PWM Pulse Output Start"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_pul_start(&mut self) -> PWM_PUL_START_W<10> {
        PWM_PUL_START_W::new(self)
    }
    #[doc = "Bits 16:31 - In pulse mode, the PWM outputs pulse for PWM_CYCLE_NUM + 1 times and then stops"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_pul_num(&mut self) -> PWM_PUL_NUM_W<16> {
        PWM_PUL_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcr](index.html) module"]
pub struct PCR_SPEC;
impl crate::RegisterSpec for PCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcr::R](R) reader structure"]
impl crate::Readable for PCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcr::W](W) writer structure"]
impl crate::Writable for PCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pcr%s to value 0"]
impl crate::Resettable for PCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
