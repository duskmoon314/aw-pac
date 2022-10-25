#[doc = "Register `cir_txsta` reader"]
pub struct R(crate::R<CIR_TXSTA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIR_TXSTA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIR_TXSTA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIR_TXSTA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cir_txsta` writer"]
pub struct W(crate::W<CIR_TXSTA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIR_TXSTA_SPEC>;
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
impl From<crate::W<CIR_TXSTA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIR_TXSTA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tpe_tur` reader - Transmitter Packet End Flag for Cyclical Pulse / TUR Transmitter FIFO Underrun Flag for Non-cyclical Pulse"]
pub type TPE_TUR_R = crate::BitReader<TPE_TUR_A>;
#[doc = "Transmitter Packet End Flag for Cyclical Pulse / TUR Transmitter FIFO Underrun Flag for Non-cyclical Pulse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TPE_TUR_A {
    #[doc = "0: Transmissions of address, control and data fields not completed / No transmitter FIFO underrun"]
    NOT_COMPLETE_OR_TRANSMIT = 0,
    #[doc = "1: Transmissions of address, control and data fields completed / Transmitter FIFO underrun"]
    COMPLETE_OR_TRANSMIT = 1,
}
impl From<TPE_TUR_A> for bool {
    #[inline(always)]
    fn from(variant: TPE_TUR_A) -> Self {
        variant as u8 != 0
    }
}
impl TPE_TUR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPE_TUR_A {
        match self.bits {
            false => TPE_TUR_A::NOT_COMPLETE_OR_TRANSMIT,
            true => TPE_TUR_A::COMPLETE_OR_TRANSMIT,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_COMPLETE_OR_TRANSMIT`"]
    #[inline(always)]
    pub fn is_not_complete_or_transmit(&self) -> bool {
        *self == TPE_TUR_A::NOT_COMPLETE_OR_TRANSMIT
    }
    #[doc = "Checks if the value of the field is `COMPLETE_OR_TRANSMIT`"]
    #[inline(always)]
    pub fn is_complete_or_transmit(&self) -> bool {
        *self == TPE_TUR_A::COMPLETE_OR_TRANSMIT
    }
}
#[doc = "Field `tpe_tur` writer - Transmitter Packet End Flag for Cyclical Pulse / TUR Transmitter FIFO Underrun Flag for Non-cyclical Pulse"]
pub type TPE_TUR_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CIR_TXSTA_SPEC, TPE_TUR_A, O>;
impl<'a, const O: u8> TPE_TUR_W<'a, O> {
    #[doc = "Transmissions of address, control and data fields not completed / No transmitter FIFO underrun"]
    #[inline(always)]
    pub fn not_complete_or_transmit(self) -> &'a mut W {
        self.variant(TPE_TUR_A::NOT_COMPLETE_OR_TRANSMIT)
    }
    #[doc = "Transmissions of address, control and data fields completed / Transmitter FIFO underrun"]
    #[inline(always)]
    pub fn complete_or_transmit(self) -> &'a mut W {
        self.variant(TPE_TUR_A::COMPLETE_OR_TRANSMIT)
    }
}
#[doc = "Field `tai` reader - TX FIFO Available Interrupt Flag"]
pub type TAI_R = crate::BitReader<TAI_A>;
#[doc = "TX FIFO Available Interrupt Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAI_A {
    #[doc = "0: TX FIFO not available by its level"]
    NOT_AVAILABLE = 0,
    #[doc = "1: TX FIFO available by its level Writing 1 clears this bit."]
    AVAILABLE = 1,
}
impl From<TAI_A> for bool {
    #[inline(always)]
    fn from(variant: TAI_A) -> Self {
        variant as u8 != 0
    }
}
impl TAI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAI_A {
        match self.bits {
            false => TAI_A::NOT_AVAILABLE,
            true => TAI_A::AVAILABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_AVAILABLE`"]
    #[inline(always)]
    pub fn is_not_available(&self) -> bool {
        *self == TAI_A::NOT_AVAILABLE
    }
    #[doc = "Checks if the value of the field is `AVAILABLE`"]
    #[inline(always)]
    pub fn is_available(&self) -> bool {
        *self == TAI_A::AVAILABLE
    }
}
#[doc = "Field `tai` writer - TX FIFO Available Interrupt Flag"]
pub type TAI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_TXSTA_SPEC, TAI_A, O>;
impl<'a, const O: u8> TAI_W<'a, O> {
    #[doc = "TX FIFO not available by its level"]
    #[inline(always)]
    pub fn not_available(self) -> &'a mut W {
        self.variant(TAI_A::NOT_AVAILABLE)
    }
    #[doc = "TX FIFO available by its level Writing 1 clears this bit."]
    #[inline(always)]
    pub fn available(self) -> &'a mut W {
        self.variant(TAI_A::AVAILABLE)
    }
}
#[doc = "Field `drq` reader - DMA Request Flag"]
pub type DRQ_R = crate::BitReader<bool>;
#[doc = "Field `stct` reader - Status of CIR Transmitter"]
pub type STCT_R = crate::BitReader<STCT_A>;
#[doc = "Status of CIR Transmitter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STCT_A {
    #[doc = "0: Idle"]
    IDLE = 0,
    #[doc = "1: Active"]
    ACTIVE = 1,
}
impl From<STCT_A> for bool {
    #[inline(always)]
    fn from(variant: STCT_A) -> Self {
        variant as u8 != 0
    }
}
impl STCT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STCT_A {
        match self.bits {
            false => STCT_A::IDLE,
            true => STCT_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == STCT_A::IDLE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == STCT_A::ACTIVE
    }
}
impl R {
    #[doc = "Bit 0 - Transmitter Packet End Flag for Cyclical Pulse / TUR Transmitter FIFO Underrun Flag for Non-cyclical Pulse"]
    #[inline(always)]
    pub fn tpe_tur(&self) -> TPE_TUR_R {
        TPE_TUR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX FIFO Available Interrupt Flag"]
    #[inline(always)]
    pub fn tai(&self) -> TAI_R {
        TAI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Request Flag"]
    #[inline(always)]
    pub fn drq(&self) -> DRQ_R {
        DRQ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Status of CIR Transmitter"]
    #[inline(always)]
    pub fn stct(&self) -> STCT_R {
        STCT_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmitter Packet End Flag for Cyclical Pulse / TUR Transmitter FIFO Underrun Flag for Non-cyclical Pulse"]
    #[inline(always)]
    #[must_use]
    pub fn tpe_tur(&mut self) -> TPE_TUR_W<0> {
        TPE_TUR_W::new(self)
    }
    #[doc = "Bit 1 - TX FIFO Available Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tai(&mut self) -> TAI_W<1> {
        TAI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CIR Transmit Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cir_txsta](index.html) module"]
pub struct CIR_TXSTA_SPEC;
impl crate::RegisterSpec for CIR_TXSTA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cir_txsta::R](R) reader structure"]
impl crate::Readable for CIR_TXSTA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cir_txsta::W](W) writer structure"]
impl crate::Writable for CIR_TXSTA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x01;
}
#[doc = "`reset()` method sets cir_txsta to value 0x02"]
impl crate::Resettable for CIR_TXSTA_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
