#[doc = "Register `pccr23` reader"]
pub type R = crate::R<PCCR23_SPEC>;
#[doc = "Register `pccr23` writer"]
pub type W = crate::W<PCCR23_SPEC>;
#[doc = "Field `pwm23_clk_div_m` reader - PWM23 Clock Divide M"]
pub type PWM23_CLK_DIV_M_R = crate::FieldReader<PWM23_CLK_DIV_M_A>;
#[doc = "PWM23 Clock Divide M\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWM23_CLK_DIV_M_A {
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
impl From<PWM23_CLK_DIV_M_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM23_CLK_DIV_M_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PWM23_CLK_DIV_M_A {
    type Ux = u8;
}
impl PWM23_CLK_DIV_M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PWM23_CLK_DIV_M_A> {
        match self.bits {
            0 => Some(PWM23_CLK_DIV_M_A::M1),
            1 => Some(PWM23_CLK_DIV_M_A::M2),
            2 => Some(PWM23_CLK_DIV_M_A::M4),
            3 => Some(PWM23_CLK_DIV_M_A::M8),
            4 => Some(PWM23_CLK_DIV_M_A::M16),
            5 => Some(PWM23_CLK_DIV_M_A::M32),
            6 => Some(PWM23_CLK_DIV_M_A::M64),
            7 => Some(PWM23_CLK_DIV_M_A::M128),
            8 => Some(PWM23_CLK_DIV_M_A::M256),
            _ => None,
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn is_m1(&self) -> bool {
        *self == PWM23_CLK_DIV_M_A::M1
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn is_m2(&self) -> bool {
        *self == PWM23_CLK_DIV_M_A::M2
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn is_m4(&self) -> bool {
        *self == PWM23_CLK_DIV_M_A::M4
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn is_m8(&self) -> bool {
        *self == PWM23_CLK_DIV_M_A::M8
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn is_m16(&self) -> bool {
        *self == PWM23_CLK_DIV_M_A::M16
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn is_m32(&self) -> bool {
        *self == PWM23_CLK_DIV_M_A::M32
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn is_m64(&self) -> bool {
        *self == PWM23_CLK_DIV_M_A::M64
    }
    #[doc = "/128"]
    #[inline(always)]
    pub fn is_m128(&self) -> bool {
        *self == PWM23_CLK_DIV_M_A::M128
    }
    #[doc = "/256"]
    #[inline(always)]
    pub fn is_m256(&self) -> bool {
        *self == PWM23_CLK_DIV_M_A::M256
    }
}
#[doc = "Field `pwm23_clk_div_m` writer - PWM23 Clock Divide M"]
pub type PWM23_CLK_DIV_M_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PWM23_CLK_DIV_M_A>;
impl<'a, REG> PWM23_CLK_DIV_M_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "/1"]
    #[inline(always)]
    pub fn m1(self) -> &'a mut crate::W<REG> {
        self.variant(PWM23_CLK_DIV_M_A::M1)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn m2(self) -> &'a mut crate::W<REG> {
        self.variant(PWM23_CLK_DIV_M_A::M2)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn m4(self) -> &'a mut crate::W<REG> {
        self.variant(PWM23_CLK_DIV_M_A::M4)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn m8(self) -> &'a mut crate::W<REG> {
        self.variant(PWM23_CLK_DIV_M_A::M8)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn m16(self) -> &'a mut crate::W<REG> {
        self.variant(PWM23_CLK_DIV_M_A::M16)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn m32(self) -> &'a mut crate::W<REG> {
        self.variant(PWM23_CLK_DIV_M_A::M32)
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn m64(self) -> &'a mut crate::W<REG> {
        self.variant(PWM23_CLK_DIV_M_A::M64)
    }
    #[doc = "/128"]
    #[inline(always)]
    pub fn m128(self) -> &'a mut crate::W<REG> {
        self.variant(PWM23_CLK_DIV_M_A::M128)
    }
    #[doc = "/256"]
    #[inline(always)]
    pub fn m256(self) -> &'a mut crate::W<REG> {
        self.variant(PWM23_CLK_DIV_M_A::M256)
    }
}
#[doc = "Field `pwm23_clk_src_sel` reader - Select PWM23 Clock Source"]
pub type PWM23_CLK_SRC_SEL_R = crate::FieldReader<PWM23_CLK_SRC_SEL_A>;
#[doc = "Select PWM23 Clock Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWM23_CLK_SRC_SEL_A {
    #[doc = "0: HOSC"]
    HOSC = 0,
    #[doc = "1: APB0"]
    APB0 = 1,
}
impl From<PWM23_CLK_SRC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM23_CLK_SRC_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PWM23_CLK_SRC_SEL_A {
    type Ux = u8;
}
impl PWM23_CLK_SRC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PWM23_CLK_SRC_SEL_A> {
        match self.bits {
            0 => Some(PWM23_CLK_SRC_SEL_A::HOSC),
            1 => Some(PWM23_CLK_SRC_SEL_A::APB0),
            _ => None,
        }
    }
    #[doc = "HOSC"]
    #[inline(always)]
    pub fn is_hosc(&self) -> bool {
        *self == PWM23_CLK_SRC_SEL_A::HOSC
    }
    #[doc = "APB0"]
    #[inline(always)]
    pub fn is_apb0(&self) -> bool {
        *self == PWM23_CLK_SRC_SEL_A::APB0
    }
}
#[doc = "Field `pwm23_clk_src_sel` writer - Select PWM23 Clock Source"]
pub type PWM23_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PWM23_CLK_SRC_SEL_A>;
impl<'a, REG> PWM23_CLK_SRC_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HOSC"]
    #[inline(always)]
    pub fn hosc(self) -> &'a mut crate::W<REG> {
        self.variant(PWM23_CLK_SRC_SEL_A::HOSC)
    }
    #[doc = "APB0"]
    #[inline(always)]
    pub fn apb0(self) -> &'a mut crate::W<REG> {
        self.variant(PWM23_CLK_SRC_SEL_A::APB0)
    }
}
impl R {
    #[doc = "Bits 0:3 - PWM23 Clock Divide M"]
    #[inline(always)]
    pub fn pwm23_clk_div_m(&self) -> PWM23_CLK_DIV_M_R {
        PWM23_CLK_DIV_M_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 7:8 - Select PWM23 Clock Source"]
    #[inline(always)]
    pub fn pwm23_clk_src_sel(&self) -> PWM23_CLK_SRC_SEL_R {
        PWM23_CLK_SRC_SEL_R::new(((self.bits >> 7) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PWM23 Clock Divide M"]
    #[inline(always)]
    #[must_use]
    pub fn pwm23_clk_div_m(&mut self) -> PWM23_CLK_DIV_M_W<PCCR23_SPEC> {
        PWM23_CLK_DIV_M_W::new(self, 0)
    }
    #[doc = "Bits 7:8 - Select PWM23 Clock Source"]
    #[inline(always)]
    #[must_use]
    pub fn pwm23_clk_src_sel(&mut self) -> PWM23_CLK_SRC_SEL_W<PCCR23_SPEC> {
        PWM23_CLK_SRC_SEL_W::new(self, 7)
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
#[doc = "PWM23 Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pccr23::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pccr23::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCCR23_SPEC;
impl crate::RegisterSpec for PCCR23_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pccr23::R`](R) reader structure"]
impl crate::Readable for PCCR23_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pccr23::W`](W) writer structure"]
impl crate::Writable for PCCR23_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pccr23 to value 0"]
impl crate::Resettable for PCCR23_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
