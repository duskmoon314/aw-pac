#[doc = "Register `dbi_debug_0` reader"]
pub type R = crate::R<DBI_DEBUG_0_SPEC>;
#[doc = "Field `mem_cs` reader - "]
pub type MEM_CS_R = crate::FieldReader;
#[doc = "Field `dbi_txcs` reader - "]
pub type DBI_TXCS_R = crate::FieldReader;
#[doc = "Field `sh_cs` reader - "]
pub type SH_CS_R = crate::FieldReader;
#[doc = "Field `dbi_rxcs` reader - "]
pub type DBI_RXCS_R = crate::FieldReader;
#[doc = "Field `te_val` reader - "]
pub type TE_VAL_R = crate::BitReader;
#[doc = "Field `dbi_fifo_avail` reader - "]
pub type DBI_FIFO_AVAIL_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn mem_cs(&self) -> MEM_CS_R {
        MEM_CS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn dbi_txcs(&self) -> DBI_TXCS_R {
        DBI_TXCS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn sh_cs(&self) -> SH_CS_R {
        SH_CS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn dbi_rxcs(&self) -> DBI_RXCS_R {
        DBI_RXCS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn te_val(&self) -> TE_VAL_R {
        TE_VAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn dbi_fifo_avail(&self) -> DBI_FIFO_AVAIL_R {
        DBI_FIFO_AVAIL_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
#[doc = "DBI BEBUG 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbi_debug_0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBI_DEBUG_0_SPEC;
impl crate::RegisterSpec for DBI_DEBUG_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbi_debug_0::R`](R) reader structure"]
impl crate::Readable for DBI_DEBUG_0_SPEC {}
#[doc = "`reset()` method sets dbi_debug_0 to value 0"]
impl crate::Resettable for DBI_DEBUG_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
