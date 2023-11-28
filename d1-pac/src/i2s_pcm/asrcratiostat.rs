#[doc = "Register `asrcratiostat` reader"]
pub type R = crate::R<ASRCRATIOSTAT_SPEC>;
#[doc = "Register `asrcratiostat` writer"]
pub type W = crate::W<ASRCRATIOSTAT_SPEC>;
#[doc = "Field `adapt_comput_value` reader - Adaptive Ratio Computational Value"]
pub type ADAPT_COMPUT_VALUE_R = crate::FieldReader<u32>;
#[doc = "Field `adapt_comput_value` writer - Adaptive Ratio Computational Value"]
pub type ADAPT_COMPUT_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
#[doc = "Field `adapt_comput_lock` reader - Adaptive Ratio Computational Lock"]
pub type ADAPT_COMPUT_LOCK_R = crate::BitReader<ADAPT_COMPUT_LOCK_A>;
#[doc = "Adaptive Ratio Computational Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADAPT_COMPUT_LOCK_A {
    #[doc = "0: Disable"]
    UNLOCKED = 0,
    #[doc = "1: Enable"]
    LOCKED = 1,
}
impl From<ADAPT_COMPUT_LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: ADAPT_COMPUT_LOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl ADAPT_COMPUT_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADAPT_COMPUT_LOCK_A {
        match self.bits {
            false => ADAPT_COMPUT_LOCK_A::UNLOCKED,
            true => ADAPT_COMPUT_LOCK_A::LOCKED,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == ADAPT_COMPUT_LOCK_A::UNLOCKED
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == ADAPT_COMPUT_LOCK_A::LOCKED
    }
}
#[doc = "Field `adapt_comput_lock` writer - Adaptive Ratio Computational Lock"]
pub type ADAPT_COMPUT_LOCK_W<'a, REG> = crate::BitWriter<'a, REG, ADAPT_COMPUT_LOCK_A>;
impl<'a, REG> ADAPT_COMPUT_LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(ADAPT_COMPUT_LOCK_A::UNLOCKED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(ADAPT_COMPUT_LOCK_A::LOCKED)
    }
}
#[doc = "Field `asrc_buf_overflow_sta` reader - ASRC Receive Data Buffer Overflow State\n\nIt can control thye mute with lock."]
pub type ASRC_BUF_OVERFLOW_STA_R = crate::BitReader<ASRC_BUF_OVERFLOW_STA_A>;
#[doc = "ASRC Receive Data Buffer Overflow State\n\nIt can control thye mute with lock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASRC_BUF_OVERFLOW_STA_A {
    #[doc = "0: Disable"]
    NO_OVERFLOW = 0,
    #[doc = "1: Enable"]
    OVERFLOW = 1,
}
impl From<ASRC_BUF_OVERFLOW_STA_A> for bool {
    #[inline(always)]
    fn from(variant: ASRC_BUF_OVERFLOW_STA_A) -> Self {
        variant as u8 != 0
    }
}
impl ASRC_BUF_OVERFLOW_STA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ASRC_BUF_OVERFLOW_STA_A {
        match self.bits {
            false => ASRC_BUF_OVERFLOW_STA_A::NO_OVERFLOW,
            true => ASRC_BUF_OVERFLOW_STA_A::OVERFLOW,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == ASRC_BUF_OVERFLOW_STA_A::NO_OVERFLOW
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == ASRC_BUF_OVERFLOW_STA_A::OVERFLOW
    }
}
#[doc = "Field `asrc_buf_overflow_sta` writer - ASRC Receive Data Buffer Overflow State\n\nIt can control thye mute with lock."]
pub type ASRC_BUF_OVERFLOW_STA_W<'a, REG> = crate::BitWriter<'a, REG, ASRC_BUF_OVERFLOW_STA_A>;
impl<'a, REG> ASRC_BUF_OVERFLOW_STA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn no_overflow(self) -> &'a mut crate::W<REG> {
        self.variant(ASRC_BUF_OVERFLOW_STA_A::NO_OVERFLOW)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(ASRC_BUF_OVERFLOW_STA_A::OVERFLOW)
    }
}
impl R {
    #[doc = "Bits 0:25 - Adaptive Ratio Computational Value"]
    #[inline(always)]
    pub fn adapt_comput_value(&self) -> ADAPT_COMPUT_VALUE_R {
        ADAPT_COMPUT_VALUE_R::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bit 28 - Adaptive Ratio Computational Lock"]
    #[inline(always)]
    pub fn adapt_comput_lock(&self) -> ADAPT_COMPUT_LOCK_R {
        ADAPT_COMPUT_LOCK_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ASRC Receive Data Buffer Overflow State\n\nIt can control thye mute with lock."]
    #[inline(always)]
    pub fn asrc_buf_overflow_sta(&self) -> ASRC_BUF_OVERFLOW_STA_R {
        ASRC_BUF_OVERFLOW_STA_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:25 - Adaptive Ratio Computational Value"]
    #[inline(always)]
    #[must_use]
    pub fn adapt_comput_value(&mut self) -> ADAPT_COMPUT_VALUE_W<ASRCRATIOSTAT_SPEC> {
        ADAPT_COMPUT_VALUE_W::new(self, 0)
    }
    #[doc = "Bit 28 - Adaptive Ratio Computational Lock"]
    #[inline(always)]
    #[must_use]
    pub fn adapt_comput_lock(&mut self) -> ADAPT_COMPUT_LOCK_W<ASRCRATIOSTAT_SPEC> {
        ADAPT_COMPUT_LOCK_W::new(self, 28)
    }
    #[doc = "Bit 29 - ASRC Receive Data Buffer Overflow State\n\nIt can control thye mute with lock."]
    #[inline(always)]
    #[must_use]
    pub fn asrc_buf_overflow_sta(&mut self) -> ASRC_BUF_OVERFLOW_STA_W<ASRCRATIOSTAT_SPEC> {
        ASRC_BUF_OVERFLOW_STA_W::new(self, 29)
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
#[doc = "ASRC Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asrcratiostat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asrcratiostat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ASRCRATIOSTAT_SPEC;
impl crate::RegisterSpec for ASRCRATIOSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`asrcratiostat::R`](R) reader structure"]
impl crate::Readable for ASRCRATIOSTAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`asrcratiostat::W`](W) writer structure"]
impl crate::Writable for ASRCRATIOSTAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets asrcratiostat to value 0"]
impl crate::Resettable for ASRCRATIOSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
