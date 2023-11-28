#[doc = "Register `ac_adc_drc_epshc` reader"]
pub type R = crate::R<AC_ADC_DRC_EPSHC_SPEC>;
#[doc = "Register `ac_adc_drc_epshc` writer"]
pub type W = crate::W<AC_ADC_DRC_EPSHC_SPEC>;
#[doc = "Field `adc_drc_epshc` reader - The gain smooth filter release and attack time parameter setting in expander region, which is determined by the equation that RT = 1-exp (-2.2Ts/tr). The format is 3.24. (The default value is 30 ms)"]
pub type ADC_DRC_EPSHC_R = crate::FieldReader<u16>;
#[doc = "Field `adc_drc_epshc` writer - The gain smooth filter release and attack time parameter setting in expander region, which is determined by the equation that RT = 1-exp (-2.2Ts/tr). The format is 3.24. (The default value is 30 ms)"]
pub type ADC_DRC_EPSHC_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - The gain smooth filter release and attack time parameter setting in expander region, which is determined by the equation that RT = 1-exp (-2.2Ts/tr). The format is 3.24. (The default value is 30 ms)"]
    #[inline(always)]
    pub fn adc_drc_epshc(&self) -> ADC_DRC_EPSHC_R {
        ADC_DRC_EPSHC_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - The gain smooth filter release and attack time parameter setting in expander region, which is determined by the equation that RT = 1-exp (-2.2Ts/tr). The format is 3.24. (The default value is 30 ms)"]
    #[inline(always)]
    #[must_use]
    pub fn adc_drc_epshc(&mut self) -> ADC_DRC_EPSHC_W<AC_ADC_DRC_EPSHC_SPEC> {
        ADC_DRC_EPSHC_W::new(self, 0)
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
#[doc = "ADC DRC Expander Smooth Time High Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_epshc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_epshc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AC_ADC_DRC_EPSHC_SPEC;
impl crate::RegisterSpec for AC_ADC_DRC_EPSHC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ac_adc_drc_epshc::R`](R) reader structure"]
impl crate::Readable for AC_ADC_DRC_EPSHC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ac_adc_drc_epshc::W`](W) writer structure"]
impl crate::Writable for AC_ADC_DRC_EPSHC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ac_adc_drc_epshc to value 0"]
impl crate::Resettable for AC_ADC_DRC_EPSHC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
