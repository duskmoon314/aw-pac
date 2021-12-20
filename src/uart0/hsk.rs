#[doc = "Register `HSK` reader"]
pub struct R(crate::R<HSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSK` writer"]
pub struct W(crate::W<HSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSK_SPEC>;
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
impl From<crate::W<HSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSK_SPEC>) -> Self {
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
#[doc = "UART DMA Handshake Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsk](index.html) module"]
pub struct HSK_SPEC;
impl crate::RegisterSpec for HSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsk::R](R) reader structure"]
impl crate::Readable for HSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hsk::W](W) writer structure"]
impl crate::Writable for HSK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSK to value 0"]
impl crate::Resettable for HSK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
