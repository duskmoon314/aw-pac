#[doc = "Register `fre_det_ctrl` reader"]
pub struct R(crate::R<FRE_DET_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRE_DET_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRE_DET_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRE_DET_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `fre_det_ctrl` writer"]
pub struct W(crate::W<FRE_DET_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRE_DET_CTRL_SPEC>;
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
impl From<crate::W<FRE_DET_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRE_DET_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `fre_det_fun_en` reader - Frequence Detect Function Enable"]
pub type FRE_DET_FUN_EN_R = crate::BitReader<FRE_DET_FUN_EN_A>;
#[doc = "Frequence Detect Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRE_DET_FUN_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<FRE_DET_FUN_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FRE_DET_FUN_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl FRE_DET_FUN_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRE_DET_FUN_EN_A {
        match self.bits {
            false => FRE_DET_FUN_EN_A::DISABLE,
            true => FRE_DET_FUN_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FRE_DET_FUN_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FRE_DET_FUN_EN_A::ENABLE
    }
}
#[doc = "Field `fre_det_fun_en` writer - Frequence Detect Function Enable"]
pub type FRE_DET_FUN_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FRE_DET_CTRL_SPEC, FRE_DET_FUN_EN_A, O>;
impl<'a, const O: u8> FRE_DET_FUN_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FRE_DET_FUN_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FRE_DET_FUN_EN_A::ENABLE)
    }
}
#[doc = "Field `fre_det_irq_en` reader - Frequence Detect IRQ Enable"]
pub type FRE_DET_IRQ_EN_R = crate::BitReader<FRE_DET_IRQ_EN_A>;
#[doc = "Frequence Detect IRQ Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRE_DET_IRQ_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<FRE_DET_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FRE_DET_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl FRE_DET_IRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRE_DET_IRQ_EN_A {
        match self.bits {
            false => FRE_DET_IRQ_EN_A::DISABLE,
            true => FRE_DET_IRQ_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FRE_DET_IRQ_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FRE_DET_IRQ_EN_A::ENABLE
    }
}
#[doc = "Field `fre_det_irq_en` writer - Frequence Detect IRQ Enable"]
pub type FRE_DET_IRQ_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FRE_DET_CTRL_SPEC, FRE_DET_IRQ_EN_A, O>;
impl<'a, const O: u8> FRE_DET_IRQ_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FRE_DET_IRQ_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FRE_DET_IRQ_EN_A::ENABLE)
    }
}
#[doc = "Field `det_time` reader - Detect Time"]
pub type DET_TIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `det_time` writer - Detect Time"]
pub type DET_TIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FRE_DET_CTRL_SPEC, u8, u8, 5, O>;
#[doc = "Field `error_flag` reader - Error Flag"]
pub type ERROR_FLAG_R = crate::BitReader<ERROR_FLAG_A>;
#[doc = "Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERROR_FLAG_A {
    #[doc = "0: `0`"]
    W0C = 0,
    #[doc = "1: `1`"]
    ERROR = 1,
}
impl From<ERROR_FLAG_A> for bool {
    #[inline(always)]
    fn from(variant: ERROR_FLAG_A) -> Self {
        variant as u8 != 0
    }
}
impl ERROR_FLAG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERROR_FLAG_A {
        match self.bits {
            false => ERROR_FLAG_A::W0C,
            true => ERROR_FLAG_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `W0C`"]
    #[inline(always)]
    pub fn is_w0c(&self) -> bool {
        *self == ERROR_FLAG_A::W0C
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == ERROR_FLAG_A::ERROR
    }
}
#[doc = "Field `error_flag` writer - Error Flag"]
pub type ERROR_FLAG_W<'a, const O: u8> =
    crate::BitWriter0C<'a, u32, FRE_DET_CTRL_SPEC, ERROR_FLAG_A, O>;
impl<'a, const O: u8> ERROR_FLAG_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn w0c(self) -> &'a mut W {
        self.variant(ERROR_FLAG_A::W0C)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(ERROR_FLAG_A::ERROR)
    }
}
impl R {
    #[doc = "Bit 0 - Frequence Detect Function Enable"]
    #[inline(always)]
    pub fn fre_det_fun_en(&self) -> FRE_DET_FUN_EN_R {
        FRE_DET_FUN_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frequence Detect IRQ Enable"]
    #[inline(always)]
    pub fn fre_det_irq_en(&self) -> FRE_DET_IRQ_EN_R {
        FRE_DET_IRQ_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:8 - Detect Time"]
    #[inline(always)]
    pub fn det_time(&self) -> DET_TIME_R {
        DET_TIME_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Error Flag"]
    #[inline(always)]
    pub fn error_flag(&self) -> ERROR_FLAG_R {
        ERROR_FLAG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Frequence Detect Function Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fre_det_fun_en(&mut self) -> FRE_DET_FUN_EN_W<0> {
        FRE_DET_FUN_EN_W::new(self)
    }
    #[doc = "Bit 1 - Frequence Detect IRQ Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fre_det_irq_en(&mut self) -> FRE_DET_IRQ_EN_W<1> {
        FRE_DET_IRQ_EN_W::new(self)
    }
    #[doc = "Bits 4:8 - Detect Time"]
    #[inline(always)]
    #[must_use]
    pub fn det_time(&mut self) -> DET_TIME_W<4> {
        DET_TIME_W::new(self)
    }
    #[doc = "Bit 31 - Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn error_flag(&mut self) -> ERROR_FLAG_W<31> {
        ERROR_FLAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frequency Detect Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fre_det_ctrl](index.html) module"]
pub struct FRE_DET_CTRL_SPEC;
impl crate::RegisterSpec for FRE_DET_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fre_det_ctrl::R](R) reader structure"]
impl crate::Readable for FRE_DET_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fre_det_ctrl::W](W) writer structure"]
impl crate::Writable for FRE_DET_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x8000_0000;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets fre_det_ctrl to value 0"]
impl crate::Resettable for FRE_DET_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
