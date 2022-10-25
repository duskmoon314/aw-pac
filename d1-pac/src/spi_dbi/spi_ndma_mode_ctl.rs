#[doc = "Register `spi_ndma_mode_ctl` reader"]
pub struct R(crate::R<SPI_NDMA_MODE_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_NDMA_MODE_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_NDMA_MODE_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_NDMA_MODE_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `spi_ndma_mode_ctl` writer"]
pub struct W(crate::W<SPI_NDMA_MODE_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_NDMA_MODE_CTL_SPEC>;
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
impl From<crate::W<SPI_NDMA_MODE_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_NDMA_MODE_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `spi_dma_wait` reader - "]
pub type SPI_DMA_WAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `spi_dma_wait` writer - "]
pub type SPI_DMA_WAIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPI_NDMA_MODE_CTL_SPEC, u8, u8, 5, O>;
#[doc = "Field `spi_ack_m` reader - SPI NDMA Acknowledge Mode"]
pub type SPI_ACK_M_R = crate::BitReader<SPI_ACK_M_A>;
#[doc = "SPI NDMA Acknowledge Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI_ACK_M_A {
    #[doc = "0: `0`"]
    IGNORE = 0,
    #[doc = "1: `1`"]
    AFTER_DETECT = 1,
}
impl From<SPI_ACK_M_A> for bool {
    #[inline(always)]
    fn from(variant: SPI_ACK_M_A) -> Self {
        variant as u8 != 0
    }
}
impl SPI_ACK_M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI_ACK_M_A {
        match self.bits {
            false => SPI_ACK_M_A::IGNORE,
            true => SPI_ACK_M_A::AFTER_DETECT,
        }
    }
    #[doc = "Checks if the value of the field is `IGNORE`"]
    #[inline(always)]
    pub fn is_ignore(&self) -> bool {
        *self == SPI_ACK_M_A::IGNORE
    }
    #[doc = "Checks if the value of the field is `AFTER_DETECT`"]
    #[inline(always)]
    pub fn is_after_detect(&self) -> bool {
        *self == SPI_ACK_M_A::AFTER_DETECT
    }
}
#[doc = "Field `spi_ack_m` writer - SPI NDMA Acknowledge Mode"]
pub type SPI_ACK_M_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_NDMA_MODE_CTL_SPEC, SPI_ACK_M_A, O>;
impl<'a, const O: u8> SPI_ACK_M_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ignore(self) -> &'a mut W {
        self.variant(SPI_ACK_M_A::IGNORE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn after_detect(self) -> &'a mut W {
        self.variant(SPI_ACK_M_A::AFTER_DETECT)
    }
}
#[doc = "Field `spi_act_m` reader - SPI NDMA Active Mode"]
pub type SPI_ACT_M_R = crate::FieldReader<u8, SPI_ACT_M_A>;
#[doc = "SPI NDMA Active Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPI_ACT_M_A {
    #[doc = "0: `0`"]
    LOW = 0,
    #[doc = "1: `1`"]
    HIGH = 1,
    #[doc = "2: `10`"]
    DRQ_CONTROL = 2,
    #[doc = "3: `11`"]
    CONTROLLER_CONTROL = 3,
}
impl From<SPI_ACT_M_A> for u8 {
    #[inline(always)]
    fn from(variant: SPI_ACT_M_A) -> Self {
        variant as _
    }
}
impl SPI_ACT_M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI_ACT_M_A {
        match self.bits {
            0 => SPI_ACT_M_A::LOW,
            1 => SPI_ACT_M_A::HIGH,
            2 => SPI_ACT_M_A::DRQ_CONTROL,
            3 => SPI_ACT_M_A::CONTROLLER_CONTROL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SPI_ACT_M_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SPI_ACT_M_A::HIGH
    }
    #[doc = "Checks if the value of the field is `DRQ_CONTROL`"]
    #[inline(always)]
    pub fn is_drq_control(&self) -> bool {
        *self == SPI_ACT_M_A::DRQ_CONTROL
    }
    #[doc = "Checks if the value of the field is `CONTROLLER_CONTROL`"]
    #[inline(always)]
    pub fn is_controller_control(&self) -> bool {
        *self == SPI_ACT_M_A::CONTROLLER_CONTROL
    }
}
#[doc = "Field `spi_act_m` writer - SPI NDMA Active Mode"]
pub type SPI_ACT_M_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SPI_NDMA_MODE_CTL_SPEC, u8, SPI_ACT_M_A, 2, O>;
impl<'a, const O: u8> SPI_ACT_M_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SPI_ACT_M_A::LOW)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SPI_ACT_M_A::HIGH)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn drq_control(self) -> &'a mut W {
        self.variant(SPI_ACT_M_A::DRQ_CONTROL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn controller_control(self) -> &'a mut W {
        self.variant(SPI_ACT_M_A::CONTROLLER_CONTROL)
    }
}
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn spi_dma_wait(&self) -> SPI_DMA_WAIT_R {
        SPI_DMA_WAIT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - SPI NDMA Acknowledge Mode"]
    #[inline(always)]
    pub fn spi_ack_m(&self) -> SPI_ACK_M_R {
        SPI_ACK_M_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - SPI NDMA Active Mode"]
    #[inline(always)]
    pub fn spi_act_m(&self) -> SPI_ACT_M_R {
        SPI_ACT_M_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn spi_dma_wait(&mut self) -> SPI_DMA_WAIT_W<0> {
        SPI_DMA_WAIT_W::new(self)
    }
    #[doc = "Bit 5 - SPI NDMA Acknowledge Mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi_ack_m(&mut self) -> SPI_ACK_M_W<5> {
        SPI_ACK_M_W::new(self)
    }
    #[doc = "Bits 6:7 - SPI NDMA Active Mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi_act_m(&mut self) -> SPI_ACT_M_W<6> {
        SPI_ACT_M_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Normal DMA Mode Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_ndma_mode_ctl](index.html) module"]
pub struct SPI_NDMA_MODE_CTL_SPEC;
impl crate::RegisterSpec for SPI_NDMA_MODE_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_ndma_mode_ctl::R](R) reader structure"]
impl crate::Readable for SPI_NDMA_MODE_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_ndma_mode_ctl::W](W) writer structure"]
impl crate::Writable for SPI_NDMA_MODE_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets spi_ndma_mode_ctl to value 0"]
impl crate::Resettable for SPI_NDMA_MODE_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
