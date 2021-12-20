#[doc = "Register `PLL_AUDIO1_PAT1_CTRL` reader"]
pub struct R(crate::R<PLL_AUDIO1_PAT1_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_AUDIO1_PAT1_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_AUDIO1_PAT1_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_AUDIO1_PAT1_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL_AUDIO1_PAT1_CTRL` writer"]
pub struct W(crate::W<PLL_AUDIO1_PAT1_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_AUDIO1_PAT1_CTRL_SPEC>;
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
impl From<crate::W<PLL_AUDIO1_PAT1_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_AUDIO1_PAT1_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL_AUDIO1 Pattern1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_audio1_pat1_ctrl](index.html) module"]
pub struct PLL_AUDIO1_PAT1_CTRL_SPEC;
impl crate::RegisterSpec for PLL_AUDIO1_PAT1_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll_audio1_pat1_ctrl::R](R) reader structure"]
impl crate::Readable for PLL_AUDIO1_PAT1_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll_audio1_pat1_ctrl::W](W) writer structure"]
impl crate::Writable for PLL_AUDIO1_PAT1_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLL_AUDIO1_PAT1_CTRL to value 0"]
impl crate::Resettable for PLL_AUDIO1_PAT1_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
