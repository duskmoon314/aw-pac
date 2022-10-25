#[doc = "Register `intosc_clk_prescal` reader"]
pub struct R(crate::R<INTOSC_CLK_PRESCAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTOSC_CLK_PRESCAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTOSC_CLK_PRESCAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTOSC_CLK_PRESCAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `intosc_clk_prescal` writer"]
pub struct W(crate::W<INTOSC_CLK_PRESCAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTOSC_CLK_PRESCAL_SPEC>;
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
impl From<crate::W<INTOSC_CLK_PRESCAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTOSC_CLK_PRESCAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `intosc_32k_clk_prescal` reader - Internal OSC 32K Clock Prescalar value N.\n\nThe clock output = Internal RC/32/N."]
pub type INTOSC_32K_CLK_PRESCAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `intosc_32k_clk_prescal` writer - Internal OSC 32K Clock Prescalar value N.\n\nThe clock output = Internal RC/32/N."]
pub type INTOSC_32K_CLK_PRESCAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTOSC_CLK_PRESCAL_SPEC, u8, u8, 5, O>;
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
    #[must_use]
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
#[doc = "Internal OSC Clock Pre-scalar Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intosc_clk_prescal](index.html) module"]
pub struct INTOSC_CLK_PRESCAL_SPEC;
impl crate::RegisterSpec for INTOSC_CLK_PRESCAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intosc_clk_prescal::R](R) reader structure"]
impl crate::Readable for INTOSC_CLK_PRESCAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intosc_clk_prescal::W](W) writer structure"]
impl crate::Writable for INTOSC_CLK_PRESCAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets intosc_clk_prescal to value 0x0f"]
impl crate::Resettable for INTOSC_CLK_PRESCAL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}
