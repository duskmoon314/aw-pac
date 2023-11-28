#[doc = "Register `dmac_fdesc_addr%s` reader"]
pub type R = crate::R<DMAC_FDESC_ADDR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<DMAC_FDESC_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "DMAC Former Descriptor Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_fdesc_addr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAC_FDESC_ADDR_SPEC;
impl crate::RegisterSpec for DMAC_FDESC_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_fdesc_addr::R`](R) reader structure"]
impl crate::Readable for DMAC_FDESC_ADDR_SPEC {}
#[doc = "`reset()` method sets dmac_fdesc_addr%s to value 0"]
impl crate::Resettable for DMAC_FDESC_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
