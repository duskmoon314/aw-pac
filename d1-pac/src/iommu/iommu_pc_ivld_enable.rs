#[doc = "Register `iommu_pc_ivld_enable` reader"]
pub type R = crate::R<IOMMU_PC_IVLD_ENABLE_SPEC>;
#[doc = "Register `iommu_pc_ivld_enable` writer"]
pub type W = crate::W<IOMMU_PC_IVLD_ENABLE_SPEC>;
#[doc = "Field `pc_ivld_enable` reader - Enable PTW Cache invalidation operation\n\nAfter invalidation operation is completed, the bit can clear automatically.\n\nAfter or before invalidation operation starts, there is no absolute relationship between the same address switch operation and invalidation operation."]
pub type PC_IVLD_ENABLE_R = crate::BitReader<PC_IVLD_ENABLE_A>;
#[doc = "Enable PTW Cache invalidation operation\n\nAfter invalidation operation is completed, the bit can clear automatically.\n\nAfter or before invalidation operation starts, there is no absolute relationship between the same address switch operation and invalidation operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PC_IVLD_ENABLE_A {
    #[doc = "0: No operation or operation is completed"]
    NO_OPERATION_OR_COMPLETED = 0,
    #[doc = "1: Enable invalidation operation"]
    ENABLE = 1,
}
impl From<PC_IVLD_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: PC_IVLD_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl PC_IVLD_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PC_IVLD_ENABLE_A {
        match self.bits {
            false => PC_IVLD_ENABLE_A::NO_OPERATION_OR_COMPLETED,
            true => PC_IVLD_ENABLE_A::ENABLE,
        }
    }
    #[doc = "No operation or operation is completed"]
    #[inline(always)]
    pub fn is_no_operation_or_completed(&self) -> bool {
        *self == PC_IVLD_ENABLE_A::NO_OPERATION_OR_COMPLETED
    }
    #[doc = "Enable invalidation operation"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PC_IVLD_ENABLE_A::ENABLE
    }
}
#[doc = "Field `pc_ivld_enable` writer - Enable PTW Cache invalidation operation\n\nAfter invalidation operation is completed, the bit can clear automatically.\n\nAfter or before invalidation operation starts, there is no absolute relationship between the same address switch operation and invalidation operation."]
pub type PC_IVLD_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, PC_IVLD_ENABLE_A>;
impl<'a, REG> PC_IVLD_ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No operation or operation is completed"]
    #[inline(always)]
    pub fn no_operation_or_completed(self) -> &'a mut crate::W<REG> {
        self.variant(PC_IVLD_ENABLE_A::NO_OPERATION_OR_COMPLETED)
    }
    #[doc = "Enable invalidation operation"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PC_IVLD_ENABLE_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Enable PTW Cache invalidation operation\n\nAfter invalidation operation is completed, the bit can clear automatically.\n\nAfter or before invalidation operation starts, there is no absolute relationship between the same address switch operation and invalidation operation."]
    #[inline(always)]
    pub fn pc_ivld_enable(&self) -> PC_IVLD_ENABLE_R {
        PC_IVLD_ENABLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable PTW Cache invalidation operation\n\nAfter invalidation operation is completed, the bit can clear automatically.\n\nAfter or before invalidation operation starts, there is no absolute relationship between the same address switch operation and invalidation operation."]
    #[inline(always)]
    #[must_use]
    pub fn pc_ivld_enable(&mut self) -> PC_IVLD_ENABLE_W<IOMMU_PC_IVLD_ENABLE_SPEC> {
        PC_IVLD_ENABLE_W::new(self, 0)
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
#[doc = "IOMMU PC Invalidation Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_pc_ivld_enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_pc_ivld_enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOMMU_PC_IVLD_ENABLE_SPEC;
impl crate::RegisterSpec for IOMMU_PC_IVLD_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iommu_pc_ivld_enable::R`](R) reader structure"]
impl crate::Readable for IOMMU_PC_IVLD_ENABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iommu_pc_ivld_enable::W`](W) writer structure"]
impl crate::Writable for IOMMU_PC_IVLD_ENABLE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iommu_pc_ivld_enable to value 0"]
impl crate::Resettable for IOMMU_PC_IVLD_ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
