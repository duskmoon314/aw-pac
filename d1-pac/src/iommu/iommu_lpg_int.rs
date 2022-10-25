#[doc = "Register `iommu_l%spg_int` reader"]
pub struct R(crate::R<IOMMU_LPG_INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOMMU_LPG_INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOMMU_LPG_INT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOMMU_LPG_INT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `master_int[0-6]` reader - Master\\[j\\] address switch causes L\\[i\\] page table to occur interrupt"]
pub type MASTER_INT_R = crate::BitReader<bool>;
#[doc = "Field `dbg_mode_int` reader - Debug mode address switch causes L\\[i\\] page table to occur interrupt"]
pub type DBG_MODE_INT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Master\\[j\\] address switch causes L\\[i\\] page table to occur interrupt"]
    #[inline(always)]
    pub unsafe fn master_int(&self, n: u8) -> MASTER_INT_R {
        MASTER_INT_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Master\\[j\\] address switch causes L\\[i\\] page table to occur interrupt"]
    #[inline(always)]
    pub fn master0_int(&self) -> MASTER_INT_R {
        MASTER_INT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Master\\[j\\] address switch causes L\\[i\\] page table to occur interrupt"]
    #[inline(always)]
    pub fn master1_int(&self) -> MASTER_INT_R {
        MASTER_INT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master\\[j\\] address switch causes L\\[i\\] page table to occur interrupt"]
    #[inline(always)]
    pub fn master2_int(&self) -> MASTER_INT_R {
        MASTER_INT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Master\\[j\\] address switch causes L\\[i\\] page table to occur interrupt"]
    #[inline(always)]
    pub fn master3_int(&self) -> MASTER_INT_R {
        MASTER_INT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Master\\[j\\] address switch causes L\\[i\\] page table to occur interrupt"]
    #[inline(always)]
    pub fn master4_int(&self) -> MASTER_INT_R {
        MASTER_INT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Master\\[j\\] address switch causes L\\[i\\] page table to occur interrupt"]
    #[inline(always)]
    pub fn master5_int(&self) -> MASTER_INT_R {
        MASTER_INT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Master\\[j\\] address switch causes L\\[i\\] page table to occur interrupt"]
    #[inline(always)]
    pub fn master6_int(&self) -> MASTER_INT_R {
        MASTER_INT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 31 - Debug mode address switch causes L\\[i\\] page table to occur interrupt"]
    #[inline(always)]
    pub fn dbg_mode_int(&self) -> DBG_MODE_INT_R {
        DBG_MODE_INT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "IOMMU L\\[i\\] Page Table Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iommu_lpg_int](index.html) module"]
pub struct IOMMU_LPG_INT_SPEC;
impl crate::RegisterSpec for IOMMU_LPG_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iommu_lpg_int::R](R) reader structure"]
impl crate::Readable for IOMMU_LPG_INT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets iommu_l%spg_int to value 0"]
impl crate::Resettable for IOMMU_LPG_INT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
