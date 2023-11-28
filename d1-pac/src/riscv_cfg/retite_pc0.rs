#[doc = "Register `retite_pc0` reader"]
pub type R = crate::R<RETITE_PC0_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RETITE_PC0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Retire PC0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`retite_pc0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RETITE_PC0_SPEC;
impl crate::RegisterSpec for RETITE_PC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`retite_pc0::R`](R) reader structure"]
impl crate::Readable for RETITE_PC0_SPEC {}
#[doc = "`reset()` method sets retite_pc0 to value 0"]
impl crate::Resettable for RETITE_PC0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
