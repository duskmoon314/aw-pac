#[doc = "Register `PGR0` reader"]
pub struct R(crate::R<PGR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PGR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PGR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PGR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PGR0` writer"]
pub struct W(crate::W<PGR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PGR0_SPEC>;
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
impl From<crate::W<PGR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PGR0_SPEC>) -> Self {
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
#[doc = "PWM Group0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pgr0](index.html) module"]
pub struct PGR0_SPEC;
impl crate::RegisterSpec for PGR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pgr0::R](R) reader structure"]
impl crate::Readable for PGR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pgr0::W](W) writer structure"]
impl crate::Writable for PGR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PGR0 to value 0"]
impl crate::Resettable for PGR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
