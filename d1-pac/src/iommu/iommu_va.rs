#[doc = "Register `iommu_va` reader"]
pub struct R(crate::R<IOMMU_VA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOMMU_VA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOMMU_VA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOMMU_VA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `iommu_va` writer"]
pub struct W(crate::W<IOMMU_VA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOMMU_VA_SPEC>;
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
impl From<crate::W<IOMMU_VA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOMMU_VA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `va` reader - Virtual address of read/write"]
pub type VA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `va` writer - Virtual address of read/write"]
pub type VA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOMMU_VA_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Virtual address of read/write"]
    #[inline(always)]
    pub fn va(&self) -> VA_R {
        VA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Virtual address of read/write"]
    #[inline(always)]
    #[must_use]
    pub fn va(&mut self) -> VA_W<0> {
        VA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IOMMU Virtual Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iommu_va](index.html) module"]
pub struct IOMMU_VA_SPEC;
impl crate::RegisterSpec for IOMMU_VA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iommu_va::R](R) reader structure"]
impl crate::Readable for IOMMU_VA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iommu_va::W](W) writer structure"]
impl crate::Writable for IOMMU_VA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iommu_va to value 0"]
impl crate::Resettable for IOMMU_VA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
