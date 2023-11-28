#[doc = "Register `smhc_ext_resp` reader"]
pub type R = crate::R<SMHC_EXT_RESP_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SMHC_EXT_RESP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Extended Response Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_ext_resp::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMHC_EXT_RESP_SPEC;
impl crate::RegisterSpec for SMHC_EXT_RESP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smhc_ext_resp::R`](R) reader structure"]
impl crate::Readable for SMHC_EXT_RESP_SPEC {}
#[doc = "`reset()` method sets smhc_ext_resp to value 0"]
impl crate::Resettable for SMHC_EXT_RESP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
