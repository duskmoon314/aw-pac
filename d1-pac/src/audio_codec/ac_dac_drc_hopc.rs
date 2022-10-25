#[doc = "Register `ac_dac_drc_hopc` reader"]
pub struct R(crate::R<AC_DAC_DRC_HOPC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AC_DAC_DRC_HOPC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AC_DAC_DRC_HOPC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AC_DAC_DRC_HOPC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ac_dac_drc_hopc` writer"]
pub struct W(crate::W<AC_DAC_DRC_HOPC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AC_DAC_DRC_HOPC_SPEC>;
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
impl From<crate::W<AC_DAC_DRC_HOPC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AC_DAC_DRC_HOPC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dac_drc_hopc` reader - The output of the compressor, which is determined by the equation -OPC/6.0206. The format is 8.24 (The default value is -40 dB)"]
pub type DAC_DRC_HOPC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `dac_drc_hopc` writer - The output of the compressor, which is determined by the equation -OPC/6.0206. The format is 8.24 (The default value is -40 dB)"]
pub type DAC_DRC_HOPC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AC_DAC_DRC_HOPC_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - The output of the compressor, which is determined by the equation -OPC/6.0206. The format is 8.24 (The default value is -40 dB)"]
    #[inline(always)]
    pub fn dac_drc_hopc(&self) -> DAC_DRC_HOPC_R {
        DAC_DRC_HOPC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The output of the compressor, which is determined by the equation -OPC/6.0206. The format is 8.24 (The default value is -40 dB)"]
    #[inline(always)]
    #[must_use]
    pub fn dac_drc_hopc(&mut self) -> DAC_DRC_HOPC_W<0> {
        DAC_DRC_HOPC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC DRC Compressor High Output at Compressor Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ac_dac_drc_hopc](index.html) module"]
pub struct AC_DAC_DRC_HOPC_SPEC;
impl crate::RegisterSpec for AC_DAC_DRC_HOPC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ac_dac_drc_hopc::R](R) reader structure"]
impl crate::Readable for AC_DAC_DRC_HOPC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ac_dac_drc_hopc::W](W) writer structure"]
impl crate::Writable for AC_DAC_DRC_HOPC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ac_dac_drc_hopc to value 0xf95b"]
impl crate::Resettable for AC_DAC_DRC_HOPC_SPEC {
    const RESET_VALUE: Self::Ux = 0xf95b;
}
