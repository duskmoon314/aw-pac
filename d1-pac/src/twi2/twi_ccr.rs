#[doc = "Register `TWI_CCR` reader"]
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
#[doc = "Register `TWI_CCR` writer"]
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
#[doc = "Setting duty cycle of clock as master\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `clk_duty` reader - Setting duty cycle of clock as master"]
pub struct CLK_DUTY_R(crate::FieldReader<bool, CLK_DUTY_A>);
impl CLK_DUTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_DUTY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == CLK_DUTY_A::P50
    }
    #[doc = "Checks if the value of the field is `P40`"]
    #[inline(always)]
    pub fn is_p40(&self) -> bool {
        **self == CLK_DUTY_A::P40
    }
}
impl core::ops::Deref for CLK_DUTY_R {
    type Target = crate::FieldReader<bool, CLK_DUTY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_duty` writer - Setting duty cycle of clock as master"]
pub struct CLK_DUTY_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_DUTY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_DUTY_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
#[doc = "Field `clk_m` reader - "]
pub struct CLK_M_R(crate::FieldReader<u8, u8>);
impl CLK_M_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLK_M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_M_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_m` writer - "]
pub struct CLK_M_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 3)) | ((value as u32 & 0x0f) << 3);
        self.w
    }
}
#[doc = "Field `clk_n` reader - "]
pub struct CLK_N_R(crate::FieldReader<u8, u8>);
impl CLK_N_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLK_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_N_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clk_n` writer - "]
pub struct CLK_N_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !7) | (value as u32 & 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - Setting duty cycle of clock as master"]
    #[inline(always)]
    pub fn clk_duty(&self) -> CLK_DUTY_R {
        CLK_DUTY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 3:6"]
    #[inline(always)]
    pub fn clk_m(&self) -> CLK_M_R {
        CLK_M_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn clk_n(&self) -> CLK_N_R {
        CLK_N_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - Setting duty cycle of clock as master"]
    #[inline(always)]
    pub fn clk_duty(&mut self) -> CLK_DUTY_W {
        CLK_DUTY_W { w: self }
    }
    #[doc = "Bits 3:6"]
    #[inline(always)]
    pub fn clk_m(&mut self) -> CLK_M_W {
        CLK_M_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn clk_n(&mut self) -> CLK_N_W {
        CLK_N_W { w: self }
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
}
#[doc = "`reset()` method sets TWI_CCR to value 0"]
impl crate::Resettable for TWI_CCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
