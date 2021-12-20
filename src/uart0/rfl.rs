#[doc = "Register `RFL` reader"]
pub struct R(crate::R<RFL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFL` writer"]
pub struct W(crate::W<RFL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFL_SPEC>;
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
impl From<crate::W<RFL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFL_SPEC>) -> Self {
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
#[doc = "UART Receive FIFO Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfl](index.html) module"]
pub struct RFL_SPEC;
impl crate::RegisterSpec for RFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfl::R](R) reader structure"]
impl crate::Readable for RFL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfl::W](W) writer structure"]
impl crate::Writable for RFL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RFL to value 0"]
impl crate::Resettable for RFL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
