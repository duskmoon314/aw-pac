#[doc = "Register `pdzcr45` reader"]
pub struct R(crate::R<PDZCR45_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDZCR45_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDZCR45_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDZCR45_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pdzcr45` writer"]
pub struct W(crate::W<PDZCR45_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDZCR45_SPEC>;
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
impl From<crate::W<PDZCR45_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDZCR45_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pwm45_dz_en` reader - PWM45 Dead Zone Enable"]
pub type PWM45_DZ_EN_R = crate::BitReader<PWM45_DZ_EN_A>;
#[doc = "PWM45 Dead Zone Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWM45_DZ_EN_A {
    #[doc = "0: Dead Zone disable"]
    DISABLE = 0,
    #[doc = "1: Dead Zone enable"]
    ENABLE = 1,
}
impl From<PWM45_DZ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM45_DZ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl PWM45_DZ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM45_DZ_EN_A {
        match self.bits {
            false => PWM45_DZ_EN_A::DISABLE,
            true => PWM45_DZ_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM45_DZ_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM45_DZ_EN_A::ENABLE
    }
}
#[doc = "Field `pwm45_dz_en` writer - PWM45 Dead Zone Enable"]
pub type PWM45_DZ_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDZCR45_SPEC, PWM45_DZ_EN_A, O>;
impl<'a, const O: u8> PWM45_DZ_EN_W<'a, O> {
    #[doc = "Dead Zone disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM45_DZ_EN_A::DISABLE)
    }
    #[doc = "Dead Zone enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM45_DZ_EN_A::ENABLE)
    }
}
#[doc = "Field `pwm45_dz_intv` reader - PWM45 Dead Zone Interval Value"]
pub type PWM45_DZ_INTV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pwm45_dz_intv` writer - PWM45 Dead Zone Interval Value"]
pub type PWM45_DZ_INTV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PDZCR45_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - PWM45 Dead Zone Enable"]
    #[inline(always)]
    pub fn pwm45_dz_en(&self) -> PWM45_DZ_EN_R {
        PWM45_DZ_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - PWM45 Dead Zone Interval Value"]
    #[inline(always)]
    pub fn pwm45_dz_intv(&self) -> PWM45_DZ_INTV_R {
        PWM45_DZ_INTV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PWM45 Dead Zone Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwm45_dz_en(&mut self) -> PWM45_DZ_EN_W<0> {
        PWM45_DZ_EN_W::new(self)
    }
    #[doc = "Bits 8:15 - PWM45 Dead Zone Interval Value"]
    #[inline(always)]
    #[must_use]
    pub fn pwm45_dz_intv(&mut self) -> PWM45_DZ_INTV_W<8> {
        PWM45_DZ_INTV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM45 Dead Zone Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdzcr45](index.html) module"]
pub struct PDZCR45_SPEC;
impl crate::RegisterSpec for PDZCR45_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdzcr45::R](R) reader structure"]
impl crate::Readable for PDZCR45_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdzcr45::W](W) writer structure"]
impl crate::Writable for PDZCR45_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pdzcr45 to value 0"]
impl crate::Resettable for PDZCR45_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
