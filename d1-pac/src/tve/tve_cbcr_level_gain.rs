#[doc = "Register `tve_cbcr_level_gain` reader"]
pub type R = crate::R<TVE_CBCR_LEVEL_GAIN_SPEC>;
#[doc = "Register `tve_cbcr_level_gain` writer"]
pub type W = crate::W<TVE_CBCR_LEVEL_GAIN_SPEC>;
#[doc = "Field `cb_burst_level` reader - Specify the amplitude of the Cb burst. 8 bit 2's complement integer. Allowed range is from (-127) to 127."]
pub type CB_BURST_LEVEL_R = crate::FieldReader;
#[doc = "Field `cb_burst_level` writer - Specify the amplitude of the Cb burst. 8 bit 2's complement integer. Allowed range is from (-127) to 127."]
pub type CB_BURST_LEVEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cr_burst_level` reader - Specify the amplitude of the Cr burst. 8 bit 2's complement integer. Allowed range is from (-127) to 127."]
pub type CR_BURST_LEVEL_R = crate::FieldReader;
#[doc = "Field `cr_burst_level` writer - Specify the amplitude of the Cr burst. 8 bit 2's complement integer. Allowed range is from (-127) to 127."]
pub type CR_BURST_LEVEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
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
    pub fn cb_burst_level(&mut self) -> CB_BURST_LEVEL_W<TVE_CBCR_LEVEL_GAIN_SPEC> {
        CB_BURST_LEVEL_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Specify the amplitude of the Cr burst. 8 bit 2's complement integer. Allowed range is from (-127) to 127."]
    #[inline(always)]
    #[must_use]
    pub fn cr_burst_level(&mut self) -> CR_BURST_LEVEL_W<TVE_CBCR_LEVEL_GAIN_SPEC> {
        CR_BURST_LEVEL_W::new(self, 8)
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
#[doc = "TV Encoder Cb/Cr Level/Gain Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_cbcr_level_gain::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_cbcr_level_gain::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TVE_CBCR_LEVEL_GAIN_SPEC;
impl crate::RegisterSpec for TVE_CBCR_LEVEL_GAIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tve_cbcr_level_gain::R`](R) reader structure"]
impl crate::Readable for TVE_CBCR_LEVEL_GAIN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tve_cbcr_level_gain::W`](W) writer structure"]
impl crate::Writable for TVE_CBCR_LEVEL_GAIN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_cbcr_level_gain to value 0x4f"]
impl crate::Resettable for TVE_CBCR_LEVEL_GAIN_SPEC {
    const RESET_VALUE: Self::Ux = 0x4f;
}
