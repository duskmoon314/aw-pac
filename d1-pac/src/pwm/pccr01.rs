#[doc = "Register `pccr01` reader"]
pub struct R(crate::R<PCCR01_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCCR01_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCCR01_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCCR01_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pccr01` writer"]
pub struct W(crate::W<PCCR01_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCCR01_SPEC>;
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
impl From<crate::W<PCCR01_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCCR01_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pwm01_clk_div_m` reader - PWM01 Clock Divide M"]
pub type PWM01_CLK_DIV_M_R = crate::FieldReader<u8, PWM01_CLK_DIV_M_A>;
#[doc = "PWM01 Clock Divide M\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWM01_CLK_DIV_M_A {
    #[doc = "0: /1"]
    M1 = 0,
    #[doc = "1: /2"]
    M2 = 1,
    #[doc = "2: /4"]
    M4 = 2,
    #[doc = "3: /8"]
    M8 = 3,
    #[doc = "4: /16"]
    M16 = 4,
    #[doc = "5: /32"]
    M32 = 5,
    #[doc = "6: /64"]
    M64 = 6,
    #[doc = "7: /128"]
    M128 = 7,
    #[doc = "8: /256"]
    M256 = 8,
}
impl From<PWM01_CLK_DIV_M_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM01_CLK_DIV_M_A) -> Self {
        variant as _
    }
}
impl PWM01_CLK_DIV_M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PWM01_CLK_DIV_M_A> {
        match self.bits {
            0 => Some(PWM01_CLK_DIV_M_A::M1),
            1 => Some(PWM01_CLK_DIV_M_A::M2),
            2 => Some(PWM01_CLK_DIV_M_A::M4),
            3 => Some(PWM01_CLK_DIV_M_A::M8),
            4 => Some(PWM01_CLK_DIV_M_A::M16),
            5 => Some(PWM01_CLK_DIV_M_A::M32),
            6 => Some(PWM01_CLK_DIV_M_A::M64),
            7 => Some(PWM01_CLK_DIV_M_A::M128),
            8 => Some(PWM01_CLK_DIV_M_A::M256),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `M1`"]
    #[inline(always)]
    pub fn is_m1(&self) -> bool {
        *self == PWM01_CLK_DIV_M_A::M1
    }
    #[doc = "Checks if the value of the field is `M2`"]
    #[inline(always)]
    pub fn is_m2(&self) -> bool {
        *self == PWM01_CLK_DIV_M_A::M2
    }
    #[doc = "Checks if the value of the field is `M4`"]
    #[inline(always)]
    pub fn is_m4(&self) -> bool {
        *self == PWM01_CLK_DIV_M_A::M4
    }
    #[doc = "Checks if the value of the field is `M8`"]
    #[inline(always)]
    pub fn is_m8(&self) -> bool {
        *self == PWM01_CLK_DIV_M_A::M8
    }
    #[doc = "Checks if the value of the field is `M16`"]
    #[inline(always)]
    pub fn is_m16(&self) -> bool {
        *self == PWM01_CLK_DIV_M_A::M16
    }
    #[doc = "Checks if the value of the field is `M32`"]
    #[inline(always)]
    pub fn is_m32(&self) -> bool {
        *self == PWM01_CLK_DIV_M_A::M32
    }
    #[doc = "Checks if the value of the field is `M64`"]
    #[inline(always)]
    pub fn is_m64(&self) -> bool {
        *self == PWM01_CLK_DIV_M_A::M64
    }
    #[doc = "Checks if the value of the field is `M128`"]
    #[inline(always)]
    pub fn is_m128(&self) -> bool {
        *self == PWM01_CLK_DIV_M_A::M128
    }
    #[doc = "Checks if the value of the field is `M256`"]
    #[inline(always)]
    pub fn is_m256(&self) -> bool {
        *self == PWM01_CLK_DIV_M_A::M256
    }
}
#[doc = "Field `pwm01_clk_div_m` writer - PWM01 Clock Divide M"]
pub type PWM01_CLK_DIV_M_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PCCR01_SPEC, u8, PWM01_CLK_DIV_M_A, 4, O>;
impl<'a, const O: u8> PWM01_CLK_DIV_M_W<'a, O> {
    #[doc = "/1"]
    #[inline(always)]
    pub fn m1(self) -> &'a mut W {
        self.variant(PWM01_CLK_DIV_M_A::M1)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn m2(self) -> &'a mut W {
        self.variant(PWM01_CLK_DIV_M_A::M2)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn m4(self) -> &'a mut W {
        self.variant(PWM01_CLK_DIV_M_A::M4)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn m8(self) -> &'a mut W {
        self.variant(PWM01_CLK_DIV_M_A::M8)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn m16(self) -> &'a mut W {
        self.variant(PWM01_CLK_DIV_M_A::M16)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn m32(self) -> &'a mut W {
        self.variant(PWM01_CLK_DIV_M_A::M32)
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn m64(self) -> &'a mut W {
        self.variant(PWM01_CLK_DIV_M_A::M64)
    }
    #[doc = "/128"]
    #[inline(always)]
    pub fn m128(self) -> &'a mut W {
        self.variant(PWM01_CLK_DIV_M_A::M128)
    }
    #[doc = "/256"]
    #[inline(always)]
    pub fn m256(self) -> &'a mut W {
        self.variant(PWM01_CLK_DIV_M_A::M256)
    }
}
#[doc = "Field `pwm01_clk_src` reader - Select PWM01 Clock Source"]
pub type PWM01_CLK_SRC_R = crate::FieldReader<u8, PWM01_CLK_SRC_A>;
#[doc = "Select PWM01 Clock Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWM01_CLK_SRC_A {
    #[doc = "0: HOSC"]
    HOSC = 0,
    #[doc = "1: APB0"]
    APB0 = 1,
}
impl From<PWM01_CLK_SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM01_CLK_SRC_A) -> Self {
        variant as _
    }
}
impl PWM01_CLK_SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PWM01_CLK_SRC_A> {
        match self.bits {
            0 => Some(PWM01_CLK_SRC_A::HOSC),
            1 => Some(PWM01_CLK_SRC_A::APB0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HOSC`"]
    #[inline(always)]
    pub fn is_hosc(&self) -> bool {
        *self == PWM01_CLK_SRC_A::HOSC
    }
    #[doc = "Checks if the value of the field is `APB0`"]
    #[inline(always)]
    pub fn is_apb0(&self) -> bool {
        *self == PWM01_CLK_SRC_A::APB0
    }
}
#[doc = "Field `pwm01_clk_src` writer - Select PWM01 Clock Source"]
pub type PWM01_CLK_SRC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PCCR01_SPEC, u8, PWM01_CLK_SRC_A, 2, O>;
impl<'a, const O: u8> PWM01_CLK_SRC_W<'a, O> {
    #[doc = "HOSC"]
    #[inline(always)]
    pub fn hosc(self) -> &'a mut W {
        self.variant(PWM01_CLK_SRC_A::HOSC)
    }
    #[doc = "APB0"]
    #[inline(always)]
    pub fn apb0(self) -> &'a mut W {
        self.variant(PWM01_CLK_SRC_A::APB0)
    }
}
impl R {
    #[doc = "Bits 0:3 - PWM01 Clock Divide M"]
    #[inline(always)]
    pub fn pwm01_clk_div_m(&self) -> PWM01_CLK_DIV_M_R {
        PWM01_CLK_DIV_M_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 7:8 - Select PWM01 Clock Source"]
    #[inline(always)]
    pub fn pwm01_clk_src(&self) -> PWM01_CLK_SRC_R {
        PWM01_CLK_SRC_R::new(((self.bits >> 7) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PWM01 Clock Divide M"]
    #[inline(always)]
    #[must_use]
    pub fn pwm01_clk_div_m(&mut self) -> PWM01_CLK_DIV_M_W<0> {
        PWM01_CLK_DIV_M_W::new(self)
    }
    #[doc = "Bits 7:8 - Select PWM01 Clock Source"]
    #[inline(always)]
    #[must_use]
    pub fn pwm01_clk_src(&mut self) -> PWM01_CLK_SRC_W<7> {
        PWM01_CLK_SRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM01 Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pccr01](index.html) module"]
pub struct PCCR01_SPEC;
impl crate::RegisterSpec for PCCR01_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pccr01::R](R) reader structure"]
impl crate::Readable for PCCR01_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pccr01::W](W) writer structure"]
impl crate::Writable for PCCR01_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pccr01 to value 0"]
impl crate::Resettable for PCCR01_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
