#[doc = "Register `iommu_tlb_ivld_enable` reader"]
pub struct R(crate::R<IOMMU_TLB_IVLD_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOMMU_TLB_IVLD_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOMMU_TLB_IVLD_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOMMU_TLB_IVLD_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `iommu_tlb_ivld_enable` writer"]
pub struct W(crate::W<IOMMU_TLB_IVLD_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOMMU_TLB_IVLD_ENABLE_SPEC>;
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
impl From<crate::W<IOMMU_TLB_IVLD_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOMMU_TLB_IVLD_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tlb_ivld_enable` reader - Enable TLB invalidation operation\n\nAfter invalidation operation is completed, the bit can clear automatically.\n\nWhen operating invalidation operation, TLB/Cache operation has not affected.\n\nAfter or before invalidation operation starts, there is no absolute relationship between the same address switch operation and invalidation operation."]
pub type TLB_IVLD_ENABLE_R = crate::BitReader<TLB_IVLD_ENABLE_A>;
#[doc = "Enable TLB invalidation operation\n\nAfter invalidation operation is completed, the bit can clear automatically.\n\nWhen operating invalidation operation, TLB/Cache operation has not affected.\n\nAfter or before invalidation operation starts, there is no absolute relationship between the same address switch operation and invalidation operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TLB_IVLD_ENABLE_A {
    #[doc = "0: No operation or operation is completed"]
    NO_OPERATION_OR_COMPLETED = 0,
    #[doc = "1: Enable invalidation operation"]
    ENABLE = 1,
}
impl From<TLB_IVLD_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: TLB_IVLD_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl TLB_IVLD_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TLB_IVLD_ENABLE_A {
        match self.bits {
            false => TLB_IVLD_ENABLE_A::NO_OPERATION_OR_COMPLETED,
            true => TLB_IVLD_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_OPERATION_OR_COMPLETED`"]
    #[inline(always)]
    pub fn is_no_operation_or_completed(&self) -> bool {
        *self == TLB_IVLD_ENABLE_A::NO_OPERATION_OR_COMPLETED
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TLB_IVLD_ENABLE_A::ENABLE
    }
}
#[doc = "Field `tlb_ivld_enable` writer - Enable TLB invalidation operation\n\nAfter invalidation operation is completed, the bit can clear automatically.\n\nWhen operating invalidation operation, TLB/Cache operation has not affected.\n\nAfter or before invalidation operation starts, there is no absolute relationship between the same address switch operation and invalidation operation."]
pub type TLB_IVLD_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IOMMU_TLB_IVLD_ENABLE_SPEC, TLB_IVLD_ENABLE_A, O>;
impl<'a, const O: u8> TLB_IVLD_ENABLE_W<'a, O> {
    #[doc = "No operation or operation is completed"]
    #[inline(always)]
    pub fn no_operation_or_completed(self) -> &'a mut W {
        self.variant(TLB_IVLD_ENABLE_A::NO_OPERATION_OR_COMPLETED)
    }
    #[doc = "Enable invalidation operation"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TLB_IVLD_ENABLE_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Enable TLB invalidation operation\n\nAfter invalidation operation is completed, the bit can clear automatically.\n\nWhen operating invalidation operation, TLB/Cache operation has not affected.\n\nAfter or before invalidation operation starts, there is no absolute relationship between the same address switch operation and invalidation operation."]
    #[inline(always)]
    pub fn tlb_ivld_enable(&self) -> TLB_IVLD_ENABLE_R {
        TLB_IVLD_ENABLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable TLB invalidation operation\n\nAfter invalidation operation is completed, the bit can clear automatically.\n\nWhen operating invalidation operation, TLB/Cache operation has not affected.\n\nAfter or before invalidation operation starts, there is no absolute relationship between the same address switch operation and invalidation operation."]
    #[inline(always)]
    #[must_use]
    pub fn tlb_ivld_enable(&mut self) -> TLB_IVLD_ENABLE_W<0> {
        TLB_IVLD_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IOMMU TLB Invalidation Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iommu_tlb_ivld_enable](index.html) module"]
pub struct IOMMU_TLB_IVLD_ENABLE_SPEC;
impl crate::RegisterSpec for IOMMU_TLB_IVLD_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iommu_tlb_ivld_enable::R](R) reader structure"]
impl crate::Readable for IOMMU_TLB_IVLD_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iommu_tlb_ivld_enable::W](W) writer structure"]
impl crate::Writable for IOMMU_TLB_IVLD_ENABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iommu_tlb_ivld_enable to value 0"]
impl crate::Resettable for IOMMU_TLB_IVLD_ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
