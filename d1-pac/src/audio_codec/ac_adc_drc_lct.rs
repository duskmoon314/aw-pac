#[doc = "Register `ac_adc_drc_lct` reader"]
pub type R = crate::R<AC_ADC_DRC_LCT_SPEC>;
#[doc = "Register `ac_adc_drc_lct` writer"]
pub type W = crate::W<AC_ADC_DRC_LCT_SPEC>;
#[doc = "Field `adc_drc_lct` reader - The compressor threshold setting, which is set by the equation that CTin = -CT/6.0206. The format is 8.24. (The default value is - 40 dB)"]
pub type ADC_DRC_LCT_R = crate::FieldReader<u16>;
#[doc = "Field `adc_drc_lct` writer - The compressor threshold setting, which is set by the equation that CTin = -CT/6.0206. The format is 8.24. (The default value is - 40 dB)"]
pub type ADC_DRC_LCT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - The compressor threshold setting, which is set by the equation that CTin = -CT/6.0206. The format is 8.24. (The default value is - 40 dB)"]
    #[inline(always)]
    pub fn adc_drc_lct(&self) -> ADC_DRC_LCT_R {
        ADC_DRC_LCT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The compressor threshold setting, which is set by the equation that CTin = -CT/6.0206. The format is 8.24. (The default value is - 40 dB)"]
    #[inline(always)]
    #[must_use]
    pub fn adc_drc_lct(&mut self) -> ADC_DRC_LCT_W<AC_ADC_DRC_LCT_SPEC> {
        ADC_DRC_LCT_W::new(self, 0)
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
#[doc = "ADC DRC Compressor Slope High Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_lct::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_lct::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AC_ADC_DRC_LCT_SPEC;
impl crate::RegisterSpec for AC_ADC_DRC_LCT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ac_adc_drc_lct::R`](R) reader structure"]
impl crate::Readable for AC_ADC_DRC_LCT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ac_adc_drc_lct::W`](W) writer structure"]
impl crate::Writable for AC_ADC_DRC_LCT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ac_adc_drc_lct to value 0xd3c0"]
impl crate::Resettable for AC_ADC_DRC_LCT_SPEC {
    const RESET_VALUE: Self::Ux = 0xd3c0;
}
