#[doc = "Register `dmac_cur_src%s` reader"]
pub type R = crate::R<DMAC_CUR_SRC_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<DMAC_CUR_SRC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "DMAC Channel Current Source Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_cur_src::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAC_CUR_SRC_SPEC;
impl crate::RegisterSpec for DMAC_CUR_SRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_cur_src::R`](R) reader structure"]
impl crate::Readable for DMAC_CUR_SRC_SPEC {}
#[doc = "`reset()` method sets dmac_cur_src%s to value 0"]
impl crate::Resettable for DMAC_CUR_SRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
