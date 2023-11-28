#[doc = "Register `iommu_tlb_ivld_mode_sel` reader"]
pub type R = crate::R<IOMMU_TLB_IVLD_MODE_SEL_SPEC>;
#[doc = "Register `iommu_tlb_ivld_mode_sel` writer"]
pub type W = crate::W<IOMMU_TLB_IVLD_MODE_SEL_SPEC>;
#[doc = "Field `tlb_ivld_mode_sel` reader - "]
pub type TLB_IVLD_MODE_SEL_R = crate::BitReader<TLB_IVLD_MODE_SEL_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TLB_IVLD_MODE_SEL_A {
    #[doc = "0: Invalidate TLB by using the Mask mode"]
    M_ASK_MODE = 0,
    #[doc = "1: Invalidate TLB by using the Start and End mode"]
    S_TART_END_MODE = 1,
}
impl From<TLB_IVLD_MODE_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: TLB_IVLD_MODE_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl TLB_IVLD_MODE_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TLB_IVLD_MODE_SEL_A {
        match self.bits {
            false => TLB_IVLD_MODE_SEL_A::M_ASK_MODE,
            true => TLB_IVLD_MODE_SEL_A::S_TART_END_MODE,
        }
    }
    #[doc = "Invalidate TLB by using the Mask mode"]
    #[inline(always)]
    pub fn is_m_ask_mode(&self) -> bool {
        *self == TLB_IVLD_MODE_SEL_A::M_ASK_MODE
    }
    #[doc = "Invalidate TLB by using the Start and End mode"]
    #[inline(always)]
    pub fn is_s_tart_end_mode(&self) -> bool {
        *self == TLB_IVLD_MODE_SEL_A::S_TART_END_MODE
    }
}
#[doc = "Field `tlb_ivld_mode_sel` writer - "]
pub type TLB_IVLD_MODE_SEL_W<'a, REG> = crate::BitWriter<'a, REG, TLB_IVLD_MODE_SEL_A>;
impl<'a, REG> TLB_IVLD_MODE_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Invalidate TLB by using the Mask mode"]
    #[inline(always)]
    pub fn m_ask_mode(self) -> &'a mut crate::W<REG> {
        self.variant(TLB_IVLD_MODE_SEL_A::M_ASK_MODE)
    }
    #[doc = "Invalidate TLB by using the Start and End mode"]
    #[inline(always)]
    pub fn s_tart_end_mode(self) -> &'a mut crate::W<REG> {
        self.variant(TLB_IVLD_MODE_SEL_A::S_TART_END_MODE)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tlb_ivld_mode_sel(&self) -> TLB_IVLD_MODE_SEL_R {
        TLB_IVLD_MODE_SEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn tlb_ivld_mode_sel(&mut self) -> TLB_IVLD_MODE_SEL_W<IOMMU_TLB_IVLD_MODE_SEL_SPEC> {
        TLB_IVLD_MODE_SEL_W::new(self, 0)
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
#[doc = "IOMMU TLB Invalidation Mode Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_tlb_ivld_mode_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_tlb_ivld_mode_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOMMU_TLB_IVLD_MODE_SEL_SPEC;
impl crate::RegisterSpec for IOMMU_TLB_IVLD_MODE_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iommu_tlb_ivld_mode_sel::R`](R) reader structure"]
impl crate::Readable for IOMMU_TLB_IVLD_MODE_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iommu_tlb_ivld_mode_sel::W`](W) writer structure"]
impl crate::Writable for IOMMU_TLB_IVLD_MODE_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iommu_tlb_ivld_mode_sel to value 0"]
impl crate::Resettable for IOMMU_TLB_IVLD_MODE_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
