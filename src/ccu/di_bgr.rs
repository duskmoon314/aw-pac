#[doc = "Register `DI_BGR` reader"]
pub struct R(crate::R<DI_BGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DI_BGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DI_BGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DI_BGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DI_BGR` writer"]
pub struct W(crate::W<DI_BGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DI_BGR_SPEC>;
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
impl From<crate::W<DI_BGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DI_BGR_SPEC>) -> Self {
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
#[doc = "DI Bus Gating Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [di_bgr](index.html) module"]
pub struct DI_BGR_SPEC;
impl crate::RegisterSpec for DI_BGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [di_bgr::R](R) reader structure"]
impl crate::Readable for DI_BGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [di_bgr::W](W) writer structure"]
impl crate::Writable for DI_BGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DI_BGR to value 0"]
impl crate::Resettable for DI_BGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
