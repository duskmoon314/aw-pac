#[doc = "Register `iommu_tlb_ivld_addr` reader"]
pub struct R(crate::R<IOMMU_TLB_IVLD_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOMMU_TLB_IVLD_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOMMU_TLB_IVLD_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOMMU_TLB_IVLD_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `iommu_tlb_ivld_addr` writer"]
pub struct W(crate::W<IOMMU_TLB_IVLD_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOMMU_TLB_IVLD_ADDR_SPEC>;
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
impl From<crate::W<IOMMU_TLB_IVLD_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOMMU_TLB_IVLD_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tlb_ivld_addr` reader - TLB invalid address, 4 KB aligned"]
pub type TLB_IVLD_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `tlb_ivld_addr` writer - TLB invalid address, 4 KB aligned"]
pub type TLB_IVLD_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IOMMU_TLB_IVLD_ADDR_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 12:31 - TLB invalid address, 4 KB aligned"]
    #[inline(always)]
    pub fn tlb_ivld_addr(&self) -> TLB_IVLD_ADDR_R {
        TLB_IVLD_ADDR_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 12:31 - TLB invalid address, 4 KB aligned"]
    #[inline(always)]
    #[must_use]
    pub fn tlb_ivld_addr(&mut self) -> TLB_IVLD_ADDR_W<12> {
        TLB_IVLD_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IOMMU TLB Invalidation Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iommu_tlb_ivld_addr](index.html) module"]
pub struct IOMMU_TLB_IVLD_ADDR_SPEC;
impl crate::RegisterSpec for IOMMU_TLB_IVLD_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iommu_tlb_ivld_addr::R](R) reader structure"]
impl crate::Readable for IOMMU_TLB_IVLD_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iommu_tlb_ivld_addr::W](W) writer structure"]
impl crate::Writable for IOMMU_TLB_IVLD_ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iommu_tlb_ivld_addr to value 0"]
impl crate::Resettable for IOMMU_TLB_IVLD_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
