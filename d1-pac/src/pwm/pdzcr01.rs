#[doc = "Register `pdzcr01` reader"]
pub type R = crate::R<PDZCR01_SPEC>;
#[doc = "Register `pdzcr01` writer"]
pub type W = crate::W<PDZCR01_SPEC>;
#[doc = "Field `pwm01_dz_en` reader - PWM01 Dead Zone Enable"]
pub type PWM01_DZ_EN_R = crate::BitReader<PWM01_DZ_EN_A>;
#[doc = "PWM01 Dead Zone Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWM01_DZ_EN_A {
    #[doc = "0: Dead Zone disable"]
    DISABLE = 0,
    #[doc = "1: Dead Zone enable"]
    ENABLE = 1,
}
impl From<PWM01_DZ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM01_DZ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl PWM01_DZ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWM01_DZ_EN_A {
        match self.bits {
            false => PWM01_DZ_EN_A::DISABLE,
            true => PWM01_DZ_EN_A::ENABLE,
        }
    }
    #[doc = "Dead Zone disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM01_DZ_EN_A::DISABLE
    }
    #[doc = "Dead Zone enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM01_DZ_EN_A::ENABLE
    }
}
#[doc = "Field `pwm01_dz_en` writer - PWM01 Dead Zone Enable"]
pub type PWM01_DZ_EN_W<'a, REG> = crate::BitWriter<'a, REG, PWM01_DZ_EN_A>;
impl<'a, REG> PWM01_DZ_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Dead Zone disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PWM01_DZ_EN_A::DISABLE)
    }
    #[doc = "Dead Zone enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PWM01_DZ_EN_A::ENABLE)
    }
}
#[doc = "Field `pwm01_dz_intv` reader - PWM01 Dead Zone Interval Value"]
pub type PWM01_DZ_INTV_R = crate::FieldReader;
#[doc = "Field `pwm01_dz_intv` writer - PWM01 Dead Zone Interval Value"]
pub type PWM01_DZ_INTV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - PWM01 Dead Zone Enable"]
    #[inline(always)]
    pub fn pwm01_dz_en(&self) -> PWM01_DZ_EN_R {
        PWM01_DZ_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - PWM01 Dead Zone Interval Value"]
    #[inline(always)]
    pub fn pwm01_dz_intv(&self) -> PWM01_DZ_INTV_R {
        PWM01_DZ_INTV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PWM01 Dead Zone Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwm01_dz_en(&mut self) -> PWM01_DZ_EN_W<PDZCR01_SPEC> {
        PWM01_DZ_EN_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - PWM01 Dead Zone Interval Value"]
    #[inline(always)]
    #[must_use]
    pub fn pwm01_dz_intv(&mut self) -> PWM01_DZ_INTV_W<PDZCR01_SPEC> {
        PWM01_DZ_INTV_W::new(self, 8)
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
#[doc = "PWM01 Dead Zone Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdzcr01::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdzcr01::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDZCR01_SPEC;
impl crate::RegisterSpec for PDZCR01_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdzcr01::R`](R) reader structure"]
impl crate::Readable for PDZCR01_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pdzcr01::W`](W) writer structure"]
impl crate::Writable for PDZCR01_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pdzcr01 to value 0"]
impl crate::Resettable for PDZCR01_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
