#[doc = "Register `iommu_l%spg_int` reader"]
pub type R = crate::R<IOMMU_LPG_INT_SPEC>;
#[doc = "Field `master_int[0-6]` reader - Master\\[j\\] address switch causes L\\[i\\] page table to occur interrupt"]
pub type MASTER_INT_R = crate::BitReader;
#[doc = "Field `dbg_mode_int` reader - Debug mode address switch causes L\\[i\\] page table to occur interrupt"]
pub type DBG_MODE_INT_R = crate::BitReader;
impl R {
    #[doc = "Master\\[j\\] address switch causes L\\[i\\] page table to occur interrupt\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `master0_int` field"]
    #[inline(always)]
    pub fn master_int(&self, n: u8) -> MASTER_INT_R {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
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
#[doc = "IOMMU L\\[i\\] Page Table Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_lpg_int::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOMMU_LPG_INT_SPEC;
impl crate::RegisterSpec for IOMMU_LPG_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iommu_lpg_int::R`](R) reader structure"]
impl crate::Readable for IOMMU_LPG_INT_SPEC {}
#[doc = "`reset()` method sets iommu_l%spg_int to value 0"]
impl crate::Resettable for IOMMU_LPG_INT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
