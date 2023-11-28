#[doc = "Register `tve_auto_detection_debounce_setting` reader"]
pub type R = crate::R<TVE_AUTO_DETECTION_DEBOUNCE_SETTING_SPEC>;
#[doc = "Register `tve_auto_detection_debounce_setting` writer"]
pub type W = crate::W<TVE_AUTO_DETECTION_DEBOUNCE_SETTING_SPEC>;
#[doc = "Field `dac0_de_bounce_times` reader - The de_bounce time for hot plug detect function."]
pub type DAC0_DE_BOUNCE_TIMES_R = crate::FieldReader;
#[doc = "Field `dac0_de_bounce_times` writer - The de_bounce time for hot plug detect function."]
pub type DAC0_DE_BOUNCE_TIMES_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dac_test_register` reader - DAC test register."]
pub type DAC_TEST_REGISTER_R = crate::FieldReader<u16>;
#[doc = "Field `dac_test_register` writer - DAC test register."]
pub type DAC_TEST_REGISTER_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:3 - The de_bounce time for hot plug detect function."]
    #[inline(always)]
    pub fn dac0_de_bounce_times(&self) -> DAC0_DE_BOUNCE_TIMES_R {
        DAC0_DE_BOUNCE_TIMES_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:25 - DAC test register."]
    #[inline(always)]
    pub fn dac_test_register(&self) -> DAC_TEST_REGISTER_R {
        DAC_TEST_REGISTER_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - The de_bounce time for hot plug detect function."]
    #[inline(always)]
    #[must_use]
    pub fn dac0_de_bounce_times(
        &mut self,
    ) -> DAC0_DE_BOUNCE_TIMES_W<TVE_AUTO_DETECTION_DEBOUNCE_SETTING_SPEC> {
        DAC0_DE_BOUNCE_TIMES_W::new(self, 0)
    }
    #[doc = "Bits 16:25 - DAC test register."]
    #[inline(always)]
    #[must_use]
    pub fn dac_test_register(
        &mut self,
    ) -> DAC_TEST_REGISTER_W<TVE_AUTO_DETECTION_DEBOUNCE_SETTING_SPEC> {
        DAC_TEST_REGISTER_W::new(self, 16)
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
#[doc = "TV Encoder Auto Detection De-bounce Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_auto_detection_debounce_setting::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_auto_detection_debounce_setting::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TVE_AUTO_DETECTION_DEBOUNCE_SETTING_SPEC;
impl crate::RegisterSpec for TVE_AUTO_DETECTION_DEBOUNCE_SETTING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tve_auto_detection_debounce_setting::R`](R) reader structure"]
impl crate::Readable for TVE_AUTO_DETECTION_DEBOUNCE_SETTING_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tve_auto_detection_debounce_setting::W`](W) writer structure"]
impl crate::Writable for TVE_AUTO_DETECTION_DEBOUNCE_SETTING_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_auto_detection_debounce_setting to value 0"]
impl crate::Resettable for TVE_AUTO_DETECTION_DEBOUNCE_SETTING_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
