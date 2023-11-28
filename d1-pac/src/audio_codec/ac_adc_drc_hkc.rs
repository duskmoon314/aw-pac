#[doc = "Register `ac_adc_drc_hkc` reader"]
pub type R = crate::R<AC_ADC_DRC_HKC_SPEC>;
#[doc = "Register `ac_adc_drc_hkc` writer"]
pub type W = crate::W<AC_ADC_DRC_HKC_SPEC>;
#[doc = "Field `adc_drc_hkc` reader - The slope of the compressor which is determined by the equation that Kc = 1/R. R is the ratio of the compressor, which is always an integer. The format is 8.24. (The default value is &lt;2:1>)"]
pub type ADC_DRC_HKC_R = crate::FieldReader<u16>;
#[doc = "Field `adc_drc_hkc` writer - The slope of the compressor which is determined by the equation that Kc = 1/R. R is the ratio of the compressor, which is always an integer. The format is 8.24. (The default value is &lt;2:1>)"]
pub type ADC_DRC_HKC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - The slope of the compressor which is determined by the equation that Kc = 1/R. R is the ratio of the compressor, which is always an integer. The format is 8.24. (The default value is &lt;2:1>)"]
    #[inline(always)]
    pub fn adc_drc_hkc(&self) -> ADC_DRC_HKC_R {
        ADC_DRC_HKC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The slope of the compressor which is determined by the equation that Kc = 1/R. R is the ratio of the compressor, which is always an integer. The format is 8.24. (The default value is &lt;2:1>)"]
    #[inline(always)]
    #[must_use]
    pub fn adc_drc_hkc(&mut self) -> ADC_DRC_HKC_W<AC_ADC_DRC_HKC_SPEC> {
        ADC_DRC_HKC_W::new(self, 0)
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
#[doc = "ADC DRC Compressor Slope High Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_hkc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_hkc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AC_ADC_DRC_HKC_SPEC;
impl crate::RegisterSpec for AC_ADC_DRC_HKC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ac_adc_drc_hkc::R`](R) reader structure"]
impl crate::Readable for AC_ADC_DRC_HKC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ac_adc_drc_hkc::W`](W) writer structure"]
impl crate::Writable for AC_ADC_DRC_HKC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ac_adc_drc_hkc to value 0x80"]
impl crate::Resettable for AC_ADC_DRC_HKC_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
