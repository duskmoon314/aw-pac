#[doc = "Register `iommu_pc_ivld_mode_sel` reader"]
pub struct R(crate::R<IOMMU_PC_IVLD_MODE_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOMMU_PC_IVLD_MODE_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOMMU_PC_IVLD_MODE_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOMMU_PC_IVLD_MODE_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `iommu_pc_ivld_mode_sel` writer"]
pub struct W(crate::W<IOMMU_PC_IVLD_MODE_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOMMU_PC_IVLD_MODE_SEL_SPEC>;
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
impl From<crate::W<IOMMU_PC_IVLD_MODE_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOMMU_PC_IVLD_MODE_SEL_SPEC>) -> Self {
        W(writer)
    }
}
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
    pub fn variant(&self) -> PC_IVLD_MODE_SEL_A {
        match self.bits {
            false => PC_IVLD_MODE_SEL_A::M_ASK_MODE,
            true => PC_IVLD_MODE_SEL_A::S_TART_END_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `M_ASK_MODE`"]
    #[inline(always)]
    pub fn is_m_ask_mode(&self) -> bool {
        *self == PC_IVLD_MODE_SEL_A::M_ASK_MODE
    }
    #[doc = "Checks if the value of the field is `S_TART_END_MODE`"]
    #[inline(always)]
    pub fn is_s_tart_end_mode(&self) -> bool {
        *self == PC_IVLD_MODE_SEL_A::S_TART_END_MODE
    }
}
#[doc = "Field `pc_ivld_mode_sel` writer - PTW Cache Invalid Mode Select"]
pub type PC_IVLD_MODE_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IOMMU_PC_IVLD_MODE_SEL_SPEC, PC_IVLD_MODE_SEL_A, O>;
impl<'a, const O: u8> PC_IVLD_MODE_SEL_W<'a, O> {
    #[doc = "Invalidate PTW by using the Mask mode"]
    #[inline(always)]
    pub fn m_ask_mode(self) -> &'a mut W {
        self.variant(PC_IVLD_MODE_SEL_A::M_ASK_MODE)
    }
    #[doc = "Invalidate PTW by using the Start and End mode"]
    #[inline(always)]
    pub fn s_tart_end_mode(self) -> &'a mut W {
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
    pub fn pc_ivld_mode_sel(&mut self) -> PC_IVLD_MODE_SEL_W<0> {
        PC_IVLD_MODE_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IOMMU PC Invalidation Mode Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iommu_pc_ivld_mode_sel](index.html) module"]
pub struct IOMMU_PC_IVLD_MODE_SEL_SPEC;
impl crate::RegisterSpec for IOMMU_PC_IVLD_MODE_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iommu_pc_ivld_mode_sel::R](R) reader structure"]
impl crate::Readable for IOMMU_PC_IVLD_MODE_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iommu_pc_ivld_mode_sel::W](W) writer structure"]
impl crate::Writable for IOMMU_PC_IVLD_MODE_SEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iommu_pc_ivld_mode_sel to value 0"]
impl crate::Resettable for IOMMU_PC_IVLD_MODE_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
