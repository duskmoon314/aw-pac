#[doc = "Register `pdzcr23` reader"]
pub type R = crate::R<PDZCR23_SPEC>;
#[doc = "Register `pdzcr23` writer"]
pub type W = crate::W<PDZCR23_SPEC>;
#[doc = "Field `pwm23_dz_en` reader - PWM23 Dead Zone Enable"]
pub type PWM23_DZ_EN_R = crate::BitReader<PWM23_DZ_EN_A>;
#[doc = "PWM23 Dead Zone Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWM23_DZ_EN_A {
    #[doc = "0: Dead Zone disable"]
    DISABLE = 0,
    #[doc = "1: Dead Zone enable"]
    ENABLE = 1,
}
impl From<PWM23_DZ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM23_DZ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl PWM23_DZ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWM23_DZ_EN_A {
        match self.bits {
            false => PWM23_DZ_EN_A::DISABLE,
            true => PWM23_DZ_EN_A::ENABLE,
        }
    }
    #[doc = "Dead Zone disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM23_DZ_EN_A::DISABLE
    }
    #[doc = "Dead Zone enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM23_DZ_EN_A::ENABLE
    }
}
#[doc = "Field `pwm23_dz_en` writer - PWM23 Dead Zone Enable"]
pub type PWM23_DZ_EN_W<'a, REG> = crate::BitWriter<'a, REG, PWM23_DZ_EN_A>;
impl<'a, REG> PWM23_DZ_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Dead Zone disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PWM23_DZ_EN_A::DISABLE)
    }
    #[doc = "Dead Zone enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PWM23_DZ_EN_A::ENABLE)
    }
}
#[doc = "Field `pwm23_dz_intv` reader - PWM23 Dead Zone Interval Value"]
pub type PWM23_DZ_INTV_R = crate::FieldReader;
#[doc = "Field `pwm23_dz_intv` writer - PWM23 Dead Zone Interval Value"]
pub type PWM23_DZ_INTV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - PWM23 Dead Zone Enable"]
    #[inline(always)]
    pub fn pwm23_dz_en(&self) -> PWM23_DZ_EN_R {
        PWM23_DZ_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - PWM23 Dead Zone Interval Value"]
    #[inline(always)]
    pub fn pwm23_dz_intv(&self) -> PWM23_DZ_INTV_R {
        PWM23_DZ_INTV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PWM23 Dead Zone Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwm23_dz_en(&mut self) -> PWM23_DZ_EN_W<PDZCR23_SPEC> {
        PWM23_DZ_EN_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - PWM23 Dead Zone Interval Value"]
    #[inline(always)]
    #[must_use]
    pub fn pwm23_dz_intv(&mut self) -> PWM23_DZ_INTV_W<PDZCR23_SPEC> {
        PWM23_DZ_INTV_W::new(self, 8)
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
#[doc = "PWM23 Dead Zone Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdzcr23::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdzcr23::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDZCR23_SPEC;
impl crate::RegisterSpec for PDZCR23_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdzcr23::R`](R) reader structure"]
impl crate::Readable for PDZCR23_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pdzcr23::W`](W) writer structure"]
impl crate::Writable for PDZCR23_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pdzcr23 to value 0"]
impl crate::Resettable for PDZCR23_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
