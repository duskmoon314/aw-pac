#[doc = "Register `ledc_data` writer"]
pub type W = crate::W<LEDC_DATA_SPEC>;
impl core::fmt::Debug for crate::generic::Reg<LEDC_DATA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
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
#[doc = "LEDC Data Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ledc_data::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LEDC_DATA_SPEC;
impl crate::RegisterSpec for LEDC_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ledc_data::W`](W) writer structure"]
impl crate::Writable for LEDC_DATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ledc_data to value 0"]
impl crate::Resettable for LEDC_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
