#[doc = "Register `smhc_fifoth` reader"]
pub struct R(crate::R<SMHC_FIFOTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_FIFOTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_FIFOTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_FIFOTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `smhc_fifoth` writer"]
pub struct W(crate::W<SMHC_FIFOTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMHC_FIFOTH_SPEC>;
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
impl From<crate::W<SMHC_FIFOTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMHC_FIFOTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tx_tl` reader - TX Trigger Level"]
pub type TX_TL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tx_tl` writer - TX Trigger Level"]
pub type TX_TL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMHC_FIFOTH_SPEC, u8, u8, 8, O>;
#[doc = "Field `rx_tl` reader - RX Trigger Level"]
pub type RX_TL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rx_tl` writer - RX Trigger Level"]
pub type RX_TL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMHC_FIFOTH_SPEC, u8, u8, 8, O>;
#[doc = "Field `bsize_of_trans` reader - sBurst Size of Multiple Transaction"]
pub type BSIZE_OF_TRANS_R = crate::FieldReader<u8, BSIZE_OF_TRANS_A>;
#[doc = "sBurst Size of Multiple Transaction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BSIZE_OF_TRANS_A {
    #[doc = "0: 1 transfer"]
    T1 = 0,
    #[doc = "1: 4 transfers"]
    T4 = 1,
    #[doc = "2: 8 transfers"]
    T8 = 2,
    #[doc = "3: 16 transfers"]
    T16 = 3,
}
impl From<BSIZE_OF_TRANS_A> for u8 {
    #[inline(always)]
    fn from(variant: BSIZE_OF_TRANS_A) -> Self {
        variant as _
    }
}
impl BSIZE_OF_TRANS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BSIZE_OF_TRANS_A> {
        match self.bits {
            0 => Some(BSIZE_OF_TRANS_A::T1),
            1 => Some(BSIZE_OF_TRANS_A::T4),
            2 => Some(BSIZE_OF_TRANS_A::T8),
            3 => Some(BSIZE_OF_TRANS_A::T16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `T1`"]
    #[inline(always)]
    pub fn is_t1(&self) -> bool {
        *self == BSIZE_OF_TRANS_A::T1
    }
    #[doc = "Checks if the value of the field is `T4`"]
    #[inline(always)]
    pub fn is_t4(&self) -> bool {
        *self == BSIZE_OF_TRANS_A::T4
    }
    #[doc = "Checks if the value of the field is `T8`"]
    #[inline(always)]
    pub fn is_t8(&self) -> bool {
        *self == BSIZE_OF_TRANS_A::T8
    }
    #[doc = "Checks if the value of the field is `T16`"]
    #[inline(always)]
    pub fn is_t16(&self) -> bool {
        *self == BSIZE_OF_TRANS_A::T16
    }
}
#[doc = "Field `bsize_of_trans` writer - sBurst Size of Multiple Transaction"]
pub type BSIZE_OF_TRANS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SMHC_FIFOTH_SPEC, u8, BSIZE_OF_TRANS_A, 3, O>;
impl<'a, const O: u8> BSIZE_OF_TRANS_W<'a, O> {
    #[doc = "1 transfer"]
    #[inline(always)]
    pub fn t1(self) -> &'a mut W {
        self.variant(BSIZE_OF_TRANS_A::T1)
    }
    #[doc = "4 transfers"]
    #[inline(always)]
    pub fn t4(self) -> &'a mut W {
        self.variant(BSIZE_OF_TRANS_A::T4)
    }
    #[doc = "8 transfers"]
    #[inline(always)]
    pub fn t8(self) -> &'a mut W {
        self.variant(BSIZE_OF_TRANS_A::T8)
    }
    #[doc = "16 transfers"]
    #[inline(always)]
    pub fn t16(self) -> &'a mut W {
        self.variant(BSIZE_OF_TRANS_A::T16)
    }
}
impl R {
    #[doc = "Bits 0:7 - TX Trigger Level"]
    #[inline(always)]
    pub fn tx_tl(&self) -> TX_TL_R {
        TX_TL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - RX Trigger Level"]
    #[inline(always)]
    pub fn rx_tl(&self) -> RX_TL_R {
        RX_TL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 28:30 - sBurst Size of Multiple Transaction"]
    #[inline(always)]
    pub fn bsize_of_trans(&self) -> BSIZE_OF_TRANS_R {
        BSIZE_OF_TRANS_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TX Trigger Level"]
    #[inline(always)]
    #[must_use]
    pub fn tx_tl(&mut self) -> TX_TL_W<0> {
        TX_TL_W::new(self)
    }
    #[doc = "Bits 16:23 - RX Trigger Level"]
    #[inline(always)]
    #[must_use]
    pub fn rx_tl(&mut self) -> RX_TL_W<16> {
        RX_TL_W::new(self)
    }
    #[doc = "Bits 28:30 - sBurst Size of Multiple Transaction"]
    #[inline(always)]
    #[must_use]
    pub fn bsize_of_trans(&mut self) -> BSIZE_OF_TRANS_W<28> {
        BSIZE_OF_TRANS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Water Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_fifoth](index.html) module"]
pub struct SMHC_FIFOTH_SPEC;
impl crate::RegisterSpec for SMHC_FIFOTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_fifoth::R](R) reader structure"]
impl crate::Readable for SMHC_FIFOTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smhc_fifoth::W](W) writer structure"]
impl crate::Writable for SMHC_FIFOTH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets smhc_fifoth to value 0"]
impl crate::Resettable for SMHC_FIFOTH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
