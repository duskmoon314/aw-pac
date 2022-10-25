#[doc = "Register `ac_dac_drc_lope` reader"]
pub struct R(crate::R<AC_DAC_DRC_LOPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AC_DAC_DRC_LOPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AC_DAC_DRC_LOPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AC_DAC_DRC_LOPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ac_dac_drc_lope` writer"]
pub struct W(crate::W<AC_DAC_DRC_LOPE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AC_DAC_DRC_LOPE_SPEC>;
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
impl From<crate::W<AC_DAC_DRC_LOPE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AC_DAC_DRC_LOPE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dac_drc_lope` reader - The output of the expander which is determined by equation OPE/6.0206. The format is 8.24. (The default value is -70 dB)"]
pub type DAC_DRC_LOPE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `dac_drc_lope` writer - The output of the expander which is determined by equation OPE/6.0206. The format is 8.24. (The default value is -70 dB)"]
pub type DAC_DRC_LOPE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AC_DAC_DRC_LOPE_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - The output of the expander which is determined by equation OPE/6.0206. The format is 8.24. (The default value is -70 dB)"]
    #[inline(always)]
    pub fn dac_drc_lope(&self) -> DAC_DRC_LOPE_R {
        DAC_DRC_LOPE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The output of the expander which is determined by equation OPE/6.0206. The format is 8.24. (The default value is -70 dB)"]
    #[inline(always)]
    #[must_use]
    pub fn dac_drc_lope(&mut self) -> DAC_DRC_LOPE_W<0> {
        DAC_DRC_LOPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC DRC Expander Low Output at Expander Threshold\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ac_dac_drc_lope](index.html) module"]
pub struct AC_DAC_DRC_LOPE_SPEC;
impl crate::RegisterSpec for AC_DAC_DRC_LOPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ac_dac_drc_lope::R](R) reader structure"]
impl crate::Readable for AC_DAC_DRC_LOPE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ac_dac_drc_lope::W](W) writer structure"]
impl crate::Writable for AC_DAC_DRC_LOPE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ac_dac_drc_lope to value 0x8d6e"]
impl crate::Resettable for AC_DAC_DRC_LOPE_SPEC {
    const RESET_VALUE: Self::Ux = 0x8d6e;
}
