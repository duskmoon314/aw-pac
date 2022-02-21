#[doc = "Register `PCCR01` reader"]
pub struct R(crate::R<PCCR01_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCCR01_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCCR01_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCCR01_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCCR01` writer"]
pub struct W(crate::W<PCCR01_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCCR01_SPEC>;
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
impl From<crate::W<PCCR01_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCCR01_SPEC>) -> Self {
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
#[doc = "PWM01 Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pccr01](index.html) module"]
pub struct PCCR01_SPEC;
impl crate::RegisterSpec for PCCR01_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pccr01::R](R) reader structure"]
impl crate::Readable for PCCR01_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pccr01::W](W) writer structure"]
impl crate::Writable for PCCR01_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCCR01 to value 0"]
impl crate::Resettable for PCCR01_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
