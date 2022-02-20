#[doc = "Register `PDZCR67` reader"]
pub struct R(crate::R<PDZCR67_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDZCR67_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDZCR67_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDZCR67_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDZCR67` writer"]
pub struct W(crate::W<PDZCR67_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDZCR67_SPEC>;
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
impl From<crate::W<PDZCR67_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDZCR67_SPEC>) -> Self {
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
#[doc = "PWM67 Dead Zone Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdzcr67](index.html) module"]
pub struct PDZCR67_SPEC;
impl crate::RegisterSpec for PDZCR67_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdzcr67::R](R) reader structure"]
impl crate::Readable for PDZCR67_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdzcr67::W](W) writer structure"]
impl crate::Writable for PDZCR67_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDZCR67 to value 0"]
impl crate::Resettable for PDZCR67_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
