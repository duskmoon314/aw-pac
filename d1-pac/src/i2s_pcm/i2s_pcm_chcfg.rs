#[doc = "Register `i2s_pcm_chcfg` reader"]
pub type R = crate::R<I2S_PCM_CHCFG_SPEC>;
#[doc = "Register `i2s_pcm_chcfg` writer"]
pub type W = crate::W<I2S_PCM_CHCFG_SPEC>;
#[doc = "Field `tx_slot_num` reader - TX Channel/Slot number between CPU/DMA and RXFIFO\n\nChannel or slot = N + 1"]
pub type TX_SLOT_NUM_R = crate::FieldReader;
#[doc = "Field `tx_slot_num` writer - TX Channel/Slot number between CPU/DMA and RXFIFO\n\nChannel or slot = N + 1"]
pub type TX_SLOT_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `rx_slot_num` reader - RX Channel/Slot number between CPU/DMA and RXFIFO\n\nChannel or slot = N + 1"]
pub type RX_SLOT_NUM_R = crate::FieldReader;
#[doc = "Field `rx_slot_num` writer - RX Channel/Slot number between CPU/DMA and RXFIFO\n\nChannel or slot = N + 1"]
pub type RX_SLOT_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `tx_state` reader - "]
pub type TX_STATE_R = crate::BitReader<TX_STATE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_STATE_A {
    #[doc = "0: Transfer level 0 in non-transferring slot"]
    ZERO = 0,
    #[doc = "1: Turn to Hi-Z State (TDM) in non-transferring slot"]
    HIZ = 1,
}
impl From<TX_STATE_A> for bool {
    #[inline(always)]
    fn from(variant: TX_STATE_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_STATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TX_STATE_A {
        match self.bits {
            false => TX_STATE_A::ZERO,
            true => TX_STATE_A::HIZ,
        }
    }
    #[doc = "Transfer level 0 in non-transferring slot"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == TX_STATE_A::ZERO
    }
    #[doc = "Turn to Hi-Z State (TDM) in non-transferring slot"]
    #[inline(always)]
    pub fn is_hiz(&self) -> bool {
        *self == TX_STATE_A::HIZ
    }
}
#[doc = "Field `tx_state` writer - "]
pub type TX_STATE_W<'a, REG> = crate::BitWriter<'a, REG, TX_STATE_A>;
impl<'a, REG> TX_STATE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transfer level 0 in non-transferring slot"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(TX_STATE_A::ZERO)
    }
    #[doc = "Turn to Hi-Z State (TDM) in non-transferring slot"]
    #[inline(always)]
    pub fn hiz(self) -> &'a mut crate::W<REG> {
        self.variant(TX_STATE_A::HIZ)
    }
}
#[doc = "Field `tx_slot_hiz` reader - "]
pub type TX_SLOT_HIZ_R = crate::BitReader<TX_SLOT_HIZ_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_SLOT_HIZ_A {
    #[doc = "0: Normal mode for the last half-cycle of BCLK in the slot"]
    NORMAL = 0,
    #[doc = "1: Turn to Hi-Z state for the last half-cycle of BCLK in the slot"]
    HIZ = 1,
}
impl From<TX_SLOT_HIZ_A> for bool {
    #[inline(always)]
    fn from(variant: TX_SLOT_HIZ_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_SLOT_HIZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TX_SLOT_HIZ_A {
        match self.bits {
            false => TX_SLOT_HIZ_A::NORMAL,
            true => TX_SLOT_HIZ_A::HIZ,
        }
    }
    #[doc = "Normal mode for the last half-cycle of BCLK in the slot"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == TX_SLOT_HIZ_A::NORMAL
    }
    #[doc = "Turn to Hi-Z state for the last half-cycle of BCLK in the slot"]
    #[inline(always)]
    pub fn is_hiz(&self) -> bool {
        *self == TX_SLOT_HIZ_A::HIZ
    }
}
#[doc = "Field `tx_slot_hiz` writer - "]
pub type TX_SLOT_HIZ_W<'a, REG> = crate::BitWriter<'a, REG, TX_SLOT_HIZ_A>;
impl<'a, REG> TX_SLOT_HIZ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode for the last half-cycle of BCLK in the slot"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(TX_SLOT_HIZ_A::NORMAL)
    }
    #[doc = "Turn to Hi-Z state for the last half-cycle of BCLK in the slot"]
    #[inline(always)]
    pub fn hiz(self) -> &'a mut crate::W<REG> {
        self.variant(TX_SLOT_HIZ_A::HIZ)
    }
}
impl R {
    #[doc = "Bits 0:3 - TX Channel/Slot number between CPU/DMA and RXFIFO\n\nChannel or slot = N + 1"]
    #[inline(always)]
    pub fn tx_slot_num(&self) -> TX_SLOT_NUM_R {
        TX_SLOT_NUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - RX Channel/Slot number between CPU/DMA and RXFIFO\n\nChannel or slot = N + 1"]
    #[inline(always)]
    pub fn rx_slot_num(&self) -> RX_SLOT_NUM_R {
        RX_SLOT_NUM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tx_state(&self) -> TX_STATE_R {
        TX_STATE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn tx_slot_hiz(&self) -> TX_SLOT_HIZ_R {
        TX_SLOT_HIZ_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - TX Channel/Slot number between CPU/DMA and RXFIFO\n\nChannel or slot = N + 1"]
    #[inline(always)]
    #[must_use]
    pub fn tx_slot_num(&mut self) -> TX_SLOT_NUM_W<I2S_PCM_CHCFG_SPEC> {
        TX_SLOT_NUM_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - RX Channel/Slot number between CPU/DMA and RXFIFO\n\nChannel or slot = N + 1"]
    #[inline(always)]
    #[must_use]
    pub fn rx_slot_num(&mut self) -> RX_SLOT_NUM_W<I2S_PCM_CHCFG_SPEC> {
        RX_SLOT_NUM_W::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn tx_state(&mut self) -> TX_STATE_W<I2S_PCM_CHCFG_SPEC> {
        TX_STATE_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn tx_slot_hiz(&mut self) -> TX_SLOT_HIZ_W<I2S_PCM_CHCFG_SPEC> {
        TX_SLOT_HIZ_W::new(self, 9)
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
#[doc = "I2S/PCM Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_chcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_chcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2S_PCM_CHCFG_SPEC;
impl crate::RegisterSpec for I2S_PCM_CHCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_pcm_chcfg::R`](R) reader structure"]
impl crate::Readable for I2S_PCM_CHCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2s_pcm_chcfg::W`](W) writer structure"]
impl crate::Writable for I2S_PCM_CHCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets i2s_pcm_chcfg to value 0"]
impl crate::Resettable for I2S_PCM_CHCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
