#[doc = "Register `iommu_pmu_clr` reader"]
pub struct R(crate::R<IOMMU_PMU_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOMMU_PMU_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOMMU_PMU_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOMMU_PMU_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `iommu_pmu_clr` writer"]
pub struct W(crate::W<IOMMU_PMU_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOMMU_PMU_CLR_SPEC>;
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
impl From<crate::W<IOMMU_PMU_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOMMU_PMU_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pmu_clr` reader - "]
pub type PMU_CLR_R = crate::BitReader<PMU_CLR_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMU_CLR_A {
    #[doc = "0: No clear operation or clear operation is completed"]
    NO_CLEAR_OR_COMPLETED = 0,
    #[doc = "1: Clear counter data"]
    CLEAR = 1,
}
impl From<PMU_CLR_A> for bool {
    #[inline(always)]
    fn from(variant: PMU_CLR_A) -> Self {
        variant as u8 != 0
    }
}
impl PMU_CLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMU_CLR_A {
        match self.bits {
            false => PMU_CLR_A::NO_CLEAR_OR_COMPLETED,
            true => PMU_CLR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR_OR_COMPLETED`"]
    #[inline(always)]
    pub fn is_no_clear_or_completed(&self) -> bool {
        *self == PMU_CLR_A::NO_CLEAR_OR_COMPLETED
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == PMU_CLR_A::CLEAR
    }
}
#[doc = "Field `pmu_clr` writer - "]
pub type PMU_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOMMU_PMU_CLR_SPEC, PMU_CLR_A, O>;
impl<'a, const O: u8> PMU_CLR_W<'a, O> {
    #[doc = "No clear operation or clear operation is completed"]
    #[inline(always)]
    pub fn no_clear_or_completed(self) -> &'a mut W {
        self.variant(PMU_CLR_A::NO_CLEAR_OR_COMPLETED)
    }
    #[doc = "Clear counter data"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PMU_CLR_A::CLEAR)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pmu_clr(&self) -> PMU_CLR_R {
        PMU_CLR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_clr(&mut self) -> PMU_CLR_W<0> {
        PMU_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IOMMU PMU Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iommu_pmu_clr](index.html) module"]
pub struct IOMMU_PMU_CLR_SPEC;
impl crate::RegisterSpec for IOMMU_PMU_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iommu_pmu_clr::R](R) reader structure"]
impl crate::Readable for IOMMU_PMU_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iommu_pmu_clr::W](W) writer structure"]
impl crate::Writable for IOMMU_PMU_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iommu_pmu_clr to value 0"]
impl crate::Resettable for IOMMU_PMU_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
