#[doc = "Register `iommu_ttb` reader"]
pub type R = crate::R<IOMMU_TTB_SPEC>;
#[doc = "Register `iommu_ttb` writer"]
pub type W = crate::W<IOMMU_TTB_SPEC>;
#[doc = "Field `ttb` reader - Translation Table Base\n\nLevel1 page table starting address, aligned to 16 KB.\n\nWhen operating the register, IOMMU address mapping function must be closed, namely IOMMU_ENABLE is 0; Or Bypass function of all main equipment is set to 1, or no the state of transfer bus commands (such as setting)."]
pub type TTB_R = crate::FieldReader<u32>;
#[doc = "Field `ttb` writer - Translation Table Base\n\nLevel1 page table starting address, aligned to 16 KB.\n\nWhen operating the register, IOMMU address mapping function must be closed, namely IOMMU_ENABLE is 0; Or Bypass function of all main equipment is set to 1, or no the state of transfer bus commands (such as setting)."]
pub type TTB_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 14:31 - Translation Table Base\n\nLevel1 page table starting address, aligned to 16 KB.\n\nWhen operating the register, IOMMU address mapping function must be closed, namely IOMMU_ENABLE is 0; Or Bypass function of all main equipment is set to 1, or no the state of transfer bus commands (such as setting)."]
    #[inline(always)]
    pub fn ttb(&self) -> TTB_R {
        TTB_R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 14:31 - Translation Table Base\n\nLevel1 page table starting address, aligned to 16 KB.\n\nWhen operating the register, IOMMU address mapping function must be closed, namely IOMMU_ENABLE is 0; Or Bypass function of all main equipment is set to 1, or no the state of transfer bus commands (such as setting)."]
    #[inline(always)]
    #[must_use]
    pub fn ttb(&mut self) -> TTB_W<IOMMU_TTB_SPEC> {
        TTB_W::new(self, 14)
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
#[doc = "IOMMU Translation Table Base Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_ttb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_ttb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOMMU_TTB_SPEC;
impl crate::RegisterSpec for IOMMU_TTB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iommu_ttb::R`](R) reader structure"]
impl crate::Readable for IOMMU_TTB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iommu_ttb::W`](W) writer structure"]
impl crate::Writable for IOMMU_TTB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iommu_ttb to value 0"]
impl crate::Resettable for IOMMU_TTB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
