#[doc = "Register `APB1_CLK` reader"]
pub struct R(crate::R<APB1_CLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1_CLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1_CLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1_CLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1_CLK` writer"]
pub struct W(crate::W<APB1_CLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1_CLK_SPEC>;
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
impl From<crate::W<APB1_CLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1_CLK_SPEC>) -> Self {
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
#[doc = "APB1 Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1_clk](index.html) module"]
pub struct APB1_CLK_SPEC;
impl crate::RegisterSpec for APB1_CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1_clk::R](R) reader structure"]
impl crate::Readable for APB1_CLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1_clk::W](W) writer structure"]
impl crate::Writable for APB1_CLK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB1_CLK to value 0"]
impl crate::Resettable for APB1_CLK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
