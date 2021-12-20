#[doc = "Register `MBUS_MAT_CLK_GATING` reader"]
pub struct R(crate::R<MBUS_MAT_CLK_GATING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MBUS_MAT_CLK_GATING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MBUS_MAT_CLK_GATING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MBUS_MAT_CLK_GATING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MBUS_MAT_CLK_GATING` writer"]
pub struct W(crate::W<MBUS_MAT_CLK_GATING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MBUS_MAT_CLK_GATING_SPEC>;
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
impl From<crate::W<MBUS_MAT_CLK_GATING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MBUS_MAT_CLK_GATING_SPEC>) -> Self {
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
#[doc = "MBUS Master Clock Gating Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mbus_mat_clk_gating](index.html) module"]
pub struct MBUS_MAT_CLK_GATING_SPEC;
impl crate::RegisterSpec for MBUS_MAT_CLK_GATING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mbus_mat_clk_gating::R](R) reader structure"]
impl crate::Readable for MBUS_MAT_CLK_GATING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mbus_mat_clk_gating::W](W) writer structure"]
impl crate::Writable for MBUS_MAT_CLK_GATING_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MBUS_MAT_CLK_GATING to value 0"]
impl crate::Resettable for MBUS_MAT_CLK_GATING_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
