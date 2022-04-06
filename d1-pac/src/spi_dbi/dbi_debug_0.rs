#[doc = "Register `DBI_DEBUG_0` reader"]
pub struct R(crate::R<DBI_DEBUG_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBI_DEBUG_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBI_DEBUG_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBI_DEBUG_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `dbi_fifo_avail` reader - "]
pub struct DBI_FIFO_AVAIL_R(crate::FieldReader<u8, u8>);
impl DBI_FIFO_AVAIL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DBI_FIFO_AVAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBI_FIFO_AVAIL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `te_val` reader - "]
pub struct TE_VAL_R(crate::FieldReader<bool, bool>);
impl TE_VAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TE_VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TE_VAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dbi_rxcs` reader - "]
pub struct DBI_RXCS_R(crate::FieldReader<u8, u8>);
impl DBI_RXCS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DBI_RXCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBI_RXCS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sh_cs` reader - "]
pub struct SH_CS_R(crate::FieldReader<u8, u8>);
impl SH_CS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SH_CS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SH_CS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dbi_txcs` reader - "]
pub struct DBI_TXCS_R(crate::FieldReader<u8, u8>);
impl DBI_TXCS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DBI_TXCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBI_TXCS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `mem_cs` reader - "]
pub struct MEM_CS_R(crate::FieldReader<u8, u8>);
impl MEM_CS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MEM_CS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_CS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn dbi_fifo_avail(&self) -> DBI_FIFO_AVAIL_R {
        DBI_FIFO_AVAIL_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn te_val(&self) -> TE_VAL_R {
        TE_VAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn dbi_rxcs(&self) -> DBI_RXCS_R {
        DBI_RXCS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn sh_cs(&self) -> SH_CS_R {
        SH_CS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn dbi_txcs(&self) -> DBI_TXCS_R {
        DBI_TXCS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn mem_cs(&self) -> MEM_CS_R {
        MEM_CS_R::new((self.bits & 3) as u8)
    }
}
#[doc = "DBI BEBUG 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbi_debug_0](index.html) module"]
pub struct DBI_DEBUG_0_SPEC;
impl crate::RegisterSpec for DBI_DEBUG_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbi_debug_0::R](R) reader structure"]
impl crate::Readable for DBI_DEBUG_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DBI_DEBUG_0 to value 0"]
impl crate::Resettable for DBI_DEBUG_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
