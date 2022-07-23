#[doc = "Register `tve_dac_map` reader"]
pub struct R(crate::R<TVE_DAC_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVE_DAC_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVE_DAC_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVE_DAC_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tve_dac_map` writer"]
pub struct W(crate::W<TVE_DAC_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVE_DAC_MAP_SPEC>;
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
impl From<crate::W<TVE_DAC_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVE_DAC_MAP_SPEC>) -> Self {
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
#[doc = "TV Encoder DAC MAP Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tve_dac_map](index.html) module"]
pub struct TVE_DAC_MAP_SPEC;
impl crate::RegisterSpec for TVE_DAC_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tve_dac_map::R](R) reader structure"]
impl crate::Readable for TVE_DAC_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tve_dac_map::W](W) writer structure"]
impl crate::Writable for TVE_DAC_MAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets tve_dac_map to value 0"]
impl crate::Resettable for TVE_DAC_MAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
