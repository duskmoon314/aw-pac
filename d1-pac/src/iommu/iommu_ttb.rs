#[doc = "Register `iommu_ttb` reader"]
pub struct R(crate::R<IOMMU_TTB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOMMU_TTB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOMMU_TTB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOMMU_TTB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `iommu_ttb` writer"]
pub struct W(crate::W<IOMMU_TTB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOMMU_TTB_SPEC>;
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
impl From<crate::W<IOMMU_TTB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOMMU_TTB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ttb` reader - Translation Table Base\n\nLevel1 page table starting address, aligned to 16 KB.\n\nWhen operating the register, IOMMU address mapping function must be closed, namely IOMMU_ENABLE is 0; Or Bypass function of all main equipment is set to 1, or no the state of transfer bus commands (such as setting)."]
pub type TTB_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ttb` writer - Translation Table Base\n\nLevel1 page table starting address, aligned to 16 KB.\n\nWhen operating the register, IOMMU address mapping function must be closed, namely IOMMU_ENABLE is 0; Or Bypass function of all main equipment is set to 1, or no the state of transfer bus commands (such as setting)."]
pub type TTB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOMMU_TTB_SPEC, u32, u32, 18, O>;
impl R {
    #[doc = "Bits 14:31 - Translation Table Base\n\nLevel1 page table starting address, aligned to 16 KB.\n\nWhen operating the register, IOMMU address mapping function must be closed, namely IOMMU_ENABLE is 0; Or Bypass function of all main equipment is set to 1, or no the state of transfer bus commands (such as setting)."]
    #[inline(always)]
    pub fn ttb(&self) -> TTB_R {
        TTB_R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 14:31 - Translation Table Base\n\nLevel1 page table starting address, aligned to 16 KB.\n\nWhen operating the register, IOMMU address mapping function must be closed, namely IOMMU_ENABLE is 0; Or Bypass function of all main equipment is set to 1, or no the state of transfer bus commands (such as setting)."]
    #[inline(always)]
    #[must_use]
    pub fn ttb(&mut self) -> TTB_W<14> {
        TTB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IOMMU Translation Table Base Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iommu_ttb](index.html) module"]
pub struct IOMMU_TTB_SPEC;
impl crate::RegisterSpec for IOMMU_TTB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iommu_ttb::R](R) reader structure"]
impl crate::Readable for IOMMU_TTB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iommu_ttb::W](W) writer structure"]
impl crate::Writable for IOMMU_TTB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iommu_ttb to value 0"]
impl crate::Resettable for IOMMU_TTB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
