#[doc = "Register `CSI_MASTER_CLK` reader"]
pub struct R(crate::R<CSI_MASTER_CLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSI_MASTER_CLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSI_MASTER_CLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSI_MASTER_CLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSI_MASTER_CLK` writer"]
pub struct W(crate::W<CSI_MASTER_CLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSI_MASTER_CLK_SPEC>;
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
impl From<crate::W<CSI_MASTER_CLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSI_MASTER_CLK_SPEC>) -> Self {
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
#[doc = "CSI Master Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csi_master_clk](index.html) module"]
pub struct CSI_MASTER_CLK_SPEC;
impl crate::RegisterSpec for CSI_MASTER_CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csi_master_clk::R](R) reader structure"]
impl crate::Readable for CSI_MASTER_CLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csi_master_clk::W](W) writer structure"]
impl crate::Writable for CSI_MASTER_CLK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSI_MASTER_CLK to value 0"]
impl crate::Resettable for CSI_MASTER_CLK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
