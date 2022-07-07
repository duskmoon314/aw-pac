#[doc = "Register `INTOSC_CLK_PRESCAL_REG` reader"]
pub struct R(crate::R<INTOSC_CLK_PRESCAL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTOSC_CLK_PRESCAL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTOSC_CLK_PRESCAL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTOSC_CLK_PRESCAL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTOSC_CLK_PRESCAL_REG` writer"]
pub struct W(crate::W<INTOSC_CLK_PRESCAL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTOSC_CLK_PRESCAL_REG_SPEC>;
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
impl From<crate::W<INTOSC_CLK_PRESCAL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTOSC_CLK_PRESCAL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTOSC_32K_CLK_PRESCAL` reader - Internal OSC 32K Clock Prescalar value N.\n\nThe clock output = Internal RC/32/N."]
pub type INTOSC_32K_CLK_PRESCAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INTOSC_32K_CLK_PRESCAL` writer - Internal OSC 32K Clock Prescalar value N.\n\nThe clock output = Internal RC/32/N."]
pub type INTOSC_32K_CLK_PRESCAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTOSC_CLK_PRESCAL_REG_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Internal OSC 32K Clock Prescalar value N.\n\nThe clock output = Internal RC/32/N."]
    #[inline(always)]
    pub fn intosc_32k_clk_prescal(&self) -> INTOSC_32K_CLK_PRESCAL_R {
        INTOSC_32K_CLK_PRESCAL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Internal OSC 32K Clock Prescalar value N.\n\nThe clock output = Internal RC/32/N."]
    #[inline(always)]
    pub fn intosc_32k_clk_prescal(&mut self) -> INTOSC_32K_CLK_PRESCAL_W<0> {
        INTOSC_32K_CLK_PRESCAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal OSC Clock Pre-scalar Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intosc_clk_prescal_reg](index.html) module"]
pub struct INTOSC_CLK_PRESCAL_REG_SPEC;
impl crate::RegisterSpec for INTOSC_CLK_PRESCAL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intosc_clk_prescal_reg::R](R) reader structure"]
impl crate::Readable for INTOSC_CLK_PRESCAL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intosc_clk_prescal_reg::W](W) writer structure"]
impl crate::Writable for INTOSC_CLK_PRESCAL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTOSC_CLK_PRESCAL_REG to value 0x0f"]
impl crate::Resettable for INTOSC_CLK_PRESCAL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
