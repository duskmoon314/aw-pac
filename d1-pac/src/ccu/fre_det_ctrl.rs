#[doc = "Register `fre_det_ctrl` reader"]
pub type R = crate::R<FRE_DET_CTRL_SPEC>;
#[doc = "Register `fre_det_ctrl` writer"]
pub type W = crate::W<FRE_DET_CTRL_SPEC>;
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
    pub const fn variant(&self) -> FRE_DET_FUN_EN_A {
        match self.bits {
            false => FRE_DET_FUN_EN_A::DISABLE,
            true => FRE_DET_FUN_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FRE_DET_FUN_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FRE_DET_FUN_EN_A::ENABLE
    }
}
#[doc = "Field `fre_det_fun_en` writer - Frequence Detect Function Enable"]
pub type FRE_DET_FUN_EN_W<'a, REG> = crate::BitWriter<'a, REG, FRE_DET_FUN_EN_A>;
impl<'a, REG> FRE_DET_FUN_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(FRE_DET_FUN_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> FRE_DET_IRQ_EN_A {
        match self.bits {
            false => FRE_DET_IRQ_EN_A::DISABLE,
            true => FRE_DET_IRQ_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FRE_DET_IRQ_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FRE_DET_IRQ_EN_A::ENABLE
    }
}
#[doc = "Field `fre_det_irq_en` writer - Frequence Detect IRQ Enable"]
pub type FRE_DET_IRQ_EN_W<'a, REG> = crate::BitWriter<'a, REG, FRE_DET_IRQ_EN_A>;
impl<'a, REG> FRE_DET_IRQ_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(FRE_DET_IRQ_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(FRE_DET_IRQ_EN_A::ENABLE)
    }
}
#[doc = "Field `det_time` reader - Detect Time"]
pub type DET_TIME_R = crate::FieldReader;
#[doc = "Field `det_time` writer - Detect Time"]
pub type DET_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
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
    pub const fn variant(&self) -> ERROR_FLAG_A {
        match self.bits {
            false => ERROR_FLAG_A::W0C,
            true => ERROR_FLAG_A::ERROR,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_w0c(&self) -> bool {
        *self == ERROR_FLAG_A::W0C
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == ERROR_FLAG_A::ERROR
    }
}
#[doc = "Field `error_flag` writer - Error Flag"]
pub type ERROR_FLAG_W<'a, REG> = crate::BitWriter0C<'a, REG, ERROR_FLAG_A>;
impl<'a, REG> ERROR_FLAG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn w0c(self) -> &'a mut crate::W<REG> {
        self.variant(ERROR_FLAG_A::W0C)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
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
    pub fn fre_det_fun_en(&mut self) -> FRE_DET_FUN_EN_W<FRE_DET_CTRL_SPEC> {
        FRE_DET_FUN_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Frequence Detect IRQ Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fre_det_irq_en(&mut self) -> FRE_DET_IRQ_EN_W<FRE_DET_CTRL_SPEC> {
        FRE_DET_IRQ_EN_W::new(self, 1)
    }
    #[doc = "Bits 4:8 - Detect Time"]
    #[inline(always)]
    #[must_use]
    pub fn det_time(&mut self) -> DET_TIME_W<FRE_DET_CTRL_SPEC> {
        DET_TIME_W::new(self, 4)
    }
    #[doc = "Bit 31 - Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn error_flag(&mut self) -> ERROR_FLAG_W<FRE_DET_CTRL_SPEC> {
        ERROR_FLAG_W::new(self, 31)
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
#[doc = "Frequency Detect Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fre_det_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fre_det_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRE_DET_CTRL_SPEC;
impl crate::RegisterSpec for FRE_DET_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fre_det_ctrl::R`](R) reader structure"]
impl crate::Readable for FRE_DET_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fre_det_ctrl::W`](W) writer structure"]
impl crate::Writable for FRE_DET_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x8000_0000;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets fre_det_ctrl to value 0"]
impl crate::Resettable for FRE_DET_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
