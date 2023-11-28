#[doc = "Register `ccr%s` reader"]
pub type R = crate::R<CCR_SPEC>;
#[doc = "Register `ccr%s` writer"]
pub type W = crate::W<CCR_SPEC>;
#[doc = "Field `capinv` reader - Inverse the signal input from capture channel before 16-bit counter of capture channel."]
pub type CAPINV_R = crate::BitReader<CAPINV_A>;
#[doc = "Inverse the signal input from capture channel before 16-bit counter of capture channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAPINV_A {
    #[doc = "0: not inverse"]
    NOT_INVERSE = 0,
    #[doc = "1: inverse"]
    INVERSE = 1,
}
impl From<CAPINV_A> for bool {
    #[inline(always)]
    fn from(variant: CAPINV_A) -> Self {
        variant as u8 != 0
    }
}
impl CAPINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CAPINV_A {
        match self.bits {
            false => CAPINV_A::NOT_INVERSE,
            true => CAPINV_A::INVERSE,
        }
    }
    #[doc = "not inverse"]
    #[inline(always)]
    pub fn is_not_inverse(&self) -> bool {
        *self == CAPINV_A::NOT_INVERSE
    }
    #[doc = "inverse"]
    #[inline(always)]
    pub fn is_inverse(&self) -> bool {
        *self == CAPINV_A::INVERSE
    }
}
#[doc = "Field `capinv` writer - Inverse the signal input from capture channel before 16-bit counter of capture channel."]
pub type CAPINV_W<'a, REG> = crate::BitWriter<'a, REG, CAPINV_A>;
impl<'a, REG> CAPINV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "not inverse"]
    #[inline(always)]
    pub fn not_inverse(self) -> &'a mut crate::W<REG> {
        self.variant(CAPINV_A::NOT_INVERSE)
    }
    #[doc = "inverse"]
    #[inline(always)]
    pub fn inverse(self) -> &'a mut crate::W<REG> {
        self.variant(CAPINV_A::INVERSE)
    }
}
#[doc = "Field `cfte` reader - Falling edge capture trigger enable"]
pub type CFTE_R = crate::BitReader;
#[doc = "Field `cfte` writer - Falling edge capture trigger enable"]
pub type CFTE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `crte` reader - Rising edge capture trigger enable"]
pub type CRTE_R = crate::BitReader;
#[doc = "Field `crte` writer - Rising edge capture trigger enable"]
pub type CRTE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cflf` reader - When the capture channel captures a falling edge, the current value of the 16-bit up-counter is latched to CFLR, and then this bit is set 1 by hardware.\n\nWrite 1 to clear this bit."]
pub type CFLF_R = crate::BitReader;
#[doc = "Field `cflf` writer - When the capture channel captures a falling edge, the current value of the 16-bit up-counter is latched to CFLR, and then this bit is set 1 by hardware.\n\nWrite 1 to clear this bit."]
pub type CFLF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `crlf` reader - When the capture channel captures a rising edge, the current value of the 16-bit up-counter is latched to CRLR, and then this bit is set 1 by hardware.\n\nWrite 1 to clear this bit."]
pub type CRLF_R = crate::BitReader;
#[doc = "Field `crlf` writer - When the capture channel captures a rising edge, the current value of the 16-bit up-counter is latched to CRLR, and then this bit is set 1 by hardware.\n\nWrite 1 to clear this bit."]
pub type CRLF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Inverse the signal input from capture channel before 16-bit counter of capture channel."]
    #[inline(always)]
    pub fn capinv(&self) -> CAPINV_R {
        CAPINV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Falling edge capture trigger enable"]
    #[inline(always)]
    pub fn cfte(&self) -> CFTE_R {
        CFTE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rising edge capture trigger enable"]
    #[inline(always)]
    pub fn crte(&self) -> CRTE_R {
        CRTE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When the capture channel captures a falling edge, the current value of the 16-bit up-counter is latched to CFLR, and then this bit is set 1 by hardware.\n\nWrite 1 to clear this bit."]
    #[inline(always)]
    pub fn cflf(&self) -> CFLF_R {
        CFLF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When the capture channel captures a rising edge, the current value of the 16-bit up-counter is latched to CRLR, and then this bit is set 1 by hardware.\n\nWrite 1 to clear this bit."]
    #[inline(always)]
    pub fn crlf(&self) -> CRLF_R {
        CRLF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Inverse the signal input from capture channel before 16-bit counter of capture channel."]
    #[inline(always)]
    #[must_use]
    pub fn capinv(&mut self) -> CAPINV_W<CCR_SPEC> {
        CAPINV_W::new(self, 0)
    }
    #[doc = "Bit 1 - Falling edge capture trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfte(&mut self) -> CFTE_W<CCR_SPEC> {
        CFTE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Rising edge capture trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn crte(&mut self) -> CRTE_W<CCR_SPEC> {
        CRTE_W::new(self, 2)
    }
    #[doc = "Bit 3 - When the capture channel captures a falling edge, the current value of the 16-bit up-counter is latched to CFLR, and then this bit is set 1 by hardware.\n\nWrite 1 to clear this bit."]
    #[inline(always)]
    #[must_use]
    pub fn cflf(&mut self) -> CFLF_W<CCR_SPEC> {
        CFLF_W::new(self, 3)
    }
    #[doc = "Bit 4 - When the capture channel captures a rising edge, the current value of the 16-bit up-counter is latched to CRLR, and then this bit is set 1 by hardware.\n\nWrite 1 to clear this bit."]
    #[inline(always)]
    #[must_use]
    pub fn crlf(&mut self) -> CRLF_W<CCR_SPEC> {
        CRLF_W::new(self, 4)
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
#[doc = "Capture Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x18;
}
#[doc = "`reset()` method sets ccr%s to value 0"]
impl crate::Resettable for CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
