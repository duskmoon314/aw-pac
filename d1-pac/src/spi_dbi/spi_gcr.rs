#[doc = "Register `spi_gcr` reader"]
pub struct R(crate::R<SPI_GCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_GCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_GCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_GCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `spi_gcr` writer"]
pub struct W(crate::W<SPI_GCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_GCR_SPEC>;
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
impl From<crate::W<SPI_GCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_GCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `en` reader - SPI Module Enable Control"]
pub type EN_R = crate::BitReader<EN_A>;
#[doc = "SPI Module Enable Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::DISABLE,
            true => EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EN_A::ENABLE
    }
}
#[doc = "Field `en` writer - SPI Module Enable Control"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_GCR_SPEC, EN_A, O>;
impl<'a, const O: u8> EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EN_A::ENABLE)
    }
}
#[doc = "Field `mode` reader - SPI Function Mode Select"]
pub type MODE_R = crate::BitReader<MODE_A>;
#[doc = "SPI Function Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODE_A {
    #[doc = "0: `0`"]
    SLAVE = 0,
    #[doc = "1: `1`"]
    MASTER = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::SLAVE,
            true => MODE_A::MASTER,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE`"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == MODE_A::SLAVE
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == MODE_A::MASTER
    }
}
#[doc = "Field `mode` writer - SPI Function Mode Select"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_GCR_SPEC, MODE_A, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn slave(self) -> &'a mut W {
        self.variant(MODE_A::SLAVE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn master(self) -> &'a mut W {
        self.variant(MODE_A::MASTER)
    }
}
#[doc = "Field `mode_selec` reader - Sample timing Mode Select"]
pub type MODE_SELEC_R = crate::BitReader<MODE_SELEC_A>;
#[doc = "Sample timing Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODE_SELEC_A {
    #[doc = "0: Old mode of Sample Timing"]
    OLD_MODE = 0,
    #[doc = "1: New mode of Sample Timing"]
    NEW_MODE = 1,
}
impl From<MODE_SELEC_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_SELEC_A) -> Self {
        variant as u8 != 0
    }
}
impl MODE_SELEC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_SELEC_A {
        match self.bits {
            false => MODE_SELEC_A::OLD_MODE,
            true => MODE_SELEC_A::NEW_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `OLD_MODE`"]
    #[inline(always)]
    pub fn is_old_mode(&self) -> bool {
        *self == MODE_SELEC_A::OLD_MODE
    }
    #[doc = "Checks if the value of the field is `NEW_MODE`"]
    #[inline(always)]
    pub fn is_new_mode(&self) -> bool {
        *self == MODE_SELEC_A::NEW_MODE
    }
}
#[doc = "Field `mode_selec` writer - Sample timing Mode Select"]
pub type MODE_SELEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_GCR_SPEC, MODE_SELEC_A, O>;
impl<'a, const O: u8> MODE_SELEC_W<'a, O> {
    #[doc = "Old mode of Sample Timing"]
    #[inline(always)]
    pub fn old_mode(self) -> &'a mut W {
        self.variant(MODE_SELEC_A::OLD_MODE)
    }
    #[doc = "New mode of Sample Timing"]
    #[inline(always)]
    pub fn new_mode(self) -> &'a mut W {
        self.variant(MODE_SELEC_A::NEW_MODE)
    }
}
#[doc = "Field `tp_en` reader - Transmit Pause Enable"]
pub type TP_EN_R = crate::BitReader<TP_EN_A>;
#[doc = "Transmit Pause Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TP_EN_A {
    #[doc = "0: normal operation, ignore RXFIFO status"]
    NORMAL = 0,
    #[doc = "1: Stop transmit data when RXFIFO full"]
    STOP_WHEN_FULL = 1,
}
impl From<TP_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TP_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TP_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP_EN_A {
        match self.bits {
            false => TP_EN_A::NORMAL,
            true => TP_EN_A::STOP_WHEN_FULL,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == TP_EN_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `STOP_WHEN_FULL`"]
    #[inline(always)]
    pub fn is_stop_when_full(&self) -> bool {
        *self == TP_EN_A::STOP_WHEN_FULL
    }
}
#[doc = "Field `tp_en` writer - Transmit Pause Enable"]
pub type TP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_GCR_SPEC, TP_EN_A, O>;
impl<'a, const O: u8> TP_EN_W<'a, O> {
    #[doc = "normal operation, ignore RXFIFO status"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TP_EN_A::NORMAL)
    }
    #[doc = "Stop transmit data when RXFIFO full"]
    #[inline(always)]
    pub fn stop_when_full(self) -> &'a mut W {
        self.variant(TP_EN_A::STOP_WHEN_FULL)
    }
}
#[doc = "Field `srst` reader - Soft reset"]
pub type SRST_R = crate::BitReader<bool>;
#[doc = "Field `srst` writer - Soft reset"]
pub type SRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_GCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SPI Module Enable Control"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI Function Mode Select"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sample timing Mode Select"]
    #[inline(always)]
    pub fn mode_selec(&self) -> MODE_SELEC_R {
        MODE_SELEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit Pause Enable"]
    #[inline(always)]
    pub fn tp_en(&self) -> TP_EN_R {
        TP_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 31 - Soft reset"]
    #[inline(always)]
    pub fn srst(&self) -> SRST_R {
        SRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI Module Enable Control"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - SPI Function Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<1> {
        MODE_W::new(self)
    }
    #[doc = "Bit 2 - Sample timing Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn mode_selec(&mut self) -> MODE_SELEC_W<2> {
        MODE_SELEC_W::new(self)
    }
    #[doc = "Bit 7 - Transmit Pause Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tp_en(&mut self) -> TP_EN_W<7> {
        TP_EN_W::new(self)
    }
    #[doc = "Bit 31 - Soft reset"]
    #[inline(always)]
    #[must_use]
    pub fn srst(&mut self) -> SRST_W<31> {
        SRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Global Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_gcr](index.html) module"]
pub struct SPI_GCR_SPEC;
impl crate::RegisterSpec for SPI_GCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_gcr::R](R) reader structure"]
impl crate::Readable for SPI_GCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_gcr::W](W) writer structure"]
impl crate::Writable for SPI_GCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets spi_gcr to value 0"]
impl crate::Resettable for SPI_GCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
