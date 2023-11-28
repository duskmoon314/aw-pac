#[doc = "Register `iommu_tlb_ivld_addr` reader"]
pub type R = crate::R<IOMMU_TLB_IVLD_ADDR_SPEC>;
#[doc = "Register `iommu_tlb_ivld_addr` writer"]
pub type W = crate::W<IOMMU_TLB_IVLD_ADDR_SPEC>;
#[doc = "Field `tlb_ivld_addr` reader - TLB invalid address, 4 KB aligned"]
pub type TLB_IVLD_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `tlb_ivld_addr` writer - TLB invalid address, 4 KB aligned"]
pub type TLB_IVLD_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
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
    pub fn tlb_ivld_addr(&mut self) -> TLB_IVLD_ADDR_W<IOMMU_TLB_IVLD_ADDR_SPEC> {
        TLB_IVLD_ADDR_W::new(self, 12)
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
#[doc = "IOMMU TLB Invalidation Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_tlb_ivld_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_tlb_ivld_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOMMU_TLB_IVLD_ADDR_SPEC;
impl crate::RegisterSpec for IOMMU_TLB_IVLD_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iommu_tlb_ivld_addr::R`](R) reader structure"]
impl crate::Readable for IOMMU_TLB_IVLD_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iommu_tlb_ivld_addr::W`](W) writer structure"]
impl crate::Writable for IOMMU_TLB_IVLD_ADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iommu_tlb_ivld_addr to value 0"]
impl crate::Resettable for IOMMU_TLB_IVLD_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
