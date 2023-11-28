#[doc = "Register `iommu_pmu_clr` reader"]
pub type R = crate::R<IOMMU_PMU_CLR_SPEC>;
#[doc = "Register `iommu_pmu_clr` writer"]
pub type W = crate::W<IOMMU_PMU_CLR_SPEC>;
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
    pub const fn variant(&self) -> PMU_CLR_A {
        match self.bits {
            false => PMU_CLR_A::NO_CLEAR_OR_COMPLETED,
            true => PMU_CLR_A::CLEAR,
        }
    }
    #[doc = "No clear operation or clear operation is completed"]
    #[inline(always)]
    pub fn is_no_clear_or_completed(&self) -> bool {
        *self == PMU_CLR_A::NO_CLEAR_OR_COMPLETED
    }
    #[doc = "Clear counter data"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == PMU_CLR_A::CLEAR
    }
}
#[doc = "Field `pmu_clr` writer - "]
pub type PMU_CLR_W<'a, REG> = crate::BitWriter<'a, REG, PMU_CLR_A>;
impl<'a, REG> PMU_CLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No clear operation or clear operation is completed"]
    #[inline(always)]
    pub fn no_clear_or_completed(self) -> &'a mut crate::W<REG> {
        self.variant(PMU_CLR_A::NO_CLEAR_OR_COMPLETED)
    }
    #[doc = "Clear counter data"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
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
    pub fn pmu_clr(&mut self) -> PMU_CLR_W<IOMMU_PMU_CLR_SPEC> {
        PMU_CLR_W::new(self, 0)
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
#[doc = "IOMMU PMU Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_pmu_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_pmu_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOMMU_PMU_CLR_SPEC;
impl crate::RegisterSpec for IOMMU_PMU_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iommu_pmu_clr::R`](R) reader structure"]
impl crate::Readable for IOMMU_PMU_CLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iommu_pmu_clr::W`](W) writer structure"]
impl crate::Writable for IOMMU_PMU_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iommu_pmu_clr to value 0"]
impl crate::Resettable for IOMMU_PMU_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
