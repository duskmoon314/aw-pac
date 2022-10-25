#[doc = "Register `ac_dac_drc_sflrt` reader"]
pub struct R(crate::R<AC_DAC_DRC_SFLRT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AC_DAC_DRC_SFLRT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AC_DAC_DRC_SFLRT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AC_DAC_DRC_SFLRT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ac_dac_drc_sflrt` writer"]
pub struct W(crate::W<AC_DAC_DRC_SFLRT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AC_DAC_DRC_SFLRT_SPEC>;
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
impl From<crate::W<AC_DAC_DRC_SFLRT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AC_DAC_DRC_SFLRT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dac_drc_sflrt` reader - The gain smooth filter release time parameter setting, which is determined by the equation that RT = 1-exp (-2.2Ts/tr). The format is 3.24. (The default value is 200 ms)"]
pub type DAC_DRC_SFLRT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `dac_drc_sflrt` writer - The gain smooth filter release time parameter setting, which is determined by the equation that RT = 1-exp (-2.2Ts/tr). The format is 3.24. (The default value is 200 ms)"]
pub type DAC_DRC_SFLRT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AC_DAC_DRC_SFLRT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - The gain smooth filter release time parameter setting, which is determined by the equation that RT = 1-exp (-2.2Ts/tr). The format is 3.24. (The default value is 200 ms)"]
    #[inline(always)]
    pub fn dac_drc_sflrt(&self) -> DAC_DRC_SFLRT_R {
        DAC_DRC_SFLRT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The gain smooth filter release time parameter setting, which is determined by the equation that RT = 1-exp (-2.2Ts/tr). The format is 3.24. (The default value is 200 ms)"]
    #[inline(always)]
    #[must_use]
    pub fn dac_drc_sflrt(&mut self) -> DAC_DRC_SFLRT_W<0> {
        DAC_DRC_SFLRT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC DRC Smooth filter Gain Low Release Time Coef Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ac_dac_drc_sflrt](index.html) module"]
pub struct AC_DAC_DRC_SFLRT_SPEC;
impl crate::RegisterSpec for AC_DAC_DRC_SFLRT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ac_dac_drc_sflrt::R](R) reader structure"]
impl crate::Readable for AC_DAC_DRC_SFLRT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ac_dac_drc_sflrt::W](W) writer structure"]
impl crate::Writable for AC_DAC_DRC_SFLRT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ac_dac_drc_sflrt to value 0x0f04"]
impl crate::Resettable for AC_DAC_DRC_SFLRT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f04;
}
