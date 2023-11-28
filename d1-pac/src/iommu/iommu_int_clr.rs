#[doc = "Register `iommu_int_clr` reader"]
pub type R = crate::R<IOMMU_INT_CLR_SPEC>;
#[doc = "Register `iommu_int_clr` writer"]
pub type W = crate::W<IOMMU_INT_CLR_SPEC>;
#[doc = "Micro TLB\\[i\\] permission invalid interrupt clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MICRO_TLB_INVALID_CLR_AW {
    #[doc = "0: Invalid operation"]
    INVALID = 0,
    #[doc = "1: Clear interrupt Note: The bit is not used."]
    CLEAR = 1,
}
impl From<MICRO_TLB_INVALID_CLR_AW> for bool {
    #[inline(always)]
    fn from(variant: MICRO_TLB_INVALID_CLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `micro_tlb_invalid_clr[0-6]` writer - Micro TLB\\[i\\] permission invalid interrupt clear bit"]
pub type MICRO_TLB_INVALID_CLR_W<'a, REG> = crate::BitWriter<'a, REG, MICRO_TLB_INVALID_CLR_AW>;
impl<'a, REG> MICRO_TLB_INVALID_CLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Invalid operation"]
    #[inline(always)]
    pub fn invalid(self) -> &'a mut crate::W<REG> {
        self.variant(MICRO_TLB_INVALID_CLR_AW::INVALID)
    }
    #[doc = "Clear interrupt Note: The bit is not used."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(MICRO_TLB_INVALID_CLR_AW::CLEAR)
    }
}
#[doc = "Level\\[i\\] page table invalid interrupt clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L_PAGE_TABLE_INVALID_CLR_AW {
    #[doc = "0: Invalid operation"]
    INVALID = 0,
    #[doc = "1: Clear interrupt"]
    CLEAR = 1,
}
impl From<L_PAGE_TABLE_INVALID_CLR_AW> for bool {
    #[inline(always)]
    fn from(variant: L_PAGE_TABLE_INVALID_CLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `l_page_table_invalid_clr[0-1]` writer - Level\\[i\\] page table invalid interrupt clear bit"]
pub type L_PAGE_TABLE_INVALID_CLR_W<'a, REG> =
    crate::BitWriter<'a, REG, L_PAGE_TABLE_INVALID_CLR_AW>;
impl<'a, REG> L_PAGE_TABLE_INVALID_CLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Invalid operation"]
    #[inline(always)]
    pub fn invalid(self) -> &'a mut crate::W<REG> {
        self.variant(L_PAGE_TABLE_INVALID_CLR_AW::INVALID)
    }
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(L_PAGE_TABLE_INVALID_CLR_AW::CLEAR)
    }
}
impl W {
    #[doc = "Micro TLB\\[i\\] permission invalid interrupt clear bit\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `micro_tlb0_invalid_clr` field"]
    #[inline(always)]
    #[must_use]
    pub fn micro_tlb_invalid_clr(&mut self, n: u8) -> MICRO_TLB_INVALID_CLR_W<IOMMU_INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        MICRO_TLB_INVALID_CLR_W::new(self, n * 2)
    }
    #[doc = "Bit 0 - Micro TLB\\[i\\] permission invalid interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn micro_tlb0_invalid_clr(&mut self) -> MICRO_TLB_INVALID_CLR_W<IOMMU_INT_CLR_SPEC> {
        MICRO_TLB_INVALID_CLR_W::new(self, 0)
    }
    #[doc = "Bit 2 - Micro TLB\\[i\\] permission invalid interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn micro_tlb1_invalid_clr(&mut self) -> MICRO_TLB_INVALID_CLR_W<IOMMU_INT_CLR_SPEC> {
        MICRO_TLB_INVALID_CLR_W::new(self, 2)
    }
    #[doc = "Bit 4 - Micro TLB\\[i\\] permission invalid interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn micro_tlb2_invalid_clr(&mut self) -> MICRO_TLB_INVALID_CLR_W<IOMMU_INT_CLR_SPEC> {
        MICRO_TLB_INVALID_CLR_W::new(self, 4)
    }
    #[doc = "Bit 6 - Micro TLB\\[i\\] permission invalid interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn micro_tlb3_invalid_clr(&mut self) -> MICRO_TLB_INVALID_CLR_W<IOMMU_INT_CLR_SPEC> {
        MICRO_TLB_INVALID_CLR_W::new(self, 6)
    }
    #[doc = "Bit 8 - Micro TLB\\[i\\] permission invalid interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn micro_tlb4_invalid_clr(&mut self) -> MICRO_TLB_INVALID_CLR_W<IOMMU_INT_CLR_SPEC> {
        MICRO_TLB_INVALID_CLR_W::new(self, 8)
    }
    #[doc = "Bit 10 - Micro TLB\\[i\\] permission invalid interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn micro_tlb5_invalid_clr(&mut self) -> MICRO_TLB_INVALID_CLR_W<IOMMU_INT_CLR_SPEC> {
        MICRO_TLB_INVALID_CLR_W::new(self, 10)
    }
    #[doc = "Bit 12 - Micro TLB\\[i\\] permission invalid interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn micro_tlb6_invalid_clr(&mut self) -> MICRO_TLB_INVALID_CLR_W<IOMMU_INT_CLR_SPEC> {
        MICRO_TLB_INVALID_CLR_W::new(self, 12)
    }
    #[doc = "Level\\[i\\] page table invalid interrupt clear bit\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `l0_page_table_invalid_clr` field"]
    #[inline(always)]
    #[must_use]
    pub fn l_page_table_invalid_clr(
        &mut self,
        n: u8,
    ) -> L_PAGE_TABLE_INVALID_CLR_W<IOMMU_INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        L_PAGE_TABLE_INVALID_CLR_W::new(self, n + 16)
    }
    #[doc = "Bit 16 - Level\\[i\\] page table invalid interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn l0_page_table_invalid_clr(&mut self) -> L_PAGE_TABLE_INVALID_CLR_W<IOMMU_INT_CLR_SPEC> {
        L_PAGE_TABLE_INVALID_CLR_W::new(self, 16)
    }
    #[doc = "Bit 17 - Level\\[i\\] page table invalid interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn l1_page_table_invalid_clr(&mut self) -> L_PAGE_TABLE_INVALID_CLR_W<IOMMU_INT_CLR_SPEC> {
        L_PAGE_TABLE_INVALID_CLR_W::new(self, 17)
    }
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
#[doc = "IOMMU Interrupt Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_int_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_int_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOMMU_INT_CLR_SPEC;
impl crate::RegisterSpec for IOMMU_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iommu_int_clr::R`](R) reader structure"]
impl crate::Readable for IOMMU_INT_CLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iommu_int_clr::W`](W) writer structure"]
impl crate::Writable for IOMMU_INT_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iommu_int_clr to value 0"]
impl crate::Resettable for IOMMU_INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
