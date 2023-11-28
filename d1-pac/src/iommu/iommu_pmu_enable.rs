#[doc = "Register `iommu_pmu_enable` reader"]
pub type R = crate::R<IOMMU_PMU_ENABLE_SPEC>;
#[doc = "Register `iommu_pmu_enable` writer"]
pub type W = crate::W<IOMMU_PMU_ENABLE_SPEC>;
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
    pub const fn variant(&self) -> PMU_ENABLE_A {
        match self.bits {
            false => PMU_ENABLE_A::DISABLE,
            true => PMU_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Disable statistical function"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PMU_ENABLE_A::DISABLE
    }
    #[doc = "Enable statistical function"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PMU_ENABLE_A::ENABLE
    }
}
#[doc = "Field `pmu_enable` writer - "]
pub type PMU_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, PMU_ENABLE_A>;
impl<'a, REG> PMU_ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable statistical function"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PMU_ENABLE_A::DISABLE)
    }
    #[doc = "Enable statistical function"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
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
    pub fn pmu_enable(&mut self) -> PMU_ENABLE_W<IOMMU_PMU_ENABLE_SPEC> {
        PMU_ENABLE_W::new(self, 0)
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
#[doc = "IOMMU PMU Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_pmu_enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_pmu_enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOMMU_PMU_ENABLE_SPEC;
impl crate::RegisterSpec for IOMMU_PMU_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iommu_pmu_enable::R`](R) reader structure"]
impl crate::Readable for IOMMU_PMU_ENABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iommu_pmu_enable::W`](W) writer structure"]
impl crate::Writable for IOMMU_PMU_ENABLE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iommu_pmu_enable to value 0"]
impl crate::Resettable for IOMMU_PMU_ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
