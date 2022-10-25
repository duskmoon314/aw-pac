#[doc = "Register `ac_dac_fifos` reader"]
pub struct R(crate::R<AC_DAC_FIFOS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AC_DAC_FIFOS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AC_DAC_FIFOS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AC_DAC_FIFOS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ac_dac_fifos` writer"]
pub struct W(crate::W<AC_DAC_FIFOS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AC_DAC_FIFOS_SPEC>;
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
impl From<crate::W<AC_DAC_FIFOS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AC_DAC_FIFOS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `txo_int` reader - TX FIFO Overrun Pending Interrupt"]
pub type TXO_INT_R = crate::BitReader<TXO_INT_A>;
#[doc = "TX FIFO Overrun Pending Interrupt\n\nValue on reset: 0"]
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
#[doc = "Field `txo_int` writer - TX FIFO Overrun Pending Interrupt"]
pub type TXO_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, AC_DAC_FIFOS_SPEC, TXO_INT_A, O>;
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
#[doc = "Field `txu_int` reader - TX FIFO Underrun Pending Interrupt"]
pub type TXU_INT_R = crate::BitReader<TXU_INT_A>;
#[doc = "TX FIFO Underrun Pending Interrupt\n\nValue on reset: 0"]
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
#[doc = "Field `txu_int` writer - TX FIFO Underrun Pending Interrupt"]
pub type TXU_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, AC_DAC_FIFOS_SPEC, TXU_INT_A, O>;
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
#[doc = "Field `txe_int` reader - TX FIFO Empty Pending Interrupt"]
pub type TXE_INT_R = crate::BitReader<TXE_INT_A>;
#[doc = "TX FIFO Empty Pending Interrupt\n\nValue on reset: 0"]
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
#[doc = "Field `txe_int` writer - TX FIFO Empty Pending Interrupt"]
pub type TXE_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, AC_DAC_FIFOS_SPEC, TXE_INT_A, O>;
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
#[doc = "Field `txe_cnt` reader - TX FIFO Empty Space Word Counter"]
pub type TXE_CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `tx_empty` reader - TX FIFO Empty"]
pub type TX_EMPTY_R = crate::BitReader<TX_EMPTY_A>;
#[doc = "TX FIFO Empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_EMPTY_A {
    #[doc = "0: `0`"]
    NO_ROOM = 0,
    #[doc = "1: `1`"]
    ROOM = 1,
}
impl From<TX_EMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: TX_EMPTY_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_EMPTY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_EMPTY_A {
        match self.bits {
            false => TX_EMPTY_A::NO_ROOM,
            true => TX_EMPTY_A::ROOM,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ROOM`"]
    #[inline(always)]
    pub fn is_no_room(&self) -> bool {
        *self == TX_EMPTY_A::NO_ROOM
    }
    #[doc = "Checks if the value of the field is `ROOM`"]
    #[inline(always)]
    pub fn is_room(&self) -> bool {
        *self == TX_EMPTY_A::ROOM
    }
}
impl R {
    #[doc = "Bit 1 - TX FIFO Overrun Pending Interrupt"]
    #[inline(always)]
    pub fn txo_int(&self) -> TXO_INT_R {
        TXO_INT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TX FIFO Underrun Pending Interrupt"]
    #[inline(always)]
    pub fn txu_int(&self) -> TXU_INT_R {
        TXU_INT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TX FIFO Empty Pending Interrupt"]
    #[inline(always)]
    pub fn txe_int(&self) -> TXE_INT_R {
        TXE_INT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:22 - TX FIFO Empty Space Word Counter"]
    #[inline(always)]
    pub fn txe_cnt(&self) -> TXE_CNT_R {
        TXE_CNT_R::new(((self.bits >> 8) & 0x7fff) as u16)
    }
    #[doc = "Bit 23 - TX FIFO Empty"]
    #[inline(always)]
    pub fn tx_empty(&self) -> TX_EMPTY_R {
        TX_EMPTY_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - TX FIFO Overrun Pending Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txo_int(&mut self) -> TXO_INT_W<1> {
        TXO_INT_W::new(self)
    }
    #[doc = "Bit 2 - TX FIFO Underrun Pending Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txu_int(&mut self) -> TXU_INT_W<2> {
        TXU_INT_W::new(self)
    }
    #[doc = "Bit 3 - TX FIFO Empty Pending Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txe_int(&mut self) -> TXE_INT_W<3> {
        TXE_INT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC FIFO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ac_dac_fifos](index.html) module"]
pub struct AC_DAC_FIFOS_SPEC;
impl crate::RegisterSpec for AC_DAC_FIFOS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ac_dac_fifos::R](R) reader structure"]
impl crate::Readable for AC_DAC_FIFOS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ac_dac_fifos::W](W) writer structure"]
impl crate::Writable for AC_DAC_FIFOS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ac_dac_fifos to value 0"]
impl crate::Resettable for AC_DAC_FIFOS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
