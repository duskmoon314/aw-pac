#[doc = "Register `riscv_axi_pmu_req_rd` reader"]
pub type R = crate::R<RISCV_AXI_PMU_REQ_RD_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RISCV_AXI_PMU_REQ_RD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "RISCV AXI PMU Read Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`riscv_axi_pmu_req_rd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RISCV_AXI_PMU_REQ_RD_SPEC;
impl crate::RegisterSpec for RISCV_AXI_PMU_REQ_RD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`riscv_axi_pmu_req_rd::R`](R) reader structure"]
impl crate::Readable for RISCV_AXI_PMU_REQ_RD_SPEC {}
#[doc = "`reset()` method sets riscv_axi_pmu_req_rd to value 0"]
impl crate::Resettable for RISCV_AXI_PMU_REQ_RD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
