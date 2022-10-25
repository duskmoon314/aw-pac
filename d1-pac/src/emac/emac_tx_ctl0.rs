#[doc = "Register `emac_tx_ctl0` reader"]
pub struct R(crate::R<EMAC_TX_CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMAC_TX_CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMAC_TX_CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMAC_TX_CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `emac_tx_ctl0` writer"]
pub struct W(crate::W<EMAC_TX_CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMAC_TX_CTL0_SPEC>;
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
impl From<crate::W<EMAC_TX_CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMAC_TX_CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tx_frm_len_ctl` reader - Frame Transmit Length Control"]
pub type TX_FRM_LEN_CTL_R = crate::BitReader<TX_FRM_LEN_CTL_A>;
#[doc = "Frame Transmit Length Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_FRM_LEN_CTL_A {
    #[doc = "0: `0`"]
    B2048 = 0,
    #[doc = "1: `1`"]
    B16384 = 1,
}
impl From<TX_FRM_LEN_CTL_A> for bool {
    #[inline(always)]
    fn from(variant: TX_FRM_LEN_CTL_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_FRM_LEN_CTL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_FRM_LEN_CTL_A {
        match self.bits {
            false => TX_FRM_LEN_CTL_A::B2048,
            true => TX_FRM_LEN_CTL_A::B16384,
        }
    }
    #[doc = "Checks if the value of the field is `B2048`"]
    #[inline(always)]
    pub fn is_b2048(&self) -> bool {
        *self == TX_FRM_LEN_CTL_A::B2048
    }
    #[doc = "Checks if the value of the field is `B16384`"]
    #[inline(always)]
    pub fn is_b16384(&self) -> bool {
        *self == TX_FRM_LEN_CTL_A::B16384
    }
}
#[doc = "Field `tx_frm_len_ctl` writer - Frame Transmit Length Control"]
pub type TX_FRM_LEN_CTL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EMAC_TX_CTL0_SPEC, TX_FRM_LEN_CTL_A, O>;
impl<'a, const O: u8> TX_FRM_LEN_CTL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn b2048(self) -> &'a mut W {
        self.variant(TX_FRM_LEN_CTL_A::B2048)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn b16384(self) -> &'a mut W {
        self.variant(TX_FRM_LEN_CTL_A::B16384)
    }
}
#[doc = "Field `tx_en` reader - Enable Transmitter"]
pub type TX_EN_R = crate::BitReader<TX_EN_A>;
#[doc = "Enable Transmitter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TX_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TX_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_EN_A {
        match self.bits {
            false => TX_EN_A::DISABLE,
            true => TX_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TX_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TX_EN_A::ENABLE
    }
}
#[doc = "Field `tx_en` writer - Enable Transmitter"]
pub type TX_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMAC_TX_CTL0_SPEC, TX_EN_A, O>;
impl<'a, const O: u8> TX_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TX_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TX_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 30 - Frame Transmit Length Control"]
    #[inline(always)]
    pub fn tx_frm_len_ctl(&self) -> TX_FRM_LEN_CTL_R {
        TX_FRM_LEN_CTL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Transmitter"]
    #[inline(always)]
    pub fn tx_en(&self) -> TX_EN_R {
        TX_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - Frame Transmit Length Control"]
    #[inline(always)]
    #[must_use]
    pub fn tx_frm_len_ctl(&mut self) -> TX_FRM_LEN_CTL_W<30> {
        TX_FRM_LEN_CTL_W::new(self)
    }
    #[doc = "Bit 31 - Enable Transmitter"]
    #[inline(always)]
    #[must_use]
    pub fn tx_en(&mut self) -> TX_EN_W<31> {
        TX_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EMAC Transmit Control Register0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emac_tx_ctl0](index.html) module"]
pub struct EMAC_TX_CTL0_SPEC;
impl crate::RegisterSpec for EMAC_TX_CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emac_tx_ctl0::R](R) reader structure"]
impl crate::Readable for EMAC_TX_CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emac_tx_ctl0::W](W) writer structure"]
impl crate::Writable for EMAC_TX_CTL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets emac_tx_ctl0 to value 0"]
impl crate::Resettable for EMAC_TX_CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
