#[doc = "Register `RISCV_AXI_PMU_REQ_WR` reader"]
pub struct R(crate::R<RISCV_AXI_PMU_REQ_WR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RISCV_AXI_PMU_REQ_WR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RISCV_AXI_PMU_REQ_WR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RISCV_AXI_PMU_REQ_WR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RISCV_AXI_PMU_REQ_WR` writer"]
pub struct W(crate::W<RISCV_AXI_PMU_REQ_WR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RISCV_AXI_PMU_REQ_WR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<RISCV_AXI_PMU_REQ_WR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RISCV_AXI_PMU_REQ_WR_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RISCV AXI PMU Write Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [riscv_axi_pmu_req_wr](index.html) module"]
pub struct RISCV_AXI_PMU_REQ_WR_SPEC;
impl crate::RegisterSpec for RISCV_AXI_PMU_REQ_WR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [riscv_axi_pmu_req_wr::R](R) reader structure"]
impl crate::Readable for RISCV_AXI_PMU_REQ_WR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [riscv_axi_pmu_req_wr::W](W) writer structure"]
impl crate::Writable for RISCV_AXI_PMU_REQ_WR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RISCV_AXI_PMU_REQ_WR to value 0"]
impl crate::Resettable for RISCV_AXI_PMU_REQ_WR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
