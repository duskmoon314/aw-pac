#[doc = "Register `iommu_enable` reader"]
pub type R = crate::R<IOMMU_ENABLE_SPEC>;
#[doc = "Register `iommu_enable` writer"]
pub type W = crate::W<IOMMU_ENABLE_SPEC>;
#[doc = "Field `enable` reader - IOMMU module enable switch\n\nBefore IOMMU address mapping function opens, configure the Translation Table Base register; or ensure all masters are in bypass status or no the status of sending bus demand(such as reset)"]
pub type ENABLE_R = crate::BitReader<ENABLE_A>;
#[doc = "IOMMU module enable switch\n\nBefore IOMMU address mapping function opens, configure the Translation Table Base register; or ensure all masters are in bypass status or no the status of sending bus demand(such as reset)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE_A {
    #[doc = "0: Disable IOMMU"]
    DISABLE = 0,
    #[doc = "1: Enable IOMMU"]
    ENABLE = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::DISABLE,
            true => ENABLE_A::ENABLE,
        }
    }
    #[doc = "Disable IOMMU"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENABLE_A::DISABLE
    }
    #[doc = "Enable IOMMU"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENABLE_A::ENABLE
    }
}
#[doc = "Field `enable` writer - IOMMU module enable switch\n\nBefore IOMMU address mapping function opens, configure the Translation Table Base register; or ensure all masters are in bypass status or no the status of sending bus demand(such as reset)"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, ENABLE_A>;
impl<'a, REG> ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable IOMMU"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLE_A::DISABLE)
    }
    #[doc = "Enable IOMMU"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLE_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - IOMMU module enable switch\n\nBefore IOMMU address mapping function opens, configure the Translation Table Base register; or ensure all masters are in bypass status or no the status of sending bus demand(such as reset)"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IOMMU module enable switch\n\nBefore IOMMU address mapping function opens, configure the Translation Table Base register; or ensure all masters are in bypass status or no the status of sending bus demand(such as reset)"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<IOMMU_ENABLE_SPEC> {
        ENABLE_W::new(self, 0)
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
#[doc = "IOMMU Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOMMU_ENABLE_SPEC;
impl crate::RegisterSpec for IOMMU_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iommu_enable::R`](R) reader structure"]
impl crate::Readable for IOMMU_ENABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iommu_enable::W`](W) writer structure"]
impl crate::Writable for IOMMU_ENABLE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iommu_enable to value 0"]
impl crate::Resettable for IOMMU_ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
