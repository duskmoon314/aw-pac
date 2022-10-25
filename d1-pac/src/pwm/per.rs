#[doc = "Register `per` reader"]
pub struct R(crate::R<PER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `per` writer"]
pub struct W(crate::W<PER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PER_SPEC>;
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
impl From<crate::W<PER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pwm_en[0-7]` reader - When PWM is enabled, the 16-bit up-counter starts working and PWM channel is permitted to output PWM waveform."]
pub type PWM_EN_R = crate::BitReader<PWM_EN_A>;
#[doc = "When PWM is enabled, the 16-bit up-counter starts working and PWM channel is permitted to output PWM waveform.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWM_EN_A {
    #[doc = "0: PWM disable"]
    DISABLE = 0,
    #[doc = "1: PWM enable"]
    ENABLE = 1,
}
impl From<PWM_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl PWM_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM_EN_A {
        match self.bits {
            false => PWM_EN_A::DISABLE,
            true => PWM_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM_EN_A::ENABLE
    }
}
#[doc = "Field `pwm_en[0-7]` writer - When PWM is enabled, the 16-bit up-counter starts working and PWM channel is permitted to output PWM waveform."]
pub type PWM_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_SPEC, PWM_EN_A, O>;
impl<'a, const O: u8> PWM_EN_W<'a, O> {
    #[doc = "PWM disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM_EN_A::DISABLE)
    }
    #[doc = "PWM enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "When PWM is enabled, the 16-bit up-counter starts working and PWM channel is permitted to output PWM waveform."]
    #[inline(always)]
    pub unsafe fn pwm_en(&self, n: u8) -> PWM_EN_R {
        PWM_EN_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - When PWM is enabled, the 16-bit up-counter starts working and PWM channel is permitted to output PWM waveform."]
    #[inline(always)]
    pub fn pwm0_en(&self) -> PWM_EN_R {
        PWM_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When PWM is enabled, the 16-bit up-counter starts working and PWM channel is permitted to output PWM waveform."]
    #[inline(always)]
    pub fn pwm1_en(&self) -> PWM_EN_R {
        PWM_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When PWM is enabled, the 16-bit up-counter starts working and PWM channel is permitted to output PWM waveform."]
    #[inline(always)]
    pub fn pwm2_en(&self) -> PWM_EN_R {
        PWM_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When PWM is enabled, the 16-bit up-counter starts working and PWM channel is permitted to output PWM waveform."]
    #[inline(always)]
    pub fn pwm3_en(&self) -> PWM_EN_R {
        PWM_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When PWM is enabled, the 16-bit up-counter starts working and PWM channel is permitted to output PWM waveform."]
    #[inline(always)]
    pub fn pwm4_en(&self) -> PWM_EN_R {
        PWM_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When PWM is enabled, the 16-bit up-counter starts working and PWM channel is permitted to output PWM waveform."]
    #[inline(always)]
    pub fn pwm5_en(&self) -> PWM_EN_R {
        PWM_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When PWM is enabled, the 16-bit up-counter starts working and PWM channel is permitted to output PWM waveform."]
    #[inline(always)]
    pub fn pwm6_en(&self) -> PWM_EN_R {
        PWM_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When PWM is enabled, the 16-bit up-counter starts working and PWM channel is permitted to output PWM waveform."]
    #[inline(always)]
    pub fn pwm7_en(&self) -> PWM_EN_R {
        PWM_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "When PWM is enabled, the 16-bit up-counter starts working and PWM channel is permitted to output PWM waveform."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn pwm_en<const O: u8>(&mut self) -> PWM_EN_W<O> {
        PWM_EN_W::new(self)
    }
    #[doc = "Bit 0 - When PWM is enabled, the 16-bit up-counter starts working and PWM channel is permitted to output PWM waveform."]
    #[inline(always)]
    #[must_use]
    pub fn pwm0_en(&mut self) -> PWM_EN_W<0> {
        PWM_EN_W::new(self)
    }
    #[doc = "Bit 1 - When PWM is enabled, the 16-bit up-counter starts working and PWM channel is permitted to output PWM waveform."]
    #[inline(always)]
    #[must_use]
    pub fn pwm1_en(&mut self) -> PWM_EN_W<1> {
        PWM_EN_W::new(self)
    }
    #[doc = "Bit 2 - When PWM is enabled, the 16-bit up-counter starts working and PWM channel is permitted to output PWM waveform."]
    #[inline(always)]
    #[must_use]
    pub fn pwm2_en(&mut self) -> PWM_EN_W<2> {
        PWM_EN_W::new(self)
    }
    #[doc = "Bit 3 - When PWM is enabled, the 16-bit up-counter starts working and PWM channel is permitted to output PWM waveform."]
    #[inline(always)]
    #[must_use]
    pub fn pwm3_en(&mut self) -> PWM_EN_W<3> {
        PWM_EN_W::new(self)
    }
    #[doc = "Bit 4 - When PWM is enabled, the 16-bit up-counter starts working and PWM channel is permitted to output PWM waveform."]
    #[inline(always)]
    #[must_use]
    pub fn pwm4_en(&mut self) -> PWM_EN_W<4> {
        PWM_EN_W::new(self)
    }
    #[doc = "Bit 5 - When PWM is enabled, the 16-bit up-counter starts working and PWM channel is permitted to output PWM waveform."]
    #[inline(always)]
    #[must_use]
    pub fn pwm5_en(&mut self) -> PWM_EN_W<5> {
        PWM_EN_W::new(self)
    }
    #[doc = "Bit 6 - When PWM is enabled, the 16-bit up-counter starts working and PWM channel is permitted to output PWM waveform."]
    #[inline(always)]
    #[must_use]
    pub fn pwm6_en(&mut self) -> PWM_EN_W<6> {
        PWM_EN_W::new(self)
    }
    #[doc = "Bit 7 - When PWM is enabled, the 16-bit up-counter starts working and PWM channel is permitted to output PWM waveform."]
    #[inline(always)]
    #[must_use]
    pub fn pwm7_en(&mut self) -> PWM_EN_W<7> {
        PWM_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [per](index.html) module"]
pub struct PER_SPEC;
impl crate::RegisterSpec for PER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [per::R](R) reader structure"]
impl crate::Readable for PER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [per::W](W) writer structure"]
impl crate::Writable for PER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets per to value 0"]
impl crate::Resettable for PER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
