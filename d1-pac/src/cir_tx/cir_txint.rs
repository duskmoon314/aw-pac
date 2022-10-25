#[doc = "Register `cir_txint` reader"]
pub struct R(crate::R<CIR_TXINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIR_TXINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIR_TXINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIR_TXINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cir_txint` writer"]
pub struct W(crate::W<CIR_TXINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIR_TXINT_SPEC>;
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
impl From<crate::W<CIR_TXINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIR_TXINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tpei_tui_en` reader - Transmit Packet End Interrupt Enable for Cyclical Pulse / Transmitter FIFO Underrun Interrupt Enable for Non-cyclical Pulse"]
pub type TPEI_TUI_EN_R = crate::BitReader<TPEI_TUI_EN_A>;
#[doc = "Transmit Packet End Interrupt Enable for Cyclical Pulse / Transmitter FIFO Underrun Interrupt Enable for Non-cyclical Pulse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TPEI_TUI_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<TPEI_TUI_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TPEI_TUI_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TPEI_TUI_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPEI_TUI_EN_A {
        match self.bits {
            false => TPEI_TUI_EN_A::DISABLE,
            true => TPEI_TUI_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TPEI_TUI_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TPEI_TUI_EN_A::ENABLE
    }
}
#[doc = "Field `tpei_tui_en` writer - Transmit Packet End Interrupt Enable for Cyclical Pulse / Transmitter FIFO Underrun Interrupt Enable for Non-cyclical Pulse"]
pub type TPEI_TUI_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CIR_TXINT_SPEC, TPEI_TUI_EN_A, O>;
impl<'a, const O: u8> TPEI_TUI_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TPEI_TUI_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TPEI_TUI_EN_A::ENABLE)
    }
}
#[doc = "Field `tai_en` reader - TX FIFO Available Interrupt Enable"]
pub type TAI_EN_R = crate::BitReader<TAI_EN_A>;
#[doc = "TX FIFO Available Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAI_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<TAI_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TAI_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TAI_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAI_EN_A {
        match self.bits {
            false => TAI_EN_A::DISABLE,
            true => TAI_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TAI_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TAI_EN_A::ENABLE
    }
}
#[doc = "Field `tai_en` writer - TX FIFO Available Interrupt Enable"]
pub type TAI_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_TXINT_SPEC, TAI_EN_A, O>;
impl<'a, const O: u8> TAI_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TAI_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TAI_EN_A::ENABLE)
    }
}
#[doc = "Field `drq_en` reader - TX FIFO DMA Enable\n\nWhen it is set to '1', the TX FIFO DRQ is asserted if the number of the transmitting data in the FIFO is less than the RAL. The DRQ is de-asserted when the condition fails."]
pub type DRQ_EN_R = crate::BitReader<DRQ_EN_A>;
#[doc = "TX FIFO DMA Enable\n\nWhen it is set to '1', the TX FIFO DRQ is asserted if the number of the transmitting data in the FIFO is less than the RAL. The DRQ is de-asserted when the condition fails.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRQ_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<DRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRQ_EN_A {
        match self.bits {
            false => DRQ_EN_A::DISABLE,
            true => DRQ_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DRQ_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DRQ_EN_A::ENABLE
    }
}
#[doc = "Field `drq_en` writer - TX FIFO DMA Enable\n\nWhen it is set to '1', the TX FIFO DRQ is asserted if the number of the transmitting data in the FIFO is less than the RAL. The DRQ is de-asserted when the condition fails."]
pub type DRQ_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_TXINT_SPEC, DRQ_EN_A, O>;
impl<'a, const O: u8> DRQ_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DRQ_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DRQ_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Transmit Packet End Interrupt Enable for Cyclical Pulse / Transmitter FIFO Underrun Interrupt Enable for Non-cyclical Pulse"]
    #[inline(always)]
    pub fn tpei_tui_en(&self) -> TPEI_TUI_EN_R {
        TPEI_TUI_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX FIFO Available Interrupt Enable"]
    #[inline(always)]
    pub fn tai_en(&self) -> TAI_EN_R {
        TAI_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TX FIFO DMA Enable\n\nWhen it is set to '1', the TX FIFO DRQ is asserted if the number of the transmitting data in the FIFO is less than the RAL. The DRQ is de-asserted when the condition fails."]
    #[inline(always)]
    pub fn drq_en(&self) -> DRQ_EN_R {
        DRQ_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Packet End Interrupt Enable for Cyclical Pulse / Transmitter FIFO Underrun Interrupt Enable for Non-cyclical Pulse"]
    #[inline(always)]
    #[must_use]
    pub fn tpei_tui_en(&mut self) -> TPEI_TUI_EN_W<0> {
        TPEI_TUI_EN_W::new(self)
    }
    #[doc = "Bit 1 - TX FIFO Available Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tai_en(&mut self) -> TAI_EN_W<1> {
        TAI_EN_W::new(self)
    }
    #[doc = "Bit 2 - TX FIFO DMA Enable\n\nWhen it is set to '1', the TX FIFO DRQ is asserted if the number of the transmitting data in the FIFO is less than the RAL. The DRQ is de-asserted when the condition fails."]
    #[inline(always)]
    #[must_use]
    pub fn drq_en(&mut self) -> DRQ_EN_W<2> {
        DRQ_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CIR Transmit Interrupt Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cir_txint](index.html) module"]
pub struct CIR_TXINT_SPEC;
impl crate::RegisterSpec for CIR_TXINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cir_txint::R](R) reader structure"]
impl crate::Readable for CIR_TXINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cir_txint::W](W) writer structure"]
impl crate::Writable for CIR_TXINT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cir_txint to value 0"]
impl crate::Resettable for CIR_TXINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
