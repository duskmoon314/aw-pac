#[doc = "Register `gp_datal_ints` reader"]
pub type R = crate::R<GP_DATAL_INTS_SPEC>;
#[doc = "Register `gp_datal_ints` writer"]
pub type W = crate::W<GP_DATAL_INTS_SPEC>;
#[doc = "Field `ch_low_pengding[0-1]` reader - Channel Voltage Low Available Interrupt Status"]
pub type CH_LOW_PENGDING_R = crate::BitReader<CH_LOW_PENGDING_A>;
#[doc = "Channel Voltage Low Available Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH_LOW_PENGDING_A {
    #[doc = "0: NO Pending IRQ"]
    NO_PENDING = 0,
    #[doc = "1: Channel Voltage Low Available Pending IRQ"]
    PENDING = 1,
}
impl From<CH_LOW_PENGDING_A> for bool {
    #[inline(always)]
    fn from(variant: CH_LOW_PENGDING_A) -> Self {
        variant as u8 != 0
    }
}
impl CH_LOW_PENGDING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH_LOW_PENGDING_A {
        match self.bits {
            false => CH_LOW_PENGDING_A::NO_PENDING,
            true => CH_LOW_PENGDING_A::PENDING,
        }
    }
    #[doc = "NO Pending IRQ"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == CH_LOW_PENGDING_A::NO_PENDING
    }
    #[doc = "Channel Voltage Low Available Pending IRQ"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == CH_LOW_PENGDING_A::PENDING
    }
}
#[doc = "Field `ch_low_pengding[0-1]` writer - Channel Voltage Low Available Interrupt Status"]
pub type CH_LOW_PENGDING_W<'a, REG> = crate::BitWriter1C<'a, REG, CH_LOW_PENGDING_A>;
impl<'a, REG> CH_LOW_PENGDING_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO Pending IRQ"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut crate::W<REG> {
        self.variant(CH_LOW_PENGDING_A::NO_PENDING)
    }
    #[doc = "Channel Voltage Low Available Pending IRQ"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(CH_LOW_PENGDING_A::PENDING)
    }
}
impl R {
    #[doc = "Channel Voltage Low Available Interrupt Status\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `ch0_low_pengding` field"]
    #[inline(always)]
    pub fn ch_low_pengding(&self, n: u8) -> CH_LOW_PENGDING_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_LOW_PENGDING_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Channel Voltage Low Available Interrupt Status"]
    #[inline(always)]
    pub fn ch0_low_pengding(&self) -> CH_LOW_PENGDING_R {
        CH_LOW_PENGDING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Voltage Low Available Interrupt Status"]
    #[inline(always)]
    pub fn ch1_low_pengding(&self) -> CH_LOW_PENGDING_R {
        CH_LOW_PENGDING_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Channel Voltage Low Available Interrupt Status\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `ch0_low_pengding` field"]
    #[inline(always)]
    #[must_use]
    pub fn ch_low_pengding(&mut self, n: u8) -> CH_LOW_PENGDING_W<GP_DATAL_INTS_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_LOW_PENGDING_W::new(self, n)
    }
    #[doc = "Bit 0 - Channel Voltage Low Available Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_low_pengding(&mut self) -> CH_LOW_PENGDING_W<GP_DATAL_INTS_SPEC> {
        CH_LOW_PENGDING_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel Voltage Low Available Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_low_pengding(&mut self) -> CH_LOW_PENGDING_W<GP_DATAL_INTS_SPEC> {
        CH_LOW_PENGDING_W::new(self, 1)
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
#[doc = "GPADC Data Low Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp_datal_ints::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp_datal_ints::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GP_DATAL_INTS_SPEC;
impl crate::RegisterSpec for GP_DATAL_INTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gp_datal_ints::R`](R) reader structure"]
impl crate::Readable for GP_DATAL_INTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gp_datal_ints::W`](W) writer structure"]
impl crate::Writable for GP_DATAL_INTS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x01;
}
#[doc = "`reset()` method sets gp_datal_ints to value 0"]
impl crate::Resettable for GP_DATAL_INTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
