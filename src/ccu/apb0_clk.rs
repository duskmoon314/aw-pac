#[doc = "Register `APB0_CLK` reader"]
pub struct R(crate::R<APB0_CLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB0_CLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB0_CLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB0_CLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB0_CLK` writer"]
pub struct W(crate::W<APB0_CLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB0_CLK_SPEC>;
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
impl From<crate::W<APB0_CLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB0_CLK_SPEC>) -> Self {
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
#[doc = "APB0 Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb0_clk](index.html) module"]
pub struct APB0_CLK_SPEC;
impl crate::RegisterSpec for APB0_CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb0_clk::R](R) reader structure"]
impl crate::Readable for APB0_CLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb0_clk::W](W) writer structure"]
impl crate::Writable for APB0_CLK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB0_CLK to value 0"]
impl crate::Resettable for APB0_CLK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
