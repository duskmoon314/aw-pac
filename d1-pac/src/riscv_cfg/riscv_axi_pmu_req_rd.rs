#[doc = "Register `riscv_axi_pmu_req_rd` reader"]
pub struct R(crate::R<RISCV_AXI_PMU_REQ_RD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RISCV_AXI_PMU_REQ_RD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RISCV_AXI_PMU_REQ_RD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RISCV_AXI_PMU_REQ_RD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "RISCV AXI PMU Read Request Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [riscv_axi_pmu_req_rd](index.html) module"]
pub struct RISCV_AXI_PMU_REQ_RD_SPEC;
impl crate::RegisterSpec for RISCV_AXI_PMU_REQ_RD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [riscv_axi_pmu_req_rd::R](R) reader structure"]
impl crate::Readable for RISCV_AXI_PMU_REQ_RD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets riscv_axi_pmu_req_rd to value 0"]
impl crate::Resettable for RISCV_AXI_PMU_REQ_RD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
