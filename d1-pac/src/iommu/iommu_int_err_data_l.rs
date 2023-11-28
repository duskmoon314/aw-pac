#[doc = "Register `iommu_int_err_data_l%s` reader"]
pub type R = crate::R<IOMMU_INT_ERR_DATA_L_SPEC>;
#[doc = "Field `int_err_data` reader - Corresponding page table of virtual address that caused L\\[i\\] page table to interrupt"]
pub type INT_ERR_DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Corresponding page table of virtual address that caused L\\[i\\] page table to interrupt"]
    #[inline(always)]
    pub fn int_err_data(&self) -> INT_ERR_DATA_R {
        INT_ERR_DATA_R::new(self.bits)
    }
}
#[doc = "IOMMU Interrupt Error Data L\\[i\\] Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_int_err_data_l::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOMMU_INT_ERR_DATA_L_SPEC;
impl crate::RegisterSpec for IOMMU_INT_ERR_DATA_L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iommu_int_err_data_l::R`](R) reader structure"]
impl crate::Readable for IOMMU_INT_ERR_DATA_L_SPEC {}
#[doc = "`reset()` method sets iommu_int_err_data_l%s to value 0"]
impl crate::Resettable for IOMMU_INT_ERR_DATA_L_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
