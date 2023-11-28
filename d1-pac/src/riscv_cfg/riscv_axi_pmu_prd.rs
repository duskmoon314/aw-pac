#[doc = "Register `riscv_axi_pmu_prd` reader"]
pub type R = crate::R<RISCV_AXI_PMU_PRD_SPEC>;
#[doc = "Register `riscv_axi_pmu_prd` writer"]
pub type W = crate::W<RISCV_AXI_PMU_PRD_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RISCV_AXI_PMU_PRD_SPEC> {
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
#[doc = "RISCV AXI PMU Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`riscv_axi_pmu_prd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`riscv_axi_pmu_prd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RISCV_AXI_PMU_PRD_SPEC;
impl crate::RegisterSpec for RISCV_AXI_PMU_PRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`riscv_axi_pmu_prd::R`](R) reader structure"]
impl crate::Readable for RISCV_AXI_PMU_PRD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`riscv_axi_pmu_prd::W`](W) writer structure"]
impl crate::Writable for RISCV_AXI_PMU_PRD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets riscv_axi_pmu_prd to value 0"]
impl crate::Resettable for RISCV_AXI_PMU_PRD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
