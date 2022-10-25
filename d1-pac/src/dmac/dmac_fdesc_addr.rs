#[doc = "Register `dmac_fdesc_addr%s` reader"]
pub struct R(crate::R<DMAC_FDESC_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC_FDESC_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC_FDESC_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC_FDESC_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "DMAC Former Descriptor Address Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_fdesc_addr](index.html) module"]
pub struct DMAC_FDESC_ADDR_SPEC;
impl crate::RegisterSpec for DMAC_FDESC_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac_fdesc_addr::R](R) reader structure"]
impl crate::Readable for DMAC_FDESC_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets dmac_fdesc_addr%s to value 0"]
impl crate::Resettable for DMAC_FDESC_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
