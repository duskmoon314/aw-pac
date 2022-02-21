#[doc = "Register `PLL_CPU_BIAS` reader"]
pub struct R(crate::R<PLL_CPU_BIAS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_CPU_BIAS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_CPU_BIAS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_CPU_BIAS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL_CPU_BIAS` writer"]
pub struct W(crate::W<PLL_CPU_BIAS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_CPU_BIAS_SPEC>;
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
impl From<crate::W<PLL_CPU_BIAS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_CPU_BIAS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLL_VCO_RST_IN` reader - VCO reset in"]
pub struct PLL_VCO_RST_IN_R(crate::FieldReader<bool, bool>);
impl PLL_VCO_RST_IN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLL_VCO_RST_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_VCO_RST_IN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_VCO_RST_IN` writer - VCO reset in"]
pub struct PLL_VCO_RST_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_VCO_RST_IN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `PLL_CP` reader - PLL current bias control"]
pub struct PLL_CP_R(crate::FieldReader<u8, u8>);
impl PLL_CP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PLL_CP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_CP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_CP` writer - PLL current bias control"]
pub struct PLL_CP_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_CP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - VCO reset in"]
    #[inline(always)]
    pub fn pll_vco_rst_in(&self) -> PLL_VCO_RST_IN_R {
        PLL_VCO_RST_IN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - PLL current bias control"]
    #[inline(always)]
    pub fn pll_cp(&self) -> PLL_CP_R {
        PLL_CP_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - VCO reset in"]
    #[inline(always)]
    pub fn pll_vco_rst_in(&mut self) -> PLL_VCO_RST_IN_W {
        PLL_VCO_RST_IN_W { w: self }
    }
    #[doc = "Bits 16:20 - PLL current bias control"]
    #[inline(always)]
    pub fn pll_cp(&mut self) -> PLL_CP_W {
        PLL_CP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL_CPU Bias Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_cpu_bias](index.html) module"]
pub struct PLL_CPU_BIAS_SPEC;
impl crate::RegisterSpec for PLL_CPU_BIAS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll_cpu_bias::R](R) reader structure"]
impl crate::Readable for PLL_CPU_BIAS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll_cpu_bias::W](W) writer structure"]
impl crate::Writable for PLL_CPU_BIAS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLL_CPU_BIAS to value 0"]
impl crate::Resettable for PLL_CPU_BIAS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
