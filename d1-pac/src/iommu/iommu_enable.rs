#[doc = "Register `iommu_enable` reader"]
pub struct R(crate::R<IOMMU_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOMMU_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOMMU_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOMMU_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `iommu_enable` writer"]
pub struct W(crate::W<IOMMU_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOMMU_ENABLE_SPEC>;
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
impl From<crate::W<IOMMU_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOMMU_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
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
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::DISABLE,
            true => ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENABLE_A::ENABLE
    }
}
#[doc = "Field `enable` writer - IOMMU module enable switch\n\nBefore IOMMU address mapping function opens, configure the Translation Table Base register; or ensure all masters are in bypass status or no the status of sending bus demand(such as reset)"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOMMU_ENABLE_SPEC, ENABLE_A, O>;
impl<'a, const O: u8> ENABLE_W<'a, O> {
    #[doc = "Disable IOMMU"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENABLE_A::DISABLE)
    }
    #[doc = "Enable IOMMU"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
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
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IOMMU Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iommu_enable](index.html) module"]
pub struct IOMMU_ENABLE_SPEC;
impl crate::RegisterSpec for IOMMU_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iommu_enable::R](R) reader structure"]
impl crate::Readable for IOMMU_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iommu_enable::W](W) writer structure"]
impl crate::Writable for IOMMU_ENABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iommu_enable to value 0"]
impl crate::Resettable for IOMMU_ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
