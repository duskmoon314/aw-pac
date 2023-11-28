#[doc = "Register `mtime` reader"]
pub type R = crate::R<MTIME_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<MTIME_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "MTIME\n\nREF: opensbi\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtime::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTIME_SPEC;
impl crate::RegisterSpec for MTIME_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [`mtime::R`](R) reader structure"]
impl crate::Readable for MTIME_SPEC {}
#[doc = "`reset()` method sets mtime to value 0"]
impl crate::Resettable for MTIME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
