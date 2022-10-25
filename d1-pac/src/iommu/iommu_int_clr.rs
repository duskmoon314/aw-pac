#[doc = "Register `iommu_int_clr` reader"]
pub struct R(crate::R<IOMMU_INT_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOMMU_INT_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOMMU_INT_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOMMU_INT_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `iommu_int_clr` writer"]
pub struct W(crate::W<IOMMU_INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOMMU_INT_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<IOMMU_INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOMMU_INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
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
pub type MICRO_TLB_INVALID_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IOMMU_INT_CLR_SPEC, MICRO_TLB_INVALID_CLR_AW, O>;
impl<'a, const O: u8> MICRO_TLB_INVALID_CLR_W<'a, O> {
    #[doc = "Invalid operation"]
    #[inline(always)]
    pub fn invalid(self) -> &'a mut W {
        self.variant(MICRO_TLB_INVALID_CLR_AW::INVALID)
    }
    #[doc = "Clear interrupt Note: The bit is not used."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
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
pub type L_PAGE_TABLE_INVALID_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IOMMU_INT_CLR_SPEC, L_PAGE_TABLE_INVALID_CLR_AW, O>;
impl<'a, const O: u8> L_PAGE_TABLE_INVALID_CLR_W<'a, O> {
    #[doc = "Invalid operation"]
    #[inline(always)]
    pub fn invalid(self) -> &'a mut W {
        self.variant(L_PAGE_TABLE_INVALID_CLR_AW::INVALID)
    }
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(L_PAGE_TABLE_INVALID_CLR_AW::CLEAR)
    }
}
impl W {
    #[doc = "Micro TLB\\[i\\] permission invalid interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn micro_tlb_invalid_clr<const O: u8>(&mut self) -> MICRO_TLB_INVALID_CLR_W<O> {
        MICRO_TLB_INVALID_CLR_W::new(self)
    }
    #[doc = "Bit 0 - Micro TLB\\[i\\] permission invalid interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn micro_tlb0_invalid_clr(&mut self) -> MICRO_TLB_INVALID_CLR_W<0> {
        MICRO_TLB_INVALID_CLR_W::new(self)
    }
    #[doc = "Bit 2 - Micro TLB\\[i\\] permission invalid interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn micro_tlb1_invalid_clr(&mut self) -> MICRO_TLB_INVALID_CLR_W<2> {
        MICRO_TLB_INVALID_CLR_W::new(self)
    }
    #[doc = "Bit 4 - Micro TLB\\[i\\] permission invalid interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn micro_tlb2_invalid_clr(&mut self) -> MICRO_TLB_INVALID_CLR_W<4> {
        MICRO_TLB_INVALID_CLR_W::new(self)
    }
    #[doc = "Bit 6 - Micro TLB\\[i\\] permission invalid interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn micro_tlb3_invalid_clr(&mut self) -> MICRO_TLB_INVALID_CLR_W<6> {
        MICRO_TLB_INVALID_CLR_W::new(self)
    }
    #[doc = "Bit 8 - Micro TLB\\[i\\] permission invalid interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn micro_tlb4_invalid_clr(&mut self) -> MICRO_TLB_INVALID_CLR_W<8> {
        MICRO_TLB_INVALID_CLR_W::new(self)
    }
    #[doc = "Bit 10 - Micro TLB\\[i\\] permission invalid interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn micro_tlb5_invalid_clr(&mut self) -> MICRO_TLB_INVALID_CLR_W<10> {
        MICRO_TLB_INVALID_CLR_W::new(self)
    }
    #[doc = "Bit 12 - Micro TLB\\[i\\] permission invalid interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn micro_tlb6_invalid_clr(&mut self) -> MICRO_TLB_INVALID_CLR_W<12> {
        MICRO_TLB_INVALID_CLR_W::new(self)
    }
    #[doc = "Level\\[i\\] page table invalid interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn l_page_table_invalid_clr<const O: u8>(
        &mut self,
    ) -> L_PAGE_TABLE_INVALID_CLR_W<O> {
        L_PAGE_TABLE_INVALID_CLR_W::new(self)
    }
    #[doc = "Bit 16 - Level\\[i\\] page table invalid interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn l0_page_table_invalid_clr(&mut self) -> L_PAGE_TABLE_INVALID_CLR_W<16> {
        L_PAGE_TABLE_INVALID_CLR_W::new(self)
    }
    #[doc = "Bit 17 - Level\\[i\\] page table invalid interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn l1_page_table_invalid_clr(&mut self) -> L_PAGE_TABLE_INVALID_CLR_W<17> {
        L_PAGE_TABLE_INVALID_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IOMMU Interrupt Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iommu_int_clr](index.html) module"]
pub struct IOMMU_INT_CLR_SPEC;
impl crate::RegisterSpec for IOMMU_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iommu_int_clr::R](R) reader structure"]
impl crate::Readable for IOMMU_INT_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iommu_int_clr::W](W) writer structure"]
impl crate::Writable for IOMMU_INT_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iommu_int_clr to value 0"]
impl crate::Resettable for IOMMU_INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
