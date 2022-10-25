#[doc = "Register `smhc_sfc` reader"]
pub struct R(crate::R<SMHC_SFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_SFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_SFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_SFC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `smhc_sfc` writer"]
pub struct W(crate::W<SMHC_SFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMHC_SFC_SPEC>;
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
impl From<crate::W<SMHC_SFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMHC_SFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `bypass_en` reader - Bypass enable"]
pub type BYPASS_EN_R = crate::BitReader<bool>;
#[doc = "Field `bypass_en` writer - Bypass enable"]
pub type BYPASS_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMHC_SFC_SPEC, bool, O>;
#[doc = "Field `stop_clk_ctrl` reader - Stop Clock Control"]
pub type STOP_CLK_CTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `stop_clk_ctrl` writer - Stop Clock Control"]
pub type STOP_CLK_CTRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SMHC_SFC_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - Bypass enable"]
    #[inline(always)]
    pub fn bypass_en(&self) -> BYPASS_EN_R {
        BYPASS_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Stop Clock Control"]
    #[inline(always)]
    pub fn stop_clk_ctrl(&self) -> STOP_CLK_CTRL_R {
        STOP_CLK_CTRL_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Bypass enable"]
    #[inline(always)]
    #[must_use]
    pub fn bypass_en(&mut self) -> BYPASS_EN_W<0> {
        BYPASS_EN_W::new(self)
    }
    #[doc = "Bits 1:4 - Stop Clock Control"]
    #[inline(always)]
    #[must_use]
    pub fn stop_clk_ctrl(&mut self) -> STOP_CLK_CTRL_W<1> {
        STOP_CLK_CTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sample FIFO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_sfc](index.html) module"]
pub struct SMHC_SFC_SPEC;
impl crate::RegisterSpec for SMHC_SFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_sfc::R](R) reader structure"]
impl crate::Readable for SMHC_SFC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smhc_sfc::W](W) writer structure"]
impl crate::Writable for SMHC_SFC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets smhc_sfc to value 0"]
impl crate::Resettable for SMHC_SFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
