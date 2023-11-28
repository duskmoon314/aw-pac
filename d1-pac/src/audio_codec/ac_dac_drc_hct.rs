#[doc = "Register `ac_dac_drc_hct` reader"]
pub type R = crate::R<AC_DAC_DRC_HCT_SPEC>;
#[doc = "Register `ac_dac_drc_hct` writer"]
pub type W = crate::W<AC_DAC_DRC_HCT_SPEC>;
#[doc = "Field `dac_drc_hct` reader - The compressor threshold setting, which is set by the equation that CTin = -CT/6.0206. The format is 8.24. (The default value is - 40 dB)"]
pub type DAC_DRC_HCT_R = crate::FieldReader<u16>;
#[doc = "Field `dac_drc_hct` writer - The compressor threshold setting, which is set by the equation that CTin = -CT/6.0206. The format is 8.24. (The default value is - 40 dB)"]
pub type DAC_DRC_HCT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - The compressor threshold setting, which is set by the equation that CTin = -CT/6.0206. The format is 8.24. (The default value is - 40 dB)"]
    #[inline(always)]
    pub fn dac_drc_hct(&self) -> DAC_DRC_HCT_R {
        DAC_DRC_HCT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The compressor threshold setting, which is set by the equation that CTin = -CT/6.0206. The format is 8.24. (The default value is - 40 dB)"]
    #[inline(always)]
    #[must_use]
    pub fn dac_drc_hct(&mut self) -> DAC_DRC_HCT_W<AC_DAC_DRC_HCT_SPEC> {
        DAC_DRC_HCT_W::new(self, 0)
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
#[doc = "DAC DRC Compressor Threshold High Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_hct::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_hct::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AC_DAC_DRC_HCT_SPEC;
impl crate::RegisterSpec for AC_DAC_DRC_HCT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ac_dac_drc_hct::R`](R) reader structure"]
impl crate::Readable for AC_DAC_DRC_HCT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ac_dac_drc_hct::W`](W) writer structure"]
impl crate::Writable for AC_DAC_DRC_HCT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ac_dac_drc_hct to value 0x06a4"]
impl crate::Resettable for AC_DAC_DRC_HCT_SPEC {
    const RESET_VALUE: Self::Ux = 0x06a4;
}
