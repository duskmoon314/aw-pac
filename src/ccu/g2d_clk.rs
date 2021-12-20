#[doc = "Register `G2D_CLK` reader"]
pub struct R(crate::R<G2D_CLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<G2D_CLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<G2D_CLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<G2D_CLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `G2D_CLK` writer"]
pub struct W(crate::W<G2D_CLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<G2D_CLK_SPEC>;
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
impl From<crate::W<G2D_CLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<G2D_CLK_SPEC>) -> Self {
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
#[doc = "G2D Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [g2d_clk](index.html) module"]
pub struct G2D_CLK_SPEC;
impl crate::RegisterSpec for G2D_CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [g2d_clk::R](R) reader structure"]
impl crate::Readable for G2D_CLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [g2d_clk::W](W) writer structure"]
impl crate::Writable for G2D_CLK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets G2D_CLK to value 0"]
impl crate::Resettable for G2D_CLK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
