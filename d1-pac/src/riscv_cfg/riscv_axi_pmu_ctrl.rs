#[doc = "Register `riscv_axi_pmu_ctrl` reader"]
pub struct R(crate::R<RISCV_AXI_PMU_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RISCV_AXI_PMU_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RISCV_AXI_PMU_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RISCV_AXI_PMU_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `riscv_axi_pmu_ctrl` writer"]
pub struct W(crate::W<RISCV_AXI_PMU_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RISCV_AXI_PMU_CTRL_SPEC>;
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
impl From<crate::W<RISCV_AXI_PMU_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RISCV_AXI_PMU_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pmu_en` reader - PMU Enable"]
pub type PMU_EN_R = crate::BitReader<PMU_EN_A>;
#[doc = "PMU Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMU_EN_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<PMU_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PMU_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl PMU_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMU_EN_A {
        match self.bits {
            false => PMU_EN_A::DISABLED,
            true => PMU_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PMU_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PMU_EN_A::ENABLED
    }
}
#[doc = "Field `pmu_en` writer - PMU Enable"]
pub type PMU_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RISCV_AXI_PMU_CTRL_SPEC, PMU_EN_A, O>;
impl<'a, const O: u8> PMU_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PMU_EN_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PMU_EN_A::ENABLED)
    }
}
#[doc = "Field `pmu_clr` reader - PMU Clear"]
pub type PMU_CLR_R = crate::BitReader<PMU_CLR_A>;
#[doc = "PMU Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMU_CLR_A {
    #[doc = "0: `0`"]
    NO_OPERATION = 0,
    #[doc = "1: `1`"]
    CLEARED = 1,
}
impl From<PMU_CLR_A> for bool {
    #[inline(always)]
    fn from(variant: PMU_CLR_A) -> Self {
        variant as u8 != 0
    }
}
impl PMU_CLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMU_CLR_A {
        match self.bits {
            false => PMU_CLR_A::NO_OPERATION,
            true => PMU_CLR_A::CLEARED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_OPERATION`"]
    #[inline(always)]
    pub fn is_no_operation(&self) -> bool {
        *self == PMU_CLR_A::NO_OPERATION
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == PMU_CLR_A::CLEARED
    }
}
#[doc = "Field `pmu_clr` writer - PMU Clear"]
pub type PMU_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RISCV_AXI_PMU_CTRL_SPEC, PMU_CLR_A, O>;
impl<'a, const O: u8> PMU_CLR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_operation(self) -> &'a mut W {
        self.variant(PMU_CLR_A::NO_OPERATION)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(PMU_CLR_A::CLEARED)
    }
}
impl R {
    #[doc = "Bit 0 - PMU Enable"]
    #[inline(always)]
    pub fn pmu_en(&self) -> PMU_EN_R {
        PMU_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PMU Clear"]
    #[inline(always)]
    pub fn pmu_clr(&self) -> PMU_CLR_R {
        PMU_CLR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PMU Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_en(&mut self) -> PMU_EN_W<0> {
        PMU_EN_W::new(self)
    }
    #[doc = "Bit 1 - PMU Clear"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_clr(&mut self) -> PMU_CLR_W<1> {
        PMU_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RISCV AXI PMU Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [riscv_axi_pmu_ctrl](index.html) module"]
pub struct RISCV_AXI_PMU_CTRL_SPEC;
impl crate::RegisterSpec for RISCV_AXI_PMU_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [riscv_axi_pmu_ctrl::R](R) reader structure"]
impl crate::Readable for RISCV_AXI_PMU_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [riscv_axi_pmu_ctrl::W](W) writer structure"]
impl crate::Writable for RISCV_AXI_PMU_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets riscv_axi_pmu_ctrl to value 0"]
impl crate::Resettable for RISCV_AXI_PMU_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
