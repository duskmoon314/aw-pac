#[doc = "Register `halt` reader"]
pub type R = crate::R<HALT_SPEC>;
#[doc = "Register `halt` writer"]
pub type W = crate::W<HALT_SPEC>;
#[doc = "Field `halt_tx` reader - "]
pub type HALT_TX_R = crate::BitReader<HALT_TX_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HALT_TX_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<HALT_TX_A> for bool {
    #[inline(always)]
    fn from(variant: HALT_TX_A) -> Self {
        variant as u8 != 0
    }
}
impl HALT_TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HALT_TX_A {
        match self.bits {
            false => HALT_TX_A::DISABLED,
            true => HALT_TX_A::ENABLED,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HALT_TX_A::DISABLED
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HALT_TX_A::ENABLED
    }
}
#[doc = "Field `halt_tx` writer - "]
pub type HALT_TX_W<'a, REG> = crate::BitWriter<'a, REG, HALT_TX_A>;
impl<'a, REG> HALT_TX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HALT_TX_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HALT_TX_A::ENABLED)
    }
}
#[doc = "Field `chcfg_at_busy` reader - "]
pub type CHCFG_AT_BUSY_R = crate::BitReader<CHCFG_AT_BUSY_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHCFG_AT_BUSY_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CHCFG_AT_BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: CHCFG_AT_BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl CHCFG_AT_BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHCFG_AT_BUSY_A {
        match self.bits {
            false => CHCFG_AT_BUSY_A::DISABLE,
            true => CHCFG_AT_BUSY_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHCFG_AT_BUSY_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHCFG_AT_BUSY_A::ENABLE
    }
}
#[doc = "Field `chcfg_at_busy` writer - "]
pub type CHCFG_AT_BUSY_W<'a, REG> = crate::BitWriter<'a, REG, CHCFG_AT_BUSY_A>;
impl<'a, REG> CHCFG_AT_BUSY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CHCFG_AT_BUSY_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CHCFG_AT_BUSY_A::ENABLE)
    }
}
#[doc = "Field `change_update` reader - "]
pub type CHANGE_UPDATE_R = crate::BitReader<CHANGE_UPDATE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHANGE_UPDATE_A {
    #[doc = "0: `0`"]
    FINISHED = 0,
    #[doc = "1: `1`"]
    UPDATE_TRIGGER = 1,
}
impl From<CHANGE_UPDATE_A> for bool {
    #[inline(always)]
    fn from(variant: CHANGE_UPDATE_A) -> Self {
        variant as u8 != 0
    }
}
impl CHANGE_UPDATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHANGE_UPDATE_A {
        match self.bits {
            false => CHANGE_UPDATE_A::FINISHED,
            true => CHANGE_UPDATE_A::UPDATE_TRIGGER,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        *self == CHANGE_UPDATE_A::FINISHED
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_update_trigger(&self) -> bool {
        *self == CHANGE_UPDATE_A::UPDATE_TRIGGER
    }
}
#[doc = "Field `change_update` writer - "]
pub type CHANGE_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG, CHANGE_UPDATE_A>;
impl<'a, REG> CHANGE_UPDATE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn finished(self) -> &'a mut crate::W<REG> {
        self.variant(CHANGE_UPDATE_A::FINISHED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn update_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(CHANGE_UPDATE_A::UPDATE_TRIGGER)
    }
}
#[doc = "Field `sir_tx_invert` reader - SIR TX Pulse Polarity Invert"]
pub type SIR_TX_INVERT_R = crate::BitReader<SIR_TX_INVERT_A>;
#[doc = "SIR TX Pulse Polarity Invert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SIR_TX_INVERT_A {
    #[doc = "0: `0`"]
    NOT_INVERT = 0,
    #[doc = "1: `1`"]
    INVERT = 1,
}
impl From<SIR_TX_INVERT_A> for bool {
    #[inline(always)]
    fn from(variant: SIR_TX_INVERT_A) -> Self {
        variant as u8 != 0
    }
}
impl SIR_TX_INVERT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SIR_TX_INVERT_A {
        match self.bits {
            false => SIR_TX_INVERT_A::NOT_INVERT,
            true => SIR_TX_INVERT_A::INVERT,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_not_invert(&self) -> bool {
        *self == SIR_TX_INVERT_A::NOT_INVERT
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == SIR_TX_INVERT_A::INVERT
    }
}
#[doc = "Field `sir_tx_invert` writer - SIR TX Pulse Polarity Invert"]
pub type SIR_TX_INVERT_W<'a, REG> = crate::BitWriter<'a, REG, SIR_TX_INVERT_A>;
impl<'a, REG> SIR_TX_INVERT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_invert(self) -> &'a mut crate::W<REG> {
        self.variant(SIR_TX_INVERT_A::NOT_INVERT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut crate::W<REG> {
        self.variant(SIR_TX_INVERT_A::INVERT)
    }
}
#[doc = "Field `sir_rx_invert` reader - SIR RX Pulse Polarity Invert"]
pub type SIR_RX_INVERT_R = crate::BitReader<SIR_RX_INVERT_A>;
#[doc = "SIR RX Pulse Polarity Invert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SIR_RX_INVERT_A {
    #[doc = "0: `0`"]
    NOT_INVERT = 0,
    #[doc = "1: `1`"]
    INVERT = 1,
}
impl From<SIR_RX_INVERT_A> for bool {
    #[inline(always)]
    fn from(variant: SIR_RX_INVERT_A) -> Self {
        variant as u8 != 0
    }
}
impl SIR_RX_INVERT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SIR_RX_INVERT_A {
        match self.bits {
            false => SIR_RX_INVERT_A::NOT_INVERT,
            true => SIR_RX_INVERT_A::INVERT,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_not_invert(&self) -> bool {
        *self == SIR_RX_INVERT_A::NOT_INVERT
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == SIR_RX_INVERT_A::INVERT
    }
}
#[doc = "Field `sir_rx_invert` writer - SIR RX Pulse Polarity Invert"]
pub type SIR_RX_INVERT_W<'a, REG> = crate::BitWriter<'a, REG, SIR_RX_INVERT_A>;
impl<'a, REG> SIR_RX_INVERT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_invert(self) -> &'a mut crate::W<REG> {
        self.variant(SIR_RX_INVERT_A::NOT_INVERT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut crate::W<REG> {
        self.variant(SIR_RX_INVERT_A::INVERT)
    }
}
#[doc = "Field `dma_pte_rx` reader - The Transmission of RX_DRQ"]
pub type DMA_PTE_RX_R = crate::BitReader;
#[doc = "Field `dma_pte_rx` writer - The Transmission of RX_DRQ"]
pub type DMA_PTE_RX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pte` reader - The sending of TX_REQ"]
pub type PTE_R = crate::BitReader;
#[doc = "Field `pte` writer - The sending of TX_REQ"]
pub type PTE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn halt_tx(&self) -> HALT_TX_R {
        HALT_TX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn chcfg_at_busy(&self) -> CHCFG_AT_BUSY_R {
        CHCFG_AT_BUSY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn change_update(&self) -> CHANGE_UPDATE_R {
        CHANGE_UPDATE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - SIR TX Pulse Polarity Invert"]
    #[inline(always)]
    pub fn sir_tx_invert(&self) -> SIR_TX_INVERT_R {
        SIR_TX_INVERT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SIR RX Pulse Polarity Invert"]
    #[inline(always)]
    pub fn sir_rx_invert(&self) -> SIR_RX_INVERT_R {
        SIR_RX_INVERT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The Transmission of RX_DRQ"]
    #[inline(always)]
    pub fn dma_pte_rx(&self) -> DMA_PTE_RX_R {
        DMA_PTE_RX_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The sending of TX_REQ"]
    #[inline(always)]
    pub fn pte(&self) -> PTE_R {
        PTE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn halt_tx(&mut self) -> HALT_TX_W<HALT_SPEC> {
        HALT_TX_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn chcfg_at_busy(&mut self) -> CHCFG_AT_BUSY_W<HALT_SPEC> {
        CHCFG_AT_BUSY_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn change_update(&mut self) -> CHANGE_UPDATE_W<HALT_SPEC> {
        CHANGE_UPDATE_W::new(self, 2)
    }
    #[doc = "Bit 4 - SIR TX Pulse Polarity Invert"]
    #[inline(always)]
    #[must_use]
    pub fn sir_tx_invert(&mut self) -> SIR_TX_INVERT_W<HALT_SPEC> {
        SIR_TX_INVERT_W::new(self, 4)
    }
    #[doc = "Bit 5 - SIR RX Pulse Polarity Invert"]
    #[inline(always)]
    #[must_use]
    pub fn sir_rx_invert(&mut self) -> SIR_RX_INVERT_W<HALT_SPEC> {
        SIR_RX_INVERT_W::new(self, 5)
    }
    #[doc = "Bit 6 - The Transmission of RX_DRQ"]
    #[inline(always)]
    #[must_use]
    pub fn dma_pte_rx(&mut self) -> DMA_PTE_RX_W<HALT_SPEC> {
        DMA_PTE_RX_W::new(self, 6)
    }
    #[doc = "Bit 7 - The sending of TX_REQ"]
    #[inline(always)]
    #[must_use]
    pub fn pte(&mut self) -> PTE_W<HALT_SPEC> {
        PTE_W::new(self, 7)
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
#[doc = "UART Halt TX Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`halt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`halt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HALT_SPEC;
impl crate::RegisterSpec for HALT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`halt::R`](R) reader structure"]
impl crate::Readable for HALT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`halt::W`](W) writer structure"]
impl crate::Writable for HALT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets halt to value 0"]
impl crate::Resettable for HALT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
