#[doc = "Register `iommu_tlb_prefetch` reader"]
pub struct R(crate::R<IOMMU_TLB_PREFETCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOMMU_TLB_PREFETCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOMMU_TLB_PREFETCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOMMU_TLB_PREFETCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `iommu_tlb_prefetch` writer"]
pub struct W(crate::W<IOMMU_TLB_PREFETCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOMMU_TLB_PREFETCH_SPEC>;
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
impl From<crate::W<IOMMU_TLB_PREFETCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOMMU_TLB_PREFETCH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `mi_tlb_pf[0-6]` reader - Micro TLB6 prefetch enable"]
pub type MI_TLB_PF_R = crate::BitReader<MI_TLB_PF_A>;
#[doc = "Micro TLB6 prefetch enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MI_TLB_PF_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<MI_TLB_PF_A> for bool {
    #[inline(always)]
    fn from(variant: MI_TLB_PF_A) -> Self {
        variant as u8 != 0
    }
}
impl MI_TLB_PF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI_TLB_PF_A {
        match self.bits {
            false => MI_TLB_PF_A::DISABLE,
            true => MI_TLB_PF_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MI_TLB_PF_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MI_TLB_PF_A::ENABLE
    }
}
#[doc = "Field `mi_tlb_pf[0-6]` writer - Micro TLB6 prefetch enable"]
pub type MI_TLB_PF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IOMMU_TLB_PREFETCH_SPEC, MI_TLB_PF_A, O>;
impl<'a, const O: u8> MI_TLB_PF_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MI_TLB_PF_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MI_TLB_PF_A::ENABLE)
    }
}
#[doc = "Field `pf_vl_pt_to_mt` reader - Prefetch Value Pagetable to Macro TLB\n\nIf the function is enabled, the prefetch function will not update the invalid Level2 page table to Macro TLB."]
pub type PF_VL_PT_TO_MT_R = crate::BitReader<PF_VL_PT_TO_MT_A>;
#[doc = "Prefetch Value Pagetable to Macro TLB\n\nIf the function is enabled, the prefetch function will not update the invalid Level2 page table to Macro TLB.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PF_VL_PT_TO_MT_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable If the function is enabled, the prefetch function will not update the invalid Level2 page table to Macro TLB."]
    ENABLE = 1,
}
impl From<PF_VL_PT_TO_MT_A> for bool {
    #[inline(always)]
    fn from(variant: PF_VL_PT_TO_MT_A) -> Self {
        variant as u8 != 0
    }
}
impl PF_VL_PT_TO_MT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PF_VL_PT_TO_MT_A {
        match self.bits {
            false => PF_VL_PT_TO_MT_A::DISABLE,
            true => PF_VL_PT_TO_MT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PF_VL_PT_TO_MT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PF_VL_PT_TO_MT_A::ENABLE
    }
}
#[doc = "Field `pf_vl_pt_to_mt` writer - Prefetch Value Pagetable to Macro TLB\n\nIf the function is enabled, the prefetch function will not update the invalid Level2 page table to Macro TLB."]
pub type PF_VL_PT_TO_MT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IOMMU_TLB_PREFETCH_SPEC, PF_VL_PT_TO_MT_A, O>;
impl<'a, const O: u8> PF_VL_PT_TO_MT_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PF_VL_PT_TO_MT_A::DISABLE)
    }
    #[doc = "Enable If the function is enabled, the prefetch function will not update the invalid Level2 page table to Macro TLB."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PF_VL_PT_TO_MT_A::ENABLE)
    }
}
#[doc = "Field `pf_vl_pt_to_pc` reader - Prefetch Value Pagetable to PTW Cache\n\nIf the function is enabled, the prefetch function will not update the invalid Level1 page table to PTW cache."]
pub type PF_VL_PT_TO_PC_R = crate::BitReader<PF_VL_PT_TO_PC_A>;
#[doc = "Prefetch Value Pagetable to PTW Cache\n\nIf the function is enabled, the prefetch function will not update the invalid Level1 page table to PTW cache.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PF_VL_PT_TO_PC_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable If the function is enabled, the prefetch function will not update the invalid Level1 page table to PTW cache."]
    ENABLE = 1,
}
impl From<PF_VL_PT_TO_PC_A> for bool {
    #[inline(always)]
    fn from(variant: PF_VL_PT_TO_PC_A) -> Self {
        variant as u8 != 0
    }
}
impl PF_VL_PT_TO_PC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PF_VL_PT_TO_PC_A {
        match self.bits {
            false => PF_VL_PT_TO_PC_A::DISABLE,
            true => PF_VL_PT_TO_PC_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PF_VL_PT_TO_PC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PF_VL_PT_TO_PC_A::ENABLE
    }
}
#[doc = "Field `pf_vl_pt_to_pc` writer - Prefetch Value Pagetable to PTW Cache\n\nIf the function is enabled, the prefetch function will not update the invalid Level1 page table to PTW cache."]
pub type PF_VL_PT_TO_PC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IOMMU_TLB_PREFETCH_SPEC, PF_VL_PT_TO_PC_A, O>;
impl<'a, const O: u8> PF_VL_PT_TO_PC_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PF_VL_PT_TO_PC_A::DISABLE)
    }
    #[doc = "Enable If the function is enabled, the prefetch function will not update the invalid Level1 page table to PTW cache."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PF_VL_PT_TO_PC_A::ENABLE)
    }
}
impl R {
    #[doc = "Micro TLB6 prefetch enable"]
    #[inline(always)]
    pub unsafe fn mi_tlb_pf(&self, n: u8) -> MI_TLB_PF_R {
        MI_TLB_PF_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Micro TLB6 prefetch enable"]
    #[inline(always)]
    pub fn mi_tlb0_pf(&self) -> MI_TLB_PF_R {
        MI_TLB_PF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Micro TLB6 prefetch enable"]
    #[inline(always)]
    pub fn mi_tlb1_pf(&self) -> MI_TLB_PF_R {
        MI_TLB_PF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Micro TLB6 prefetch enable"]
    #[inline(always)]
    pub fn mi_tlb2_pf(&self) -> MI_TLB_PF_R {
        MI_TLB_PF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Micro TLB6 prefetch enable"]
    #[inline(always)]
    pub fn mi_tlb3_pf(&self) -> MI_TLB_PF_R {
        MI_TLB_PF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Micro TLB6 prefetch enable"]
    #[inline(always)]
    pub fn mi_tlb4_pf(&self) -> MI_TLB_PF_R {
        MI_TLB_PF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Micro TLB6 prefetch enable"]
    #[inline(always)]
    pub fn mi_tlb5_pf(&self) -> MI_TLB_PF_R {
        MI_TLB_PF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Micro TLB6 prefetch enable"]
    #[inline(always)]
    pub fn mi_tlb6_pf(&self) -> MI_TLB_PF_R {
        MI_TLB_PF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - Prefetch Value Pagetable to Macro TLB\n\nIf the function is enabled, the prefetch function will not update the invalid Level2 page table to Macro TLB."]
    #[inline(always)]
    pub fn pf_vl_pt_to_mt(&self) -> PF_VL_PT_TO_MT_R {
        PF_VL_PT_TO_MT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Prefetch Value Pagetable to PTW Cache\n\nIf the function is enabled, the prefetch function will not update the invalid Level1 page table to PTW cache."]
    #[inline(always)]
    pub fn pf_vl_pt_to_pc(&self) -> PF_VL_PT_TO_PC_R {
        PF_VL_PT_TO_PC_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Micro TLB6 prefetch enable"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn mi_tlb_pf<const O: u8>(&mut self) -> MI_TLB_PF_W<O> {
        MI_TLB_PF_W::new(self)
    }
    #[doc = "Bit 0 - Micro TLB6 prefetch enable"]
    #[inline(always)]
    #[must_use]
    pub fn mi_tlb0_pf(&mut self) -> MI_TLB_PF_W<0> {
        MI_TLB_PF_W::new(self)
    }
    #[doc = "Bit 1 - Micro TLB6 prefetch enable"]
    #[inline(always)]
    #[must_use]
    pub fn mi_tlb1_pf(&mut self) -> MI_TLB_PF_W<1> {
        MI_TLB_PF_W::new(self)
    }
    #[doc = "Bit 2 - Micro TLB6 prefetch enable"]
    #[inline(always)]
    #[must_use]
    pub fn mi_tlb2_pf(&mut self) -> MI_TLB_PF_W<2> {
        MI_TLB_PF_W::new(self)
    }
    #[doc = "Bit 3 - Micro TLB6 prefetch enable"]
    #[inline(always)]
    #[must_use]
    pub fn mi_tlb3_pf(&mut self) -> MI_TLB_PF_W<3> {
        MI_TLB_PF_W::new(self)
    }
    #[doc = "Bit 4 - Micro TLB6 prefetch enable"]
    #[inline(always)]
    #[must_use]
    pub fn mi_tlb4_pf(&mut self) -> MI_TLB_PF_W<4> {
        MI_TLB_PF_W::new(self)
    }
    #[doc = "Bit 5 - Micro TLB6 prefetch enable"]
    #[inline(always)]
    #[must_use]
    pub fn mi_tlb5_pf(&mut self) -> MI_TLB_PF_W<5> {
        MI_TLB_PF_W::new(self)
    }
    #[doc = "Bit 6 - Micro TLB6 prefetch enable"]
    #[inline(always)]
    #[must_use]
    pub fn mi_tlb6_pf(&mut self) -> MI_TLB_PF_W<6> {
        MI_TLB_PF_W::new(self)
    }
    #[doc = "Bit 16 - Prefetch Value Pagetable to Macro TLB\n\nIf the function is enabled, the prefetch function will not update the invalid Level2 page table to Macro TLB."]
    #[inline(always)]
    #[must_use]
    pub fn pf_vl_pt_to_mt(&mut self) -> PF_VL_PT_TO_MT_W<16> {
        PF_VL_PT_TO_MT_W::new(self)
    }
    #[doc = "Bit 17 - Prefetch Value Pagetable to PTW Cache\n\nIf the function is enabled, the prefetch function will not update the invalid Level1 page table to PTW cache."]
    #[inline(always)]
    #[must_use]
    pub fn pf_vl_pt_to_pc(&mut self) -> PF_VL_PT_TO_PC_W<17> {
        PF_VL_PT_TO_PC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IOMMU TLB Prefetch Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iommu_tlb_prefetch](index.html) module"]
pub struct IOMMU_TLB_PREFETCH_SPEC;
impl crate::RegisterSpec for IOMMU_TLB_PREFETCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iommu_tlb_prefetch::R](R) reader structure"]
impl crate::Readable for IOMMU_TLB_PREFETCH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iommu_tlb_prefetch::W](W) writer structure"]
impl crate::Writable for IOMMU_TLB_PREFETCH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iommu_tlb_prefetch to value 0x0003_0000"]
impl crate::Resettable for IOMMU_TLB_PREFETCH_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_0000;
}
