#[doc = "Register `i2s_pcm_fctl` reader"]
pub type R = crate::R<I2S_PCM_FCTL_SPEC>;
#[doc = "Register `i2s_pcm_fctl` writer"]
pub type W = crate::W<I2S_PCM_FCTL_SPEC>;
#[doc = "Field `rxom` reader - RXFIFO Output Mode\n\nExample for 20-bit received audio sample:\n\nMode 0: APB_RDATA\\[31:0\\] = {RXFIFO\\[31:12\\], 12’h0}\n\nMode 1: APB_RDATA\\[31:0\\] = {12{RXFIFO\\[31\\]}, RXFIFO\\[31:12\\]}\n\nMode 2: APB_RDATA \\[31:0\\] = {RXFIFO\\[31:16\\], 16’h0}\n\nMode 3: APB_RDATA\\[31:0\\] = {16{RXFIFO\\[31\\], RXFIFO\\[31:16\\]}"]
pub type RXOM_R = crate::FieldReader<RXOM_A>;
#[doc = "RXFIFO Output Mode\n\nExample for 20-bit received audio sample:\n\nMode 0: APB_RDATA\\[31:0\\] = {RXFIFO\\[31:12\\], 12’h0}\n\nMode 1: APB_RDATA\\[31:0\\] = {12{RXFIFO\\[31\\]}, RXFIFO\\[31:12\\]}\n\nMode 2: APB_RDATA \\[31:0\\] = {RXFIFO\\[31:16\\], 16’h0}\n\nMode 3: APB_RDATA\\[31:0\\] = {16{RXFIFO\\[31\\], RXFIFO\\[31:16\\]}\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXOM_A {
    #[doc = "0: Expanding ‘0’ at LSB of RXFIFO register"]
    MODE0 = 0,
    #[doc = "1: Expanding received sample sign bit at MSB of RXFIFO register"]
    MODE1 = 1,
    #[doc = "2: Truncating received samples at high half-word of RXFIFO register and low half-word of RXFIFO register is filled by ‘0’"]
    MODE2 = 2,
    #[doc = "3: Truncating received samples at low half-word of RXFIFO register and high half-word of RXFIFO register is expanded by its sign bit"]
    MODE3 = 3,
}
impl From<RXOM_A> for u8 {
    #[inline(always)]
    fn from(variant: RXOM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RXOM_A {
    type Ux = u8;
}
impl RXOM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXOM_A {
        match self.bits {
            0 => RXOM_A::MODE0,
            1 => RXOM_A::MODE1,
            2 => RXOM_A::MODE2,
            3 => RXOM_A::MODE3,
            _ => unreachable!(),
        }
    }
    #[doc = "Expanding ‘0’ at LSB of RXFIFO register"]
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == RXOM_A::MODE0
    }
    #[doc = "Expanding received sample sign bit at MSB of RXFIFO register"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == RXOM_A::MODE1
    }
    #[doc = "Truncating received samples at high half-word of RXFIFO register and low half-word of RXFIFO register is filled by ‘0’"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == RXOM_A::MODE2
    }
    #[doc = "Truncating received samples at low half-word of RXFIFO register and high half-word of RXFIFO register is expanded by its sign bit"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == RXOM_A::MODE3
    }
}
#[doc = "Field `rxom` writer - RXFIFO Output Mode\n\nExample for 20-bit received audio sample:\n\nMode 0: APB_RDATA\\[31:0\\] = {RXFIFO\\[31:12\\], 12’h0}\n\nMode 1: APB_RDATA\\[31:0\\] = {12{RXFIFO\\[31\\]}, RXFIFO\\[31:12\\]}\n\nMode 2: APB_RDATA \\[31:0\\] = {RXFIFO\\[31:16\\], 16’h0}\n\nMode 3: APB_RDATA\\[31:0\\] = {16{RXFIFO\\[31\\], RXFIFO\\[31:16\\]}"]
pub type RXOM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, RXOM_A>;
impl<'a, REG> RXOM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Expanding ‘0’ at LSB of RXFIFO register"]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut crate::W<REG> {
        self.variant(RXOM_A::MODE0)
    }
    #[doc = "Expanding received sample sign bit at MSB of RXFIFO register"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut crate::W<REG> {
        self.variant(RXOM_A::MODE1)
    }
    #[doc = "Truncating received samples at high half-word of RXFIFO register and low half-word of RXFIFO register is filled by ‘0’"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut crate::W<REG> {
        self.variant(RXOM_A::MODE2)
    }
    #[doc = "Truncating received samples at low half-word of RXFIFO register and high half-word of RXFIFO register is expanded by its sign bit"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut crate::W<REG> {
        self.variant(RXOM_A::MODE3)
    }
}
#[doc = "Field `txim` reader - TXFIFO Input Mode\n\nExample for 20-bit transmitted audio sample:\n\nMode 0: TXFIFO\\[31:0\\] = {APB_WDATA\\[31:12\\], 12’h0}\n\nMode 1: TXFIFO\\[31:0\\] = {APB_WDATA\\[19:0\\], 12’h0}"]
pub type TXIM_R = crate::BitReader<TXIM_A>;
#[doc = "TXFIFO Input Mode\n\nExample for 20-bit transmitted audio sample:\n\nMode 0: TXFIFO\\[31:0\\] = {APB_WDATA\\[31:12\\], 12’h0}\n\nMode 1: TXFIFO\\[31:0\\] = {APB_WDATA\\[19:0\\], 12’h0}\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXIM_A {
    #[doc = "0: Valid data at the MSB of TXFIFO register"]
    MODE0 = 0,
    #[doc = "1: Valid data at the LSB of TXFIFO register"]
    MODE1 = 1,
}
impl From<TXIM_A> for bool {
    #[inline(always)]
    fn from(variant: TXIM_A) -> Self {
        variant as u8 != 0
    }
}
impl TXIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXIM_A {
        match self.bits {
            false => TXIM_A::MODE0,
            true => TXIM_A::MODE1,
        }
    }
    #[doc = "Valid data at the MSB of TXFIFO register"]
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == TXIM_A::MODE0
    }
    #[doc = "Valid data at the LSB of TXFIFO register"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == TXIM_A::MODE1
    }
}
#[doc = "Field `txim` writer - TXFIFO Input Mode\n\nExample for 20-bit transmitted audio sample:\n\nMode 0: TXFIFO\\[31:0\\] = {APB_WDATA\\[31:12\\], 12’h0}\n\nMode 1: TXFIFO\\[31:0\\] = {APB_WDATA\\[19:0\\], 12’h0}"]
pub type TXIM_W<'a, REG> = crate::BitWriter<'a, REG, TXIM_A>;
impl<'a, REG> TXIM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Valid data at the MSB of TXFIFO register"]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut crate::W<REG> {
        self.variant(TXIM_A::MODE0)
    }
    #[doc = "Valid data at the LSB of TXFIFO register"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut crate::W<REG> {
        self.variant(TXIM_A::MODE1)
    }
}
#[doc = "Field `rxtl` reader - RXFIFO Empty Trigger Level\n\nInterrupt and DMA request trigger level for RXFIFO normal condition.\n\nTrigger Level = RXTL + 1"]
pub type RXTL_R = crate::FieldReader;
#[doc = "Field `rxtl` writer - RXFIFO Empty Trigger Level\n\nInterrupt and DMA request trigger level for RXFIFO normal condition.\n\nTrigger Level = RXTL + 1"]
pub type RXTL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `txtl` reader - TXFIFO Empty Trigger Level\n\nInterrupt and DMA request trigger level for TXFIFO normal condition.\n\nTrigger Level = TXTL"]
pub type TXTL_R = crate::FieldReader;
#[doc = "Field `txtl` writer - TXFIFO Empty Trigger Level\n\nInterrupt and DMA request trigger level for TXFIFO normal condition.\n\nTrigger Level = TXTL"]
pub type TXTL_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `frx` reader - Flush RXFIFO"]
pub type FRX_R = crate::BitReader<FRX_A>;
#[doc = "Flush RXFIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRX_A {
    #[doc = "0: `0`"]
    SELF_CLEAR = 0,
    #[doc = "1: `1`"]
    FLUSH = 1,
}
impl From<FRX_A> for bool {
    #[inline(always)]
    fn from(variant: FRX_A) -> Self {
        variant as u8 != 0
    }
}
impl FRX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FRX_A {
        match self.bits {
            false => FRX_A::SELF_CLEAR,
            true => FRX_A::FLUSH,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_self_clear(&self) -> bool {
        *self == FRX_A::SELF_CLEAR
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        *self == FRX_A::FLUSH
    }
}
#[doc = "Field `frx` writer - Flush RXFIFO"]
pub type FRX_W<'a, REG> = crate::BitWriter<'a, REG, FRX_A>;
impl<'a, REG> FRX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn self_clear(self) -> &'a mut crate::W<REG> {
        self.variant(FRX_A::SELF_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn flush(self) -> &'a mut crate::W<REG> {
        self.variant(FRX_A::FLUSH)
    }
}
#[doc = "Field `ftx` reader - Flush TXFIFO"]
pub type FTX_R = crate::BitReader<FTX_A>;
#[doc = "Flush TXFIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTX_A {
    #[doc = "0: `0`"]
    SELF_CLEAR = 0,
    #[doc = "1: `1`"]
    FLUSH = 1,
}
impl From<FTX_A> for bool {
    #[inline(always)]
    fn from(variant: FTX_A) -> Self {
        variant as u8 != 0
    }
}
impl FTX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FTX_A {
        match self.bits {
            false => FTX_A::SELF_CLEAR,
            true => FTX_A::FLUSH,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_self_clear(&self) -> bool {
        *self == FTX_A::SELF_CLEAR
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        *self == FTX_A::FLUSH
    }
}
#[doc = "Field `ftx` writer - Flush TXFIFO"]
pub type FTX_W<'a, REG> = crate::BitWriter<'a, REG, FTX_A>;
impl<'a, REG> FTX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn self_clear(self) -> &'a mut crate::W<REG> {
        self.variant(FTX_A::SELF_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn flush(self) -> &'a mut crate::W<REG> {
        self.variant(FTX_A::FLUSH)
    }
}
#[doc = "Field `hub_en` reader - Audio Hub Enable"]
pub type HUB_EN_R = crate::BitReader;
#[doc = "Field `hub_en` writer - Audio Hub Enable"]
pub type HUB_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - RXFIFO Output Mode\n\nExample for 20-bit received audio sample:\n\nMode 0: APB_RDATA\\[31:0\\] = {RXFIFO\\[31:12\\], 12’h0}\n\nMode 1: APB_RDATA\\[31:0\\] = {12{RXFIFO\\[31\\]}, RXFIFO\\[31:12\\]}\n\nMode 2: APB_RDATA \\[31:0\\] = {RXFIFO\\[31:16\\], 16’h0}\n\nMode 3: APB_RDATA\\[31:0\\] = {16{RXFIFO\\[31\\], RXFIFO\\[31:16\\]}"]
    #[inline(always)]
    pub fn rxom(&self) -> RXOM_R {
        RXOM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - TXFIFO Input Mode\n\nExample for 20-bit transmitted audio sample:\n\nMode 0: TXFIFO\\[31:0\\] = {APB_WDATA\\[31:12\\], 12’h0}\n\nMode 1: TXFIFO\\[31:0\\] = {APB_WDATA\\[19:0\\], 12’h0}"]
    #[inline(always)]
    pub fn txim(&self) -> TXIM_R {
        TXIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:9 - RXFIFO Empty Trigger Level\n\nInterrupt and DMA request trigger level for RXFIFO normal condition.\n\nTrigger Level = RXTL + 1"]
    #[inline(always)]
    pub fn rxtl(&self) -> RXTL_R {
        RXTL_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 12:18 - TXFIFO Empty Trigger Level\n\nInterrupt and DMA request trigger level for TXFIFO normal condition.\n\nTrigger Level = TXTL"]
    #[inline(always)]
    pub fn txtl(&self) -> TXTL_R {
        TXTL_R::new(((self.bits >> 12) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - Flush RXFIFO"]
    #[inline(always)]
    pub fn frx(&self) -> FRX_R {
        FRX_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Flush TXFIFO"]
    #[inline(always)]
    pub fn ftx(&self) -> FTX_R {
        FTX_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 31 - Audio Hub Enable"]
    #[inline(always)]
    pub fn hub_en(&self) -> HUB_EN_R {
        HUB_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - RXFIFO Output Mode\n\nExample for 20-bit received audio sample:\n\nMode 0: APB_RDATA\\[31:0\\] = {RXFIFO\\[31:12\\], 12’h0}\n\nMode 1: APB_RDATA\\[31:0\\] = {12{RXFIFO\\[31\\]}, RXFIFO\\[31:12\\]}\n\nMode 2: APB_RDATA \\[31:0\\] = {RXFIFO\\[31:16\\], 16’h0}\n\nMode 3: APB_RDATA\\[31:0\\] = {16{RXFIFO\\[31\\], RXFIFO\\[31:16\\]}"]
    #[inline(always)]
    #[must_use]
    pub fn rxom(&mut self) -> RXOM_W<I2S_PCM_FCTL_SPEC> {
        RXOM_W::new(self, 0)
    }
    #[doc = "Bit 2 - TXFIFO Input Mode\n\nExample for 20-bit transmitted audio sample:\n\nMode 0: TXFIFO\\[31:0\\] = {APB_WDATA\\[31:12\\], 12’h0}\n\nMode 1: TXFIFO\\[31:0\\] = {APB_WDATA\\[19:0\\], 12’h0}"]
    #[inline(always)]
    #[must_use]
    pub fn txim(&mut self) -> TXIM_W<I2S_PCM_FCTL_SPEC> {
        TXIM_W::new(self, 2)
    }
    #[doc = "Bits 4:9 - RXFIFO Empty Trigger Level\n\nInterrupt and DMA request trigger level for RXFIFO normal condition.\n\nTrigger Level = RXTL + 1"]
    #[inline(always)]
    #[must_use]
    pub fn rxtl(&mut self) -> RXTL_W<I2S_PCM_FCTL_SPEC> {
        RXTL_W::new(self, 4)
    }
    #[doc = "Bits 12:18 - TXFIFO Empty Trigger Level\n\nInterrupt and DMA request trigger level for TXFIFO normal condition.\n\nTrigger Level = TXTL"]
    #[inline(always)]
    #[must_use]
    pub fn txtl(&mut self) -> TXTL_W<I2S_PCM_FCTL_SPEC> {
        TXTL_W::new(self, 12)
    }
    #[doc = "Bit 24 - Flush RXFIFO"]
    #[inline(always)]
    #[must_use]
    pub fn frx(&mut self) -> FRX_W<I2S_PCM_FCTL_SPEC> {
        FRX_W::new(self, 24)
    }
    #[doc = "Bit 25 - Flush TXFIFO"]
    #[inline(always)]
    #[must_use]
    pub fn ftx(&mut self) -> FTX_W<I2S_PCM_FCTL_SPEC> {
        FTX_W::new(self, 25)
    }
    #[doc = "Bit 31 - Audio Hub Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hub_en(&mut self) -> HUB_EN_W<I2S_PCM_FCTL_SPEC> {
        HUB_EN_W::new(self, 31)
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
#[doc = "I2S/PCM FIFO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_fctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_fctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2S_PCM_FCTL_SPEC;
impl crate::RegisterSpec for I2S_PCM_FCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_pcm_fctl::R`](R) reader structure"]
impl crate::Readable for I2S_PCM_FCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2s_pcm_fctl::W`](W) writer structure"]
impl crate::Writable for I2S_PCM_FCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets i2s_pcm_fctl to value 0"]
impl crate::Resettable for I2S_PCM_FCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
