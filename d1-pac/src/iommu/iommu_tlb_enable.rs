#[doc = "Register `iommu_tlb_enable` reader"]
pub struct R(crate::R<IOMMU_TLB_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOMMU_TLB_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOMMU_TLB_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOMMU_TLB_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `iommu_tlb_enable` writer"]
pub struct W(crate::W<IOMMU_TLB_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOMMU_TLB_ENABLE_SPEC>;
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
impl From<crate::W<IOMMU_TLB_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOMMU_TLB_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
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
    pub fn variant(&self) -> MICRO_TLB_ENABLE_A {
        match self.bits {
            false => MICRO_TLB_ENABLE_A::DISABLE,
            true => MICRO_TLB_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MICRO_TLB_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MICRO_TLB_ENABLE_A::ENABLE
    }
}
#[doc = "Field `micro_tlb_enable[0-6]` writer - Micro TLB\\[i\\] enable bit"]
pub type MICRO_TLB_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IOMMU_TLB_ENABLE_SPEC, MICRO_TLB_ENABLE_A, O>;
impl<'a, const O: u8> MICRO_TLB_ENABLE_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MICRO_TLB_ENABLE_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
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
    pub fn variant(&self) -> MACRO_TLB_ENABLE_A {
        match self.bits {
            false => MACRO_TLB_ENABLE_A::DISABLE,
            true => MACRO_TLB_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MACRO_TLB_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MACRO_TLB_ENABLE_A::ENABLE
    }
}
#[doc = "Field `macro_tlb_enable` writer - Macro TLB enable bit"]
pub type MACRO_TLB_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IOMMU_TLB_ENABLE_SPEC, MACRO_TLB_ENABLE_A, O>;
impl<'a, const O: u8> MACRO_TLB_ENABLE_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MACRO_TLB_ENABLE_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
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
    pub fn variant(&self) -> PTW_CACHE_ENABLE_A {
        match self.bits {
            false => PTW_CACHE_ENABLE_A::DISABLE,
            true => PTW_CACHE_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PTW_CACHE_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PTW_CACHE_ENABLE_A::ENABLE
    }
}
#[doc = "Field `ptw_cache_enable` writer - PTW Cache enable bit"]
pub type PTW_CACHE_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IOMMU_TLB_ENABLE_SPEC, PTW_CACHE_ENABLE_A, O>;
impl<'a, const O: u8> PTW_CACHE_ENABLE_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PTW_CACHE_ENABLE_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PTW_CACHE_ENABLE_A::ENABLE)
    }
}
impl R {
    #[doc = "Micro TLB\\[i\\] enable bit"]
    #[inline(always)]
    pub unsafe fn micro_tlb_enable(&self, n: u8) -> MICRO_TLB_ENABLE_R {
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
    #[doc = "Micro TLB\\[i\\] enable bit"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn micro_tlb_enable<const O: u8>(&mut self) -> MICRO_TLB_ENABLE_W<O> {
        MICRO_TLB_ENABLE_W::new(self)
    }
    #[doc = "Bit 0 - Micro TLB\\[i\\] enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn micro_tlb0_enable(&mut self) -> MICRO_TLB_ENABLE_W<0> {
        MICRO_TLB_ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Micro TLB\\[i\\] enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn micro_tlb1_enable(&mut self) -> MICRO_TLB_ENABLE_W<1> {
        MICRO_TLB_ENABLE_W::new(self)
    }
    #[doc = "Bit 2 - Micro TLB\\[i\\] enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn micro_tlb2_enable(&mut self) -> MICRO_TLB_ENABLE_W<2> {
        MICRO_TLB_ENABLE_W::new(self)
    }
    #[doc = "Bit 3 - Micro TLB\\[i\\] enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn micro_tlb3_enable(&mut self) -> MICRO_TLB_ENABLE_W<3> {
        MICRO_TLB_ENABLE_W::new(self)
    }
    #[doc = "Bit 4 - Micro TLB\\[i\\] enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn micro_tlb4_enable(&mut self) -> MICRO_TLB_ENABLE_W<4> {
        MICRO_TLB_ENABLE_W::new(self)
    }
    #[doc = "Bit 5 - Micro TLB\\[i\\] enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn micro_tlb5_enable(&mut self) -> MICRO_TLB_ENABLE_W<5> {
        MICRO_TLB_ENABLE_W::new(self)
    }
    #[doc = "Bit 6 - Micro TLB\\[i\\] enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn micro_tlb6_enable(&mut self) -> MICRO_TLB_ENABLE_W<6> {
        MICRO_TLB_ENABLE_W::new(self)
    }
    #[doc = "Bit 16 - Macro TLB enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn macro_tlb_enable(&mut self) -> MACRO_TLB_ENABLE_W<16> {
        MACRO_TLB_ENABLE_W::new(self)
    }
    #[doc = "Bit 17 - PTW Cache enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn ptw_cache_enable(&mut self) -> PTW_CACHE_ENABLE_W<17> {
        PTW_CACHE_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IOMMU TLB Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iommu_tlb_enable](index.html) module"]
pub struct IOMMU_TLB_ENABLE_SPEC;
impl crate::RegisterSpec for IOMMU_TLB_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iommu_tlb_enable::R](R) reader structure"]
impl crate::Readable for IOMMU_TLB_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iommu_tlb_enable::W](W) writer structure"]
impl crate::Writable for IOMMU_TLB_ENABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iommu_tlb_enable to value 0x0003_007f"]
impl crate::Resettable for IOMMU_TLB_ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_007f;
}
