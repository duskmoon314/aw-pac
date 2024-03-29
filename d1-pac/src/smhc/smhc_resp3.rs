#[doc = "Register `smhc_resp3` reader"]
pub type R = crate::R<SMHC_RESP3_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SMHC_RESP3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Response 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_resp3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMHC_RESP3_SPEC;
impl crate::RegisterSpec for SMHC_RESP3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smhc_resp3::R`](R) reader structure"]
impl crate::Readable for SMHC_RESP3_SPEC {}
#[doc = "`reset()` method sets smhc_resp3 to value 0"]
impl crate::Resettable for SMHC_RESP3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
