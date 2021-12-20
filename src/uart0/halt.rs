#[doc = "Register `HALT` reader"]
pub struct R(crate::R<HALT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HALT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HALT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HALT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HALT` writer"]
pub struct W(crate::W<HALT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HALT_SPEC>;
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
impl From<crate::W<HALT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HALT_SPEC>) -> Self {
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
#[doc = "UART Halt TX Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [halt](index.html) module"]
pub struct HALT_SPEC;
impl crate::RegisterSpec for HALT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [halt::R](R) reader structure"]
impl crate::Readable for HALT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [halt::W](W) writer structure"]
impl crate::Writable for HALT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HALT to value 0"]
impl crate::Resettable for HALT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
