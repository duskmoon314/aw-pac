#[doc = "Register `owa_fctl` reader"]
pub type R = crate::R<OWA_FCTL_SPEC>;
#[doc = "Register `owa_fctl` writer"]
pub type W = crate::W<OWA_FCTL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<OWA_FCTL_SPEC> {
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
#[doc = "OWA FIFO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`owa_fctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`owa_fctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OWA_FCTL_SPEC;
impl crate::RegisterSpec for OWA_FCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`owa_fctl::R`](R) reader structure"]
impl crate::Readable for OWA_FCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`owa_fctl::W`](W) writer structure"]
impl crate::Writable for OWA_FCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets owa_fctl to value 0"]
impl crate::Resettable for OWA_FCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
