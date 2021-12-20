#[doc = "Register `USB0_CLK` reader"]
pub struct R(crate::R<USB0_CLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB0_CLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB0_CLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB0_CLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB0_CLK` writer"]
pub struct W(crate::W<USB0_CLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB0_CLK_SPEC>;
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
impl From<crate::W<USB0_CLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB0_CLK_SPEC>) -> Self {
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
#[doc = "USB0 Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb0_clk](index.html) module"]
pub struct USB0_CLK_SPEC;
impl crate::RegisterSpec for USB0_CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb0_clk::R](R) reader structure"]
impl crate::Readable for USB0_CLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb0_clk::W](W) writer structure"]
impl crate::Writable for USB0_CLK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USB0_CLK to value 0"]
impl crate::Resettable for USB0_CLK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
