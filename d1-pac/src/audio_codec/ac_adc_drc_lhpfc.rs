#[doc = "Register `ac_adc_drc_lhpfc` reader"]
pub type R = crate::R<AC_ADC_DRC_LHPFC_SPEC>;
#[doc = "Register `ac_adc_drc_lhpfc` writer"]
pub type W = crate::W<AC_ADC_DRC_LHPFC_SPEC>;
#[doc = "Field `lhpfc` reader - HPF coefficient setting and the data is 3.24 format."]
pub type LHPFC_R = crate::FieldReader<u16>;
#[doc = "Field `lhpfc` writer - HPF coefficient setting and the data is 3.24 format."]
pub type LHPFC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - HPF coefficient setting and the data is 3.24 format."]
    #[inline(always)]
    pub fn lhpfc(&self) -> LHPFC_R {
        LHPFC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - HPF coefficient setting and the data is 3.24 format."]
    #[inline(always)]
    #[must_use]
    pub fn lhpfc(&mut self) -> LHPFC_W<AC_ADC_DRC_LHPFC_SPEC> {
        LHPFC_W::new(self, 0)
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
#[doc = "ADC DRC Low HPF Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_lhpfc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_lhpfc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AC_ADC_DRC_LHPFC_SPEC;
impl crate::RegisterSpec for AC_ADC_DRC_LHPFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ac_adc_drc_lhpfc::R`](R) reader structure"]
impl crate::Readable for AC_ADC_DRC_LHPFC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ac_adc_drc_lhpfc::W`](W) writer structure"]
impl crate::Writable for AC_ADC_DRC_LHPFC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ac_adc_drc_lhpfc to value 0xfac1"]
impl crate::Resettable for AC_ADC_DRC_LHPFC_SPEC {
    const RESET_VALUE: Self::Ux = 0xfac1;
}
