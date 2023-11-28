#[doc = "Register `tve_chroma_frequency` reader"]
pub type R = crate::R<TVE_CHROMA_FREQUENCY_SPEC>;
#[doc = "Register `tve_chroma_frequency` writer"]
pub type W = crate::W<TVE_CHROMA_FREQUENCY_SPEC>;
#[doc = "Field `chroma_freq` reader - Specify the ratio between the color burst frequency. 32 bits unsigned fraction. The default value is h21f07c1f, which is compatible with NTSC spec."]
pub type CHROMA_FREQ_R = crate::FieldReader<u32>;
#[doc = "Field `chroma_freq` writer - Specify the ratio between the color burst frequency. 32 bits unsigned fraction. The default value is h21f07c1f, which is compatible with NTSC spec."]
pub type CHROMA_FREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
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
    pub fn chroma_freq(&mut self) -> CHROMA_FREQ_W<TVE_CHROMA_FREQUENCY_SPEC> {
        CHROMA_FREQ_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "TV Encoder Chroma Frequency Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_chroma_frequency::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_chroma_frequency::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TVE_CHROMA_FREQUENCY_SPEC;
impl crate::RegisterSpec for TVE_CHROMA_FREQUENCY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tve_chroma_frequency::R`](R) reader structure"]
impl crate::Readable for TVE_CHROMA_FREQUENCY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tve_chroma_frequency::W`](W) writer structure"]
impl crate::Writable for TVE_CHROMA_FREQUENCY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_chroma_frequency to value 0x21f0_7c1f"]
impl crate::Resettable for TVE_CHROMA_FREQUENCY_SPEC {
    const RESET_VALUE: Self::Ux = 0x21f0_7c1f;
}
