#[doc = "Register `iommu_va` reader"]
pub type R = crate::R<IOMMU_VA_SPEC>;
#[doc = "Register `iommu_va` writer"]
pub type W = crate::W<IOMMU_VA_SPEC>;
#[doc = "Field `va` reader - Virtual address of read/write"]
pub type VA_R = crate::FieldReader<u32>;
#[doc = "Field `va` writer - Virtual address of read/write"]
pub type VA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Virtual address of read/write"]
    #[inline(always)]
    pub fn va(&self) -> VA_R {
        VA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Virtual address of read/write"]
    #[inline(always)]
    #[must_use]
    pub fn va(&mut self) -> VA_W<IOMMU_VA_SPEC> {
        VA_W::new(self, 0)
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
#[doc = "IOMMU Virtual Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_va::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_va::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOMMU_VA_SPEC;
impl crate::RegisterSpec for IOMMU_VA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iommu_va::R`](R) reader structure"]
impl crate::Readable for IOMMU_VA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iommu_va::W`](W) writer structure"]
impl crate::Writable for IOMMU_VA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iommu_va to value 0"]
impl crate::Resettable for IOMMU_VA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
