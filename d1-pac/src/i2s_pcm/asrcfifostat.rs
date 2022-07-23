#[doc = "Register `asrcfifostat` reader"]
pub struct R(crate::R<ASRCFIFOSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASRCFIFOSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASRCFIFOSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASRCFIFOSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `asrcfifostat` writer"]
pub struct W(crate::W<ASRCFIFOSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASRCFIFOSTAT_SPEC>;
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
impl From<crate::W<ASRCFIFOSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASRCFIFOSTAT_SPEC>) -> Self {
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
#[doc = "ASRC FIFO Level Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asrcfifostat](index.html) module"]
pub struct ASRCFIFOSTAT_SPEC;
impl crate::RegisterSpec for ASRCFIFOSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [asrcfifostat::R](R) reader structure"]
impl crate::Readable for ASRCFIFOSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [asrcfifostat::W](W) writer structure"]
impl crate::Writable for ASRCFIFOSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets asrcfifostat to value 0"]
impl crate::Resettable for ASRCFIFOSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
