#[doc = "Register `tve_dac_cfg2` reader"]
pub struct R(crate::R<TVE_DAC_CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVE_DAC_CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVE_DAC_CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVE_DAC_CFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tve_dac_cfg2` writer"]
pub struct W(crate::W<TVE_DAC_CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVE_DAC_CFG2_SPEC>;
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
impl From<crate::W<TVE_DAC_CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVE_DAC_CFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `r_set` reader - "]
pub type R_SET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `r_set` writer - "]
pub type R_SET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TVE_DAC_CFG2_SPEC, u8, u8, 6, O>;
#[doc = "Field `s2s1` reader - "]
pub type S2S1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `s2s1` writer - "]
pub type S2S1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TVE_DAC_CFG2_SPEC, u8, u8, 2, O>;
#[doc = "Field `ab` reader - (I config output current for different peak voltage)"]
pub type AB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ab` writer - (I config output current for different peak voltage)"]
pub type AB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TVE_DAC_CFG2_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn r_set(&self) -> R_SET_R {
        R_SET_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn s2s1(&self) -> S2S1_R {
        S2S1_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - (I config output current for different peak voltage)"]
    #[inline(always)]
    pub fn ab(&self) -> AB_R {
        AB_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn r_set(&mut self) -> R_SET_W<0> {
        R_SET_W::new(self)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn s2s1(&mut self) -> S2S1_W<6> {
        S2S1_W::new(self)
    }
    #[doc = "Bits 8:12 - (I config output current for different peak voltage)"]
    #[inline(always)]
    #[must_use]
    pub fn ab(&mut self) -> AB_W<8> {
        AB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Encoder DAC CFG2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tve_dac_cfg2](index.html) module"]
pub struct TVE_DAC_CFG2_SPEC;
impl crate::RegisterSpec for TVE_DAC_CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tve_dac_cfg2::R](R) reader structure"]
impl crate::Readable for TVE_DAC_CFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tve_dac_cfg2::W](W) writer structure"]
impl crate::Writable for TVE_DAC_CFG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_dac_cfg2 to value 0"]
impl crate::Resettable for TVE_DAC_CFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
