#[doc = "Register `twi_ccr` reader"]
pub struct R(crate::R<TWI_CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWI_CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWI_CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWI_CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `twi_ccr` writer"]
pub struct W(crate::W<TWI_CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWI_CCR_SPEC>;
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
impl From<crate::W<TWI_CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWI_CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `clk_n` reader - "]
pub type CLK_N_R = crate::FieldReader<u8, u8>;
#[doc = "Field `clk_n` writer - "]
pub type CLK_N_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TWI_CCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `clk_m` reader - "]
pub type CLK_M_R = crate::FieldReader<u8, u8>;
#[doc = "Field `clk_m` writer - "]
pub type CLK_M_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TWI_CCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `clk_duty` reader - Setting duty cycle of clock as master"]
pub type CLK_DUTY_R = crate::BitReader<CLK_DUTY_A>;
#[doc = "Setting duty cycle of clock as master\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLK_DUTY_A {
    #[doc = "0: 50%"]
    P50 = 0,
    #[doc = "1: 40%"]
    P40 = 1,
}
impl From<CLK_DUTY_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_DUTY_A) -> Self {
        variant as u8 != 0
    }
}
impl CLK_DUTY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_DUTY_A {
        match self.bits {
            false => CLK_DUTY_A::P50,
            true => CLK_DUTY_A::P40,
        }
    }
    #[doc = "Checks if the value of the field is `P50`"]
    #[inline(always)]
    pub fn is_p50(&self) -> bool {
        *self == CLK_DUTY_A::P50
    }
    #[doc = "Checks if the value of the field is `P40`"]
    #[inline(always)]
    pub fn is_p40(&self) -> bool {
        *self == CLK_DUTY_A::P40
    }
}
#[doc = "Field `clk_duty` writer - Setting duty cycle of clock as master"]
pub type CLK_DUTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, TWI_CCR_SPEC, CLK_DUTY_A, O>;
impl<'a, const O: u8> CLK_DUTY_W<'a, O> {
    #[doc = "50%"]
    #[inline(always)]
    pub fn p50(self) -> &'a mut W {
        self.variant(CLK_DUTY_A::P50)
    }
    #[doc = "40%"]
    #[inline(always)]
    pub fn p40(self) -> &'a mut W {
        self.variant(CLK_DUTY_A::P40)
    }
}
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn clk_n(&self) -> CLK_N_R {
        CLK_N_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:6"]
    #[inline(always)]
    pub fn clk_m(&self) -> CLK_M_R {
        CLK_M_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Setting duty cycle of clock as master"]
    #[inline(always)]
    pub fn clk_duty(&self) -> CLK_DUTY_R {
        CLK_DUTY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn clk_n(&mut self) -> CLK_N_W<0> {
        CLK_N_W::new(self)
    }
    #[doc = "Bits 3:6"]
    #[inline(always)]
    #[must_use]
    pub fn clk_m(&mut self) -> CLK_M_W<3> {
        CLK_M_W::new(self)
    }
    #[doc = "Bit 7 - Setting duty cycle of clock as master"]
    #[inline(always)]
    #[must_use]
    pub fn clk_duty(&mut self) -> CLK_DUTY_W<7> {
        CLK_DUTY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TWI Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twi_ccr](index.html) module"]
pub struct TWI_CCR_SPEC;
impl crate::RegisterSpec for TWI_CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [twi_ccr::R](R) reader structure"]
impl crate::Readable for TWI_CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [twi_ccr::W](W) writer structure"]
impl crate::Writable for TWI_CCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets twi_ccr to value 0"]
impl crate::Resettable for TWI_CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
