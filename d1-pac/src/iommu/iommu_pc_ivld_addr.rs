#[doc = "Register `iommu_pc_ivld_addr` reader"]
pub type R = crate::R<IOMMU_PC_IVLD_ADDR_SPEC>;
#[doc = "Register `iommu_pc_ivld_addr` writer"]
pub type W = crate::W<IOMMU_PC_IVLD_ADDR_SPEC>;
#[doc = "Field `pc_ivld_addr` reader - PTW Cache invalid address, 1 MB aligned."]
pub type PC_IVLD_ADDR_R = crate::FieldReader<u16>;
#[doc = "Field `pc_ivld_addr` writer - PTW Cache invalid address, 1 MB aligned."]
pub type PC_IVLD_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 20:31 - PTW Cache invalid address, 1 MB aligned."]
    #[inline(always)]
    pub fn pc_ivld_addr(&self) -> PC_IVLD_ADDR_R {
        PC_IVLD_ADDR_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 20:31 - PTW Cache invalid address, 1 MB aligned."]
    #[inline(always)]
    #[must_use]
    pub fn pc_ivld_addr(&mut self) -> PC_IVLD_ADDR_W<IOMMU_PC_IVLD_ADDR_SPEC> {
        PC_IVLD_ADDR_W::new(self, 20)
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
#[doc = "IOMMU PC Invalidation Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_pc_ivld_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_pc_ivld_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOMMU_PC_IVLD_ADDR_SPEC;
impl crate::RegisterSpec for IOMMU_PC_IVLD_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iommu_pc_ivld_addr::R`](R) reader structure"]
impl crate::Readable for IOMMU_PC_IVLD_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iommu_pc_ivld_addr::W`](W) writer structure"]
impl crate::Writable for IOMMU_PC_IVLD_ADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iommu_pc_ivld_addr to value 0"]
impl crate::Resettable for IOMMU_PC_IVLD_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
