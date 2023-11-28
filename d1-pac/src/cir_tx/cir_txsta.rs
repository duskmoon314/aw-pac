#[doc = "Register `cir_txsta` reader"]
pub type R = crate::R<CIR_TXSTA_SPEC>;
#[doc = "Register `cir_txsta` writer"]
pub type W = crate::W<CIR_TXSTA_SPEC>;
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
    pub const fn variant(&self) -> TPE_TUR_A {
        match self.bits {
            false => TPE_TUR_A::NOT_COMPLETE_OR_TRANSMIT,
            true => TPE_TUR_A::COMPLETE_OR_TRANSMIT,
        }
    }
    #[doc = "Transmissions of address, control and data fields not completed / No transmitter FIFO underrun"]
    #[inline(always)]
    pub fn is_not_complete_or_transmit(&self) -> bool {
        *self == TPE_TUR_A::NOT_COMPLETE_OR_TRANSMIT
    }
    #[doc = "Transmissions of address, control and data fields completed / Transmitter FIFO underrun"]
    #[inline(always)]
    pub fn is_complete_or_transmit(&self) -> bool {
        *self == TPE_TUR_A::COMPLETE_OR_TRANSMIT
    }
}
#[doc = "Field `tpe_tur` writer - Transmitter Packet End Flag for Cyclical Pulse / TUR Transmitter FIFO Underrun Flag for Non-cyclical Pulse"]
pub type TPE_TUR_W<'a, REG> = crate::BitWriter1C<'a, REG, TPE_TUR_A>;
impl<'a, REG> TPE_TUR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmissions of address, control and data fields not completed / No transmitter FIFO underrun"]
    #[inline(always)]
    pub fn not_complete_or_transmit(self) -> &'a mut crate::W<REG> {
        self.variant(TPE_TUR_A::NOT_COMPLETE_OR_TRANSMIT)
    }
    #[doc = "Transmissions of address, control and data fields completed / Transmitter FIFO underrun"]
    #[inline(always)]
    pub fn complete_or_transmit(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> TAI_A {
        match self.bits {
            false => TAI_A::NOT_AVAILABLE,
            true => TAI_A::AVAILABLE,
        }
    }
    #[doc = "TX FIFO not available by its level"]
    #[inline(always)]
    pub fn is_not_available(&self) -> bool {
        *self == TAI_A::NOT_AVAILABLE
    }
    #[doc = "TX FIFO available by its level Writing 1 clears this bit."]
    #[inline(always)]
    pub fn is_available(&self) -> bool {
        *self == TAI_A::AVAILABLE
    }
}
#[doc = "Field `tai` writer - TX FIFO Available Interrupt Flag"]
pub type TAI_W<'a, REG> = crate::BitWriter<'a, REG, TAI_A>;
impl<'a, REG> TAI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TX FIFO not available by its level"]
    #[inline(always)]
    pub fn not_available(self) -> &'a mut crate::W<REG> {
        self.variant(TAI_A::NOT_AVAILABLE)
    }
    #[doc = "TX FIFO available by its level Writing 1 clears this bit."]
    #[inline(always)]
    pub fn available(self) -> &'a mut crate::W<REG> {
        self.variant(TAI_A::AVAILABLE)
    }
}
#[doc = "Field `drq` reader - DMA Request Flag"]
pub type DRQ_R = crate::BitReader;
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
    pub const fn variant(&self) -> STCT_A {
        match self.bits {
            false => STCT_A::IDLE,
            true => STCT_A::ACTIVE,
        }
    }
    #[doc = "Idle"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == STCT_A::IDLE
    }
    #[doc = "Active"]
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
    pub fn tpe_tur(&mut self) -> TPE_TUR_W<CIR_TXSTA_SPEC> {
        TPE_TUR_W::new(self, 0)
    }
    #[doc = "Bit 1 - TX FIFO Available Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tai(&mut self) -> TAI_W<CIR_TXSTA_SPEC> {
        TAI_W::new(self, 1)
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
#[doc = "CIR Transmit Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cir_txsta::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cir_txsta::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIR_TXSTA_SPEC;
impl crate::RegisterSpec for CIR_TXSTA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cir_txsta::R`](R) reader structure"]
impl crate::Readable for CIR_TXSTA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cir_txsta::W`](W) writer structure"]
impl crate::Writable for CIR_TXSTA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x01;
}
#[doc = "`reset()` method sets cir_txsta to value 0x02"]
impl crate::Resettable for CIR_TXSTA_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
