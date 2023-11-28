#[doc = "Register `i2s_pcm_ista` reader"]
pub type R = crate::R<I2S_PCM_ISTA_SPEC>;
#[doc = "Register `i2s_pcm_ista` writer"]
pub type W = crate::W<I2S_PCM_ISTA_SPEC>;
#[doc = "Field `txe_int` reader - TXFIFO Empty Pending Interrupt"]
pub type TXE_INT_R = crate::BitReader<TXE_INT_A>;
#[doc = "TXFIFO Empty Pending Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXE_INT_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<TXE_INT_A> for bool {
    #[inline(always)]
    fn from(variant: TXE_INT_A) -> Self {
        variant as u8 != 0
    }
}
impl TXE_INT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXE_INT_A {
        match self.bits {
            false => TXE_INT_A::NO_PENDING,
            true => TXE_INT_A::PENDING,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == TXE_INT_A::NO_PENDING
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == TXE_INT_A::PENDING
    }
}
#[doc = "Field `txe_int` writer - TXFIFO Empty Pending Interrupt"]
pub type TXE_INT_W<'a, REG> = crate::BitWriter<'a, REG, TXE_INT_A>;
impl<'a, REG> TXE_INT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut crate::W<REG> {
        self.variant(TXE_INT_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(TXE_INT_A::PENDING)
    }
}
#[doc = "Field `rxa_int` reader - RXFIFO Data Available Pending Interrupt"]
pub type RXA_INT_R = crate::BitReader<RXA_INT_A>;
#[doc = "RXFIFO Data Available Pending Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXA_INT_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<RXA_INT_A> for bool {
    #[inline(always)]
    fn from(variant: RXA_INT_A) -> Self {
        variant as u8 != 0
    }
}
impl RXA_INT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXA_INT_A {
        match self.bits {
            false => RXA_INT_A::NO_PENDING,
            true => RXA_INT_A::PENDING,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == RXA_INT_A::NO_PENDING
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RXA_INT_A::PENDING
    }
}
#[doc = "Field `rxa_int` writer - RXFIFO Data Available Pending Interrupt"]
pub type RXA_INT_W<'a, REG> = crate::BitWriter<'a, REG, RXA_INT_A>;
impl<'a, REG> RXA_INT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut crate::W<REG> {
        self.variant(RXA_INT_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(RXA_INT_A::PENDING)
    }
}
#[doc = "Field `txo_int` reader - TXFIFO Overrun Pending Interrupt"]
pub type TXO_INT_R = crate::BitReader<TXO_INT_A>;
#[doc = "TXFIFO Overrun Pending Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXO_INT_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<TXO_INT_A> for bool {
    #[inline(always)]
    fn from(variant: TXO_INT_A) -> Self {
        variant as u8 != 0
    }
}
impl TXO_INT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXO_INT_A {
        match self.bits {
            false => TXO_INT_A::NO_PENDING,
            true => TXO_INT_A::PENDING,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == TXO_INT_A::NO_PENDING
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == TXO_INT_A::PENDING
    }
}
#[doc = "Field `txo_int` writer - TXFIFO Overrun Pending Interrupt"]
pub type TXO_INT_W<'a, REG> = crate::BitWriter<'a, REG, TXO_INT_A>;
impl<'a, REG> TXO_INT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut crate::W<REG> {
        self.variant(TXO_INT_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(TXO_INT_A::PENDING)
    }
}
#[doc = "Field `rxo_int` reader - RXFIFO Overrun Pending Interrupt"]
pub type RXO_INT_R = crate::BitReader<RXO_INT_A>;
#[doc = "RXFIFO Overrun Pending Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXO_INT_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<RXO_INT_A> for bool {
    #[inline(always)]
    fn from(variant: RXO_INT_A) -> Self {
        variant as u8 != 0
    }
}
impl RXO_INT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXO_INT_A {
        match self.bits {
            false => RXO_INT_A::NO_PENDING,
            true => RXO_INT_A::PENDING,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == RXO_INT_A::NO_PENDING
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RXO_INT_A::PENDING
    }
}
#[doc = "Field `rxo_int` writer - RXFIFO Overrun Pending Interrupt"]
pub type RXO_INT_W<'a, REG> = crate::BitWriter<'a, REG, RXO_INT_A>;
impl<'a, REG> RXO_INT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut crate::W<REG> {
        self.variant(RXO_INT_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(RXO_INT_A::PENDING)
    }
}
#[doc = "Field `txu_int` reader - TXFIFO Underrun Pending Interrupt"]
pub type TXU_INT_R = crate::BitReader<TXU_INT_A>;
#[doc = "TXFIFO Underrun Pending Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXU_INT_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<TXU_INT_A> for bool {
    #[inline(always)]
    fn from(variant: TXU_INT_A) -> Self {
        variant as u8 != 0
    }
}
impl TXU_INT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXU_INT_A {
        match self.bits {
            false => TXU_INT_A::NO_PENDING,
            true => TXU_INT_A::PENDING,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == TXU_INT_A::NO_PENDING
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == TXU_INT_A::PENDING
    }
}
#[doc = "Field `txu_int` writer - TXFIFO Underrun Pending Interrupt"]
pub type TXU_INT_W<'a, REG> = crate::BitWriter<'a, REG, TXU_INT_A>;
impl<'a, REG> TXU_INT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut crate::W<REG> {
        self.variant(TXU_INT_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(TXU_INT_A::PENDING)
    }
}
#[doc = "Field `rxu_int` reader - RXFIFO Underrun Pending Interrupt"]
pub type RXU_INT_R = crate::BitReader<RXU_INT_A>;
#[doc = "RXFIFO Underrun Pending Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXU_INT_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<RXU_INT_A> for bool {
    #[inline(always)]
    fn from(variant: RXU_INT_A) -> Self {
        variant as u8 != 0
    }
}
impl RXU_INT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXU_INT_A {
        match self.bits {
            false => RXU_INT_A::NO_PENDING,
            true => RXU_INT_A::PENDING,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == RXU_INT_A::NO_PENDING
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RXU_INT_A::PENDING
    }
}
#[doc = "Field `rxu_int` writer - RXFIFO Underrun Pending Interrupt"]
pub type RXU_INT_W<'a, REG> = crate::BitWriter<'a, REG, RXU_INT_A>;
impl<'a, REG> RXU_INT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut crate::W<REG> {
        self.variant(RXU_INT_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(RXU_INT_A::PENDING)
    }
}
impl R {
    #[doc = "Bit 4 - TXFIFO Empty Pending Interrupt"]
    #[inline(always)]
    pub fn txe_int(&self) -> TXE_INT_R {
        TXE_INT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 4 - RXFIFO Data Available Pending Interrupt"]
    #[inline(always)]
    pub fn rxa_int(&self) -> RXA_INT_R {
        RXA_INT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TXFIFO Overrun Pending Interrupt"]
    #[inline(always)]
    pub fn txo_int(&self) -> TXO_INT_R {
        TXO_INT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 5 - RXFIFO Overrun Pending Interrupt"]
    #[inline(always)]
    pub fn rxo_int(&self) -> RXO_INT_R {
        RXO_INT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TXFIFO Underrun Pending Interrupt"]
    #[inline(always)]
    pub fn txu_int(&self) -> TXU_INT_R {
        TXU_INT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 6 - RXFIFO Underrun Pending Interrupt"]
    #[inline(always)]
    pub fn rxu_int(&self) -> RXU_INT_R {
        RXU_INT_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - TXFIFO Empty Pending Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txe_int(&mut self) -> TXE_INT_W<I2S_PCM_ISTA_SPEC> {
        TXE_INT_W::new(self, 4)
    }
    #[doc = "Bit 4 - RXFIFO Data Available Pending Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxa_int(&mut self) -> RXA_INT_W<I2S_PCM_ISTA_SPEC> {
        RXA_INT_W::new(self, 4)
    }
    #[doc = "Bit 5 - TXFIFO Overrun Pending Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txo_int(&mut self) -> TXO_INT_W<I2S_PCM_ISTA_SPEC> {
        TXO_INT_W::new(self, 5)
    }
    #[doc = "Bit 5 - RXFIFO Overrun Pending Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxo_int(&mut self) -> RXO_INT_W<I2S_PCM_ISTA_SPEC> {
        RXO_INT_W::new(self, 5)
    }
    #[doc = "Bit 6 - TXFIFO Underrun Pending Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txu_int(&mut self) -> TXU_INT_W<I2S_PCM_ISTA_SPEC> {
        TXU_INT_W::new(self, 6)
    }
    #[doc = "Bit 6 - RXFIFO Underrun Pending Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxu_int(&mut self) -> RXU_INT_W<I2S_PCM_ISTA_SPEC> {
        RXU_INT_W::new(self, 6)
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
#[doc = "I2S/PCM Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_ista::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_ista::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2S_PCM_ISTA_SPEC;
impl crate::RegisterSpec for I2S_PCM_ISTA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_pcm_ista::R`](R) reader structure"]
impl crate::Readable for I2S_PCM_ISTA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2s_pcm_ista::W`](W) writer structure"]
impl crate::Writable for I2S_PCM_ISTA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets i2s_pcm_ista to value 0"]
impl crate::Resettable for I2S_PCM_ISTA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
