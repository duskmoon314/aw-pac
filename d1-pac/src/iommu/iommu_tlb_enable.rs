#[doc = "Register `iommu_tlb_enable` reader"]
pub type R = crate::R<IOMMU_TLB_ENABLE_SPEC>;
#[doc = "Register `iommu_tlb_enable` writer"]
pub type W = crate::W<IOMMU_TLB_ENABLE_SPEC>;
#[doc = "Field `micro_tlb_enable[0-6]` reader - Micro TLB\\[i\\] enable bit"]
pub type MICRO_TLB_ENABLE_R = crate::BitReader<MICRO_TLB_ENABLE_A>;
#[doc = "Micro TLB\\[i\\] enable bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MICRO_TLB_ENABLE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<MICRO_TLB_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: MICRO_TLB_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl MICRO_TLB_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MICRO_TLB_ENABLE_A {
        match self.bits {
            false => MICRO_TLB_ENABLE_A::DISABLE,
            true => MICRO_TLB_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MICRO_TLB_ENABLE_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MICRO_TLB_ENABLE_A::ENABLE
    }
}
#[doc = "Field `micro_tlb_enable[0-6]` writer - Micro TLB\\[i\\] enable bit"]
pub type MICRO_TLB_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, MICRO_TLB_ENABLE_A>;
impl<'a, REG> MICRO_TLB_ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(MICRO_TLB_ENABLE_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(MICRO_TLB_ENABLE_A::ENABLE)
    }
}
#[doc = "Field `macro_tlb_enable` reader - Macro TLB enable bit"]
pub type MACRO_TLB_ENABLE_R = crate::BitReader<MACRO_TLB_ENABLE_A>;
#[doc = "Macro TLB enable bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MACRO_TLB_ENABLE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<MACRO_TLB_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: MACRO_TLB_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl MACRO_TLB_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MACRO_TLB_ENABLE_A {
        match self.bits {
            false => MACRO_TLB_ENABLE_A::DISABLE,
            true => MACRO_TLB_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MACRO_TLB_ENABLE_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MACRO_TLB_ENABLE_A::ENABLE
    }
}
#[doc = "Field `macro_tlb_enable` writer - Macro TLB enable bit"]
pub type MACRO_TLB_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, MACRO_TLB_ENABLE_A>;
impl<'a, REG> MACRO_TLB_ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(MACRO_TLB_ENABLE_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(MACRO_TLB_ENABLE_A::ENABLE)
    }
}
#[doc = "Field `ptw_cache_enable` reader - PTW Cache enable bit"]
pub type PTW_CACHE_ENABLE_R = crate::BitReader<PTW_CACHE_ENABLE_A>;
#[doc = "PTW Cache enable bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTW_CACHE_ENABLE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<PTW_CACHE_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: PTW_CACHE_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl PTW_CACHE_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PTW_CACHE_ENABLE_A {
        match self.bits {
            false => PTW_CACHE_ENABLE_A::DISABLE,
            true => PTW_CACHE_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PTW_CACHE_ENABLE_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PTW_CACHE_ENABLE_A::ENABLE
    }
}
#[doc = "Field `ptw_cache_enable` writer - PTW Cache enable bit"]
pub type PTW_CACHE_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, PTW_CACHE_ENABLE_A>;
impl<'a, REG> PTW_CACHE_ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PTW_CACHE_ENABLE_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PTW_CACHE_ENABLE_A::ENABLE)
    }
}
impl R {
    #[doc = "Micro TLB\\[i\\] enable bit\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `micro_tlb0_enable` field"]
    #[inline(always)]
    pub fn micro_tlb_enable(&self, n: u8) -> MICRO_TLB_ENABLE_R {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        MICRO_TLB_ENABLE_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Micro TLB\\[i\\] enable bit"]
    #[inline(always)]
    pub fn micro_tlb0_enable(&self) -> MICRO_TLB_ENABLE_R {
        MICRO_TLB_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Micro TLB\\[i\\] enable bit"]
    #[inline(always)]
    pub fn micro_tlb1_enable(&self) -> MICRO_TLB_ENABLE_R {
        MICRO_TLB_ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Micro TLB\\[i\\] enable bit"]
    #[inline(always)]
    pub fn micro_tlb2_enable(&self) -> MICRO_TLB_ENABLE_R {
        MICRO_TLB_ENABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Micro TLB\\[i\\] enable bit"]
    #[inline(always)]
    pub fn micro_tlb3_enable(&self) -> MICRO_TLB_ENABLE_R {
        MICRO_TLB_ENABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Micro TLB\\[i\\] enable bit"]
    #[inline(always)]
    pub fn micro_tlb4_enable(&self) -> MICRO_TLB_ENABLE_R {
        MICRO_TLB_ENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Micro TLB\\[i\\] enable bit"]
    #[inline(always)]
    pub fn micro_tlb5_enable(&self) -> MICRO_TLB_ENABLE_R {
        MICRO_TLB_ENABLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Micro TLB\\[i\\] enable bit"]
    #[inline(always)]
    pub fn micro_tlb6_enable(&self) -> MICRO_TLB_ENABLE_R {
        MICRO_TLB_ENABLE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - Macro TLB enable bit"]
    #[inline(always)]
    pub fn macro_tlb_enable(&self) -> MACRO_TLB_ENABLE_R {
        MACRO_TLB_ENABLE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PTW Cache enable bit"]
    #[inline(always)]
    pub fn ptw_cache_enable(&self) -> PTW_CACHE_ENABLE_R {
        PTW_CACHE_ENABLE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Micro TLB\\[i\\] enable bit\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `micro_tlb0_enable` field"]
    #[inline(always)]
    #[must_use]
    pub fn micro_tlb_enable(&mut self, n: u8) -> MICRO_TLB_ENABLE_W<IOMMU_TLB_ENABLE_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        MICRO_TLB_ENABLE_W::new(self, n)
    }
    #[doc = "Bit 0 - Micro TLB\\[i\\] enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn micro_tlb0_enable(&mut self) -> MICRO_TLB_ENABLE_W<IOMMU_TLB_ENABLE_SPEC> {
        MICRO_TLB_ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Micro TLB\\[i\\] enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn micro_tlb1_enable(&mut self) -> MICRO_TLB_ENABLE_W<IOMMU_TLB_ENABLE_SPEC> {
        MICRO_TLB_ENABLE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Micro TLB\\[i\\] enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn micro_tlb2_enable(&mut self) -> MICRO_TLB_ENABLE_W<IOMMU_TLB_ENABLE_SPEC> {
        MICRO_TLB_ENABLE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Micro TLB\\[i\\] enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn micro_tlb3_enable(&mut self) -> MICRO_TLB_ENABLE_W<IOMMU_TLB_ENABLE_SPEC> {
        MICRO_TLB_ENABLE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Micro TLB\\[i\\] enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn micro_tlb4_enable(&mut self) -> MICRO_TLB_ENABLE_W<IOMMU_TLB_ENABLE_SPEC> {
        MICRO_TLB_ENABLE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Micro TLB\\[i\\] enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn micro_tlb5_enable(&mut self) -> MICRO_TLB_ENABLE_W<IOMMU_TLB_ENABLE_SPEC> {
        MICRO_TLB_ENABLE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Micro TLB\\[i\\] enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn micro_tlb6_enable(&mut self) -> MICRO_TLB_ENABLE_W<IOMMU_TLB_ENABLE_SPEC> {
        MICRO_TLB_ENABLE_W::new(self, 6)
    }
    #[doc = "Bit 16 - Macro TLB enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn macro_tlb_enable(&mut self) -> MACRO_TLB_ENABLE_W<IOMMU_TLB_ENABLE_SPEC> {
        MACRO_TLB_ENABLE_W::new(self, 16)
    }
    #[doc = "Bit 17 - PTW Cache enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn ptw_cache_enable(&mut self) -> PTW_CACHE_ENABLE_W<IOMMU_TLB_ENABLE_SPEC> {
        PTW_CACHE_ENABLE_W::new(self, 17)
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
#[doc = "IOMMU TLB Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_tlb_enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_tlb_enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOMMU_TLB_ENABLE_SPEC;
impl crate::RegisterSpec for IOMMU_TLB_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iommu_tlb_enable::R`](R) reader structure"]
impl crate::Readable for IOMMU_TLB_ENABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iommu_tlb_enable::W`](W) writer structure"]
impl crate::Writable for IOMMU_TLB_ENABLE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iommu_tlb_enable to value 0x0003_007f"]
impl crate::Resettable for IOMMU_TLB_ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_007f;
}
