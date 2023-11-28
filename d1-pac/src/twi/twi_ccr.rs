#[doc = "Register `twi_ccr` reader"]
pub type R = crate::R<TWI_CCR_SPEC>;
#[doc = "Register `twi_ccr` writer"]
pub type W = crate::W<TWI_CCR_SPEC>;
#[doc = "Field `clk_n` reader - "]
pub type CLK_N_R = crate::FieldReader;
#[doc = "Field `clk_n` writer - "]
pub type CLK_N_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `clk_m` reader - "]
pub type CLK_M_R = crate::FieldReader;
#[doc = "Field `clk_m` writer - "]
pub type CLK_M_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `clk_duty` reader - Setting duty cycle of clock as master"]
pub type CLK_DUTY_R = crate::BitReader<CLK_DUTY_A>;
#[doc = "Setting duty cycle of clock as master\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLK_DUTY_A {
    #[doc = "0: 50%"]
    P50 = 0,
    #[doc = "1: 40%"]
    P40 = 1,
}
impl From<CLK_DUTY_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_DUTY_A) -> Self {
        variant as u8 != 0
    }
}
impl CLK_DUTY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLK_DUTY_A {
        match self.bits {
            false => CLK_DUTY_A::P50,
            true => CLK_DUTY_A::P40,
        }
    }
    #[doc = "50%"]
    #[inline(always)]
    pub fn is_p50(&self) -> bool {
        *self == CLK_DUTY_A::P50
    }
    #[doc = "40%"]
    #[inline(always)]
    pub fn is_p40(&self) -> bool {
        *self == CLK_DUTY_A::P40
    }
}
#[doc = "Field `clk_duty` writer - Setting duty cycle of clock as master"]
pub type CLK_DUTY_W<'a, REG> = crate::BitWriter<'a, REG, CLK_DUTY_A>;
impl<'a, REG> CLK_DUTY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "50%"]
    #[inline(always)]
    pub fn p50(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_DUTY_A::P50)
    }
    #[doc = "40%"]
    #[inline(always)]
    pub fn p40(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_DUTY_A::P40)
    }
}
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn clk_n(&self) -> CLK_N_R {
        CLK_N_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:6"]
    #[inline(always)]
    pub fn clk_m(&self) -> CLK_M_R {
        CLK_M_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Setting duty cycle of clock as master"]
    #[inline(always)]
    pub fn clk_duty(&self) -> CLK_DUTY_R {
        CLK_DUTY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn clk_n(&mut self) -> CLK_N_W<TWI_CCR_SPEC> {
        CLK_N_W::new(self, 0)
    }
    #[doc = "Bits 3:6"]
    #[inline(always)]
    #[must_use]
    pub fn clk_m(&mut self) -> CLK_M_W<TWI_CCR_SPEC> {
        CLK_M_W::new(self, 3)
    }
    #[doc = "Bit 7 - Setting duty cycle of clock as master"]
    #[inline(always)]
    #[must_use]
    pub fn clk_duty(&mut self) -> CLK_DUTY_W<TWI_CCR_SPEC> {
        CLK_DUTY_W::new(self, 7)
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
#[doc = "TWI Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twi_ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twi_ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWI_CCR_SPEC;
impl crate::RegisterSpec for TWI_CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`twi_ccr::R`](R) reader structure"]
impl crate::Readable for TWI_CCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`twi_ccr::W`](W) writer structure"]
impl crate::Writable for TWI_CCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets twi_ccr to value 0"]
impl crate::Resettable for TWI_CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
