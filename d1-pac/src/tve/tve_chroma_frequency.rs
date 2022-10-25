#[doc = "Register `tve_chroma_frequency` reader"]
pub struct R(crate::R<TVE_CHROMA_FREQUENCY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVE_CHROMA_FREQUENCY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVE_CHROMA_FREQUENCY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVE_CHROMA_FREQUENCY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tve_chroma_frequency` writer"]
pub struct W(crate::W<TVE_CHROMA_FREQUENCY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVE_CHROMA_FREQUENCY_SPEC>;
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
impl From<crate::W<TVE_CHROMA_FREQUENCY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVE_CHROMA_FREQUENCY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `chroma_freq` reader - Specify the ratio between the color burst frequency. 32 bits unsigned fraction. The default value is h21f07c1f, which is compatible with NTSC spec."]
pub type CHROMA_FREQ_R = crate::FieldReader<u32, u32>;
#[doc = "Field `chroma_freq` writer - Specify the ratio between the color burst frequency. 32 bits unsigned fraction. The default value is h21f07c1f, which is compatible with NTSC spec."]
pub type CHROMA_FREQ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TVE_CHROMA_FREQUENCY_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Specify the ratio between the color burst frequency. 32 bits unsigned fraction. The default value is h21f07c1f, which is compatible with NTSC spec."]
    #[inline(always)]
    pub fn chroma_freq(&self) -> CHROMA_FREQ_R {
        CHROMA_FREQ_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specify the ratio between the color burst frequency. 32 bits unsigned fraction. The default value is h21f07c1f, which is compatible with NTSC spec."]
    #[inline(always)]
    #[must_use]
    pub fn chroma_freq(&mut self) -> CHROMA_FREQ_W<0> {
        CHROMA_FREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Encoder Chroma Frequency Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tve_chroma_frequency](index.html) module"]
pub struct TVE_CHROMA_FREQUENCY_SPEC;
impl crate::RegisterSpec for TVE_CHROMA_FREQUENCY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tve_chroma_frequency::R](R) reader structure"]
impl crate::Readable for TVE_CHROMA_FREQUENCY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tve_chroma_frequency::W](W) writer structure"]
impl crate::Writable for TVE_CHROMA_FREQUENCY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_chroma_frequency to value 0x21f0_7c1f"]
impl crate::Resettable for TVE_CHROMA_FREQUENCY_SPEC {
    const RESET_VALUE: Self::Ux = 0x21f0_7c1f;
}
