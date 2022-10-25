#[doc = "Register `cir_tglr` reader"]
pub struct R(crate::R<CIR_TGLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIR_TGLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIR_TGLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIR_TGLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cir_tglr` writer"]
pub struct W(crate::W<CIR_TGLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIR_TGLR_SPEC>;
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
impl From<crate::W<CIR_TGLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIR_TGLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `txen` reader - Transmit Block Enable"]
pub type TXEN_R = crate::BitReader<TXEN_A>;
#[doc = "Transmit Block Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXEN_A {
    #[doc = "0: Disable the CIR Transmitter"]
    DISABLE = 0,
    #[doc = "1: Enable the CIR Transmitter"]
    ENABLE = 1,
}
impl From<TXEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TXEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXEN_A {
        match self.bits {
            false => TXEN_A::DISABLE,
            true => TXEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TXEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TXEN_A::ENABLE
    }
}
#[doc = "Field `txen` writer - Transmit Block Enable"]
pub type TXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_TGLR_SPEC, TXEN_A, O>;
impl<'a, const O: u8> TXEN_W<'a, O> {
    #[doc = "Disable the CIR Transmitter"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXEN_A::DISABLE)
    }
    #[doc = "Enable the CIR Transmitter"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXEN_A::ENABLE)
    }
}
#[doc = "Field `tr` reader - Transmit Reset\n\nWhen this bit is set, the transmitting is reset. The FIFO will be flushed, the TIC filed and the CSS field will be cleared during Transmit Reset. This field will automatically be cleared when the Transmit Reset is finished, and the CIR transmitter will state Idle."]
pub type TR_R = crate::BitReader<bool>;
#[doc = "Field `tr` writer - Transmit Reset\n\nWhen this bit is set, the transmitting is reset. The FIFO will be flushed, the TIC filed and the CSS field will be cleared during Transmit Reset. This field will automatically be cleared when the Transmit Reset is finished, and the CIR transmitter will state Idle."]
pub type TR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_TGLR_SPEC, bool, O>;
#[doc = "Field `tppi` reader - Transmit Pulse Polarity Invert"]
pub type TPPI_R = crate::BitReader<TPPI_A>;
#[doc = "Transmit Pulse Polarity Invert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TPPI_A {
    #[doc = "0: Not invert transmit pulse"]
    NOT_INVERT = 0,
    #[doc = "1: Invert transmit pulse"]
    INVERT = 1,
}
impl From<TPPI_A> for bool {
    #[inline(always)]
    fn from(variant: TPPI_A) -> Self {
        variant as u8 != 0
    }
}
impl TPPI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPPI_A {
        match self.bits {
            false => TPPI_A::NOT_INVERT,
            true => TPPI_A::INVERT,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_INVERT`"]
    #[inline(always)]
    pub fn is_not_invert(&self) -> bool {
        *self == TPPI_A::NOT_INVERT
    }
    #[doc = "Checks if the value of the field is `INVERT`"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == TPPI_A::INVERT
    }
}
#[doc = "Field `tppi` writer - Transmit Pulse Polarity Invert"]
pub type TPPI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_TGLR_SPEC, TPPI_A, O>;
impl<'a, const O: u8> TPPI_W<'a, O> {
    #[doc = "Not invert transmit pulse"]
    #[inline(always)]
    pub fn not_invert(self) -> &'a mut W {
        self.variant(TPPI_A::NOT_INVERT)
    }
    #[doc = "Invert transmit pulse"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut W {
        self.variant(TPPI_A::INVERT)
    }
}
#[doc = "Field `drmc` reader - Duty ratio of modulated carrier is high level/low level."]
pub type DRMC_R = crate::FieldReader<u8, DRMC_A>;
#[doc = "Duty ratio of modulated carrier is high level/low level.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DRMC_A {
    #[doc = "0: Low level is equal to high level"]
    EQUAL = 0,
    #[doc = "1: Low level is the double of high level"]
    DOUBLE = 1,
    #[doc = "2: Low level is the triple of high level"]
    TRIPLE = 2,
}
impl From<DRMC_A> for u8 {
    #[inline(always)]
    fn from(variant: DRMC_A) -> Self {
        variant as _
    }
}
impl DRMC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DRMC_A> {
        match self.bits {
            0 => Some(DRMC_A::EQUAL),
            1 => Some(DRMC_A::DOUBLE),
            2 => Some(DRMC_A::TRIPLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EQUAL`"]
    #[inline(always)]
    pub fn is_equal(&self) -> bool {
        *self == DRMC_A::EQUAL
    }
    #[doc = "Checks if the value of the field is `DOUBLE`"]
    #[inline(always)]
    pub fn is_double(&self) -> bool {
        *self == DRMC_A::DOUBLE
    }
    #[doc = "Checks if the value of the field is `TRIPLE`"]
    #[inline(always)]
    pub fn is_triple(&self) -> bool {
        *self == DRMC_A::TRIPLE
    }
}
#[doc = "Field `drmc` writer - Duty ratio of modulated carrier is high level/low level."]
pub type DRMC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CIR_TGLR_SPEC, u8, DRMC_A, 2, O>;
impl<'a, const O: u8> DRMC_W<'a, O> {
    #[doc = "Low level is equal to high level"]
    #[inline(always)]
    pub fn equal(self) -> &'a mut W {
        self.variant(DRMC_A::EQUAL)
    }
    #[doc = "Low level is the double of high level"]
    #[inline(always)]
    pub fn double(self) -> &'a mut W {
        self.variant(DRMC_A::DOUBLE)
    }
    #[doc = "Low level is the triple of high level"]
    #[inline(always)]
    pub fn triple(self) -> &'a mut W {
        self.variant(DRMC_A::TRIPLE)
    }
}
#[doc = "Field `ims` reader - Internal Modulation Select"]
pub type IMS_R = crate::BitReader<IMS_A>;
#[doc = "Internal Modulation Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IMS_A {
    #[doc = "0: The transmitting signal is not modulated"]
    NOT_MODULATED = 0,
    #[doc = "1: The transmitting signal is modulated internally"]
    MODULATED = 1,
}
impl From<IMS_A> for bool {
    #[inline(always)]
    fn from(variant: IMS_A) -> Self {
        variant as u8 != 0
    }
}
impl IMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IMS_A {
        match self.bits {
            false => IMS_A::NOT_MODULATED,
            true => IMS_A::MODULATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_MODULATED`"]
    #[inline(always)]
    pub fn is_not_modulated(&self) -> bool {
        *self == IMS_A::NOT_MODULATED
    }
    #[doc = "Checks if the value of the field is `MODULATED`"]
    #[inline(always)]
    pub fn is_modulated(&self) -> bool {
        *self == IMS_A::MODULATED
    }
}
#[doc = "Field `ims` writer - Internal Modulation Select"]
pub type IMS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_TGLR_SPEC, IMS_A, O>;
impl<'a, const O: u8> IMS_W<'a, O> {
    #[doc = "The transmitting signal is not modulated"]
    #[inline(always)]
    pub fn not_modulated(self) -> &'a mut W {
        self.variant(IMS_A::NOT_MODULATED)
    }
    #[doc = "The transmitting signal is modulated internally"]
    #[inline(always)]
    pub fn modulated(self) -> &'a mut W {
        self.variant(IMS_A::MODULATED)
    }
}
impl R {
    #[doc = "Bit 0 - Transmit Block Enable"]
    #[inline(always)]
    pub fn txen(&self) -> TXEN_R {
        TXEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Reset\n\nWhen this bit is set, the transmitting is reset. The FIFO will be flushed, the TIC filed and the CSS field will be cleared during Transmit Reset. This field will automatically be cleared when the Transmit Reset is finished, and the CIR transmitter will state Idle."]
    #[inline(always)]
    pub fn tr(&self) -> TR_R {
        TR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Pulse Polarity Invert"]
    #[inline(always)]
    pub fn tppi(&self) -> TPPI_R {
        TPPI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Duty ratio of modulated carrier is high level/low level."]
    #[inline(always)]
    pub fn drmc(&self) -> DRMC_R {
        DRMC_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Internal Modulation Select"]
    #[inline(always)]
    pub fn ims(&self) -> IMS_R {
        IMS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Block Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txen(&mut self) -> TXEN_W<0> {
        TXEN_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Reset\n\nWhen this bit is set, the transmitting is reset. The FIFO will be flushed, the TIC filed and the CSS field will be cleared during Transmit Reset. This field will automatically be cleared when the Transmit Reset is finished, and the CIR transmitter will state Idle."]
    #[inline(always)]
    #[must_use]
    pub fn tr(&mut self) -> TR_W<1> {
        TR_W::new(self)
    }
    #[doc = "Bit 2 - Transmit Pulse Polarity Invert"]
    #[inline(always)]
    #[must_use]
    pub fn tppi(&mut self) -> TPPI_W<2> {
        TPPI_W::new(self)
    }
    #[doc = "Bits 5:6 - Duty ratio of modulated carrier is high level/low level."]
    #[inline(always)]
    #[must_use]
    pub fn drmc(&mut self) -> DRMC_W<5> {
        DRMC_W::new(self)
    }
    #[doc = "Bit 7 - Internal Modulation Select"]
    #[inline(always)]
    #[must_use]
    pub fn ims(&mut self) -> IMS_W<7> {
        IMS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CIR Transmit Global Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cir_tglr](index.html) module"]
pub struct CIR_TGLR_SPEC;
impl crate::RegisterSpec for CIR_TGLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cir_tglr::R](R) reader structure"]
impl crate::Readable for CIR_TGLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cir_tglr::W](W) writer structure"]
impl crate::Writable for CIR_TGLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cir_tglr to value 0"]
impl crate::Resettable for CIR_TGLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
