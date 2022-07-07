#[doc = "Register `PLL_VIDEO0_PAT1_CTRL` reader"]
pub struct R(crate::R<PLL_VIDEO0_PAT1_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_VIDEO0_PAT1_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_VIDEO0_PAT1_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_VIDEO0_PAT1_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL_VIDEO0_PAT1_CTRL` writer"]
pub struct W(crate::W<PLL_VIDEO0_PAT1_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_VIDEO0_PAT1_CTRL_SPEC>;
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
impl From<crate::W<PLL_VIDEO0_PAT1_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_VIDEO0_PAT1_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DITHER_EN` reader - Dither Enable"]
pub type DITHER_EN_R = crate::BitReader<bool>;
#[doc = "Field `DITHER_EN` writer - Dither Enable"]
pub type DITHER_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PLL_VIDEO0_PAT1_CTRL_SPEC, bool, O>;
#[doc = "Field `FRAC_EN` reader - Fraction Enable"]
pub type FRAC_EN_R = crate::BitReader<bool>;
#[doc = "Field `FRAC_EN` writer - Fraction Enable"]
pub type FRAC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_VIDEO0_PAT1_CTRL_SPEC, bool, O>;
#[doc = "Field `FRAC_IN` reader - Fraction In"]
pub type FRAC_IN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FRAC_IN` writer - Fraction In"]
pub type FRAC_IN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PLL_VIDEO0_PAT1_CTRL_SPEC, u32, u32, 17, O>;
impl R {
    #[doc = "Bit 24 - Dither Enable"]
    #[inline(always)]
    pub fn dither_en(&self) -> DITHER_EN_R {
        DITHER_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 20 - Fraction Enable"]
    #[inline(always)]
    pub fn frac_en(&self) -> FRAC_EN_R {
        FRAC_EN_R::new(((self.bits >> 20) & 1) != 0)
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
    pub fn dither_en(&mut self) -> DITHER_EN_W<24> {
        DITHER_EN_W::new(self)
    }
    #[doc = "Bit 20 - Fraction Enable"]
    #[inline(always)]
    pub fn frac_en(&mut self) -> FRAC_EN_W<20> {
        FRAC_EN_W::new(self)
    }
    #[doc = "Bits 0:16 - Fraction In"]
    #[inline(always)]
    pub fn frac_in(&mut self) -> FRAC_IN_W<0> {
        FRAC_IN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL_VIDEO0 Pattern1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_video0_pat1_ctrl](index.html) module"]
pub struct PLL_VIDEO0_PAT1_CTRL_SPEC;
impl crate::RegisterSpec for PLL_VIDEO0_PAT1_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll_video0_pat1_ctrl::R](R) reader structure"]
impl crate::Readable for PLL_VIDEO0_PAT1_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll_video0_pat1_ctrl::W](W) writer structure"]
impl crate::Writable for PLL_VIDEO0_PAT1_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLL_VIDEO0_PAT1_CTRL to value 0"]
impl crate::Resettable for PLL_VIDEO0_PAT1_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
