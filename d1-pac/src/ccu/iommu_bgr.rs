#[doc = "Register `iommu_bgr` reader"]
pub type R = crate::R<IOMMU_BGR_SPEC>;
#[doc = "Register `iommu_bgr` writer"]
pub type W = crate::W<IOMMU_BGR_SPEC>;
#[doc = "Field `gating` reader - Gating Clock"]
pub type GATING_R = crate::BitReader<GATING_A>;
#[doc = "Gating Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GATING_A {
    #[doc = "0: `0`"]
    MASK = 0,
    #[doc = "1: `1`"]
    PASS = 1,
}
impl From<GATING_A> for bool {
    #[inline(always)]
    fn from(variant: GATING_A) -> Self {
        variant as u8 != 0
    }
}
impl GATING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GATING_A {
        match self.bits {
            false => GATING_A::MASK,
            true => GATING_A::PASS,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == GATING_A::MASK
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        *self == GATING_A::PASS
    }
}
#[doc = "Field `gating` writer - Gating Clock"]
pub type GATING_W<'a, REG> = crate::BitWriter<'a, REG, GATING_A>;
impl<'a, REG> GATING_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(GATING_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut crate::W<REG> {
        self.variant(GATING_A::PASS)
    }
}
impl R {
    #[doc = "Bit 0 - Gating Clock"]
    #[inline(always)]
    pub fn gating(&self) -> GATING_R {
        GATING_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Gating Clock"]
    #[inline(always)]
    #[must_use]
    pub fn gating(&mut self) -> GATING_W<IOMMU_BGR_SPEC> {
        GATING_W::new(self, 0)
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
#[doc = "IOMMU Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_bgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_bgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOMMU_BGR_SPEC;
impl crate::RegisterSpec for IOMMU_BGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iommu_bgr::R`](R) reader structure"]
impl crate::Readable for IOMMU_BGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iommu_bgr::W`](W) writer structure"]
impl crate::Writable for IOMMU_BGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iommu_bgr to value 0"]
impl crate::Resettable for IOMMU_BGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
