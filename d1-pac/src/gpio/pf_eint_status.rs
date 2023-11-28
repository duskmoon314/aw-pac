#[doc = "Register `pf_eint_status` reader"]
pub type R = crate::R<PF_EINT_STATUS_SPEC>;
#[doc = "Register `pf_eint_status` writer"]
pub type W = crate::W<PF_EINT_STATUS_SPEC>;
#[doc = "Field `eint_status[0-6]` reader - External INT Pending Bit"]
pub type EINT_STATUS_R = crate::BitReader<EINT_STATUS_A>;
#[doc = "External INT Pending Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EINT_STATUS_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<EINT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: EINT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl EINT_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EINT_STATUS_A {
        match self.bits {
            false => EINT_STATUS_A::NO_PENDING,
            true => EINT_STATUS_A::PENDING,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == EINT_STATUS_A::NO_PENDING
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == EINT_STATUS_A::PENDING
    }
}
#[doc = "Field `eint_status[0-6]` writer - External INT Pending Bit"]
pub type EINT_STATUS_W<'a, REG> = crate::BitWriter<'a, REG, EINT_STATUS_A>;
impl<'a, REG> EINT_STATUS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut crate::W<REG> {
        self.variant(EINT_STATUS_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(EINT_STATUS_A::PENDING)
    }
}
impl R {
    #[doc = "External INT Pending Bit\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `eint0_status` field"]
    #[inline(always)]
    pub fn eint_status(&self, n: u8) -> EINT_STATUS_R {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        EINT_STATUS_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint0_status(&self) -> EINT_STATUS_R {
        EINT_STATUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint1_status(&self) -> EINT_STATUS_R {
        EINT_STATUS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint2_status(&self) -> EINT_STATUS_R {
        EINT_STATUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint3_status(&self) -> EINT_STATUS_R {
        EINT_STATUS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint4_status(&self) -> EINT_STATUS_R {
        EINT_STATUS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint5_status(&self) -> EINT_STATUS_R {
        EINT_STATUS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint6_status(&self) -> EINT_STATUS_R {
        EINT_STATUS_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "External INT Pending Bit\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `eint0_status` field"]
    #[inline(always)]
    #[must_use]
    pub fn eint_status(&mut self, n: u8) -> EINT_STATUS_W<PF_EINT_STATUS_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        EINT_STATUS_W::new(self, n)
    }
    #[doc = "Bit 0 - External INT Pending Bit"]
    #[inline(always)]
    #[must_use]
    pub fn eint0_status(&mut self) -> EINT_STATUS_W<PF_EINT_STATUS_SPEC> {
        EINT_STATUS_W::new(self, 0)
    }
    #[doc = "Bit 1 - External INT Pending Bit"]
    #[inline(always)]
    #[must_use]
    pub fn eint1_status(&mut self) -> EINT_STATUS_W<PF_EINT_STATUS_SPEC> {
        EINT_STATUS_W::new(self, 1)
    }
    #[doc = "Bit 2 - External INT Pending Bit"]
    #[inline(always)]
    #[must_use]
    pub fn eint2_status(&mut self) -> EINT_STATUS_W<PF_EINT_STATUS_SPEC> {
        EINT_STATUS_W::new(self, 2)
    }
    #[doc = "Bit 3 - External INT Pending Bit"]
    #[inline(always)]
    #[must_use]
    pub fn eint3_status(&mut self) -> EINT_STATUS_W<PF_EINT_STATUS_SPEC> {
        EINT_STATUS_W::new(self, 3)
    }
    #[doc = "Bit 4 - External INT Pending Bit"]
    #[inline(always)]
    #[must_use]
    pub fn eint4_status(&mut self) -> EINT_STATUS_W<PF_EINT_STATUS_SPEC> {
        EINT_STATUS_W::new(self, 4)
    }
    #[doc = "Bit 5 - External INT Pending Bit"]
    #[inline(always)]
    #[must_use]
    pub fn eint5_status(&mut self) -> EINT_STATUS_W<PF_EINT_STATUS_SPEC> {
        EINT_STATUS_W::new(self, 5)
    }
    #[doc = "Bit 6 - External INT Pending Bit"]
    #[inline(always)]
    #[must_use]
    pub fn eint6_status(&mut self) -> EINT_STATUS_W<PF_EINT_STATUS_SPEC> {
        EINT_STATUS_W::new(self, 6)
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
#[doc = "PF External Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pf_eint_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pf_eint_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PF_EINT_STATUS_SPEC;
impl crate::RegisterSpec for PF_EINT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pf_eint_status::R`](R) reader structure"]
impl crate::Readable for PF_EINT_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pf_eint_status::W`](W) writer structure"]
impl crate::Writable for PF_EINT_STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pf_eint_status to value 0"]
impl crate::Resettable for PF_EINT_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
