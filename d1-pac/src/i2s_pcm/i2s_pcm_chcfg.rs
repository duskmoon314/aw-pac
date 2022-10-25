#[doc = "Register `i2s_pcm_chcfg` reader"]
pub struct R(crate::R<I2S_PCM_CHCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_PCM_CHCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_PCM_CHCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_PCM_CHCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `i2s_pcm_chcfg` writer"]
pub struct W(crate::W<I2S_PCM_CHCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_PCM_CHCFG_SPEC>;
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
impl From<crate::W<I2S_PCM_CHCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_PCM_CHCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tx_slot_num` reader - TX Channel/Slot number between CPU/DMA and RXFIFO\n\nChannel or slot = N + 1"]
pub type TX_SLOT_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tx_slot_num` writer - TX Channel/Slot number between CPU/DMA and RXFIFO\n\nChannel or slot = N + 1"]
pub type TX_SLOT_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_PCM_CHCFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `rx_slot_num` reader - RX Channel/Slot number between CPU/DMA and RXFIFO\n\nChannel or slot = N + 1"]
pub type RX_SLOT_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rx_slot_num` writer - RX Channel/Slot number between CPU/DMA and RXFIFO\n\nChannel or slot = N + 1"]
pub type RX_SLOT_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_PCM_CHCFG_SPEC, u8, u8, 4, O>;
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
    pub fn variant(&self) -> TX_STATE_A {
        match self.bits {
            false => TX_STATE_A::ZERO,
            true => TX_STATE_A::HIZ,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == TX_STATE_A::ZERO
    }
    #[doc = "Checks if the value of the field is `HIZ`"]
    #[inline(always)]
    pub fn is_hiz(&self) -> bool {
        *self == TX_STATE_A::HIZ
    }
}
#[doc = "Field `tx_state` writer - "]
pub type TX_STATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_PCM_CHCFG_SPEC, TX_STATE_A, O>;
impl<'a, const O: u8> TX_STATE_W<'a, O> {
    #[doc = "Transfer level 0 in non-transferring slot"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(TX_STATE_A::ZERO)
    }
    #[doc = "Turn to Hi-Z State (TDM) in non-transferring slot"]
    #[inline(always)]
    pub fn hiz(self) -> &'a mut W {
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
    pub fn variant(&self) -> TX_SLOT_HIZ_A {
        match self.bits {
            false => TX_SLOT_HIZ_A::NORMAL,
            true => TX_SLOT_HIZ_A::HIZ,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == TX_SLOT_HIZ_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `HIZ`"]
    #[inline(always)]
    pub fn is_hiz(&self) -> bool {
        *self == TX_SLOT_HIZ_A::HIZ
    }
}
#[doc = "Field `tx_slot_hiz` writer - "]
pub type TX_SLOT_HIZ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, I2S_PCM_CHCFG_SPEC, TX_SLOT_HIZ_A, O>;
impl<'a, const O: u8> TX_SLOT_HIZ_W<'a, O> {
    #[doc = "Normal mode for the last half-cycle of BCLK in the slot"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TX_SLOT_HIZ_A::NORMAL)
    }
    #[doc = "Turn to Hi-Z state for the last half-cycle of BCLK in the slot"]
    #[inline(always)]
    pub fn hiz(self) -> &'a mut W {
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
    pub fn tx_slot_num(&mut self) -> TX_SLOT_NUM_W<0> {
        TX_SLOT_NUM_W::new(self)
    }
    #[doc = "Bits 4:7 - RX Channel/Slot number between CPU/DMA and RXFIFO\n\nChannel or slot = N + 1"]
    #[inline(always)]
    #[must_use]
    pub fn rx_slot_num(&mut self) -> RX_SLOT_NUM_W<4> {
        RX_SLOT_NUM_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn tx_state(&mut self) -> TX_STATE_W<8> {
        TX_STATE_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn tx_slot_hiz(&mut self) -> TX_SLOT_HIZ_W<9> {
        TX_SLOT_HIZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S/PCM Channel Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_pcm_chcfg](index.html) module"]
pub struct I2S_PCM_CHCFG_SPEC;
impl crate::RegisterSpec for I2S_PCM_CHCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_pcm_chcfg::R](R) reader structure"]
impl crate::Readable for I2S_PCM_CHCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_pcm_chcfg::W](W) writer structure"]
impl crate::Writable for I2S_PCM_CHCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets i2s_pcm_chcfg to value 0"]
impl crate::Resettable for I2S_PCM_CHCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
