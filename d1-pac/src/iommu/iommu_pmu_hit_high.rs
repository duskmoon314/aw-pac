#[doc = "Register `iommu_pmu_hit_high%s` reader"]
pub struct R(crate::R<IOMMU_PMU_HIT_HIGH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOMMU_PMU_HIT_HIGH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOMMU_PMU_HIT_HIGH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOMMU_PMU_HIT_HIGH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `iommu_pmu_hit_high%s` writer"]
pub struct W(crate::W<IOMMU_PMU_HIT_HIGH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOMMU_PMU_HIT_HIGH_SPEC>;
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
impl From<crate::W<IOMMU_PMU_HIT_HIGH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOMMU_PMU_HIT_HIGH_SPEC>) -> Self {
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
#[doc = "IOMMU PMU Hit High \\[i\\] Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iommu_pmu_hit_high](index.html) module"]
pub struct IOMMU_PMU_HIT_HIGH_SPEC;
impl crate::RegisterSpec for IOMMU_PMU_HIT_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iommu_pmu_hit_high::R](R) reader structure"]
impl crate::Readable for IOMMU_PMU_HIT_HIGH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iommu_pmu_hit_high::W](W) writer structure"]
impl crate::Writable for IOMMU_PMU_HIT_HIGH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iommu_pmu_hit_high%s to value 0"]
impl crate::Resettable for IOMMU_PMU_HIT_HIGH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
