#[doc = "Register `per` reader"]
pub type R = crate::R<PER_SPEC>;
#[doc = "Register `per` writer"]
pub type W = crate::W<PER_SPEC>;
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
    pub const fn variant(&self) -> PWM_EN_A {
        match self.bits {
            false => PWM_EN_A::DISABLE,
            true => PWM_EN_A::ENABLE,
        }
    }
    #[doc = "PWM disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM_EN_A::DISABLE
    }
    #[doc = "PWM enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM_EN_A::ENABLE
    }
}
#[doc = "Field `pwm_en[0-7]` writer - When PWM is enabled, the 16-bit up-counter starts working and PWM channel is permitted to output PWM waveform."]
pub type PWM_EN_W<'a, REG> = crate::BitWriter<'a, REG, PWM_EN_A>;
impl<'a, REG> PWM_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PWM disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_EN_A::DISABLE)
    }
    #[doc = "PWM enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PWM_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "When PWM is enabled, the 16-bit up-counter starts working and PWM channel is permitted to output PWM waveform.\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `pwm0_en` field"]
    #[inline(always)]
    pub fn pwm_en(&self, n: u8) -> PWM_EN_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
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
    #[doc = "When PWM is enabled, the 16-bit up-counter starts working and PWM channel is permitted to output PWM waveform.\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `pwm0_en` field"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_en(&mut self, n: u8) -> PWM_EN_W<PER_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        PWM_EN_W::new(self, n)
    }
    #[doc = "Bit 0 - When PWM is enabled, the 16-bit up-counter starts working and PWM channel is permitted to output PWM waveform."]
    #[inline(always)]
    #[must_use]
    pub fn pwm0_en(&mut self) -> PWM_EN_W<PER_SPEC> {
        PWM_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - When PWM is enabled, the 16-bit up-counter starts working and PWM channel is permitted to output PWM waveform."]
    #[inline(always)]
    #[must_use]
    pub fn pwm1_en(&mut self) -> PWM_EN_W<PER_SPEC> {
        PWM_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - When PWM is enabled, the 16-bit up-counter starts working and PWM channel is permitted to output PWM waveform."]
    #[inline(always)]
    #[must_use]
    pub fn pwm2_en(&mut self) -> PWM_EN_W<PER_SPEC> {
        PWM_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - When PWM is enabled, the 16-bit up-counter starts working and PWM channel is permitted to output PWM waveform."]
    #[inline(always)]
    #[must_use]
    pub fn pwm3_en(&mut self) -> PWM_EN_W<PER_SPEC> {
        PWM_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - When PWM is enabled, the 16-bit up-counter starts working and PWM channel is permitted to output PWM waveform."]
    #[inline(always)]
    #[must_use]
    pub fn pwm4_en(&mut self) -> PWM_EN_W<PER_SPEC> {
        PWM_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - When PWM is enabled, the 16-bit up-counter starts working and PWM channel is permitted to output PWM waveform."]
    #[inline(always)]
    #[must_use]
    pub fn pwm5_en(&mut self) -> PWM_EN_W<PER_SPEC> {
        PWM_EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - When PWM is enabled, the 16-bit up-counter starts working and PWM channel is permitted to output PWM waveform."]
    #[inline(always)]
    #[must_use]
    pub fn pwm6_en(&mut self) -> PWM_EN_W<PER_SPEC> {
        PWM_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - When PWM is enabled, the 16-bit up-counter starts working and PWM channel is permitted to output PWM waveform."]
    #[inline(always)]
    #[must_use]
    pub fn pwm7_en(&mut self) -> PWM_EN_W<PER_SPEC> {
        PWM_EN_W::new(self, 7)
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
#[doc = "PWM Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`per::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`per::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PER_SPEC;
impl crate::RegisterSpec for PER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`per::R`](R) reader structure"]
impl crate::Readable for PER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`per::W`](W) writer structure"]
impl crate::Writable for PER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets per to value 0"]
impl crate::Resettable for PER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
