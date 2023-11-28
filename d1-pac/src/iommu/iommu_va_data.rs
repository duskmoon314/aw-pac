#[doc = "Register `iommu_va_data` reader"]
pub type R = crate::R<IOMMU_VA_DATA_SPEC>;
#[doc = "Register `iommu_va_data` writer"]
pub type W = crate::W<IOMMU_VA_DATA_SPEC>;
#[doc = "Field `va_data` reader - Data corresponding to read/write virtual address"]
pub type VA_DATA_R = crate::FieldReader<u32>;
#[doc = "Field `va_data` writer - Data corresponding to read/write virtual address"]
pub type VA_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data corresponding to read/write virtual address"]
    #[inline(always)]
    pub fn va_data(&self) -> VA_DATA_R {
        VA_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data corresponding to read/write virtual address"]
    #[inline(always)]
    #[must_use]
    pub fn va_data(&mut self) -> VA_DATA_W<IOMMU_VA_DATA_SPEC> {
        VA_DATA_W::new(self, 0)
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
#[doc = "IOMMU Virtual Address Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_va_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_va_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOMMU_VA_DATA_SPEC;
impl crate::RegisterSpec for IOMMU_VA_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iommu_va_data::R`](R) reader structure"]
impl crate::Readable for IOMMU_VA_DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iommu_va_data::W`](W) writer structure"]
impl crate::Writable for IOMMU_VA_DATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iommu_va_data to value 0"]
impl crate::Resettable for IOMMU_VA_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
