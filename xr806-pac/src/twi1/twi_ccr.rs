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
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:6"]
    #[inline(always)]
    pub fn clk_m(&self) -> CLK_M_R {
        CLK_M_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn clk_n(&self) -> CLK_N_R {
        CLK_N_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
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
