#[doc = "Register `ledc_int_ctrl` reader"]
pub type R = crate::R<LEDC_INT_CTRL_SPEC>;
#[doc = "Register `ledc_int_ctrl` writer"]
pub type W = crate::W<LEDC_INT_CTRL_SPEC>;
#[doc = "Field `led_trans_finish_int_en` reader - "]
pub type LED_TRANS_FINISH_INT_EN_R = crate::BitReader<LED_TRANS_FINISH_INT_EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LED_TRANS_FINISH_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<LED_TRANS_FINISH_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: LED_TRANS_FINISH_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl LED_TRANS_FINISH_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LED_TRANS_FINISH_INT_EN_A {
        match self.bits {
            false => LED_TRANS_FINISH_INT_EN_A::DISABLE,
            true => LED_TRANS_FINISH_INT_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LED_TRANS_FINISH_INT_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LED_TRANS_FINISH_INT_EN_A::ENABLE
    }
}
#[doc = "Field `led_trans_finish_int_en` writer - "]
pub type LED_TRANS_FINISH_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG, LED_TRANS_FINISH_INT_EN_A>;
impl<'a, REG> LED_TRANS_FINISH_INT_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(LED_TRANS_FINISH_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(LED_TRANS_FINISH_INT_EN_A::ENABLE)
    }
}
#[doc = "Field `fifo_cpureq_int_en` reader - "]
pub type FIFO_CPUREQ_INT_EN_R = crate::BitReader<FIFO_CPUREQ_INT_EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFO_CPUREQ_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<FIFO_CPUREQ_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_CPUREQ_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFO_CPUREQ_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FIFO_CPUREQ_INT_EN_A {
        match self.bits {
            false => FIFO_CPUREQ_INT_EN_A::DISABLE,
            true => FIFO_CPUREQ_INT_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FIFO_CPUREQ_INT_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FIFO_CPUREQ_INT_EN_A::ENABLE
    }
}
#[doc = "Field `fifo_cpureq_int_en` writer - "]
pub type FIFO_CPUREQ_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG, FIFO_CPUREQ_INT_EN_A>;
impl<'a, REG> FIFO_CPUREQ_INT_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(FIFO_CPUREQ_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(FIFO_CPUREQ_INT_EN_A::ENABLE)
    }
}
#[doc = "Field `waitdata_timeout_int_en` reader - "]
pub type WAITDATA_TIMEOUT_INT_EN_R = crate::BitReader<WAITDATA_TIMEOUT_INT_EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAITDATA_TIMEOUT_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<WAITDATA_TIMEOUT_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: WAITDATA_TIMEOUT_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl WAITDATA_TIMEOUT_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WAITDATA_TIMEOUT_INT_EN_A {
        match self.bits {
            false => WAITDATA_TIMEOUT_INT_EN_A::DISABLE,
            true => WAITDATA_TIMEOUT_INT_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WAITDATA_TIMEOUT_INT_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WAITDATA_TIMEOUT_INT_EN_A::ENABLE
    }
}
#[doc = "Field `waitdata_timeout_int_en` writer - "]
pub type WAITDATA_TIMEOUT_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG, WAITDATA_TIMEOUT_INT_EN_A>;
impl<'a, REG> WAITDATA_TIMEOUT_INT_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(WAITDATA_TIMEOUT_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(WAITDATA_TIMEOUT_INT_EN_A::ENABLE)
    }
}
#[doc = "Field `fifo_overflow_int_en` reader - "]
pub type FIFO_OVERFLOW_INT_EN_R = crate::BitReader<FIFO_OVERFLOW_INT_EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFO_OVERFLOW_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<FIFO_OVERFLOW_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_OVERFLOW_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFO_OVERFLOW_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FIFO_OVERFLOW_INT_EN_A {
        match self.bits {
            false => FIFO_OVERFLOW_INT_EN_A::DISABLE,
            true => FIFO_OVERFLOW_INT_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FIFO_OVERFLOW_INT_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FIFO_OVERFLOW_INT_EN_A::ENABLE
    }
}
#[doc = "Field `fifo_overflow_int_en` writer - "]
pub type FIFO_OVERFLOW_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG, FIFO_OVERFLOW_INT_EN_A>;
impl<'a, REG> FIFO_OVERFLOW_INT_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(FIFO_OVERFLOW_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(FIFO_OVERFLOW_INT_EN_A::ENABLE)
    }
}
#[doc = "Field `global_int_en` reader - "]
pub type GLOBAL_INT_EN_R = crate::BitReader<GLOBAL_INT_EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GLOBAL_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<GLOBAL_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: GLOBAL_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl GLOBAL_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GLOBAL_INT_EN_A {
        match self.bits {
            false => GLOBAL_INT_EN_A::DISABLE,
            true => GLOBAL_INT_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GLOBAL_INT_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GLOBAL_INT_EN_A::ENABLE
    }
}
#[doc = "Field `global_int_en` writer - "]
pub type GLOBAL_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG, GLOBAL_INT_EN_A>;
impl<'a, REG> GLOBAL_INT_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(GLOBAL_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(GLOBAL_INT_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn led_trans_finish_int_en(&self) -> LED_TRANS_FINISH_INT_EN_R {
        LED_TRANS_FINISH_INT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn fifo_cpureq_int_en(&self) -> FIFO_CPUREQ_INT_EN_R {
        FIFO_CPUREQ_INT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn waitdata_timeout_int_en(&self) -> WAITDATA_TIMEOUT_INT_EN_R {
        WAITDATA_TIMEOUT_INT_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fifo_overflow_int_en(&self) -> FIFO_OVERFLOW_INT_EN_R {
        FIFO_OVERFLOW_INT_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn global_int_en(&self) -> GLOBAL_INT_EN_R {
        GLOBAL_INT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn led_trans_finish_int_en(&mut self) -> LED_TRANS_FINISH_INT_EN_W<LEDC_INT_CTRL_SPEC> {
        LED_TRANS_FINISH_INT_EN_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_cpureq_int_en(&mut self) -> FIFO_CPUREQ_INT_EN_W<LEDC_INT_CTRL_SPEC> {
        FIFO_CPUREQ_INT_EN_W::new(self, 1)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn waitdata_timeout_int_en(&mut self) -> WAITDATA_TIMEOUT_INT_EN_W<LEDC_INT_CTRL_SPEC> {
        WAITDATA_TIMEOUT_INT_EN_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_overflow_int_en(&mut self) -> FIFO_OVERFLOW_INT_EN_W<LEDC_INT_CTRL_SPEC> {
        FIFO_OVERFLOW_INT_EN_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn global_int_en(&mut self) -> GLOBAL_INT_EN_W<LEDC_INT_CTRL_SPEC> {
        GLOBAL_INT_EN_W::new(self, 5)
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
#[doc = "LEDC Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ledc_int_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ledc_int_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LEDC_INT_CTRL_SPEC;
impl crate::RegisterSpec for LEDC_INT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ledc_int_ctrl::R`](R) reader structure"]
impl crate::Readable for LEDC_INT_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ledc_int_ctrl::W`](W) writer structure"]
impl crate::Writable for LEDC_INT_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ledc_int_ctrl to value 0"]
impl crate::Resettable for LEDC_INT_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
