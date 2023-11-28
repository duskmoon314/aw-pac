#[doc = "Register `ctrl` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `ctrl` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `ctrl` reader - PLIC Control"]
pub type CTRL_R = crate::BitReader<CTRL_A>;
#[doc = "PLIC Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTRL_A {
    #[doc = "0: Only the machine mode can access to all registers in PLIC. Supervisor mode can only access the interrupt threshold register and the interrupt response/completion register."]
    M = 0,
    #[doc = "1: The machine mode and the supervisor mode can access all registers. CTRL is accessible only in the machine mode."]
    MS = 1,
}
impl From<CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: CTRL_A) -> Self {
        variant as u8 != 0
    }
}
impl CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CTRL_A {
        match self.bits {
            false => CTRL_A::M,
            true => CTRL_A::MS,
        }
    }
    #[doc = "Only the machine mode can access to all registers in PLIC. Supervisor mode can only access the interrupt threshold register and the interrupt response/completion register."]
    #[inline(always)]
    pub fn is_m(&self) -> bool {
        *self == CTRL_A::M
    }
    #[doc = "The machine mode and the supervisor mode can access all registers. CTRL is accessible only in the machine mode."]
    #[inline(always)]
    pub fn is_ms(&self) -> bool {
        *self == CTRL_A::MS
    }
}
#[doc = "Field `ctrl` writer - PLIC Control"]
pub type CTRL_W<'a, REG> = crate::BitWriter<'a, REG, CTRL_A>;
impl<'a, REG> CTRL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Only the machine mode can access to all registers in PLIC. Supervisor mode can only access the interrupt threshold register and the interrupt response/completion register."]
    #[inline(always)]
    pub fn m(self) -> &'a mut crate::W<REG> {
        self.variant(CTRL_A::M)
    }
    #[doc = "The machine mode and the supervisor mode can access all registers. CTRL is accessible only in the machine mode."]
    #[inline(always)]
    pub fn ms(self) -> &'a mut crate::W<REG> {
        self.variant(CTRL_A::MS)
    }
}
impl R {
    #[doc = "Bit 0 - PLIC Control"]
    #[inline(always)]
    pub fn ctrl(&self) -> CTRL_R {
        CTRL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLIC Control"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl(&mut self) -> CTRL_W<CTRL_SPEC> {
        CTRL_W::new(self, 0)
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
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ctrl to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
