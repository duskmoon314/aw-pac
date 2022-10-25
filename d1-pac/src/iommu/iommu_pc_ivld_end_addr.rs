#[doc = "Register `iommu_pc_ivld_end_addr` reader"]
pub struct R(crate::R<IOMMU_PC_IVLD_END_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOMMU_PC_IVLD_END_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOMMU_PC_IVLD_END_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOMMU_PC_IVLD_END_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `iommu_pc_ivld_end_addr` writer"]
pub struct W(crate::W<IOMMU_PC_IVLD_END_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOMMU_PC_IVLD_END_ADDR_SPEC>;
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
impl From<crate::W<IOMMU_PC_IVLD_END_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOMMU_PC_IVLD_END_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pc_ivld_ea` reader - PTW Cache invalid end address, 1 MB aligned."]
pub type PC_IVLD_EA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `pc_ivld_ea` writer - PTW Cache invalid end address, 1 MB aligned."]
pub type PC_IVLD_EA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IOMMU_PC_IVLD_END_ADDR_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 20:31 - PTW Cache invalid end address, 1 MB aligned."]
    #[inline(always)]
    pub fn pc_ivld_ea(&self) -> PC_IVLD_EA_R {
        PC_IVLD_EA_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 20:31 - PTW Cache invalid end address, 1 MB aligned."]
    #[inline(always)]
    #[must_use]
    pub fn pc_ivld_ea(&mut self) -> PC_IVLD_EA_W<20> {
        PC_IVLD_EA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IOMMU PC Invalidation End Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iommu_pc_ivld_end_addr](index.html) module"]
pub struct IOMMU_PC_IVLD_END_ADDR_SPEC;
impl crate::RegisterSpec for IOMMU_PC_IVLD_END_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iommu_pc_ivld_end_addr::R](R) reader structure"]
impl crate::Readable for IOMMU_PC_IVLD_END_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iommu_pc_ivld_end_addr::W](W) writer structure"]
impl crate::Writable for IOMMU_PC_IVLD_END_ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iommu_pc_ivld_end_addr to value 0"]
impl crate::Resettable for IOMMU_PC_IVLD_END_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
