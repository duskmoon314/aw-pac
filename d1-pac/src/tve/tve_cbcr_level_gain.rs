#[doc = "Register `tve_cbcr_level_gain` reader"]
pub struct R(crate::R<TVE_CBCR_LEVEL_GAIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVE_CBCR_LEVEL_GAIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVE_CBCR_LEVEL_GAIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVE_CBCR_LEVEL_GAIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tve_cbcr_level_gain` writer"]
pub struct W(crate::W<TVE_CBCR_LEVEL_GAIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVE_CBCR_LEVEL_GAIN_SPEC>;
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
impl From<crate::W<TVE_CBCR_LEVEL_GAIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVE_CBCR_LEVEL_GAIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cb_burst_level` reader - Specify the amplitude of the Cb burst. 8 bit 2's complement integer. Allowed range is from (-127) to 127."]
pub type CB_BURST_LEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cb_burst_level` writer - Specify the amplitude of the Cb burst. 8 bit 2's complement integer. Allowed range is from (-127) to 127."]
pub type CB_BURST_LEVEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TVE_CBCR_LEVEL_GAIN_SPEC, u8, u8, 8, O>;
#[doc = "Field `cr_burst_level` reader - Specify the amplitude of the Cr burst. 8 bit 2's complement integer. Allowed range is from (-127) to 127."]
pub type CR_BURST_LEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_burst_level` writer - Specify the amplitude of the Cr burst. 8 bit 2's complement integer. Allowed range is from (-127) to 127."]
pub type CR_BURST_LEVEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TVE_CBCR_LEVEL_GAIN_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Specify the amplitude of the Cb burst. 8 bit 2's complement integer. Allowed range is from (-127) to 127."]
    #[inline(always)]
    pub fn cb_burst_level(&self) -> CB_BURST_LEVEL_R {
        CB_BURST_LEVEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Specify the amplitude of the Cr burst. 8 bit 2's complement integer. Allowed range is from (-127) to 127."]
    #[inline(always)]
    pub fn cr_burst_level(&self) -> CR_BURST_LEVEL_R {
        CR_BURST_LEVEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Specify the amplitude of the Cb burst. 8 bit 2's complement integer. Allowed range is from (-127) to 127."]
    #[inline(always)]
    #[must_use]
    pub fn cb_burst_level(&mut self) -> CB_BURST_LEVEL_W<0> {
        CB_BURST_LEVEL_W::new(self)
    }
    #[doc = "Bits 8:15 - Specify the amplitude of the Cr burst. 8 bit 2's complement integer. Allowed range is from (-127) to 127."]
    #[inline(always)]
    #[must_use]
    pub fn cr_burst_level(&mut self) -> CR_BURST_LEVEL_W<8> {
        CR_BURST_LEVEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Encoder Cb/Cr Level/Gain Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tve_cbcr_level_gain](index.html) module"]
pub struct TVE_CBCR_LEVEL_GAIN_SPEC;
impl crate::RegisterSpec for TVE_CBCR_LEVEL_GAIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tve_cbcr_level_gain::R](R) reader structure"]
impl crate::Readable for TVE_CBCR_LEVEL_GAIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tve_cbcr_level_gain::W](W) writer structure"]
impl crate::Writable for TVE_CBCR_LEVEL_GAIN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_cbcr_level_gain to value 0x4f"]
impl crate::Resettable for TVE_CBCR_LEVEL_GAIN_SPEC {
    const RESET_VALUE: Self::Ux = 0x4f;
}
