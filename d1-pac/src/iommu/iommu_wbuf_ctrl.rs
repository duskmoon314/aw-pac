#[doc = "Register `iommu_wbuf_ctrl` reader"]
pub type R = crate::R<IOMMU_WBUF_CTRL_SPEC>;
#[doc = "Register `iommu_wbuf_ctrl` writer"]
pub type W = crate::W<IOMMU_WBUF_CTRL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<IOMMU_WBUF_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
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
#[doc = "IOMMU Write Buffer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_wbuf_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_wbuf_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOMMU_WBUF_CTRL_SPEC;
impl crate::RegisterSpec for IOMMU_WBUF_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iommu_wbuf_ctrl::R`](R) reader structure"]
impl crate::Readable for IOMMU_WBUF_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iommu_wbuf_ctrl::W`](W) writer structure"]
impl crate::Writable for IOMMU_WBUF_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iommu_wbuf_ctrl to value 0x7f"]
impl crate::Resettable for IOMMU_WBUF_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x7f;
}
