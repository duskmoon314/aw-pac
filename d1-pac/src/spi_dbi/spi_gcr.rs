#[doc = "Register `spi_gcr` reader"]
pub type R = crate::R<SPI_GCR_SPEC>;
#[doc = "Register `spi_gcr` writer"]
pub type W = crate::W<SPI_GCR_SPEC>;
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
    pub const fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::DISABLE,
            true => EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EN_A::ENABLE
    }
}
#[doc = "Field `en` writer - SPI Module Enable Control"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG, EN_A>;
impl<'a, REG> EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::SLAVE,
            true => MODE_A::MASTER,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == MODE_A::SLAVE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == MODE_A::MASTER
    }
}
#[doc = "Field `mode` writer - SPI Function Mode Select"]
pub type MODE_W<'a, REG> = crate::BitWriter<'a, REG, MODE_A>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn slave(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::SLAVE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn master(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> MODE_SELEC_A {
        match self.bits {
            false => MODE_SELEC_A::OLD_MODE,
            true => MODE_SELEC_A::NEW_MODE,
        }
    }
    #[doc = "Old mode of Sample Timing"]
    #[inline(always)]
    pub fn is_old_mode(&self) -> bool {
        *self == MODE_SELEC_A::OLD_MODE
    }
    #[doc = "New mode of Sample Timing"]
    #[inline(always)]
    pub fn is_new_mode(&self) -> bool {
        *self == MODE_SELEC_A::NEW_MODE
    }
}
#[doc = "Field `mode_selec` writer - Sample timing Mode Select"]
pub type MODE_SELEC_W<'a, REG> = crate::BitWriter<'a, REG, MODE_SELEC_A>;
impl<'a, REG> MODE_SELEC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Old mode of Sample Timing"]
    #[inline(always)]
    pub fn old_mode(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_SELEC_A::OLD_MODE)
    }
    #[doc = "New mode of Sample Timing"]
    #[inline(always)]
    pub fn new_mode(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> TP_EN_A {
        match self.bits {
            false => TP_EN_A::NORMAL,
            true => TP_EN_A::STOP_WHEN_FULL,
        }
    }
    #[doc = "normal operation, ignore RXFIFO status"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == TP_EN_A::NORMAL
    }
    #[doc = "Stop transmit data when RXFIFO full"]
    #[inline(always)]
    pub fn is_stop_when_full(&self) -> bool {
        *self == TP_EN_A::STOP_WHEN_FULL
    }
}
#[doc = "Field `tp_en` writer - Transmit Pause Enable"]
pub type TP_EN_W<'a, REG> = crate::BitWriter<'a, REG, TP_EN_A>;
impl<'a, REG> TP_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation, ignore RXFIFO status"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(TP_EN_A::NORMAL)
    }
    #[doc = "Stop transmit data when RXFIFO full"]
    #[inline(always)]
    pub fn stop_when_full(self) -> &'a mut crate::W<REG> {
        self.variant(TP_EN_A::STOP_WHEN_FULL)
    }
}
#[doc = "Field `srst` reader - Soft reset"]
pub type SRST_R = crate::BitReader;
#[doc = "Field `srst` writer - Soft reset"]
pub type SRST_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn en(&mut self) -> EN_W<SPI_GCR_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - SPI Function Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<SPI_GCR_SPEC> {
        MODE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Sample timing Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn mode_selec(&mut self) -> MODE_SELEC_W<SPI_GCR_SPEC> {
        MODE_SELEC_W::new(self, 2)
    }
    #[doc = "Bit 7 - Transmit Pause Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tp_en(&mut self) -> TP_EN_W<SPI_GCR_SPEC> {
        TP_EN_W::new(self, 7)
    }
    #[doc = "Bit 31 - Soft reset"]
    #[inline(always)]
    #[must_use]
    pub fn srst(&mut self) -> SRST_W<SPI_GCR_SPEC> {
        SRST_W::new(self, 31)
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
#[doc = "SPI Global Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_gcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_gcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_GCR_SPEC;
impl crate::RegisterSpec for SPI_GCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_gcr::R`](R) reader structure"]
impl crate::Readable for SPI_GCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_gcr::W`](W) writer structure"]
impl crate::Writable for SPI_GCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets spi_gcr to value 0"]
impl crate::Resettable for SPI_GCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
