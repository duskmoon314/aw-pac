#[doc = "Register `adc2` reader"]
pub struct R(crate::R<ADC2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `adc2` writer"]
pub struct W(crate::W<ADC2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC2_SPEC>;
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
impl From<crate::W<ADC2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC2_SPEC>) -> Self {
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
#[doc = "ADC2 Analog Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc2](index.html) module"]
pub struct ADC2_SPEC;
impl crate::RegisterSpec for ADC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc2::R](R) reader structure"]
impl crate::Readable for ADC2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc2::W](W) writer structure"]
impl crate::Writable for ADC2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets adc2 to value 0"]
impl crate::Resettable for ADC2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
