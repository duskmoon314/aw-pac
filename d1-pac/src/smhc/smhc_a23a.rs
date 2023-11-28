#[doc = "Register `smhc_a23a` reader"]
pub type R = crate::R<SMHC_A23A_SPEC>;
#[doc = "Register `smhc_a23a` writer"]
pub type W = crate::W<SMHC_A23A_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SMHC_A23A_SPEC> {
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
#[doc = "Auto Command 23 Argument Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_a23a::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_a23a::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMHC_A23A_SPEC;
impl crate::RegisterSpec for SMHC_A23A_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smhc_a23a::R`](R) reader structure"]
impl crate::Readable for SMHC_A23A_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smhc_a23a::W`](W) writer structure"]
impl crate::Writable for SMHC_A23A_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets smhc_a23a to value 0"]
impl crate::Resettable for SMHC_A23A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
