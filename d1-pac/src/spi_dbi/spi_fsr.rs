#[doc = "Register `SPI_FSR` reader"]
pub struct R(crate::R<SPI_FSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_FSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_FSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_FSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `tb_wr` reader - TXFIFO Write Buffer Write Enable"]
pub struct TB_WR_R(crate::FieldReader<bool>);
impl TB_WR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TB_WR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TB_WR_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tb_cnt` reader - TXFIFO Write Buffer Counter"]
pub struct TB_CNT_R(crate::FieldReader<u8>);
impl TB_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TB_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TB_CNT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tf_cnt` reader - TXFIFO Counter\n\nThese bits indicate the number of bytes in TXFIFO"]
pub struct TF_CNT_R(crate::FieldReader<u8>);
impl TF_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TF_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TF_CNT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rb_wr` reader - RXFIFO Write Buffer Write Enable"]
pub struct RB_WR_R(crate::FieldReader<bool>);
impl RB_WR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_WR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_WR_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rb_cnt` reader - RXFIFO Write Buffer Counter"]
pub struct RB_CNT_R(crate::FieldReader<u8>);
impl RB_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RB_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_CNT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_cnt` reader - RXFIFO Counter\n\nThese bits indicate the number of bytes in RXFIFO"]
pub struct RF_CNT_R(crate::FieldReader<u8>);
impl RF_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RF_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_CNT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 31 - TXFIFO Write Buffer Write Enable"]
    #[inline(always)]
    pub fn tb_wr(&self) -> TB_WR_R {
        TB_WR_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bits 28:30 - TXFIFO Write Buffer Counter"]
    #[inline(always)]
    pub fn tb_cnt(&self) -> TB_CNT_R {
        TB_CNT_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bits 16:23 - TXFIFO Counter\n\nThese bits indicate the number of bytes in TXFIFO"]
    #[inline(always)]
    pub fn tf_cnt(&self) -> TF_CNT_R {
        TF_CNT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 15 - RXFIFO Write Buffer Write Enable"]
    #[inline(always)]
    pub fn rb_wr(&self) -> RB_WR_R {
        RB_WR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 12:14 - RXFIFO Write Buffer Counter"]
    #[inline(always)]
    pub fn rb_cnt(&self) -> RB_CNT_R {
        RB_CNT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 0:7 - RXFIFO Counter\n\nThese bits indicate the number of bytes in RXFIFO"]
    #[inline(always)]
    pub fn rf_cnt(&self) -> RF_CNT_R {
        RF_CNT_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "SPI FIFO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_fsr](index.html) module"]
pub struct SPI_FSR_SPEC;
impl crate::RegisterSpec for SPI_FSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_fsr::R](R) reader structure"]
impl crate::Readable for SPI_FSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPI_FSR to value 0"]
impl crate::Resettable for SPI_FSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
