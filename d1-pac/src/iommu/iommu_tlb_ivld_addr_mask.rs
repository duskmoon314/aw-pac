#[doc = "Register `iommu_tlb_ivld_addr_mask` reader"]
pub struct R(crate::R<IOMMU_TLB_IVLD_ADDR_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOMMU_TLB_IVLD_ADDR_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOMMU_TLB_IVLD_ADDR_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOMMU_TLB_IVLD_ADDR_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `iommu_tlb_ivld_addr_mask` writer"]
pub struct W(crate::W<IOMMU_TLB_IVLD_ADDR_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOMMU_TLB_IVLD_ADDR_MASK_SPEC>;
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
impl From<crate::W<IOMMU_TLB_IVLD_ADDR_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOMMU_TLB_IVLD_ADDR_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tlb_ivld_addr_mask` reader - TLB invalid address mask register, 4 KB aligned"]
pub type TLB_IVLD_ADDR_MASK_R = crate::FieldReader<u32, u32>;
#[doc = "Field `tlb_ivld_addr_mask` writer - TLB invalid address mask register, 4 KB aligned"]
pub type TLB_IVLD_ADDR_MASK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IOMMU_TLB_IVLD_ADDR_MASK_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 12:31 - TLB invalid address mask register, 4 KB aligned"]
    #[inline(always)]
    pub fn tlb_ivld_addr_mask(&self) -> TLB_IVLD_ADDR_MASK_R {
        TLB_IVLD_ADDR_MASK_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 12:31 - TLB invalid address mask register, 4 KB aligned"]
    #[inline(always)]
    #[must_use]
    pub fn tlb_ivld_addr_mask(&mut self) -> TLB_IVLD_ADDR_MASK_W<12> {
        TLB_IVLD_ADDR_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IOMMU TLB Invalidation Address Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iommu_tlb_ivld_addr_mask](index.html) module"]
pub struct IOMMU_TLB_IVLD_ADDR_MASK_SPEC;
impl crate::RegisterSpec for IOMMU_TLB_IVLD_ADDR_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iommu_tlb_ivld_addr_mask::R](R) reader structure"]
impl crate::Readable for IOMMU_TLB_IVLD_ADDR_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iommu_tlb_ivld_addr_mask::W](W) writer structure"]
impl crate::Writable for IOMMU_TLB_IVLD_ADDR_MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iommu_tlb_ivld_addr_mask to value 0"]
impl crate::Resettable for IOMMU_TLB_IVLD_ADDR_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
