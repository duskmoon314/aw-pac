#[doc = "Register `iommu_int_sta` reader"]
pub type R = crate::R<IOMMU_INT_STA_SPEC>;
#[doc = "Register `iommu_int_sta` writer"]
pub type W = crate::W<IOMMU_INT_STA_SPEC>;
#[doc = "Field `micro_tlb_invalid_sta[0-6]` reader - Micro TLB\\[i\\] permission invalid interrupt status bit"]
pub type MICRO_TLB_INVALID_STA_R = crate::BitReader<MICRO_TLB_INVALID_STA_A>;
#[doc = "Micro TLB\\[i\\] permission invalid interrupt status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MICRO_TLB_INVALID_STA_A {
    #[doc = "0: Interrupt does not happen or interrupt is cleared"]
    NOT_HAPPEN_OR_CLEARED = 0,
    #[doc = "1: Interrupt happens"]
    HAPPENS = 1,
}
impl From<MICRO_TLB_INVALID_STA_A> for bool {
    #[inline(always)]
    fn from(variant: MICRO_TLB_INVALID_STA_A) -> Self {
        variant as u8 != 0
    }
}
impl MICRO_TLB_INVALID_STA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MICRO_TLB_INVALID_STA_A {
        match self.bits {
            false => MICRO_TLB_INVALID_STA_A::NOT_HAPPEN_OR_CLEARED,
            true => MICRO_TLB_INVALID_STA_A::HAPPENS,
        }
    }
    #[doc = "Interrupt does not happen or interrupt is cleared"]
    #[inline(always)]
    pub fn is_not_happen_or_cleared(&self) -> bool {
        *self == MICRO_TLB_INVALID_STA_A::NOT_HAPPEN_OR_CLEARED
    }
    #[doc = "Interrupt happens"]
    #[inline(always)]
    pub fn is_happens(&self) -> bool {
        *self == MICRO_TLB_INVALID_STA_A::HAPPENS
    }
}
#[doc = "Field `l_page_table_invalid_sta[0-1]` reader - Level\\[i\\] page table invalid interrupt status bit"]
pub type L_PAGE_TABLE_INVALID_STA_R = crate::BitReader<L_PAGE_TABLE_INVALID_STA_A>;
#[doc = "Level\\[i\\] page table invalid interrupt status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L_PAGE_TABLE_INVALID_STA_A {
    #[doc = "0: Interrupt does not happen or interrupt is cleared"]
    NOT_HAPPEN_OR_CLEARED = 0,
    #[doc = "1: Interrupt happens"]
    HAPPENS = 1,
}
impl From<L_PAGE_TABLE_INVALID_STA_A> for bool {
    #[inline(always)]
    fn from(variant: L_PAGE_TABLE_INVALID_STA_A) -> Self {
        variant as u8 != 0
    }
}
impl L_PAGE_TABLE_INVALID_STA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> L_PAGE_TABLE_INVALID_STA_A {
        match self.bits {
            false => L_PAGE_TABLE_INVALID_STA_A::NOT_HAPPEN_OR_CLEARED,
            true => L_PAGE_TABLE_INVALID_STA_A::HAPPENS,
        }
    }
    #[doc = "Interrupt does not happen or interrupt is cleared"]
    #[inline(always)]
    pub fn is_not_happen_or_cleared(&self) -> bool {
        *self == L_PAGE_TABLE_INVALID_STA_A::NOT_HAPPEN_OR_CLEARED
    }
    #[doc = "Interrupt happens"]
    #[inline(always)]
    pub fn is_happens(&self) -> bool {
        *self == L_PAGE_TABLE_INVALID_STA_A::HAPPENS
    }
}
impl R {
    #[doc = "Micro TLB\\[i\\] permission invalid interrupt status bit\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `micro_tlb0_invalid_sta` field"]
    #[inline(always)]
    pub fn micro_tlb_invalid_sta(&self, n: u8) -> MICRO_TLB_INVALID_STA_R {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        MICRO_TLB_INVALID_STA_R::new(((self.bits >> (n * 2)) & 1) != 0)
    }
    #[doc = "Bit 0 - Micro TLB\\[i\\] permission invalid interrupt status bit"]
    #[inline(always)]
    pub fn micro_tlb0_invalid_sta(&self) -> MICRO_TLB_INVALID_STA_R {
        MICRO_TLB_INVALID_STA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Micro TLB\\[i\\] permission invalid interrupt status bit"]
    #[inline(always)]
    pub fn micro_tlb1_invalid_sta(&self) -> MICRO_TLB_INVALID_STA_R {
        MICRO_TLB_INVALID_STA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Micro TLB\\[i\\] permission invalid interrupt status bit"]
    #[inline(always)]
    pub fn micro_tlb2_invalid_sta(&self) -> MICRO_TLB_INVALID_STA_R {
        MICRO_TLB_INVALID_STA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Micro TLB\\[i\\] permission invalid interrupt status bit"]
    #[inline(always)]
    pub fn micro_tlb3_invalid_sta(&self) -> MICRO_TLB_INVALID_STA_R {
        MICRO_TLB_INVALID_STA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Micro TLB\\[i\\] permission invalid interrupt status bit"]
    #[inline(always)]
    pub fn micro_tlb4_invalid_sta(&self) -> MICRO_TLB_INVALID_STA_R {
        MICRO_TLB_INVALID_STA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Micro TLB\\[i\\] permission invalid interrupt status bit"]
    #[inline(always)]
    pub fn micro_tlb5_invalid_sta(&self) -> MICRO_TLB_INVALID_STA_R {
        MICRO_TLB_INVALID_STA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Micro TLB\\[i\\] permission invalid interrupt status bit"]
    #[inline(always)]
    pub fn micro_tlb6_invalid_sta(&self) -> MICRO_TLB_INVALID_STA_R {
        MICRO_TLB_INVALID_STA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Level\\[i\\] page table invalid interrupt status bit\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `l0_page_table_invalid_sta` field"]
    #[inline(always)]
    pub fn l_page_table_invalid_sta(&self, n: u8) -> L_PAGE_TABLE_INVALID_STA_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        L_PAGE_TABLE_INVALID_STA_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    #[doc = "Bit 16 - Level\\[i\\] page table invalid interrupt status bit"]
    #[inline(always)]
    pub fn l0_page_table_invalid_sta(&self) -> L_PAGE_TABLE_INVALID_STA_R {
        L_PAGE_TABLE_INVALID_STA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Level\\[i\\] page table invalid interrupt status bit"]
    #[inline(always)]
    pub fn l1_page_table_invalid_sta(&self) -> L_PAGE_TABLE_INVALID_STA_R {
        L_PAGE_TABLE_INVALID_STA_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "IOMMU Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_int_sta::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_int_sta::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOMMU_INT_STA_SPEC;
impl crate::RegisterSpec for IOMMU_INT_STA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iommu_int_sta::R`](R) reader structure"]
impl crate::Readable for IOMMU_INT_STA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iommu_int_sta::W`](W) writer structure"]
impl crate::Writable for IOMMU_INT_STA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iommu_int_sta to value 0"]
impl crate::Resettable for IOMMU_INT_STA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
