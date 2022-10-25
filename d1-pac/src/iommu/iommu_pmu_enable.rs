#[doc = "Register `iommu_pmu_enable` reader"]
pub struct R(crate::R<IOMMU_PMU_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOMMU_PMU_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOMMU_PMU_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOMMU_PMU_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `iommu_pmu_enable` writer"]
pub struct W(crate::W<IOMMU_PMU_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOMMU_PMU_ENABLE_SPEC>;
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
impl From<crate::W<IOMMU_PMU_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOMMU_PMU_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pmu_enable` reader - "]
pub type PMU_ENABLE_R = crate::BitReader<PMU_ENABLE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMU_ENABLE_A {
    #[doc = "0: Disable statistical function"]
    DISABLE = 0,
    #[doc = "1: Enable statistical function"]
    ENABLE = 1,
}
impl From<PMU_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: PMU_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl PMU_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMU_ENABLE_A {
        match self.bits {
            false => PMU_ENABLE_A::DISABLE,
            true => PMU_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PMU_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PMU_ENABLE_A::ENABLE
    }
}
#[doc = "Field `pmu_enable` writer - "]
pub type PMU_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IOMMU_PMU_ENABLE_SPEC, PMU_ENABLE_A, O>;
impl<'a, const O: u8> PMU_ENABLE_W<'a, O> {
    #[doc = "Disable statistical function"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PMU_ENABLE_A::DISABLE)
    }
    #[doc = "Enable statistical function"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PMU_ENABLE_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pmu_enable(&self) -> PMU_ENABLE_R {
        PMU_ENABLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_enable(&mut self) -> PMU_ENABLE_W<0> {
        PMU_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IOMMU PMU Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iommu_pmu_enable](index.html) module"]
pub struct IOMMU_PMU_ENABLE_SPEC;
impl crate::RegisterSpec for IOMMU_PMU_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iommu_pmu_enable::R](R) reader structure"]
impl crate::Readable for IOMMU_PMU_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iommu_pmu_enable::W](W) writer structure"]
impl crate::Writable for IOMMU_PMU_ENABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iommu_pmu_enable to value 0"]
impl crate::Resettable for IOMMU_PMU_ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
