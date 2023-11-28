#[doc = "Register `smhc_tbc1` reader"]
pub type R = crate::R<SMHC_TBC1_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SMHC_TBC1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Transferred Byte Count between Host Memory and Internal FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_tbc1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMHC_TBC1_SPEC;
impl crate::RegisterSpec for SMHC_TBC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smhc_tbc1::R`](R) reader structure"]
impl crate::Readable for SMHC_TBC1_SPEC {}
#[doc = "`reset()` method sets smhc_tbc1 to value 0"]
impl crate::Resettable for SMHC_TBC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
