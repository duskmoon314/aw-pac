#[doc = "Register `iommu_auto_gating` reader"]
pub struct R(crate::R<IOMMU_AUTO_GATING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOMMU_AUTO_GATING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOMMU_AUTO_GATING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOMMU_AUTO_GATING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `iommu_auto_gating` writer"]
pub struct W(crate::W<IOMMU_AUTO_GATING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOMMU_AUTO_GATING_SPEC>;
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
impl From<crate::W<IOMMU_AUTO_GATING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOMMU_AUTO_GATING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "IOMMU circuit auto gating control\n\nThe purpose is to decrease power consumption of the module.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOMMU_AUTO_GATING_A {
    #[doc = "0: Disable auto gating function"]
    D_ISABLE = 0,
    #[doc = "1: Enable auto gating function"]
    E_NABLE = 1,
}
impl From<IOMMU_AUTO_GATING_A> for bool {
    #[inline(always)]
    fn from(variant: IOMMU_AUTO_GATING_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `iommu_auto_gating` reader - IOMMU circuit auto gating control\n\nThe purpose is to decrease power consumption of the module."]
pub type IOMMU_AUTO_GATING_R = crate::BitReader<IOMMU_AUTO_GATING_A>;
impl IOMMU_AUTO_GATING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOMMU_AUTO_GATING_A {
        match self.bits {
            false => IOMMU_AUTO_GATING_A::D_ISABLE,
            true => IOMMU_AUTO_GATING_A::E_NABLE,
        }
    }
    #[doc = "Checks if the value of the field is `D_ISABLE`"]
    #[inline(always)]
    pub fn is_d_isable(&self) -> bool {
        *self == IOMMU_AUTO_GATING_A::D_ISABLE
    }
    #[doc = "Checks if the value of the field is `E_NABLE`"]
    #[inline(always)]
    pub fn is_e_nable(&self) -> bool {
        *self == IOMMU_AUTO_GATING_A::E_NABLE
    }
}
#[doc = "Field `iommu_auto_gating` writer - IOMMU circuit auto gating control\n\nThe purpose is to decrease power consumption of the module."]
pub type IOMMU_AUTO_GATING_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IOMMU_AUTO_GATING_SPEC, IOMMU_AUTO_GATING_A, O>;
impl<'a, const O: u8> IOMMU_AUTO_GATING_W<'a, O> {
    #[doc = "Disable auto gating function"]
    #[inline(always)]
    pub fn d_isable(self) -> &'a mut W {
        self.variant(IOMMU_AUTO_GATING_A::D_ISABLE)
    }
    #[doc = "Enable auto gating function"]
    #[inline(always)]
    pub fn e_nable(self) -> &'a mut W {
        self.variant(IOMMU_AUTO_GATING_A::E_NABLE)
    }
}
impl R {
    #[doc = "Bit 0 - IOMMU circuit auto gating control\n\nThe purpose is to decrease power consumption of the module."]
    #[inline(always)]
    pub fn iommu_auto_gating(&self) -> IOMMU_AUTO_GATING_R {
        IOMMU_AUTO_GATING_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IOMMU circuit auto gating control\n\nThe purpose is to decrease power consumption of the module."]
    #[inline(always)]
    pub fn iommu_auto_gating(&mut self) -> IOMMU_AUTO_GATING_W<0> {
        IOMMU_AUTO_GATING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IOMMU Auto Gating Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iommu_auto_gating](index.html) module"]
pub struct IOMMU_AUTO_GATING_SPEC;
impl crate::RegisterSpec for IOMMU_AUTO_GATING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iommu_auto_gating::R](R) reader structure"]
impl crate::Readable for IOMMU_AUTO_GATING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iommu_auto_gating::W](W) writer structure"]
impl crate::Writable for IOMMU_AUTO_GATING_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets iommu_auto_gating to value 0x01"]
impl crate::Resettable for IOMMU_AUTO_GATING_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
