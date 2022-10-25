#[doc = "Register `iommu_int_enable` reader"]
pub struct R(crate::R<IOMMU_INT_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOMMU_INT_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOMMU_INT_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOMMU_INT_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `iommu_int_enable` writer"]
pub struct W(crate::W<IOMMU_INT_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOMMU_INT_ENABLE_SPEC>;
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
impl From<crate::W<IOMMU_INT_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOMMU_INT_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `micro_tlb_invalid_en[0-6]` reader - Micro TLB\\[i\\] permission invalid interrupt enable"]
pub type MICRO_TLB_INVALID_EN_R = crate::BitReader<MICRO_TLB_INVALID_EN_A>;
#[doc = "Micro TLB\\[i\\] permission invalid interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MICRO_TLB_INVALID_EN_A {
    #[doc = "0: Mask interrupt"]
    M_ASK = 0,
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
}
impl From<MICRO_TLB_INVALID_EN_A> for bool {
    #[inline(always)]
    fn from(variant: MICRO_TLB_INVALID_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl MICRO_TLB_INVALID_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MICRO_TLB_INVALID_EN_A {
        match self.bits {
            false => MICRO_TLB_INVALID_EN_A::M_ASK,
            true => MICRO_TLB_INVALID_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `M_ASK`"]
    #[inline(always)]
    pub fn is_m_ask(&self) -> bool {
        *self == MICRO_TLB_INVALID_EN_A::M_ASK
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MICRO_TLB_INVALID_EN_A::ENABLE
    }
}
#[doc = "Field `micro_tlb_invalid_en[0-6]` writer - Micro TLB\\[i\\] permission invalid interrupt enable"]
pub type MICRO_TLB_INVALID_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IOMMU_INT_ENABLE_SPEC, MICRO_TLB_INVALID_EN_A, O>;
impl<'a, const O: u8> MICRO_TLB_INVALID_EN_W<'a, O> {
    #[doc = "Mask interrupt"]
    #[inline(always)]
    pub fn m_ask(self) -> &'a mut W {
        self.variant(MICRO_TLB_INVALID_EN_A::M_ASK)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MICRO_TLB_INVALID_EN_A::ENABLE)
    }
}
#[doc = "Field `l_page_table_invalid_en[0-1]` reader - Level\\[i\\] page table invalid interrupt enable"]
pub type L_PAGE_TABLE_INVALID_EN_R = crate::BitReader<L_PAGE_TABLE_INVALID_EN_A>;
#[doc = "Level\\[i\\] page table invalid interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L_PAGE_TABLE_INVALID_EN_A {
    #[doc = "0: Mask interrupt"]
    M_ASK = 0,
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
}
impl From<L_PAGE_TABLE_INVALID_EN_A> for bool {
    #[inline(always)]
    fn from(variant: L_PAGE_TABLE_INVALID_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl L_PAGE_TABLE_INVALID_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L_PAGE_TABLE_INVALID_EN_A {
        match self.bits {
            false => L_PAGE_TABLE_INVALID_EN_A::M_ASK,
            true => L_PAGE_TABLE_INVALID_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `M_ASK`"]
    #[inline(always)]
    pub fn is_m_ask(&self) -> bool {
        *self == L_PAGE_TABLE_INVALID_EN_A::M_ASK
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == L_PAGE_TABLE_INVALID_EN_A::ENABLE
    }
}
#[doc = "Field `l_page_table_invalid_en[0-1]` writer - Level\\[i\\] page table invalid interrupt enable"]
pub type L_PAGE_TABLE_INVALID_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IOMMU_INT_ENABLE_SPEC, L_PAGE_TABLE_INVALID_EN_A, O>;
impl<'a, const O: u8> L_PAGE_TABLE_INVALID_EN_W<'a, O> {
    #[doc = "Mask interrupt"]
    #[inline(always)]
    pub fn m_ask(self) -> &'a mut W {
        self.variant(L_PAGE_TABLE_INVALID_EN_A::M_ASK)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(L_PAGE_TABLE_INVALID_EN_A::ENABLE)
    }
}
#[doc = "Field `dbg_pf_dram_iv_l1_pt_en` reader - Debug or Prefetch DRAM Invalid Level1 Page Table Enable"]
pub type DBG_PF_DRAM_IV_L1_PT_EN_R = crate::BitReader<DBG_PF_DRAM_IV_L1_PT_EN_A>;
#[doc = "Debug or Prefetch DRAM Invalid Level1 Page Table Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_PF_DRAM_IV_L1_PT_EN_A {
    #[doc = "0: Mask interrupt"]
    M_ASK = 0,
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
}
impl From<DBG_PF_DRAM_IV_L1_PT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_PF_DRAM_IV_L1_PT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_PF_DRAM_IV_L1_PT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_PF_DRAM_IV_L1_PT_EN_A {
        match self.bits {
            false => DBG_PF_DRAM_IV_L1_PT_EN_A::M_ASK,
            true => DBG_PF_DRAM_IV_L1_PT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `M_ASK`"]
    #[inline(always)]
    pub fn is_m_ask(&self) -> bool {
        *self == DBG_PF_DRAM_IV_L1_PT_EN_A::M_ASK
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DBG_PF_DRAM_IV_L1_PT_EN_A::ENABLE
    }
}
#[doc = "Field `dbg_pf_dram_iv_l1_pt_en` writer - Debug or Prefetch DRAM Invalid Level1 Page Table Enable"]
pub type DBG_PF_DRAM_IV_L1_PT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IOMMU_INT_ENABLE_SPEC, DBG_PF_DRAM_IV_L1_PT_EN_A, O>;
impl<'a, const O: u8> DBG_PF_DRAM_IV_L1_PT_EN_W<'a, O> {
    #[doc = "Mask interrupt"]
    #[inline(always)]
    pub fn m_ask(self) -> &'a mut W {
        self.variant(DBG_PF_DRAM_IV_L1_PT_EN_A::M_ASK)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DBG_PF_DRAM_IV_L1_PT_EN_A::ENABLE)
    }
}
#[doc = "Field `dbg_pf_pc_iv_l1_pt_en` reader - Debug or Prefetch PTW Cache Invalid Level1 Page Table Enable"]
pub type DBG_PF_PC_IV_L1_PT_EN_R = crate::BitReader<DBG_PF_PC_IV_L1_PT_EN_A>;
#[doc = "Debug or Prefetch PTW Cache Invalid Level1 Page Table Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_PF_PC_IV_L1_PT_EN_A {
    #[doc = "0: Mask interrupt"]
    M_ASK = 0,
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
}
impl From<DBG_PF_PC_IV_L1_PT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_PF_PC_IV_L1_PT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_PF_PC_IV_L1_PT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_PF_PC_IV_L1_PT_EN_A {
        match self.bits {
            false => DBG_PF_PC_IV_L1_PT_EN_A::M_ASK,
            true => DBG_PF_PC_IV_L1_PT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `M_ASK`"]
    #[inline(always)]
    pub fn is_m_ask(&self) -> bool {
        *self == DBG_PF_PC_IV_L1_PT_EN_A::M_ASK
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DBG_PF_PC_IV_L1_PT_EN_A::ENABLE
    }
}
#[doc = "Field `dbg_pf_pc_iv_l1_pt_en` writer - Debug or Prefetch PTW Cache Invalid Level1 Page Table Enable"]
pub type DBG_PF_PC_IV_L1_PT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IOMMU_INT_ENABLE_SPEC, DBG_PF_PC_IV_L1_PT_EN_A, O>;
impl<'a, const O: u8> DBG_PF_PC_IV_L1_PT_EN_W<'a, O> {
    #[doc = "Mask interrupt"]
    #[inline(always)]
    pub fn m_ask(self) -> &'a mut W {
        self.variant(DBG_PF_PC_IV_L1_PT_EN_A::M_ASK)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DBG_PF_PC_IV_L1_PT_EN_A::ENABLE)
    }
}
#[doc = "Field `dbg_pf_l2_iv_pt_en` reader - Debug or Prefetch Invalid Page Table Enable"]
pub type DBG_PF_L2_IV_PT_EN_R = crate::BitReader<DBG_PF_L2_IV_PT_EN_A>;
#[doc = "Debug or Prefetch Invalid Page Table Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_PF_L2_IV_PT_EN_A {
    #[doc = "0: Mask interrupt"]
    M_ASK = 0,
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
}
impl From<DBG_PF_L2_IV_PT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_PF_L2_IV_PT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_PF_L2_IV_PT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_PF_L2_IV_PT_EN_A {
        match self.bits {
            false => DBG_PF_L2_IV_PT_EN_A::M_ASK,
            true => DBG_PF_L2_IV_PT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `M_ASK`"]
    #[inline(always)]
    pub fn is_m_ask(&self) -> bool {
        *self == DBG_PF_L2_IV_PT_EN_A::M_ASK
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DBG_PF_L2_IV_PT_EN_A::ENABLE
    }
}
#[doc = "Field `dbg_pf_l2_iv_pt_en` writer - Debug or Prefetch Invalid Page Table Enable"]
pub type DBG_PF_L2_IV_PT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IOMMU_INT_ENABLE_SPEC, DBG_PF_L2_IV_PT_EN_A, O>;
impl<'a, const O: u8> DBG_PF_L2_IV_PT_EN_W<'a, O> {
    #[doc = "Mask interrupt"]
    #[inline(always)]
    pub fn m_ask(self) -> &'a mut W {
        self.variant(DBG_PF_L2_IV_PT_EN_A::M_ASK)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DBG_PF_L2_IV_PT_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Micro TLB\\[i\\] permission invalid interrupt enable"]
    #[inline(always)]
    pub unsafe fn micro_tlb_invalid_en(&self, n: u8) -> MICRO_TLB_INVALID_EN_R {
        MICRO_TLB_INVALID_EN_R::new(((self.bits >> (n * 2)) & 1) != 0)
    }
    #[doc = "Bit 0 - Micro TLB\\[i\\] permission invalid interrupt enable"]
    #[inline(always)]
    pub fn micro_tlb0_invalid_en(&self) -> MICRO_TLB_INVALID_EN_R {
        MICRO_TLB_INVALID_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Micro TLB\\[i\\] permission invalid interrupt enable"]
    #[inline(always)]
    pub fn micro_tlb1_invalid_en(&self) -> MICRO_TLB_INVALID_EN_R {
        MICRO_TLB_INVALID_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Micro TLB\\[i\\] permission invalid interrupt enable"]
    #[inline(always)]
    pub fn micro_tlb2_invalid_en(&self) -> MICRO_TLB_INVALID_EN_R {
        MICRO_TLB_INVALID_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Micro TLB\\[i\\] permission invalid interrupt enable"]
    #[inline(always)]
    pub fn micro_tlb3_invalid_en(&self) -> MICRO_TLB_INVALID_EN_R {
        MICRO_TLB_INVALID_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Micro TLB\\[i\\] permission invalid interrupt enable"]
    #[inline(always)]
    pub fn micro_tlb4_invalid_en(&self) -> MICRO_TLB_INVALID_EN_R {
        MICRO_TLB_INVALID_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Micro TLB\\[i\\] permission invalid interrupt enable"]
    #[inline(always)]
    pub fn micro_tlb5_invalid_en(&self) -> MICRO_TLB_INVALID_EN_R {
        MICRO_TLB_INVALID_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Micro TLB\\[i\\] permission invalid interrupt enable"]
    #[inline(always)]
    pub fn micro_tlb6_invalid_en(&self) -> MICRO_TLB_INVALID_EN_R {
        MICRO_TLB_INVALID_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Level\\[i\\] page table invalid interrupt enable"]
    #[inline(always)]
    pub unsafe fn l_page_table_invalid_en(&self, n: u8) -> L_PAGE_TABLE_INVALID_EN_R {
        L_PAGE_TABLE_INVALID_EN_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    #[doc = "Bit 16 - Level\\[i\\] page table invalid interrupt enable"]
    #[inline(always)]
    pub fn l0_page_table_invalid_en(&self) -> L_PAGE_TABLE_INVALID_EN_R {
        L_PAGE_TABLE_INVALID_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Level\\[i\\] page table invalid interrupt enable"]
    #[inline(always)]
    pub fn l1_page_table_invalid_en(&self) -> L_PAGE_TABLE_INVALID_EN_R {
        L_PAGE_TABLE_INVALID_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Debug or Prefetch DRAM Invalid Level1 Page Table Enable"]
    #[inline(always)]
    pub fn dbg_pf_dram_iv_l1_pt_en(&self) -> DBG_PF_DRAM_IV_L1_PT_EN_R {
        DBG_PF_DRAM_IV_L1_PT_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Debug or Prefetch PTW Cache Invalid Level1 Page Table Enable"]
    #[inline(always)]
    pub fn dbg_pf_pc_iv_l1_pt_en(&self) -> DBG_PF_PC_IV_L1_PT_EN_R {
        DBG_PF_PC_IV_L1_PT_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Debug or Prefetch Invalid Page Table Enable"]
    #[inline(always)]
    pub fn dbg_pf_l2_iv_pt_en(&self) -> DBG_PF_L2_IV_PT_EN_R {
        DBG_PF_L2_IV_PT_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Micro TLB\\[i\\] permission invalid interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn micro_tlb_invalid_en<const O: u8>(&mut self) -> MICRO_TLB_INVALID_EN_W<O> {
        MICRO_TLB_INVALID_EN_W::new(self)
    }
    #[doc = "Bit 0 - Micro TLB\\[i\\] permission invalid interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn micro_tlb0_invalid_en(&mut self) -> MICRO_TLB_INVALID_EN_W<0> {
        MICRO_TLB_INVALID_EN_W::new(self)
    }
    #[doc = "Bit 2 - Micro TLB\\[i\\] permission invalid interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn micro_tlb1_invalid_en(&mut self) -> MICRO_TLB_INVALID_EN_W<2> {
        MICRO_TLB_INVALID_EN_W::new(self)
    }
    #[doc = "Bit 4 - Micro TLB\\[i\\] permission invalid interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn micro_tlb2_invalid_en(&mut self) -> MICRO_TLB_INVALID_EN_W<4> {
        MICRO_TLB_INVALID_EN_W::new(self)
    }
    #[doc = "Bit 6 - Micro TLB\\[i\\] permission invalid interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn micro_tlb3_invalid_en(&mut self) -> MICRO_TLB_INVALID_EN_W<6> {
        MICRO_TLB_INVALID_EN_W::new(self)
    }
    #[doc = "Bit 8 - Micro TLB\\[i\\] permission invalid interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn micro_tlb4_invalid_en(&mut self) -> MICRO_TLB_INVALID_EN_W<8> {
        MICRO_TLB_INVALID_EN_W::new(self)
    }
    #[doc = "Bit 10 - Micro TLB\\[i\\] permission invalid interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn micro_tlb5_invalid_en(&mut self) -> MICRO_TLB_INVALID_EN_W<10> {
        MICRO_TLB_INVALID_EN_W::new(self)
    }
    #[doc = "Bit 12 - Micro TLB\\[i\\] permission invalid interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn micro_tlb6_invalid_en(&mut self) -> MICRO_TLB_INVALID_EN_W<12> {
        MICRO_TLB_INVALID_EN_W::new(self)
    }
    #[doc = "Level\\[i\\] page table invalid interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn l_page_table_invalid_en<const O: u8>(&mut self) -> L_PAGE_TABLE_INVALID_EN_W<O> {
        L_PAGE_TABLE_INVALID_EN_W::new(self)
    }
    #[doc = "Bit 16 - Level\\[i\\] page table invalid interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn l0_page_table_invalid_en(&mut self) -> L_PAGE_TABLE_INVALID_EN_W<16> {
        L_PAGE_TABLE_INVALID_EN_W::new(self)
    }
    #[doc = "Bit 17 - Level\\[i\\] page table invalid interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn l1_page_table_invalid_en(&mut self) -> L_PAGE_TABLE_INVALID_EN_W<17> {
        L_PAGE_TABLE_INVALID_EN_W::new(self)
    }
    #[doc = "Bit 18 - Debug or Prefetch DRAM Invalid Level1 Page Table Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_pf_dram_iv_l1_pt_en(&mut self) -> DBG_PF_DRAM_IV_L1_PT_EN_W<18> {
        DBG_PF_DRAM_IV_L1_PT_EN_W::new(self)
    }
    #[doc = "Bit 19 - Debug or Prefetch PTW Cache Invalid Level1 Page Table Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_pf_pc_iv_l1_pt_en(&mut self) -> DBG_PF_PC_IV_L1_PT_EN_W<19> {
        DBG_PF_PC_IV_L1_PT_EN_W::new(self)
    }
    #[doc = "Bit 20 - Debug or Prefetch Invalid Page Table Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_pf_l2_iv_pt_en(&mut self) -> DBG_PF_L2_IV_PT_EN_W<20> {
        DBG_PF_L2_IV_PT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IOMMU Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iommu_int_enable](index.html) module"]
pub struct IOMMU_INT_ENABLE_SPEC;
impl crate::RegisterSpec for IOMMU_INT_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iommu_int_enable::R](R) reader structure"]
impl crate::Readable for IOMMU_INT_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iommu_int_enable::W](W) writer structure"]
impl crate::Writable for IOMMU_INT_ENABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iommu_int_enable to value 0"]
impl crate::Resettable for IOMMU_INT_ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
