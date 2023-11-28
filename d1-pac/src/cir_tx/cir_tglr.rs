#[doc = "Register `cir_tglr` reader"]
pub type R = crate::R<CIR_TGLR_SPEC>;
#[doc = "Register `cir_tglr` writer"]
pub type W = crate::W<CIR_TGLR_SPEC>;
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
    pub const fn variant(&self) -> TXEN_A {
        match self.bits {
            false => TXEN_A::DISABLE,
            true => TXEN_A::ENABLE,
        }
    }
    #[doc = "Disable the CIR Transmitter"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TXEN_A::DISABLE
    }
    #[doc = "Enable the CIR Transmitter"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TXEN_A::ENABLE
    }
}
#[doc = "Field `txen` writer - Transmit Block Enable"]
pub type TXEN_W<'a, REG> = crate::BitWriter<'a, REG, TXEN_A>;
impl<'a, REG> TXEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the CIR Transmitter"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TXEN_A::DISABLE)
    }
    #[doc = "Enable the CIR Transmitter"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TXEN_A::ENABLE)
    }
}
#[doc = "Field `tr` reader - Transmit Reset\n\nWhen this bit is set, the transmitting is reset. The FIFO will be flushed, the TIC filed and the CSS field will be cleared during Transmit Reset. This field will automatically be cleared when the Transmit Reset is finished, and the CIR transmitter will state Idle."]
pub type TR_R = crate::BitReader;
#[doc = "Field `tr` writer - Transmit Reset\n\nWhen this bit is set, the transmitting is reset. The FIFO will be flushed, the TIC filed and the CSS field will be cleared during Transmit Reset. This field will automatically be cleared when the Transmit Reset is finished, and the CIR transmitter will state Idle."]
pub type TR_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub const fn variant(&self) -> TPPI_A {
        match self.bits {
            false => TPPI_A::NOT_INVERT,
            true => TPPI_A::INVERT,
        }
    }
    #[doc = "Not invert transmit pulse"]
    #[inline(always)]
    pub fn is_not_invert(&self) -> bool {
        *self == TPPI_A::NOT_INVERT
    }
    #[doc = "Invert transmit pulse"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == TPPI_A::INVERT
    }
}
#[doc = "Field `tppi` writer - Transmit Pulse Polarity Invert"]
pub type TPPI_W<'a, REG> = crate::BitWriter<'a, REG, TPPI_A>;
impl<'a, REG> TPPI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not invert transmit pulse"]
    #[inline(always)]
    pub fn not_invert(self) -> &'a mut crate::W<REG> {
        self.variant(TPPI_A::NOT_INVERT)
    }
    #[doc = "Invert transmit pulse"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut crate::W<REG> {
        self.variant(TPPI_A::INVERT)
    }
}
#[doc = "Field `drmc` reader - Duty ratio of modulated carrier is high level/low level."]
pub type DRMC_R = crate::FieldReader<DRMC_A>;
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
impl crate::FieldSpec for DRMC_A {
    type Ux = u8;
}
impl DRMC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DRMC_A> {
        match self.bits {
            0 => Some(DRMC_A::EQUAL),
            1 => Some(DRMC_A::DOUBLE),
            2 => Some(DRMC_A::TRIPLE),
            _ => None,
        }
    }
    #[doc = "Low level is equal to high level"]
    #[inline(always)]
    pub fn is_equal(&self) -> bool {
        *self == DRMC_A::EQUAL
    }
    #[doc = "Low level is the double of high level"]
    #[inline(always)]
    pub fn is_double(&self) -> bool {
        *self == DRMC_A::DOUBLE
    }
    #[doc = "Low level is the triple of high level"]
    #[inline(always)]
    pub fn is_triple(&self) -> bool {
        *self == DRMC_A::TRIPLE
    }
}
#[doc = "Field `drmc` writer - Duty ratio of modulated carrier is high level/low level."]
pub type DRMC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DRMC_A>;
impl<'a, REG> DRMC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low level is equal to high level"]
    #[inline(always)]
    pub fn equal(self) -> &'a mut crate::W<REG> {
        self.variant(DRMC_A::EQUAL)
    }
    #[doc = "Low level is the double of high level"]
    #[inline(always)]
    pub fn double(self) -> &'a mut crate::W<REG> {
        self.variant(DRMC_A::DOUBLE)
    }
    #[doc = "Low level is the triple of high level"]
    #[inline(always)]
    pub fn triple(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> IMS_A {
        match self.bits {
            false => IMS_A::NOT_MODULATED,
            true => IMS_A::MODULATED,
        }
    }
    #[doc = "The transmitting signal is not modulated"]
    #[inline(always)]
    pub fn is_not_modulated(&self) -> bool {
        *self == IMS_A::NOT_MODULATED
    }
    #[doc = "The transmitting signal is modulated internally"]
    #[inline(always)]
    pub fn is_modulated(&self) -> bool {
        *self == IMS_A::MODULATED
    }
}
#[doc = "Field `ims` writer - Internal Modulation Select"]
pub type IMS_W<'a, REG> = crate::BitWriter<'a, REG, IMS_A>;
impl<'a, REG> IMS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The transmitting signal is not modulated"]
    #[inline(always)]
    pub fn not_modulated(self) -> &'a mut crate::W<REG> {
        self.variant(IMS_A::NOT_MODULATED)
    }
    #[doc = "The transmitting signal is modulated internally"]
    #[inline(always)]
    pub fn modulated(self) -> &'a mut crate::W<REG> {
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
    pub fn txen(&mut self) -> TXEN_W<CIR_TGLR_SPEC> {
        TXEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Reset\n\nWhen this bit is set, the transmitting is reset. The FIFO will be flushed, the TIC filed and the CSS field will be cleared during Transmit Reset. This field will automatically be cleared when the Transmit Reset is finished, and the CIR transmitter will state Idle."]
    #[inline(always)]
    #[must_use]
    pub fn tr(&mut self) -> TR_W<CIR_TGLR_SPEC> {
        TR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit Pulse Polarity Invert"]
    #[inline(always)]
    #[must_use]
    pub fn tppi(&mut self) -> TPPI_W<CIR_TGLR_SPEC> {
        TPPI_W::new(self, 2)
    }
    #[doc = "Bits 5:6 - Duty ratio of modulated carrier is high level/low level."]
    #[inline(always)]
    #[must_use]
    pub fn drmc(&mut self) -> DRMC_W<CIR_TGLR_SPEC> {
        DRMC_W::new(self, 5)
    }
    #[doc = "Bit 7 - Internal Modulation Select"]
    #[inline(always)]
    #[must_use]
    pub fn ims(&mut self) -> IMS_W<CIR_TGLR_SPEC> {
        IMS_W::new(self, 7)
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
#[doc = "CIR Transmit Global Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cir_tglr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cir_tglr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIR_TGLR_SPEC;
impl crate::RegisterSpec for CIR_TGLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cir_tglr::R`](R) reader structure"]
impl crate::Readable for CIR_TGLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cir_tglr::W`](W) writer structure"]
impl crate::Writable for CIR_TGLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cir_tglr to value 0"]
impl crate::Resettable for CIR_TGLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
