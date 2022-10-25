#[doc = "Register `iommu_tlb_flush_enable` reader"]
pub struct R(crate::R<IOMMU_TLB_FLUSH_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOMMU_TLB_FLUSH_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOMMU_TLB_FLUSH_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOMMU_TLB_FLUSH_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `iommu_tlb_flush_enable` writer"]
pub struct W(crate::W<IOMMU_TLB_FLUSH_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOMMU_TLB_FLUSH_ENABLE_SPEC>;
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
impl From<crate::W<IOMMU_TLB_FLUSH_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOMMU_TLB_FLUSH_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `mi_tlb_fs[0-6]` reader - Micro TLB\\[i\\] Flush Clear Micro TLB6\n\nAfter the Flush operation is completed, the bit can clear automatically."]
pub type MI_TLB_FS_R = crate::BitReader<MI_TLB_FS_A>;
#[doc = "Micro TLB\\[i\\] Flush Clear Micro TLB6\n\nAfter the Flush operation is completed, the bit can clear automatically.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MI_TLB_FS_A {
    #[doc = "0: No clear operation or clear operation is completed"]
    NO_CLEAR_OR_COMPLETED = 0,
    #[doc = "1: Enable clear operation"]
    ENABLE = 1,
}
impl From<MI_TLB_FS_A> for bool {
    #[inline(always)]
    fn from(variant: MI_TLB_FS_A) -> Self {
        variant as u8 != 0
    }
}
impl MI_TLB_FS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI_TLB_FS_A {
        match self.bits {
            false => MI_TLB_FS_A::NO_CLEAR_OR_COMPLETED,
            true => MI_TLB_FS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR_OR_COMPLETED`"]
    #[inline(always)]
    pub fn is_no_clear_or_completed(&self) -> bool {
        *self == MI_TLB_FS_A::NO_CLEAR_OR_COMPLETED
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MI_TLB_FS_A::ENABLE
    }
}
#[doc = "Field `mi_tlb_fs[0-6]` writer - Micro TLB\\[i\\] Flush Clear Micro TLB6\n\nAfter the Flush operation is completed, the bit can clear automatically."]
pub type MI_TLB_FS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IOMMU_TLB_FLUSH_ENABLE_SPEC, MI_TLB_FS_A, O>;
impl<'a, const O: u8> MI_TLB_FS_W<'a, O> {
    #[doc = "No clear operation or clear operation is completed"]
    #[inline(always)]
    pub fn no_clear_or_completed(self) -> &'a mut W {
        self.variant(MI_TLB_FS_A::NO_CLEAR_OR_COMPLETED)
    }
    #[doc = "Enable clear operation"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MI_TLB_FS_A::ENABLE)
    }
}
#[doc = "Field `ma_tlb_fs` reader - Macro TLB Flush\n\nClear Macro TLB\n\nAfter the Flush operation is completed, the bit can clear automatically."]
pub type MA_TLB_FS_R = crate::BitReader<MA_TLB_FS_A>;
#[doc = "Macro TLB Flush\n\nClear Macro TLB\n\nAfter the Flush operation is completed, the bit can clear automatically.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MA_TLB_FS_A {
    #[doc = "0: No clear operation or clear operation is completed"]
    NO_CLEAR_OR_COMPLETED = 0,
    #[doc = "1: Enable clear operation"]
    ENABLE = 1,
}
impl From<MA_TLB_FS_A> for bool {
    #[inline(always)]
    fn from(variant: MA_TLB_FS_A) -> Self {
        variant as u8 != 0
    }
}
impl MA_TLB_FS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MA_TLB_FS_A {
        match self.bits {
            false => MA_TLB_FS_A::NO_CLEAR_OR_COMPLETED,
            true => MA_TLB_FS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR_OR_COMPLETED`"]
    #[inline(always)]
    pub fn is_no_clear_or_completed(&self) -> bool {
        *self == MA_TLB_FS_A::NO_CLEAR_OR_COMPLETED
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MA_TLB_FS_A::ENABLE
    }
}
#[doc = "Field `ma_tlb_fs` writer - Macro TLB Flush\n\nClear Macro TLB\n\nAfter the Flush operation is completed, the bit can clear automatically."]
pub type MA_TLB_FS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IOMMU_TLB_FLUSH_ENABLE_SPEC, MA_TLB_FS_A, O>;
impl<'a, const O: u8> MA_TLB_FS_W<'a, O> {
    #[doc = "No clear operation or clear operation is completed"]
    #[inline(always)]
    pub fn no_clear_or_completed(self) -> &'a mut W {
        self.variant(MA_TLB_FS_A::NO_CLEAR_OR_COMPLETED)
    }
    #[doc = "Enable clear operation"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MA_TLB_FS_A::ENABLE)
    }
}
#[doc = "Field `pc_fs` reader - PTW Cache Flush Clear PTW Cache\n\nAfter the Flush operation is completed, the bit can clear automatically."]
pub type PC_FS_R = crate::BitReader<PC_FS_A>;
#[doc = "PTW Cache Flush Clear PTW Cache\n\nAfter the Flush operation is completed, the bit can clear automatically.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PC_FS_A {
    #[doc = "0: No clear operation or clear operation is completed"]
    NO_CLEAR_OR_COMPLETED = 0,
    #[doc = "1: Enable clear operation"]
    ENABLE = 1,
}
impl From<PC_FS_A> for bool {
    #[inline(always)]
    fn from(variant: PC_FS_A) -> Self {
        variant as u8 != 0
    }
}
impl PC_FS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PC_FS_A {
        match self.bits {
            false => PC_FS_A::NO_CLEAR_OR_COMPLETED,
            true => PC_FS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR_OR_COMPLETED`"]
    #[inline(always)]
    pub fn is_no_clear_or_completed(&self) -> bool {
        *self == PC_FS_A::NO_CLEAR_OR_COMPLETED
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PC_FS_A::ENABLE
    }
}
#[doc = "Field `pc_fs` writer - PTW Cache Flush Clear PTW Cache\n\nAfter the Flush operation is completed, the bit can clear automatically."]
pub type PC_FS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IOMMU_TLB_FLUSH_ENABLE_SPEC, PC_FS_A, O>;
impl<'a, const O: u8> PC_FS_W<'a, O> {
    #[doc = "No clear operation or clear operation is completed"]
    #[inline(always)]
    pub fn no_clear_or_completed(self) -> &'a mut W {
        self.variant(PC_FS_A::NO_CLEAR_OR_COMPLETED)
    }
    #[doc = "Enable clear operation"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PC_FS_A::ENABLE)
    }
}
impl R {
    #[doc = "Micro TLB\\[i\\] Flush Clear Micro TLB6\n\nAfter the Flush operation is completed, the bit can clear automatically."]
    #[inline(always)]
    pub unsafe fn mi_tlb_fs(&self, n: u8) -> MI_TLB_FS_R {
        MI_TLB_FS_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Micro TLB\\[i\\] Flush Clear Micro TLB6\n\nAfter the Flush operation is completed, the bit can clear automatically."]
    #[inline(always)]
    pub fn mi_tlb0_fs(&self) -> MI_TLB_FS_R {
        MI_TLB_FS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Micro TLB\\[i\\] Flush Clear Micro TLB6\n\nAfter the Flush operation is completed, the bit can clear automatically."]
    #[inline(always)]
    pub fn mi_tlb1_fs(&self) -> MI_TLB_FS_R {
        MI_TLB_FS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Micro TLB\\[i\\] Flush Clear Micro TLB6\n\nAfter the Flush operation is completed, the bit can clear automatically."]
    #[inline(always)]
    pub fn mi_tlb2_fs(&self) -> MI_TLB_FS_R {
        MI_TLB_FS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Micro TLB\\[i\\] Flush Clear Micro TLB6\n\nAfter the Flush operation is completed, the bit can clear automatically."]
    #[inline(always)]
    pub fn mi_tlb3_fs(&self) -> MI_TLB_FS_R {
        MI_TLB_FS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Micro TLB\\[i\\] Flush Clear Micro TLB6\n\nAfter the Flush operation is completed, the bit can clear automatically."]
    #[inline(always)]
    pub fn mi_tlb4_fs(&self) -> MI_TLB_FS_R {
        MI_TLB_FS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Micro TLB\\[i\\] Flush Clear Micro TLB6\n\nAfter the Flush operation is completed, the bit can clear automatically."]
    #[inline(always)]
    pub fn mi_tlb5_fs(&self) -> MI_TLB_FS_R {
        MI_TLB_FS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Micro TLB\\[i\\] Flush Clear Micro TLB6\n\nAfter the Flush operation is completed, the bit can clear automatically."]
    #[inline(always)]
    pub fn mi_tlb6_fs(&self) -> MI_TLB_FS_R {
        MI_TLB_FS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - Macro TLB Flush\n\nClear Macro TLB\n\nAfter the Flush operation is completed, the bit can clear automatically."]
    #[inline(always)]
    pub fn ma_tlb_fs(&self) -> MA_TLB_FS_R {
        MA_TLB_FS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PTW Cache Flush Clear PTW Cache\n\nAfter the Flush operation is completed, the bit can clear automatically."]
    #[inline(always)]
    pub fn pc_fs(&self) -> PC_FS_R {
        PC_FS_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Micro TLB\\[i\\] Flush Clear Micro TLB6\n\nAfter the Flush operation is completed, the bit can clear automatically."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn mi_tlb_fs<const O: u8>(&mut self) -> MI_TLB_FS_W<O> {
        MI_TLB_FS_W::new(self)
    }
    #[doc = "Bit 0 - Micro TLB\\[i\\] Flush Clear Micro TLB6\n\nAfter the Flush operation is completed, the bit can clear automatically."]
    #[inline(always)]
    #[must_use]
    pub fn mi_tlb0_fs(&mut self) -> MI_TLB_FS_W<0> {
        MI_TLB_FS_W::new(self)
    }
    #[doc = "Bit 1 - Micro TLB\\[i\\] Flush Clear Micro TLB6\n\nAfter the Flush operation is completed, the bit can clear automatically."]
    #[inline(always)]
    #[must_use]
    pub fn mi_tlb1_fs(&mut self) -> MI_TLB_FS_W<1> {
        MI_TLB_FS_W::new(self)
    }
    #[doc = "Bit 2 - Micro TLB\\[i\\] Flush Clear Micro TLB6\n\nAfter the Flush operation is completed, the bit can clear automatically."]
    #[inline(always)]
    #[must_use]
    pub fn mi_tlb2_fs(&mut self) -> MI_TLB_FS_W<2> {
        MI_TLB_FS_W::new(self)
    }
    #[doc = "Bit 3 - Micro TLB\\[i\\] Flush Clear Micro TLB6\n\nAfter the Flush operation is completed, the bit can clear automatically."]
    #[inline(always)]
    #[must_use]
    pub fn mi_tlb3_fs(&mut self) -> MI_TLB_FS_W<3> {
        MI_TLB_FS_W::new(self)
    }
    #[doc = "Bit 4 - Micro TLB\\[i\\] Flush Clear Micro TLB6\n\nAfter the Flush operation is completed, the bit can clear automatically."]
    #[inline(always)]
    #[must_use]
    pub fn mi_tlb4_fs(&mut self) -> MI_TLB_FS_W<4> {
        MI_TLB_FS_W::new(self)
    }
    #[doc = "Bit 5 - Micro TLB\\[i\\] Flush Clear Micro TLB6\n\nAfter the Flush operation is completed, the bit can clear automatically."]
    #[inline(always)]
    #[must_use]
    pub fn mi_tlb5_fs(&mut self) -> MI_TLB_FS_W<5> {
        MI_TLB_FS_W::new(self)
    }
    #[doc = "Bit 6 - Micro TLB\\[i\\] Flush Clear Micro TLB6\n\nAfter the Flush operation is completed, the bit can clear automatically."]
    #[inline(always)]
    #[must_use]
    pub fn mi_tlb6_fs(&mut self) -> MI_TLB_FS_W<6> {
        MI_TLB_FS_W::new(self)
    }
    #[doc = "Bit 16 - Macro TLB Flush\n\nClear Macro TLB\n\nAfter the Flush operation is completed, the bit can clear automatically."]
    #[inline(always)]
    #[must_use]
    pub fn ma_tlb_fs(&mut self) -> MA_TLB_FS_W<16> {
        MA_TLB_FS_W::new(self)
    }
    #[doc = "Bit 17 - PTW Cache Flush Clear PTW Cache\n\nAfter the Flush operation is completed, the bit can clear automatically."]
    #[inline(always)]
    #[must_use]
    pub fn pc_fs(&mut self) -> PC_FS_W<17> {
        PC_FS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IOMMU TLB Flush Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iommu_tlb_flush_enable](index.html) module"]
pub struct IOMMU_TLB_FLUSH_ENABLE_SPEC;
impl crate::RegisterSpec for IOMMU_TLB_FLUSH_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iommu_tlb_flush_enable::R](R) reader structure"]
impl crate::Readable for IOMMU_TLB_FLUSH_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iommu_tlb_flush_enable::W](W) writer structure"]
impl crate::Writable for IOMMU_TLB_FLUSH_ENABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iommu_tlb_flush_enable to value 0"]
impl crate::Resettable for IOMMU_TLB_FLUSH_ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
