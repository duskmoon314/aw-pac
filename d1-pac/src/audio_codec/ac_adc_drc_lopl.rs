#[doc = "Register `ac_adc_drc_lopl` reader"]
pub type R = crate::R<AC_ADC_DRC_LOPL_SPEC>;
#[doc = "Register `ac_adc_drc_lopl` writer"]
pub type W = crate::W<AC_ADC_DRC_LOPL_SPEC>;
#[doc = "Field `adc_drc_lopl` reader - The output of the limiter which is determined by equation OPT/6.0206. The format is 8.24. (The default value is -25 dB)"]
pub type ADC_DRC_LOPL_R = crate::FieldReader<u16>;
#[doc = "Field `adc_drc_lopl` writer - The output of the limiter which is determined by equation OPT/6.0206. The format is 8.24. (The default value is -25 dB)"]
pub type ADC_DRC_LOPL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - The output of the limiter which is determined by equation OPT/6.0206. The format is 8.24. (The default value is -25 dB)"]
    #[inline(always)]
    pub fn adc_drc_lopl(&self) -> ADC_DRC_LOPL_R {
        ADC_DRC_LOPL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The output of the limiter which is determined by equation OPT/6.0206. The format is 8.24. (The default value is -25 dB)"]
    #[inline(always)]
    #[must_use]
    pub fn adc_drc_lopl(&mut self) -> ADC_DRC_LOPL_W<AC_ADC_DRC_LOPL_SPEC> {
        ADC_DRC_LOPL_W::new(self, 0)
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
#[doc = "ADC DRC Limiter Low Output at Limiter Threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_lopl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_lopl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AC_ADC_DRC_LOPL_SPEC;
impl crate::RegisterSpec for AC_ADC_DRC_LOPL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ac_adc_drc_lopl::R`](R) reader structure"]
impl crate::Readable for AC_ADC_DRC_LOPL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ac_adc_drc_lopl::W`](W) writer structure"]
impl crate::Writable for AC_ADC_DRC_LOPL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ac_adc_drc_lopl to value 0xfba7"]
impl crate::Resettable for AC_ADC_DRC_LOPL_SPEC {
    const RESET_VALUE: Self::Ux = 0xfba7;
}
