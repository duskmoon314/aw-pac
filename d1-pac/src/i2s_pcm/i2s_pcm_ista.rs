#[doc = "Register `i2s_pcm_ista` reader"]
pub struct R(crate::R<I2S_PCM_ISTA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_PCM_ISTA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_PCM_ISTA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_PCM_ISTA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `i2s_pcm_ista` writer"]
pub struct W(crate::W<I2S_PCM_ISTA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_PCM_ISTA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<I2S_PCM_ISTA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_PCM_ISTA_SPEC>) -> Self {
        W(writer)
    }
}
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
    pub fn variant(&self) -> TXE_INT_A {
        match self.bits {
            false => TXE_INT_A::NO_PENDING,
            true => TXE_INT_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == TXE_INT_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == TXE_INT_A::PENDING
    }
}
#[doc = "Field `txe_int` writer - TXFIFO Empty Pending Interrupt"]
pub type TXE_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_PCM_ISTA_SPEC, TXE_INT_A, O>;
impl<'a, const O: u8> TXE_INT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(TXE_INT_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
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
    pub fn variant(&self) -> RXA_INT_A {
        match self.bits {
            false => RXA_INT_A::NO_PENDING,
            true => RXA_INT_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == RXA_INT_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RXA_INT_A::PENDING
    }
}
#[doc = "Field `rxa_int` writer - RXFIFO Data Available Pending Interrupt"]
pub type RXA_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_PCM_ISTA_SPEC, RXA_INT_A, O>;
impl<'a, const O: u8> RXA_INT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(RXA_INT_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
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
    pub fn variant(&self) -> TXO_INT_A {
        match self.bits {
            false => TXO_INT_A::NO_PENDING,
            true => TXO_INT_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == TXO_INT_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == TXO_INT_A::PENDING
    }
}
#[doc = "Field `txo_int` writer - TXFIFO Overrun Pending Interrupt"]
pub type TXO_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_PCM_ISTA_SPEC, TXO_INT_A, O>;
impl<'a, const O: u8> TXO_INT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(TXO_INT_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
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
    pub fn variant(&self) -> RXO_INT_A {
        match self.bits {
            false => RXO_INT_A::NO_PENDING,
            true => RXO_INT_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == RXO_INT_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RXO_INT_A::PENDING
    }
}
#[doc = "Field `rxo_int` writer - RXFIFO Overrun Pending Interrupt"]
pub type RXO_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_PCM_ISTA_SPEC, RXO_INT_A, O>;
impl<'a, const O: u8> RXO_INT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(RXO_INT_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
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
    pub fn variant(&self) -> TXU_INT_A {
        match self.bits {
            false => TXU_INT_A::NO_PENDING,
            true => TXU_INT_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == TXU_INT_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == TXU_INT_A::PENDING
    }
}
#[doc = "Field `txu_int` writer - TXFIFO Underrun Pending Interrupt"]
pub type TXU_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_PCM_ISTA_SPEC, TXU_INT_A, O>;
impl<'a, const O: u8> TXU_INT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(TXU_INT_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
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
    pub fn variant(&self) -> RXU_INT_A {
        match self.bits {
            false => RXU_INT_A::NO_PENDING,
            true => RXU_INT_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == RXU_INT_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RXU_INT_A::PENDING
    }
}
#[doc = "Field `rxu_int` writer - RXFIFO Underrun Pending Interrupt"]
pub type RXU_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_PCM_ISTA_SPEC, RXU_INT_A, O>;
impl<'a, const O: u8> RXU_INT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(RXU_INT_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
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
    pub fn txe_int(&mut self) -> TXE_INT_W<4> {
        TXE_INT_W::new(self)
    }
    #[doc = "Bit 4 - RXFIFO Data Available Pending Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxa_int(&mut self) -> RXA_INT_W<4> {
        RXA_INT_W::new(self)
    }
    #[doc = "Bit 5 - TXFIFO Overrun Pending Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txo_int(&mut self) -> TXO_INT_W<5> {
        TXO_INT_W::new(self)
    }
    #[doc = "Bit 5 - RXFIFO Overrun Pending Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxo_int(&mut self) -> RXO_INT_W<5> {
        RXO_INT_W::new(self)
    }
    #[doc = "Bit 6 - TXFIFO Underrun Pending Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txu_int(&mut self) -> TXU_INT_W<6> {
        TXU_INT_W::new(self)
    }
    #[doc = "Bit 6 - RXFIFO Underrun Pending Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxu_int(&mut self) -> RXU_INT_W<6> {
        RXU_INT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S/PCM Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_pcm_ista](index.html) module"]
pub struct I2S_PCM_ISTA_SPEC;
impl crate::RegisterSpec for I2S_PCM_ISTA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_pcm_ista::R](R) reader structure"]
impl crate::Readable for I2S_PCM_ISTA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_pcm_ista::W](W) writer structure"]
impl crate::Writable for I2S_PCM_ISTA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets i2s_pcm_ista to value 0"]
impl crate::Resettable for I2S_PCM_ISTA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
