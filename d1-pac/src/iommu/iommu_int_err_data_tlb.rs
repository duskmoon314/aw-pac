#[doc = "Register `iommu_int_err_data_tlb%s` reader"]
pub type R = crate::R<IOMMU_INT_ERR_DATA_TLB_SPEC>;
#[doc = "Field `int_err_data` reader - Corresponding page table of virtual address that caused Micro TLB\\[i\\] to interrupt"]
pub type INT_ERR_DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Corresponding page table of virtual address that caused Micro TLB\\[i\\] to interrupt"]
    #[inline(always)]
    pub fn int_err_data(&self) -> INT_ERR_DATA_R {
        INT_ERR_DATA_R::new(self.bits)
    }
}
#[doc = "IOMMU Interrupt Error Data \\[i\\] Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_int_err_data_tlb::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOMMU_INT_ERR_DATA_TLB_SPEC;
impl crate::RegisterSpec for IOMMU_INT_ERR_DATA_TLB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iommu_int_err_data_tlb::R`](R) reader structure"]
impl crate::Readable for IOMMU_INT_ERR_DATA_TLB_SPEC {}
#[doc = "`reset()` method sets iommu_int_err_data_tlb%s to value 0"]
impl crate::Resettable for IOMMU_INT_ERR_DATA_TLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
