#[doc = "Register `ccu_clk_mode` reader"]
pub struct R(crate::R<CCU_CLK_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCU_CLK_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCU_CLK_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCU_CLK_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ccu_clk_mode` writer"]
pub struct W(crate::W<CCU_CLK_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCU_CLK_MODE_SPEC>;
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
impl From<crate::W<CCU_CLK_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCU_CLK_MODE_SPEC>) -> Self {
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
#[doc = "CCU Clock Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccu_clk_mode](index.html) module"]
pub struct CCU_CLK_MODE_SPEC;
impl crate::RegisterSpec for CCU_CLK_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccu_clk_mode::R](R) reader structure"]
impl crate::Readable for CCU_CLK_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccu_clk_mode::W](W) writer structure"]
impl crate::Writable for CCU_CLK_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ccu_clk_mode to value 0"]
impl crate::Resettable for CCU_CLK_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
