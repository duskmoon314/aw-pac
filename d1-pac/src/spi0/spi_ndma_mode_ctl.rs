#[doc = "Register `spi_ndma_mode_ctl` reader"]
pub type R = crate::R<SPI_NDMA_MODE_CTL_SPEC>;
#[doc = "Register `spi_ndma_mode_ctl` writer"]
pub type W = crate::W<SPI_NDMA_MODE_CTL_SPEC>;
#[doc = "Field `spi_dma_wait` reader - "]
pub type SPI_DMA_WAIT_R = crate::FieldReader;
#[doc = "Field `spi_dma_wait` writer - "]
pub type SPI_DMA_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
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
    pub const fn variant(&self) -> SPI_ACK_M_A {
        match self.bits {
            false => SPI_ACK_M_A::IGNORE,
            true => SPI_ACK_M_A::AFTER_DETECT,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_ignore(&self) -> bool {
        *self == SPI_ACK_M_A::IGNORE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_after_detect(&self) -> bool {
        *self == SPI_ACK_M_A::AFTER_DETECT
    }
}
#[doc = "Field `spi_ack_m` writer - SPI NDMA Acknowledge Mode"]
pub type SPI_ACK_M_W<'a, REG> = crate::BitWriter<'a, REG, SPI_ACK_M_A>;
impl<'a, REG> SPI_ACK_M_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ignore(self) -> &'a mut crate::W<REG> {
        self.variant(SPI_ACK_M_A::IGNORE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn after_detect(self) -> &'a mut crate::W<REG> {
        self.variant(SPI_ACK_M_A::AFTER_DETECT)
    }
}
#[doc = "Field `spi_act_m` reader - SPI NDMA Active Mode"]
pub type SPI_ACT_M_R = crate::FieldReader<SPI_ACT_M_A>;
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
impl crate::FieldSpec for SPI_ACT_M_A {
    type Ux = u8;
}
impl SPI_ACT_M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPI_ACT_M_A {
        match self.bits {
            0 => SPI_ACT_M_A::LOW,
            1 => SPI_ACT_M_A::HIGH,
            2 => SPI_ACT_M_A::DRQ_CONTROL,
            3 => SPI_ACT_M_A::CONTROLLER_CONTROL,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SPI_ACT_M_A::LOW
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SPI_ACT_M_A::HIGH
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_drq_control(&self) -> bool {
        *self == SPI_ACT_M_A::DRQ_CONTROL
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_controller_control(&self) -> bool {
        *self == SPI_ACT_M_A::CONTROLLER_CONTROL
    }
}
#[doc = "Field `spi_act_m` writer - SPI NDMA Active Mode"]
pub type SPI_ACT_M_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SPI_ACT_M_A>;
impl<'a, REG> SPI_ACT_M_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(SPI_ACT_M_A::LOW)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(SPI_ACT_M_A::HIGH)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn drq_control(self) -> &'a mut crate::W<REG> {
        self.variant(SPI_ACT_M_A::DRQ_CONTROL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn controller_control(self) -> &'a mut crate::W<REG> {
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
    pub fn spi_dma_wait(&mut self) -> SPI_DMA_WAIT_W<SPI_NDMA_MODE_CTL_SPEC> {
        SPI_DMA_WAIT_W::new(self, 0)
    }
    #[doc = "Bit 5 - SPI NDMA Acknowledge Mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi_ack_m(&mut self) -> SPI_ACK_M_W<SPI_NDMA_MODE_CTL_SPEC> {
        SPI_ACK_M_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - SPI NDMA Active Mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi_act_m(&mut self) -> SPI_ACT_M_W<SPI_NDMA_MODE_CTL_SPEC> {
        SPI_ACT_M_W::new(self, 6)
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
#[doc = "SPI Normal DMA Mode Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_ndma_mode_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_ndma_mode_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_NDMA_MODE_CTL_SPEC;
impl crate::RegisterSpec for SPI_NDMA_MODE_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_ndma_mode_ctl::R`](R) reader structure"]
impl crate::Readable for SPI_NDMA_MODE_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_ndma_mode_ctl::W`](W) writer structure"]
impl crate::Writable for SPI_NDMA_MODE_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets spi_ndma_mode_ctl to value 0"]
impl crate::Resettable for SPI_NDMA_MODE_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
