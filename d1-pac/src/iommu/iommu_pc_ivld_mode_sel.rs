#[doc = "Register `iommu_pc_ivld_mode_sel` reader"]
pub type R = crate::R<IOMMU_PC_IVLD_MODE_SEL_SPEC>;
#[doc = "Register `iommu_pc_ivld_mode_sel` writer"]
pub type W = crate::W<IOMMU_PC_IVLD_MODE_SEL_SPEC>;
#[doc = "Field `pc_ivld_mode_sel` reader - PTW Cache Invalid Mode Select"]
pub type PC_IVLD_MODE_SEL_R = crate::BitReader<PC_IVLD_MODE_SEL_A>;
#[doc = "PTW Cache Invalid Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PC_IVLD_MODE_SEL_A {
    #[doc = "0: Invalidate PTW by using the Mask mode"]
    M_ASK_MODE = 0,
    #[doc = "1: Invalidate PTW by using the Start and End mode"]
    S_TART_END_MODE = 1,
}
impl From<PC_IVLD_MODE_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: PC_IVLD_MODE_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl PC_IVLD_MODE_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PC_IVLD_MODE_SEL_A {
        match self.bits {
            false => PC_IVLD_MODE_SEL_A::M_ASK_MODE,
            true => PC_IVLD_MODE_SEL_A::S_TART_END_MODE,
        }
    }
    #[doc = "Invalidate PTW by using the Mask mode"]
    #[inline(always)]
    pub fn is_m_ask_mode(&self) -> bool {
        *self == PC_IVLD_MODE_SEL_A::M_ASK_MODE
    }
    #[doc = "Invalidate PTW by using the Start and End mode"]
    #[inline(always)]
    pub fn is_s_tart_end_mode(&self) -> bool {
        *self == PC_IVLD_MODE_SEL_A::S_TART_END_MODE
    }
}
#[doc = "Field `pc_ivld_mode_sel` writer - PTW Cache Invalid Mode Select"]
pub type PC_IVLD_MODE_SEL_W<'a, REG> = crate::BitWriter<'a, REG, PC_IVLD_MODE_SEL_A>;
impl<'a, REG> PC_IVLD_MODE_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Invalidate PTW by using the Mask mode"]
    #[inline(always)]
    pub fn m_ask_mode(self) -> &'a mut crate::W<REG> {
        self.variant(PC_IVLD_MODE_SEL_A::M_ASK_MODE)
    }
    #[doc = "Invalidate PTW by using the Start and End mode"]
    #[inline(always)]
    pub fn s_tart_end_mode(self) -> &'a mut crate::W<REG> {
        self.variant(PC_IVLD_MODE_SEL_A::S_TART_END_MODE)
    }
}
impl R {
    #[doc = "Bit 0 - PTW Cache Invalid Mode Select"]
    #[inline(always)]
    pub fn pc_ivld_mode_sel(&self) -> PC_IVLD_MODE_SEL_R {
        PC_IVLD_MODE_SEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PTW Cache Invalid Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn pc_ivld_mode_sel(&mut self) -> PC_IVLD_MODE_SEL_W<IOMMU_PC_IVLD_MODE_SEL_SPEC> {
        PC_IVLD_MODE_SEL_W::new(self, 0)
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
#[doc = "IOMMU PC Invalidation Mode Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_pc_ivld_mode_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_pc_ivld_mode_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOMMU_PC_IVLD_MODE_SEL_SPEC;
impl crate::RegisterSpec for IOMMU_PC_IVLD_MODE_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iommu_pc_ivld_mode_sel::R`](R) reader structure"]
impl crate::Readable for IOMMU_PC_IVLD_MODE_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iommu_pc_ivld_mode_sel::W`](W) writer structure"]
impl crate::Writable for IOMMU_PC_IVLD_MODE_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iommu_pc_ivld_mode_sel to value 0"]
impl crate::Resettable for IOMMU_PC_IVLD_MODE_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
