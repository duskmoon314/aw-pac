#[doc = "Register `dmic_data` reader"]
pub type R = crate::R<DMIC_DATA_SPEC>;
#[doc = "Register `dmic_data` writer"]
pub type W = crate::W<DMIC_DATA_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<DMIC_DATA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
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
#[doc = "DMIC Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmic_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmic_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMIC_DATA_SPEC;
impl crate::RegisterSpec for DMIC_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmic_data::R`](R) reader structure"]
impl crate::Readable for DMIC_DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmic_data::W`](W) writer structure"]
impl crate::Writable for DMIC_DATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dmic_data to value 0"]
impl crate::Resettable for DMIC_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
