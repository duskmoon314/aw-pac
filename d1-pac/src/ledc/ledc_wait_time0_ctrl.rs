#[doc = "Register `ledc_wait_time0_ctrl` reader"]
pub type R = crate::R<LEDC_WAIT_TIME0_CTRL_SPEC>;
#[doc = "Register `ledc_wait_time0_ctrl` writer"]
pub type W = crate::W<LEDC_WAIT_TIME0_CTRL_SPEC>;
#[doc = "Field `total_wait_time0` reader - "]
pub type TOTAL_WAIT_TIME0_R = crate::FieldReader;
#[doc = "Field `total_wait_time0` writer - "]
pub type TOTAL_WAIT_TIME0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `wait_tim0_en` reader - "]
pub type WAIT_TIM0_EN_R = crate::BitReader<WAIT_TIM0_EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAIT_TIM0_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<WAIT_TIM0_EN_A> for bool {
    #[inline(always)]
    fn from(variant: WAIT_TIM0_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl WAIT_TIM0_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WAIT_TIM0_EN_A {
        match self.bits {
            false => WAIT_TIM0_EN_A::DISABLE,
            true => WAIT_TIM0_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WAIT_TIM0_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WAIT_TIM0_EN_A::ENABLE
    }
}
#[doc = "Field `wait_tim0_en` writer - "]
pub type WAIT_TIM0_EN_W<'a, REG> = crate::BitWriter<'a, REG, WAIT_TIM0_EN_A>;
impl<'a, REG> WAIT_TIM0_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(WAIT_TIM0_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(WAIT_TIM0_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn total_wait_time0(&self) -> TOTAL_WAIT_TIME0_R {
        TOTAL_WAIT_TIME0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn wait_tim0_en(&self) -> WAIT_TIM0_EN_R {
        WAIT_TIM0_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn total_wait_time0(&mut self) -> TOTAL_WAIT_TIME0_W<LEDC_WAIT_TIME0_CTRL_SPEC> {
        TOTAL_WAIT_TIME0_W::new(self, 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn wait_tim0_en(&mut self) -> WAIT_TIM0_EN_W<LEDC_WAIT_TIME0_CTRL_SPEC> {
        WAIT_TIM0_EN_W::new(self, 8)
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
#[doc = "LEDC Wait Time0 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ledc_wait_time0_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ledc_wait_time0_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LEDC_WAIT_TIME0_CTRL_SPEC;
impl crate::RegisterSpec for LEDC_WAIT_TIME0_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ledc_wait_time0_ctrl::R`](R) reader structure"]
impl crate::Readable for LEDC_WAIT_TIME0_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ledc_wait_time0_ctrl::W`](W) writer structure"]
impl crate::Writable for LEDC_WAIT_TIME0_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ledc_wait_time0_ctrl to value 0"]
impl crate::Resettable for LEDC_WAIT_TIME0_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
