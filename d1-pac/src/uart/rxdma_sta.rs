#[doc = "Register `rxdma_sta` reader"]
pub struct R(crate::R<RXDMA_STA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXDMA_STA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXDMA_STA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXDMA_STA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rxdma_sta` writer"]
pub struct W(crate::W<RXDMA_STA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXDMA_STA_SPEC>;
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
impl From<crate::W<RXDMA_STA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXDMA_STA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `busy` reader - "]
pub type BUSY_R = crate::BitReader<BUSY_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY_A {
    #[doc = "0: `0`"]
    IDLE = 0,
    #[doc = "1: `1`"]
    BUSY = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::IDLE,
            true => BUSY_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == BUSY_A::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY_A::BUSY
    }
}
#[doc = "Field `busy` writer - "]
pub type BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXDMA_STA_SPEC, BUSY_A, O>;
impl<'a, const O: u8> BUSY_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(BUSY_A::IDLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn busy(self) -> &'a mut W {
        self.variant(BUSY_A::BUSY)
    }
}
#[doc = "Field `buffer_read_address_updating` reader - "]
pub type BUFFER_READ_ADDRESS_UPDATING_R = crate::BitReader<BUFFER_READ_ADDRESS_UPDATING_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFFER_READ_ADDRESS_UPDATING_A {
    #[doc = "0: `0`"]
    READY = 0,
    #[doc = "1: `1`"]
    BUSY = 1,
}
impl From<BUFFER_READ_ADDRESS_UPDATING_A> for bool {
    #[inline(always)]
    fn from(variant: BUFFER_READ_ADDRESS_UPDATING_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFFER_READ_ADDRESS_UPDATING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFFER_READ_ADDRESS_UPDATING_A {
        match self.bits {
            false => BUFFER_READ_ADDRESS_UPDATING_A::READY,
            true => BUFFER_READ_ADDRESS_UPDATING_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == BUFFER_READ_ADDRESS_UPDATING_A::READY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUFFER_READ_ADDRESS_UPDATING_A::BUSY
    }
}
#[doc = "Field `buffer_read_address_updating` writer - "]
pub type BUFFER_READ_ADDRESS_UPDATING_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RXDMA_STA_SPEC, BUFFER_READ_ADDRESS_UPDATING_A, O>;
impl<'a, const O: u8> BUFFER_READ_ADDRESS_UPDATING_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(BUFFER_READ_ADDRESS_UPDATING_A::READY)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn busy(self) -> &'a mut W {
        self.variant(BUFFER_READ_ADDRESS_UPDATING_A::BUSY)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn buffer_read_address_updating(&self) -> BUFFER_READ_ADDRESS_UPDATING_R {
        BUFFER_READ_ADDRESS_UPDATING_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BUSY_W<0> {
        BUSY_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn buffer_read_address_updating(&mut self) -> BUFFER_READ_ADDRESS_UPDATING_W<1> {
        BUFFER_READ_ADDRESS_UPDATING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART RXDMA Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdma_sta](index.html) module"]
pub struct RXDMA_STA_SPEC;
impl crate::RegisterSpec for RXDMA_STA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxdma_sta::R](R) reader structure"]
impl crate::Readable for RXDMA_STA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxdma_sta::W](W) writer structure"]
impl crate::Writable for RXDMA_STA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rxdma_sta to value 0"]
impl crate::Resettable for RXDMA_STA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
