#[doc = "Register `RBR` reader"]
pub struct R(crate::R<RBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RBR` writer"]
pub struct W(crate::W<RBR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RBR_SPEC>;
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
impl From<crate::W<RBR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RBR_SPEC>) -> Self {
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
#[doc = "UART Receive Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbr](index.html) module"]
pub struct RBR_SPEC;
impl crate::RegisterSpec for RBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbr::R](R) reader structure"]
impl crate::Readable for RBR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rbr::W](W) writer structure"]
impl crate::Writable for RBR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RBR to value 0"]
impl crate::Resettable for RBR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
