#[doc = "Register `riscv_axi_pmu_lat_wr` reader"]
pub struct R(crate::R<RISCV_AXI_PMU_LAT_WR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RISCV_AXI_PMU_LAT_WR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RISCV_AXI_PMU_LAT_WR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RISCV_AXI_PMU_LAT_WR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "RISCV AXI PMU Write Latency Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [riscv_axi_pmu_lat_wr](index.html) module"]
pub struct RISCV_AXI_PMU_LAT_WR_SPEC;
impl crate::RegisterSpec for RISCV_AXI_PMU_LAT_WR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [riscv_axi_pmu_lat_wr::R](R) reader structure"]
impl crate::Readable for RISCV_AXI_PMU_LAT_WR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets riscv_axi_pmu_lat_wr to value 0"]
impl crate::Resettable for RISCV_AXI_PMU_LAT_WR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
