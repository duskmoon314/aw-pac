#[doc = "Register `PLL_PERI_PAT1_CTRL` reader"]
pub struct R(crate::R<PLL_PERI_PAT1_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_PERI_PAT1_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_PERI_PAT1_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_PERI_PAT1_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL_PERI_PAT1_CTRL` writer"]
pub struct W(crate::W<PLL_PERI_PAT1_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_PERI_PAT1_CTRL_SPEC>;
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
impl From<crate::W<PLL_PERI_PAT1_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_PERI_PAT1_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DITHER_EN` reader - Dither Enable"]
pub struct DITHER_EN_R(crate::FieldReader<bool, bool>);
impl DITHER_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DITHER_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DITHER_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DITHER_EN` writer - Dither Enable"]
pub struct DITHER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DITHER_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `FRAC_EN` reader - Fraction Enable"]
pub struct FRAC_EN_R(crate::FieldReader<bool, bool>);
impl FRAC_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRAC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRAC_EN` writer - Fraction Enable"]
pub struct FRAC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `FRAC_IN` reader - Fraction In"]
pub struct FRAC_IN_R(crate::FieldReader<u32, u32>);
impl FRAC_IN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        FRAC_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAC_IN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRAC_IN` writer - Fraction In"]
pub struct FRAC_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAC_IN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | (value as u32 & 0x0001_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 24 - Dither Enable"]
    #[inline(always)]
    pub fn dither_en(&self) -> DITHER_EN_R {
        DITHER_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Fraction Enable"]
    #[inline(always)]
    pub fn frac_en(&self) -> FRAC_EN_R {
        FRAC_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 0:16 - Fraction In"]
    #[inline(always)]
    pub fn frac_in(&self) -> FRAC_IN_R {
        FRAC_IN_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 24 - Dither Enable"]
    #[inline(always)]
    pub fn dither_en(&mut self) -> DITHER_EN_W {
        DITHER_EN_W { w: self }
    }
    #[doc = "Bit 20 - Fraction Enable"]
    #[inline(always)]
    pub fn frac_en(&mut self) -> FRAC_EN_W {
        FRAC_EN_W { w: self }
    }
    #[doc = "Bits 0:16 - Fraction In"]
    #[inline(always)]
    pub fn frac_in(&mut self) -> FRAC_IN_W {
        FRAC_IN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL_PERI Pattern1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_peri_pat1_ctrl](index.html) module"]
pub struct PLL_PERI_PAT1_CTRL_SPEC;
impl crate::RegisterSpec for PLL_PERI_PAT1_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll_peri_pat1_ctrl::R](R) reader structure"]
impl crate::Readable for PLL_PERI_PAT1_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll_peri_pat1_ctrl::W](W) writer structure"]
impl crate::Writable for PLL_PERI_PAT1_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLL_PERI_PAT1_CTRL to value 0"]
impl crate::Resettable for PLL_PERI_PAT1_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
